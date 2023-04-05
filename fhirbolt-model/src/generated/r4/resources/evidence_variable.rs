// Generated on 2023-04-05 by fhirbolt-codegen v0.1.0
#[doc = "Define members of the evidence element using Codes (such as condition, medication, or observation), Expressions ( using an expression language such as FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the last year)."]
#[derive(Debug, Clone, PartialEq)]
pub enum EvidenceVariableCharacteristicDefinition {
    Reference(Box<super::super::types::Reference>),
    Canonical(Box<super::super::types::Canonical>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Expression(Box<super::super::types::Expression>),
    DataRequirement(Box<super::super::types::DataRequirement>),
    TriggerDefinition(Box<super::super::types::TriggerDefinition>),
    Invalid,
}
impl Default for EvidenceVariableCharacteristicDefinition {
    fn default() -> EvidenceVariableCharacteristicDefinition {
        EvidenceVariableCharacteristicDefinition::Invalid
    }
}
#[doc = "Indicates what effective period the study covers."]
#[derive(Debug, Clone, PartialEq)]
pub enum EvidenceVariableCharacteristicParticipantEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for EvidenceVariableCharacteristicParticipantEffective {
    fn default() -> EvidenceVariableCharacteristicParticipantEffective {
        EvidenceVariableCharacteristicParticipantEffective::Invalid
    }
}
#[doc = "A characteristic that defines the members of the evidence element. Multiple characteristics are applied with \"and\" semantics."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EvidenceVariableCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A short, natural language description of the characteristic that could be used to communicate the criteria to an end-user."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Define members of the evidence element using Codes (such as condition, medication, or observation), Expressions ( using an expression language such as FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the last year)."]
    pub r#definition: EvidenceVariableCharacteristicDefinition,
    #[doc = "Use UsageContext to define the members of the population, such as Age Ranges, Genders, Settings."]
    pub r#usage_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "When true, members with this characteristic are excluded from the element."]
    pub r#exclude: Option<super::super::types::Boolean>,
    #[doc = "Indicates what effective period the study covers."]
    pub r#participant_effective: Option<EvidenceVariableCharacteristicParticipantEffective>,
    #[doc = "Indicates duration from the participant's study entry."]
    pub r#time_from_start: Option<Box<super::super::types::Duration>>,
    #[doc = "Indicates how elements are aggregated within the study effective period."]
    pub r#group_measure: Option<super::super::types::Code>,
}
impl serde::ser::Serialize for EvidenceVariableCharacteristic {
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
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            match self.r#definition {
                EvidenceVariableCharacteristicDefinition::Reference(ref value) => {
                    state.serialize_entry("definitionReference", value)?;
                }
                EvidenceVariableCharacteristicDefinition::Canonical(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("definitionCanonical", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_definitionCanonical", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("definitionCanonical", value)?;
                    }
                }
                EvidenceVariableCharacteristicDefinition::CodeableConcept(ref value) => {
                    state.serialize_entry("definitionCodeableConcept", value)?;
                }
                EvidenceVariableCharacteristicDefinition::Expression(ref value) => {
                    state.serialize_entry("definitionExpression", value)?;
                }
                EvidenceVariableCharacteristicDefinition::DataRequirement(ref value) => {
                    state.serialize_entry("definitionDataRequirement", value)?;
                }
                EvidenceVariableCharacteristicDefinition::TriggerDefinition(ref value) => {
                    state.serialize_entry("definitionTriggerDefinition", value)?;
                }
                EvidenceVariableCharacteristicDefinition::Invalid => {
                    return Err(serde::ser::Error::custom("definition is a required field"))
                }
            }
            if !self.r#usage_context.is_empty() {
                state.serialize_entry("usageContext", &self.r#usage_context)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#exclude.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("exclude", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_exclude", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#exclude.as_ref() {
                    state.serialize_entry("exclude", some)?;
                }
            }
            if let Some(some) = self.r#participant_effective.as_ref() {
                match some {
                    EvidenceVariableCharacteristicParticipantEffective::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("participantEffectiveDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry(
                                    "_participantEffectiveDateTime",
                                    &primitive_element,
                                )?;
                            }
                        } else {
                            state.serialize_entry("participantEffectiveDateTime", value)?;
                        }
                    }
                    EvidenceVariableCharacteristicParticipantEffective::Period(ref value) => {
                        state.serialize_entry("participantEffectivePeriod", value)?;
                    }
                    EvidenceVariableCharacteristicParticipantEffective::Duration(ref value) => {
                        state.serialize_entry("participantEffectiveDuration", value)?;
                    }
                    EvidenceVariableCharacteristicParticipantEffective::Timing(ref value) => {
                        state.serialize_entry("participantEffectiveTiming", value)?;
                    }
                    EvidenceVariableCharacteristicParticipantEffective::Invalid => {
                        return Err(serde::ser::Error::custom(
                            "participant_effective is invalid",
                        ))
                    }
                }
            }
            if let Some(some) = self.r#time_from_start.as_ref() {
                state.serialize_entry("timeFromStart", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#group_measure.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("groupMeasure", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_groupMeasure", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#group_measure.as_ref() {
                    state.serialize_entry("groupMeasure", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceVariableCharacteristic {
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
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "definitionReference")]
            DefinitionReference,
            #[serde(rename = "definitionCanonical")]
            DefinitionCanonical,
            #[serde(rename = "_definitionCanonical")]
            DefinitionCanonicalPrimitiveElement,
            #[serde(rename = "definitionCodeableConcept")]
            DefinitionCodeableConcept,
            #[serde(rename = "definitionExpression")]
            DefinitionExpression,
            #[serde(rename = "definitionDataRequirement")]
            DefinitionDataRequirement,
            #[serde(rename = "definitionTriggerDefinition")]
            DefinitionTriggerDefinition,
            #[serde(rename = "usageContext")]
            UsageContext,
            #[serde(rename = "exclude")]
            Exclude,
            #[serde(rename = "_exclude")]
            ExcludePrimitiveElement,
            #[serde(rename = "participantEffectiveDateTime")]
            ParticipantEffectiveDateTime,
            #[serde(rename = "_participantEffectiveDateTime")]
            ParticipantEffectiveDateTimePrimitiveElement,
            #[serde(rename = "participantEffectivePeriod")]
            ParticipantEffectivePeriod,
            #[serde(rename = "participantEffectiveDuration")]
            ParticipantEffectiveDuration,
            #[serde(rename = "participantEffectiveTiming")]
            ParticipantEffectiveTiming,
            #[serde(rename = "timeFromStart")]
            TimeFromStart,
            #[serde(rename = "groupMeasure")]
            GroupMeasure,
            #[serde(rename = "_groupMeasure")]
            GroupMeasurePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceVariableCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceVariableCharacteristic")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<EvidenceVariableCharacteristic, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#definition: Option<EvidenceVariableCharacteristicDefinition> = None;
                let mut r#usage_context: Option<Vec<Box<super::super::types::UsageContext>>> = None;
                let mut r#exclude: Option<super::super::types::Boolean> = None;
                let mut r#participant_effective: Option<
                    EvidenceVariableCharacteristicParticipantEffective,
                > = None;
                let mut r#time_from_start: Option<Box<super::super::types::Duration>> = None;
                let mut r#group_measure: Option<super::super::types::Code> = None;
                fhirbolt_shared :: serde_context :: de :: DESERIALIZATION_CONTEXT . with (| _ctx | { let _ctx = _ctx . borrow () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if _ctx . from_json { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } else { let vec = r#extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value () ?) ; } } , Field :: ModifierExtension => { if _ctx . from_json { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } else { let vec = r#modifier_extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value () ?) ; } } , Field :: Description => { if _ctx . from_json { let some = r#description . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("description")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#description . is_some () { return Err (serde :: de :: Error :: duplicate_field ("description")) ; } r#description = Some (map_access . next_value () ?) ; } } , Field :: DescriptionPrimitiveElement => { if _ctx . from_json { let some = r#description . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_description")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("description" , & ["id" , "extension" , "modifierExtension" , "description" , "definitionReference" , "definitionCanonical" , "definitionCodeableConcept" , "definitionExpression" , "definitionDataRequirement" , "definitionTriggerDefinition" , "usageContext" , "exclude" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "timeFromStart" , "groupMeasure" ,])) ; } } , Field :: DefinitionReference => { if r#definition . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionReference")) ; } r#definition = Some (EvidenceVariableCharacteristicDefinition :: Reference (map_access . next_value () ?)) ; } , Field :: DefinitionCanonical => { if _ctx . from_json { let r#enum = r#definition . get_or_insert (EvidenceVariableCharacteristicDefinition :: Canonical (Default :: default ())) ; if let EvidenceVariableCharacteristicDefinition :: Canonical (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionCanonical")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("definition[x]")) ; } } else { if r#definition . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionCanonical")) ; } r#definition = Some (EvidenceVariableCharacteristicDefinition :: Canonical (map_access . next_value () ?)) ; } } , Field :: DefinitionCanonicalPrimitiveElement => { if _ctx . from_json { let r#enum = r#definition . get_or_insert (EvidenceVariableCharacteristicDefinition :: Canonical (Default :: default ())) ; if let EvidenceVariableCharacteristicDefinition :: Canonical (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_definitionCanonical")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_definition[x]")) ; } } else { return Err (serde :: de :: Error :: unknown_field ("definitionCanonical" , & ["id" , "extension" , "modifierExtension" , "description" , "definitionReference" , "definitionCanonical" , "definitionCodeableConcept" , "definitionExpression" , "definitionDataRequirement" , "definitionTriggerDefinition" , "usageContext" , "exclude" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "timeFromStart" , "groupMeasure" ,])) ; } } , Field :: DefinitionCodeableConcept => { if r#definition . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionCodeableConcept")) ; } r#definition = Some (EvidenceVariableCharacteristicDefinition :: CodeableConcept (map_access . next_value () ?)) ; } , Field :: DefinitionExpression => { if r#definition . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionExpression")) ; } r#definition = Some (EvidenceVariableCharacteristicDefinition :: Expression (map_access . next_value () ?)) ; } , Field :: DefinitionDataRequirement => { if r#definition . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionDataRequirement")) ; } r#definition = Some (EvidenceVariableCharacteristicDefinition :: DataRequirement (map_access . next_value () ?)) ; } , Field :: DefinitionTriggerDefinition => { if r#definition . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionTriggerDefinition")) ; } r#definition = Some (EvidenceVariableCharacteristicDefinition :: TriggerDefinition (map_access . next_value () ?)) ; } , Field :: UsageContext => { if _ctx . from_json { if r#usage_context . is_some () { return Err (serde :: de :: Error :: duplicate_field ("usageContext")) ; } r#usage_context = Some (map_access . next_value () ?) ; } else { let vec = r#usage_context . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value () ?) ; } } , Field :: Exclude => { if _ctx . from_json { let some = r#exclude . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("exclude")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#exclude . is_some () { return Err (serde :: de :: Error :: duplicate_field ("exclude")) ; } r#exclude = Some (map_access . next_value () ?) ; } } , Field :: ExcludePrimitiveElement => { if _ctx . from_json { let some = r#exclude . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_exclude")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("exclude" , & ["id" , "extension" , "modifierExtension" , "description" , "definitionReference" , "definitionCanonical" , "definitionCodeableConcept" , "definitionExpression" , "definitionDataRequirement" , "definitionTriggerDefinition" , "usageContext" , "exclude" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "timeFromStart" , "groupMeasure" ,])) ; } } , Field :: ParticipantEffectiveDateTime => { if _ctx . from_json { let r#enum = r#participant_effective . get_or_insert (EvidenceVariableCharacteristicParticipantEffective :: DateTime (Default :: default ())) ; if let EvidenceVariableCharacteristicParticipantEffective :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("participantEffective[x]")) ; } } else { if r#participant_effective . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveDateTime")) ; } r#participant_effective = Some (EvidenceVariableCharacteristicParticipantEffective :: DateTime (map_access . next_value () ?)) ; } } , Field :: ParticipantEffectiveDateTimePrimitiveElement => { if _ctx . from_json { let r#enum = r#participant_effective . get_or_insert (EvidenceVariableCharacteristicParticipantEffective :: DateTime (Default :: default ())) ; if let EvidenceVariableCharacteristicParticipantEffective :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_participantEffectiveDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_participantEffective[x]")) ; } } else { return Err (serde :: de :: Error :: unknown_field ("participantEffectiveDateTime" , & ["id" , "extension" , "modifierExtension" , "description" , "definitionReference" , "definitionCanonical" , "definitionCodeableConcept" , "definitionExpression" , "definitionDataRequirement" , "definitionTriggerDefinition" , "usageContext" , "exclude" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "timeFromStart" , "groupMeasure" ,])) ; } } , Field :: ParticipantEffectivePeriod => { if r#participant_effective . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectivePeriod")) ; } r#participant_effective = Some (EvidenceVariableCharacteristicParticipantEffective :: Period (map_access . next_value () ?)) ; } , Field :: ParticipantEffectiveDuration => { if r#participant_effective . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveDuration")) ; } r#participant_effective = Some (EvidenceVariableCharacteristicParticipantEffective :: Duration (map_access . next_value () ?)) ; } , Field :: ParticipantEffectiveTiming => { if r#participant_effective . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveTiming")) ; } r#participant_effective = Some (EvidenceVariableCharacteristicParticipantEffective :: Timing (map_access . next_value () ?)) ; } , Field :: TimeFromStart => { if r#time_from_start . is_some () { return Err (serde :: de :: Error :: duplicate_field ("timeFromStart")) ; } r#time_from_start = Some (map_access . next_value () ?) ; } , Field :: GroupMeasure => { if _ctx . from_json { let some = r#group_measure . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("groupMeasure")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#group_measure . is_some () { return Err (serde :: de :: Error :: duplicate_field ("groupMeasure")) ; } r#group_measure = Some (map_access . next_value () ?) ; } } , Field :: GroupMeasurePrimitiveElement => { if _ctx . from_json { let some = r#group_measure . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_groupMeasure")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("groupMeasure" , & ["id" , "extension" , "modifierExtension" , "description" , "definitionReference" , "definitionCanonical" , "definitionCodeableConcept" , "definitionExpression" , "definitionDataRequirement" , "definitionTriggerDefinition" , "usageContext" , "exclude" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "timeFromStart" , "groupMeasure" ,])) ; } } , Field :: Unknown (key) => if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "description" , "definitionReference" , "definitionCanonical" , "definitionCodeableConcept" , "definitionExpression" , "definitionDataRequirement" , "definitionTriggerDefinition" , "usageContext" , "exclude" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "timeFromStart" , "groupMeasure" ,])) ; } } } Ok (EvidenceVariableCharacteristic { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#description , r#definition : if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Lax { r#definition . unwrap_or (Default :: default ()) } else { r#definition . ok_or (serde :: de :: Error :: missing_field ("definition[x]")) ? } , r#usage_context : r#usage_context . unwrap_or (vec ! []) , r#exclude , r#participant_effective , r#time_from_start , r#group_measure , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The EvidenceVariable resource describes a \"PICO\" element that knowledge (evidence, assertion, recommendation) is about.\n\nNeed to be able to define and reuse the definition of individual elements of a research question."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EvidenceVariable {
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
    #[doc = "An absolute URI that is used to identify this evidence variable when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this evidence variable is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the evidence variable is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this evidence variable when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the evidence variable when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the evidence variable author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence. To provide a version consistent with the Decision Support Service specification, use the format Major.Minor.Revision (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the Decision Support Service specification. Note that a version is required for non-experimental active artifacts."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the evidence variable. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the evidence variable."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The short title provides an alternate title for use in informal descriptive contexts where the full, formal title is not necessary."]
    pub r#short_title: Option<super::super::types::String>,
    #[doc = "An explanatory or alternate title for the EvidenceVariable giving additional information about its content."]
    pub r#subtitle: Option<super::super::types::String>,
    #[doc = "The status of this evidence variable. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "The date  (and optionally time) when the evidence variable was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the evidence variable changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the evidence variable."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the evidence variable from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A human-readable string to clarify or explain concepts about the resource."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate evidence variable instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the evidence variable is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A copyright statement relating to the evidence variable and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the evidence variable."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the evidence variable content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Descriptive topics related to the content of the EvidenceVariable. Topics provide a high-level categorization grouping types of EvidenceVariables that can be useful for filtering and searching."]
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
    #[doc = "The type of evidence element, a population, an exposure, or an outcome."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "A characteristic that defines the members of the evidence element. Multiple characteristics are applied with \"and\" semantics."]
    pub r#characteristic: Vec<EvidenceVariableCharacteristic>,
}
impl crate::AnyResource for EvidenceVariable {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for EvidenceVariable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "EvidenceVariable")?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#url.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("url", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_url", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#url.as_ref() {
                    state.serialize_entry("url", some)?;
                }
            }
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#version.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("version", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_version", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#version.as_ref() {
                    state.serialize_entry("version", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("name", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_name", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#name.as_ref() {
                    state.serialize_entry("name", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("title", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_title", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#title.as_ref() {
                    state.serialize_entry("title", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#short_title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("shortTitle", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_shortTitle", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#short_title.as_ref() {
                    state.serialize_entry("shortTitle", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#subtitle.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("subtitle", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_subtitle", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#subtitle.as_ref() {
                    state.serialize_entry("subtitle", some)?;
                }
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
            if _ctx.output_json {
                if let Some(some) = self.r#date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("date", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_date", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#date.as_ref() {
                    state.serialize_entry("date", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#publisher.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("publisher", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_publisher", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#publisher.as_ref() {
                    state.serialize_entry("publisher", some)?;
                }
            }
            if !self.r#contact.is_empty() {
                state.serialize_entry("contact", &self.r#contact)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#use_context.is_empty() {
                state.serialize_entry("useContext", &self.r#use_context)?;
            }
            if !self.r#jurisdiction.is_empty() {
                state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#copyright.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("copyright", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_copyright", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#copyright.as_ref() {
                    state.serialize_entry("copyright", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#approval_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("approvalDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_approvalDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#approval_date.as_ref() {
                    state.serialize_entry("approvalDate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#last_review_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("lastReviewDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lastReviewDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#last_review_date.as_ref() {
                    state.serialize_entry("lastReviewDate", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("type", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_type", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#type.as_ref() {
                    state.serialize_entry("type", some)?;
                }
            }
            if !self.r#characteristic.is_empty() {
                state.serialize_entry("characteristic", &self.r#characteristic)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceVariable {
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
            #[serde(rename = "shortTitle")]
            ShortTitle,
            #[serde(rename = "_shortTitle")]
            ShortTitlePrimitiveElement,
            #[serde(rename = "subtitle")]
            Subtitle,
            #[serde(rename = "_subtitle")]
            SubtitlePrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
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
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "useContext")]
            UseContext,
            #[serde(rename = "jurisdiction")]
            Jurisdiction,
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "characteristic")]
            Characteristic,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceVariable;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceVariable")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EvidenceVariable, V::Error>
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
                let mut r#short_title: Option<super::super::types::String> = None;
                let mut r#subtitle: Option<super::super::types::String> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#publisher: Option<super::super::types::String> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#use_context: Option<Vec<Box<super::super::types::UsageContext>>> = None;
                let mut r#jurisdiction: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
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
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#characteristic: Option<Vec<EvidenceVariableCharacteristic>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "EvidenceVariable" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"EvidenceVariable",
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
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
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
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
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
                                if _ctx.from_json {
                                    if r#contained.is_some() {
                                        return Err(serde::de::Error::duplicate_field("contained"));
                                    }
                                    r#contained = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#contained.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Url => {
                                if _ctx.from_json {
                                    let some = r#url.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#url.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    r#url = Some(map_access.next_value()?);
                                }
                            }
                            Field::UrlPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "url",
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Identifier => {
                                if _ctx.from_json {
                                    if r#identifier.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "identifier",
                                        ));
                                    }
                                    r#identifier = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#identifier.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Version => {
                                if _ctx.from_json {
                                    let some = r#version.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#version.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    r#version = Some(map_access.next_value()?);
                                }
                            }
                            Field::VersionPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "version",
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Title => {
                                if _ctx.from_json {
                                    let some = r#title.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("title"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#title.is_some() {
                                        return Err(serde::de::Error::duplicate_field("title"));
                                    }
                                    r#title = Some(map_access.next_value()?);
                                }
                            }
                            Field::TitlePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "title",
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::ShortTitle => {
                                if _ctx.from_json {
                                    let some = r#short_title.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "shortTitle",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#short_title.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "shortTitle",
                                        ));
                                    }
                                    r#short_title = Some(map_access.next_value()?);
                                }
                            }
                            Field::ShortTitlePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#short_title.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_shortTitle",
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
                                        "shortTitle",
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Subtitle => {
                                if _ctx.from_json {
                                    let some = r#subtitle.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("subtitle"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#subtitle.is_some() {
                                        return Err(serde::de::Error::duplicate_field("subtitle"));
                                    }
                                    r#subtitle = Some(map_access.next_value()?);
                                }
                            }
                            Field::SubtitlePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "subtitle",
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
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
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Date => {
                                if _ctx.from_json {
                                    let some = r#date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("date"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#date.is_some() {
                                        return Err(serde::de::Error::duplicate_field("date"));
                                    }
                                    r#date = Some(map_access.next_value()?);
                                }
                            }
                            Field::DatePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "date",
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Publisher => {
                                if _ctx.from_json {
                                    let some = r#publisher.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("publisher"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#publisher.is_some() {
                                        return Err(serde::de::Error::duplicate_field("publisher"));
                                    }
                                    r#publisher = Some(map_access.next_value()?);
                                }
                            }
                            Field::PublisherPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#publisher.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_publisher",
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
                                        "publisher",
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Contact => {
                                if _ctx.from_json {
                                    if r#contact.is_some() {
                                        return Err(serde::de::Error::duplicate_field("contact"));
                                    }
                                    r#contact = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#contact.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
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
                                        "description",
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Note => {
                                if _ctx.from_json {
                                    if r#note.is_some() {
                                        return Err(serde::de::Error::duplicate_field("note"));
                                    }
                                    r#note = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#note.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::UseContext => {
                                if _ctx.from_json {
                                    if r#use_context.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "useContext",
                                        ));
                                    }
                                    r#use_context = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#use_context.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Jurisdiction => {
                                if _ctx.from_json {
                                    if r#jurisdiction.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "jurisdiction",
                                        ));
                                    }
                                    r#jurisdiction = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#jurisdiction.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Copyright => {
                                if _ctx.from_json {
                                    let some = r#copyright.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("copyright"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#copyright.is_some() {
                                        return Err(serde::de::Error::duplicate_field("copyright"));
                                    }
                                    r#copyright = Some(map_access.next_value()?);
                                }
                            }
                            Field::CopyrightPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#copyright.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_copyright",
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
                                        "copyright",
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::ApprovalDate => {
                                if _ctx.from_json {
                                    let some = r#approval_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "approvalDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#approval_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "approvalDate",
                                        ));
                                    }
                                    r#approval_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::ApprovalDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#approval_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_approvalDate",
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
                                        "approvalDate",
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::LastReviewDate => {
                                if _ctx.from_json {
                                    let some = r#last_review_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lastReviewDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#last_review_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lastReviewDate",
                                        ));
                                    }
                                    r#last_review_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::LastReviewDatePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "lastReviewDate",
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
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
                                if _ctx.from_json {
                                    if r#topic.is_some() {
                                        return Err(serde::de::Error::duplicate_field("topic"));
                                    }
                                    r#topic = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#topic.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Author => {
                                if _ctx.from_json {
                                    if r#author.is_some() {
                                        return Err(serde::de::Error::duplicate_field("author"));
                                    }
                                    r#author = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#author.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Editor => {
                                if _ctx.from_json {
                                    if r#editor.is_some() {
                                        return Err(serde::de::Error::duplicate_field("editor"));
                                    }
                                    r#editor = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#editor.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Reviewer => {
                                if _ctx.from_json {
                                    if r#reviewer.is_some() {
                                        return Err(serde::de::Error::duplicate_field("reviewer"));
                                    }
                                    r#reviewer = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#reviewer.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Endorser => {
                                if _ctx.from_json {
                                    if r#endorser.is_some() {
                                        return Err(serde::de::Error::duplicate_field("endorser"));
                                    }
                                    r#endorser = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#endorser.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::RelatedArtifact => {
                                if _ctx.from_json {
                                    if r#related_artifact.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "relatedArtifact",
                                        ));
                                    }
                                    r#related_artifact = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#related_artifact.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Type => {
                                if _ctx.from_json {
                                    let some = r#type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    r#type = Some(map_access.next_value()?);
                                }
                            }
                            Field::TypePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "type",
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
                                            "shortTitle",
                                            "subtitle",
                                            "status",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "note",
                                            "useContext",
                                            "jurisdiction",
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
                                            "type",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Characteristic => {
                                if _ctx.from_json {
                                    if r#characteristic.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "characteristic",
                                        ));
                                    }
                                    r#characteristic = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#characteristic.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                        "url",
                                        "identifier",
                                        "version",
                                        "name",
                                        "title",
                                        "shortTitle",
                                        "subtitle",
                                        "status",
                                        "date",
                                        "publisher",
                                        "contact",
                                        "description",
                                        "note",
                                        "useContext",
                                        "jurisdiction",
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
                                        "type",
                                        "characteristic",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceVariable {
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
                        r#short_title,
                        r#subtitle,
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#date,
                        r#publisher,
                        r#contact: r#contact.unwrap_or(vec![]),
                        r#description,
                        r#note: r#note.unwrap_or(vec![]),
                        r#use_context: r#use_context.unwrap_or(vec![]),
                        r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
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
                        r#type,
                        r#characteristic: r#characteristic.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
