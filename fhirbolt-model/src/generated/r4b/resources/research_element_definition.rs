// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "The intended subjects for the ResearchElementDefinition. If this element is not provided, a Patient subject is assumed, but the subject of the ResearchElementDefinition can be anything."]
#[derive(Debug, Clone)]
pub enum ResearchElementDefinitionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ResearchElementDefinitionSubject {
    fn default() -> ResearchElementDefinitionSubject {
        ResearchElementDefinitionSubject::Invalid
    }
}
#[doc = "Define members of the research element using Codes (such as condition, medication, or observation), Expressions ( using an expression language such as FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the last year)."]
#[derive(Debug, Clone)]
pub enum ResearchElementDefinitionCharacteristicDefinition {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Canonical(Box<super::super::types::Canonical>),
    Expression(Box<super::super::types::Expression>),
    DataRequirement(Box<super::super::types::DataRequirement>),
    Invalid,
}
impl Default for ResearchElementDefinitionCharacteristicDefinition {
    fn default() -> ResearchElementDefinitionCharacteristicDefinition {
        ResearchElementDefinitionCharacteristicDefinition::Invalid
    }
}
#[doc = "Indicates what effective period the study covers."]
#[derive(Debug, Clone)]
pub enum ResearchElementDefinitionCharacteristicStudyEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for ResearchElementDefinitionCharacteristicStudyEffective {
    fn default() -> ResearchElementDefinitionCharacteristicStudyEffective {
        ResearchElementDefinitionCharacteristicStudyEffective::Invalid
    }
}
#[doc = "Indicates what effective period the study covers."]
#[derive(Debug, Clone)]
pub enum ResearchElementDefinitionCharacteristicParticipantEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for ResearchElementDefinitionCharacteristicParticipantEffective {
    fn default() -> ResearchElementDefinitionCharacteristicParticipantEffective {
        ResearchElementDefinitionCharacteristicParticipantEffective::Invalid
    }
}
#[doc = "A characteristic that defines the members of the research element. Multiple characteristics are applied with \"and\" semantics."]
#[derive(Default, Debug, Clone)]
pub struct ResearchElementDefinitionCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Define members of the research element using Codes (such as condition, medication, or observation), Expressions ( using an expression language such as FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the last year)."]
    pub r#definition: ResearchElementDefinitionCharacteristicDefinition,
    #[doc = "Use UsageContext to define the members of the population, such as Age Ranges, Genders, Settings."]
    pub r#usage_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "When true, members with this characteristic are excluded from the element."]
    pub r#exclude: Option<super::super::types::Boolean>,
    #[doc = "Specifies the UCUM unit for the outcome."]
    pub r#unit_of_measure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A narrative description of the time period the study covers."]
    pub r#study_effective_description: Option<super::super::types::String>,
    #[doc = "Indicates what effective period the study covers."]
    pub r#study_effective: Option<ResearchElementDefinitionCharacteristicStudyEffective>,
    #[doc = "Indicates duration from the study initiation."]
    pub r#study_effective_time_from_start: Option<Box<super::super::types::Duration>>,
    #[doc = "Indicates how elements are aggregated within the study effective period."]
    pub r#study_effective_group_measure: Option<super::super::types::Code>,
    #[doc = "A narrative description of the time period the study covers."]
    pub r#participant_effective_description: Option<super::super::types::String>,
    #[doc = "Indicates what effective period the study covers."]
    pub r#participant_effective:
        Option<ResearchElementDefinitionCharacteristicParticipantEffective>,
    #[doc = "Indicates duration from the participant's study entry."]
    pub r#participant_effective_time_from_start: Option<Box<super::super::types::Duration>>,
    #[doc = "Indicates how elements are aggregated within the study effective period."]
    pub r#participant_effective_group_measure: Option<super::super::types::Code>,
}
impl serde::ser::Serialize for ResearchElementDefinitionCharacteristic {
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
            match self.r#definition {
                ResearchElementDefinitionCharacteristicDefinition::CodeableConcept(ref value) => {
                    state.serialize_entry("definitionCodeableConcept", value)?;
                }
                ResearchElementDefinitionCharacteristicDefinition::Canonical(ref value) => {
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
                ResearchElementDefinitionCharacteristicDefinition::Expression(ref value) => {
                    state.serialize_entry("definitionExpression", value)?;
                }
                ResearchElementDefinitionCharacteristicDefinition::DataRequirement(ref value) => {
                    state.serialize_entry("definitionDataRequirement", value)?;
                }
                ResearchElementDefinitionCharacteristicDefinition::Invalid => {
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
            if let Some(some) = self.r#unit_of_measure.as_ref() {
                state.serialize_entry("unitOfMeasure", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#study_effective_description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("studyEffectiveDescription", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_studyEffectiveDescription", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#study_effective_description.as_ref() {
                    state.serialize_entry("studyEffectiveDescription", some)?;
                }
            }
            if let Some(some) = self.r#study_effective.as_ref() {
                match some {
                    ResearchElementDefinitionCharacteristicStudyEffective::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("studyEffectiveDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry(
                                    "_studyEffectiveDateTime",
                                    &primitive_element,
                                )?;
                            }
                        } else {
                            state.serialize_entry("studyEffectiveDateTime", value)?;
                        }
                    }
                    ResearchElementDefinitionCharacteristicStudyEffective::Period(ref value) => {
                        state.serialize_entry("studyEffectivePeriod", value)?;
                    }
                    ResearchElementDefinitionCharacteristicStudyEffective::Duration(ref value) => {
                        state.serialize_entry("studyEffectiveDuration", value)?;
                    }
                    ResearchElementDefinitionCharacteristicStudyEffective::Timing(ref value) => {
                        state.serialize_entry("studyEffectiveTiming", value)?;
                    }
                    ResearchElementDefinitionCharacteristicStudyEffective::Invalid => {
                        return Err(serde::ser::Error::custom("study_effective is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#study_effective_time_from_start.as_ref() {
                state.serialize_entry("studyEffectiveTimeFromStart", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#study_effective_group_measure.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("studyEffectiveGroupMeasure", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_studyEffectiveGroupMeasure", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#study_effective_group_measure.as_ref() {
                    state.serialize_entry("studyEffectiveGroupMeasure", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#participant_effective_description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("participantEffectiveDescription", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry(
                            "_participantEffectiveDescription",
                            &primitive_element,
                        )?;
                    }
                }
            } else {
                if let Some(some) = self.r#participant_effective_description.as_ref() {
                    state.serialize_entry("participantEffectiveDescription", some)?;
                }
            }
            if let Some(some) = self.r#participant_effective.as_ref() {
                match some {
                    ResearchElementDefinitionCharacteristicParticipantEffective::DateTime(
                        ref value,
                    ) => {
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
                    ResearchElementDefinitionCharacteristicParticipantEffective::Period(
                        ref value,
                    ) => {
                        state.serialize_entry("participantEffectivePeriod", value)?;
                    }
                    ResearchElementDefinitionCharacteristicParticipantEffective::Duration(
                        ref value,
                    ) => {
                        state.serialize_entry("participantEffectiveDuration", value)?;
                    }
                    ResearchElementDefinitionCharacteristicParticipantEffective::Timing(
                        ref value,
                    ) => {
                        state.serialize_entry("participantEffectiveTiming", value)?;
                    }
                    ResearchElementDefinitionCharacteristicParticipantEffective::Invalid => {
                        return Err(serde::ser::Error::custom(
                            "participant_effective is invalid",
                        ))
                    }
                }
            }
            if let Some(some) = self.r#participant_effective_time_from_start.as_ref() {
                state.serialize_entry("participantEffectiveTimeFromStart", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#participant_effective_group_measure.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("participantEffectiveGroupMeasure", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry(
                            "_participantEffectiveGroupMeasure",
                            &primitive_element,
                        )?;
                    }
                }
            } else {
                if let Some(some) = self.r#participant_effective_group_measure.as_ref() {
                    state.serialize_entry("participantEffectiveGroupMeasure", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ResearchElementDefinitionCharacteristic {
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
            #[serde(rename = "definitionCodeableConcept")]
            DefinitionCodeableConcept,
            #[serde(rename = "definitionCanonical")]
            DefinitionCanonical,
            #[serde(rename = "_definitionCanonical")]
            DefinitionCanonicalPrimitiveElement,
            #[serde(rename = "definitionExpression")]
            DefinitionExpression,
            #[serde(rename = "definitionDataRequirement")]
            DefinitionDataRequirement,
            #[serde(rename = "usageContext")]
            UsageContext,
            #[serde(rename = "exclude")]
            Exclude,
            #[serde(rename = "_exclude")]
            ExcludePrimitiveElement,
            #[serde(rename = "unitOfMeasure")]
            UnitOfMeasure,
            #[serde(rename = "studyEffectiveDescription")]
            StudyEffectiveDescription,
            #[serde(rename = "_studyEffectiveDescription")]
            StudyEffectiveDescriptionPrimitiveElement,
            #[serde(rename = "studyEffectiveDateTime")]
            StudyEffectiveDateTime,
            #[serde(rename = "_studyEffectiveDateTime")]
            StudyEffectiveDateTimePrimitiveElement,
            #[serde(rename = "studyEffectivePeriod")]
            StudyEffectivePeriod,
            #[serde(rename = "studyEffectiveDuration")]
            StudyEffectiveDuration,
            #[serde(rename = "studyEffectiveTiming")]
            StudyEffectiveTiming,
            #[serde(rename = "studyEffectiveTimeFromStart")]
            StudyEffectiveTimeFromStart,
            #[serde(rename = "studyEffectiveGroupMeasure")]
            StudyEffectiveGroupMeasure,
            #[serde(rename = "_studyEffectiveGroupMeasure")]
            StudyEffectiveGroupMeasurePrimitiveElement,
            #[serde(rename = "participantEffectiveDescription")]
            ParticipantEffectiveDescription,
            #[serde(rename = "_participantEffectiveDescription")]
            ParticipantEffectiveDescriptionPrimitiveElement,
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
            #[serde(rename = "participantEffectiveTimeFromStart")]
            ParticipantEffectiveTimeFromStart,
            #[serde(rename = "participantEffectiveGroupMeasure")]
            ParticipantEffectiveGroupMeasure,
            #[serde(rename = "_participantEffectiveGroupMeasure")]
            ParticipantEffectiveGroupMeasurePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResearchElementDefinitionCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ResearchElementDefinitionCharacteristic")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ResearchElementDefinitionCharacteristic, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#definition: Option<ResearchElementDefinitionCharacteristicDefinition> =
                    None;
                let mut r#usage_context: Option<Vec<Box<super::super::types::UsageContext>>> = None;
                let mut r#exclude: Option<super::super::types::Boolean> = None;
                let mut r#unit_of_measure: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#study_effective_description: Option<super::super::types::String> = None;
                let mut r#study_effective: Option<
                    ResearchElementDefinitionCharacteristicStudyEffective,
                > = None;
                let mut r#study_effective_time_from_start: Option<
                    Box<super::super::types::Duration>,
                > = None;
                let mut r#study_effective_group_measure: Option<super::super::types::Code> = None;
                let mut r#participant_effective_description: Option<super::super::types::String> =
                    None;
                let mut r#participant_effective: Option<
                    ResearchElementDefinitionCharacteristicParticipantEffective,
                > = None;
                let mut r#participant_effective_time_from_start: Option<
                    Box<super::super::types::Duration>,
                > = None;
                let mut r#participant_effective_group_measure: Option<super::super::types::Code> =
                    None;
                fhirbolt_shared :: serde_context :: de :: DESERIALIZATION_CONTEXT . with (| _ctx | { let _ctx = _ctx . borrow () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } , Field :: ModifierExtension => { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } , Field :: DefinitionCodeableConcept => { if r#definition . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionCodeableConcept")) ; } r#definition = Some (ResearchElementDefinitionCharacteristicDefinition :: CodeableConcept (map_access . next_value () ?)) ; } , Field :: DefinitionCanonical => { if _ctx . from_json { let r#enum = r#definition . get_or_insert (ResearchElementDefinitionCharacteristicDefinition :: Canonical (Default :: default ())) ; if let ResearchElementDefinitionCharacteristicDefinition :: Canonical (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionCanonical")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("definition[x]")) ; } } else { if r#definition . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionCanonical")) ; } r#definition = Some (ResearchElementDefinitionCharacteristicDefinition :: Canonical (map_access . next_value () ?)) ; } } , Field :: DefinitionCanonicalPrimitiveElement => { if _ctx . from_json { let r#enum = r#definition . get_or_insert (ResearchElementDefinitionCharacteristicDefinition :: Canonical (Default :: default ())) ; if let ResearchElementDefinitionCharacteristicDefinition :: Canonical (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_definitionCanonical")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_definition[x]")) ; } } else { return Err (serde :: de :: Error :: unknown_field ("definitionCanonical" , & ["id" , "extension" , "modifierExtension" , "definitionCodeableConcept" , "definitionCanonical" , "definitionExpression" , "definitionDataRequirement" , "usageContext" , "exclude" , "unitOfMeasure" , "studyEffectiveDescription" , "studyEffectiveDateTime" , "studyEffectivePeriod" , "studyEffectiveDuration" , "studyEffectiveTiming" , "studyEffectiveTimeFromStart" , "studyEffectiveGroupMeasure" , "participantEffectiveDescription" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "participantEffectiveTimeFromStart" , "participantEffectiveGroupMeasure" ,])) ; } } , Field :: DefinitionExpression => { if r#definition . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionExpression")) ; } r#definition = Some (ResearchElementDefinitionCharacteristicDefinition :: Expression (map_access . next_value () ?)) ; } , Field :: DefinitionDataRequirement => { if r#definition . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionDataRequirement")) ; } r#definition = Some (ResearchElementDefinitionCharacteristicDefinition :: DataRequirement (map_access . next_value () ?)) ; } , Field :: UsageContext => { if r#usage_context . is_some () { return Err (serde :: de :: Error :: duplicate_field ("usageContext")) ; } r#usage_context = Some (map_access . next_value () ?) ; } , Field :: Exclude => { if _ctx . from_json { let some = r#exclude . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("exclude")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#exclude . is_some () { return Err (serde :: de :: Error :: duplicate_field ("exclude")) ; } r#exclude = Some (map_access . next_value () ?) ; } } , Field :: ExcludePrimitiveElement => { if _ctx . from_json { let some = r#exclude . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_exclude")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("exclude" , & ["id" , "extension" , "modifierExtension" , "definitionCodeableConcept" , "definitionCanonical" , "definitionExpression" , "definitionDataRequirement" , "usageContext" , "exclude" , "unitOfMeasure" , "studyEffectiveDescription" , "studyEffectiveDateTime" , "studyEffectivePeriod" , "studyEffectiveDuration" , "studyEffectiveTiming" , "studyEffectiveTimeFromStart" , "studyEffectiveGroupMeasure" , "participantEffectiveDescription" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "participantEffectiveTimeFromStart" , "participantEffectiveGroupMeasure" ,])) ; } } , Field :: UnitOfMeasure => { if r#unit_of_measure . is_some () { return Err (serde :: de :: Error :: duplicate_field ("unitOfMeasure")) ; } r#unit_of_measure = Some (map_access . next_value () ?) ; } , Field :: StudyEffectiveDescription => { if _ctx . from_json { let some = r#study_effective_description . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("studyEffectiveDescription")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#study_effective_description . is_some () { return Err (serde :: de :: Error :: duplicate_field ("studyEffectiveDescription")) ; } r#study_effective_description = Some (map_access . next_value () ?) ; } } , Field :: StudyEffectiveDescriptionPrimitiveElement => { if _ctx . from_json { let some = r#study_effective_description . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_studyEffectiveDescription")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("studyEffectiveDescription" , & ["id" , "extension" , "modifierExtension" , "definitionCodeableConcept" , "definitionCanonical" , "definitionExpression" , "definitionDataRequirement" , "usageContext" , "exclude" , "unitOfMeasure" , "studyEffectiveDescription" , "studyEffectiveDateTime" , "studyEffectivePeriod" , "studyEffectiveDuration" , "studyEffectiveTiming" , "studyEffectiveTimeFromStart" , "studyEffectiveGroupMeasure" , "participantEffectiveDescription" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "participantEffectiveTimeFromStart" , "participantEffectiveGroupMeasure" ,])) ; } } , Field :: StudyEffectiveDateTime => { if _ctx . from_json { let r#enum = r#study_effective . get_or_insert (ResearchElementDefinitionCharacteristicStudyEffective :: DateTime (Default :: default ())) ; if let ResearchElementDefinitionCharacteristicStudyEffective :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("studyEffectiveDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("studyEffective[x]")) ; } } else { if r#study_effective . is_some () { return Err (serde :: de :: Error :: duplicate_field ("studyEffectiveDateTime")) ; } r#study_effective = Some (ResearchElementDefinitionCharacteristicStudyEffective :: DateTime (map_access . next_value () ?)) ; } } , Field :: StudyEffectiveDateTimePrimitiveElement => { if _ctx . from_json { let r#enum = r#study_effective . get_or_insert (ResearchElementDefinitionCharacteristicStudyEffective :: DateTime (Default :: default ())) ; if let ResearchElementDefinitionCharacteristicStudyEffective :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_studyEffectiveDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_studyEffective[x]")) ; } } else { return Err (serde :: de :: Error :: unknown_field ("studyEffectiveDateTime" , & ["id" , "extension" , "modifierExtension" , "definitionCodeableConcept" , "definitionCanonical" , "definitionExpression" , "definitionDataRequirement" , "usageContext" , "exclude" , "unitOfMeasure" , "studyEffectiveDescription" , "studyEffectiveDateTime" , "studyEffectivePeriod" , "studyEffectiveDuration" , "studyEffectiveTiming" , "studyEffectiveTimeFromStart" , "studyEffectiveGroupMeasure" , "participantEffectiveDescription" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "participantEffectiveTimeFromStart" , "participantEffectiveGroupMeasure" ,])) ; } } , Field :: StudyEffectivePeriod => { if r#study_effective . is_some () { return Err (serde :: de :: Error :: duplicate_field ("studyEffectivePeriod")) ; } r#study_effective = Some (ResearchElementDefinitionCharacteristicStudyEffective :: Period (map_access . next_value () ?)) ; } , Field :: StudyEffectiveDuration => { if r#study_effective . is_some () { return Err (serde :: de :: Error :: duplicate_field ("studyEffectiveDuration")) ; } r#study_effective = Some (ResearchElementDefinitionCharacteristicStudyEffective :: Duration (map_access . next_value () ?)) ; } , Field :: StudyEffectiveTiming => { if r#study_effective . is_some () { return Err (serde :: de :: Error :: duplicate_field ("studyEffectiveTiming")) ; } r#study_effective = Some (ResearchElementDefinitionCharacteristicStudyEffective :: Timing (map_access . next_value () ?)) ; } , Field :: StudyEffectiveTimeFromStart => { if r#study_effective_time_from_start . is_some () { return Err (serde :: de :: Error :: duplicate_field ("studyEffectiveTimeFromStart")) ; } r#study_effective_time_from_start = Some (map_access . next_value () ?) ; } , Field :: StudyEffectiveGroupMeasure => { if _ctx . from_json { let some = r#study_effective_group_measure . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("studyEffectiveGroupMeasure")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#study_effective_group_measure . is_some () { return Err (serde :: de :: Error :: duplicate_field ("studyEffectiveGroupMeasure")) ; } r#study_effective_group_measure = Some (map_access . next_value () ?) ; } } , Field :: StudyEffectiveGroupMeasurePrimitiveElement => { if _ctx . from_json { let some = r#study_effective_group_measure . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_studyEffectiveGroupMeasure")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("studyEffectiveGroupMeasure" , & ["id" , "extension" , "modifierExtension" , "definitionCodeableConcept" , "definitionCanonical" , "definitionExpression" , "definitionDataRequirement" , "usageContext" , "exclude" , "unitOfMeasure" , "studyEffectiveDescription" , "studyEffectiveDateTime" , "studyEffectivePeriod" , "studyEffectiveDuration" , "studyEffectiveTiming" , "studyEffectiveTimeFromStart" , "studyEffectiveGroupMeasure" , "participantEffectiveDescription" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "participantEffectiveTimeFromStart" , "participantEffectiveGroupMeasure" ,])) ; } } , Field :: ParticipantEffectiveDescription => { if _ctx . from_json { let some = r#participant_effective_description . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveDescription")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#participant_effective_description . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveDescription")) ; } r#participant_effective_description = Some (map_access . next_value () ?) ; } } , Field :: ParticipantEffectiveDescriptionPrimitiveElement => { if _ctx . from_json { let some = r#participant_effective_description . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_participantEffectiveDescription")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("participantEffectiveDescription" , & ["id" , "extension" , "modifierExtension" , "definitionCodeableConcept" , "definitionCanonical" , "definitionExpression" , "definitionDataRequirement" , "usageContext" , "exclude" , "unitOfMeasure" , "studyEffectiveDescription" , "studyEffectiveDateTime" , "studyEffectivePeriod" , "studyEffectiveDuration" , "studyEffectiveTiming" , "studyEffectiveTimeFromStart" , "studyEffectiveGroupMeasure" , "participantEffectiveDescription" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "participantEffectiveTimeFromStart" , "participantEffectiveGroupMeasure" ,])) ; } } , Field :: ParticipantEffectiveDateTime => { if _ctx . from_json { let r#enum = r#participant_effective . get_or_insert (ResearchElementDefinitionCharacteristicParticipantEffective :: DateTime (Default :: default ())) ; if let ResearchElementDefinitionCharacteristicParticipantEffective :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("participantEffective[x]")) ; } } else { if r#participant_effective . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveDateTime")) ; } r#participant_effective = Some (ResearchElementDefinitionCharacteristicParticipantEffective :: DateTime (map_access . next_value () ?)) ; } } , Field :: ParticipantEffectiveDateTimePrimitiveElement => { if _ctx . from_json { let r#enum = r#participant_effective . get_or_insert (ResearchElementDefinitionCharacteristicParticipantEffective :: DateTime (Default :: default ())) ; if let ResearchElementDefinitionCharacteristicParticipantEffective :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_participantEffectiveDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_participantEffective[x]")) ; } } else { return Err (serde :: de :: Error :: unknown_field ("participantEffectiveDateTime" , & ["id" , "extension" , "modifierExtension" , "definitionCodeableConcept" , "definitionCanonical" , "definitionExpression" , "definitionDataRequirement" , "usageContext" , "exclude" , "unitOfMeasure" , "studyEffectiveDescription" , "studyEffectiveDateTime" , "studyEffectivePeriod" , "studyEffectiveDuration" , "studyEffectiveTiming" , "studyEffectiveTimeFromStart" , "studyEffectiveGroupMeasure" , "participantEffectiveDescription" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "participantEffectiveTimeFromStart" , "participantEffectiveGroupMeasure" ,])) ; } } , Field :: ParticipantEffectivePeriod => { if r#participant_effective . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectivePeriod")) ; } r#participant_effective = Some (ResearchElementDefinitionCharacteristicParticipantEffective :: Period (map_access . next_value () ?)) ; } , Field :: ParticipantEffectiveDuration => { if r#participant_effective . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveDuration")) ; } r#participant_effective = Some (ResearchElementDefinitionCharacteristicParticipantEffective :: Duration (map_access . next_value () ?)) ; } , Field :: ParticipantEffectiveTiming => { if r#participant_effective . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveTiming")) ; } r#participant_effective = Some (ResearchElementDefinitionCharacteristicParticipantEffective :: Timing (map_access . next_value () ?)) ; } , Field :: ParticipantEffectiveTimeFromStart => { if r#participant_effective_time_from_start . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveTimeFromStart")) ; } r#participant_effective_time_from_start = Some (map_access . next_value () ?) ; } , Field :: ParticipantEffectiveGroupMeasure => { if _ctx . from_json { let some = r#participant_effective_group_measure . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveGroupMeasure")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#participant_effective_group_measure . is_some () { return Err (serde :: de :: Error :: duplicate_field ("participantEffectiveGroupMeasure")) ; } r#participant_effective_group_measure = Some (map_access . next_value () ?) ; } } , Field :: ParticipantEffectiveGroupMeasurePrimitiveElement => { if _ctx . from_json { let some = r#participant_effective_group_measure . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_participantEffectiveGroupMeasure")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("participantEffectiveGroupMeasure" , & ["id" , "extension" , "modifierExtension" , "definitionCodeableConcept" , "definitionCanonical" , "definitionExpression" , "definitionDataRequirement" , "usageContext" , "exclude" , "unitOfMeasure" , "studyEffectiveDescription" , "studyEffectiveDateTime" , "studyEffectivePeriod" , "studyEffectiveDuration" , "studyEffectiveTiming" , "studyEffectiveTimeFromStart" , "studyEffectiveGroupMeasure" , "participantEffectiveDescription" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "participantEffectiveTimeFromStart" , "participantEffectiveGroupMeasure" ,])) ; } } , Field :: Unknown (key) => if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "definitionCodeableConcept" , "definitionCanonical" , "definitionExpression" , "definitionDataRequirement" , "usageContext" , "exclude" , "unitOfMeasure" , "studyEffectiveDescription" , "studyEffectiveDateTime" , "studyEffectivePeriod" , "studyEffectiveDuration" , "studyEffectiveTiming" , "studyEffectiveTimeFromStart" , "studyEffectiveGroupMeasure" , "participantEffectiveDescription" , "participantEffectiveDateTime" , "participantEffectivePeriod" , "participantEffectiveDuration" , "participantEffectiveTiming" , "participantEffectiveTimeFromStart" , "participantEffectiveGroupMeasure" ,])) ; } } } Ok (ResearchElementDefinitionCharacteristic { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#definition : if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Lax { r#definition . unwrap_or (Default :: default ()) } else { r#definition . ok_or (serde :: de :: Error :: missing_field ("definition[x]")) ? } , r#usage_context : r#usage_context . unwrap_or (vec ! []) , r#exclude , r#unit_of_measure , r#study_effective_description , r#study_effective , r#study_effective_time_from_start , r#study_effective_group_measure , r#participant_effective_description , r#participant_effective , r#participant_effective_time_from_start , r#participant_effective_group_measure , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The ResearchElementDefinition resource describes a \"PICO\" element that knowledge (evidence, assertion, recommendation) is about.\n\nNeed to be able to define and reuse the definition of individual elements of a research question."]
#[derive(Default, Debug, Clone)]
pub struct ResearchElementDefinition {
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
    #[doc = "An absolute URI that is used to identify this research element definition when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this research element definition is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the research element definition is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this research element definition when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the research element definition when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the research element definition author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence. To provide a version consistent with the Decision Support Service specification, use the format Major.Minor.Revision (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the Decision Support Service specification. Note that a version is required for non-experimental active artifacts."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the research element definition. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the research element definition."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The short title provides an alternate title for use in informal descriptive contexts where the full, formal title is not necessary."]
    pub r#short_title: Option<super::super::types::String>,
    #[doc = "An explanatory or alternate title for the ResearchElementDefinition giving additional information about its content."]
    pub r#subtitle: Option<super::super::types::String>,
    #[doc = "The status of this research element definition. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this research element definition is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The intended subjects for the ResearchElementDefinition. If this element is not provided, a Patient subject is assumed, but the subject of the ResearchElementDefinition can be anything."]
    pub r#subject: Option<ResearchElementDefinitionSubject>,
    #[doc = "The date  (and optionally time) when the research element definition was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the research element definition changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the research element definition."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the research element definition from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A human-readable string to clarify or explain concepts about the resource."]
    pub r#comment: Vec<super::super::types::String>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate research element definition instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the research element definition is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this research element definition is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A detailed description, from a clinical perspective, of how the ResearchElementDefinition is used."]
    pub r#usage: Option<super::super::types::String>,
    #[doc = "A copyright statement relating to the research element definition and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the research element definition."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the research element definition content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Descriptive topics related to the content of the ResearchElementDefinition. Topics provide a high-level categorization grouping types of ResearchElementDefinitions that can be useful for filtering and searching."]
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
    #[doc = "A reference to a Library resource containing the formal logic used by the ResearchElementDefinition."]
    pub r#library: Vec<super::super::types::Canonical>,
    #[doc = "The type of research element, a population, an exposure, or an outcome."]
    pub r#type: super::super::types::Code,
    #[doc = "The type of the outcome (e.g. Dichotomous, Continuous, or Descriptive)."]
    pub r#variable_type: Option<super::super::types::Code>,
    #[doc = "A characteristic that defines the members of the research element. Multiple characteristics are applied with \"and\" semantics."]
    pub r#characteristic: Vec<ResearchElementDefinitionCharacteristic>,
}
impl crate::AnyResource for ResearchElementDefinition {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4B;
}
impl serde::ser::Serialize for ResearchElementDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ResearchElementDefinition")?;
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
                if let Some(some) = self.r#experimental.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("experimental", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_experimental", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#experimental.as_ref() {
                    state.serialize_entry("experimental", some)?;
                }
            }
            if let Some(some) = self.r#subject.as_ref() {
                match some {
                    ResearchElementDefinitionSubject::CodeableConcept(ref value) => {
                        state.serialize_entry("subjectCodeableConcept", value)?;
                    }
                    ResearchElementDefinitionSubject::Reference(ref value) => {
                        state.serialize_entry("subjectReference", value)?;
                    }
                    ResearchElementDefinitionSubject::Invalid => {
                        return Err(serde::ser::Error::custom("subject is invalid"))
                    }
                }
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
            if _ctx.output_json {
                if !self.r#comment.is_empty() {
                    let values = self
                        .r#comment
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("comment", &values)?;
                    }
                    let requires_elements = self
                        .r#comment
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#comment
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_comment", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#comment.is_empty() {
                    state.serialize_entry("comment", &self.r#comment)?;
                }
            }
            if !self.r#use_context.is_empty() {
                state.serialize_entry("useContext", &self.r#use_context)?;
            }
            if !self.r#jurisdiction.is_empty() {
                state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#purpose.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("purpose", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_purpose", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#purpose.as_ref() {
                    state.serialize_entry("purpose", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#usage.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("usage", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_usage", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#usage.as_ref() {
                    state.serialize_entry("usage", some)?;
                }
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#library.is_empty() {
                    state.serialize_entry("library", &self.r#library)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("type", &some)?;
                }
                if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#type.id.as_ref(),
                        extension: &self.r#type.extension,
                    };
                    state.serialize_entry("_type", &primitive_element)?;
                }
            } else {
                state.serialize_entry("type", &self.r#type)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#variable_type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("variableType", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_variableType", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#variable_type.as_ref() {
                    state.serialize_entry("variableType", some)?;
                }
            }
            if !self.r#characteristic.is_empty() {
                state.serialize_entry("characteristic", &self.r#characteristic)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ResearchElementDefinition {
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
            #[serde(rename = "experimental")]
            Experimental,
            #[serde(rename = "_experimental")]
            ExperimentalPrimitiveElement,
            #[serde(rename = "subjectCodeableConcept")]
            SubjectCodeableConcept,
            #[serde(rename = "subjectReference")]
            SubjectReference,
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
            #[serde(rename = "comment")]
            Comment,
            #[serde(rename = "_comment")]
            CommentPrimitiveElement,
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "variableType")]
            VariableType,
            #[serde(rename = "_variableType")]
            VariableTypePrimitiveElement,
            #[serde(rename = "characteristic")]
            Characteristic,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResearchElementDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ResearchElementDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ResearchElementDefinition, V::Error>
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
                let mut r#experimental: Option<super::super::types::Boolean> = None;
                let mut r#subject: Option<ResearchElementDefinitionSubject> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#publisher: Option<super::super::types::String> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#comment: Option<Vec<super::super::types::String>> = None;
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
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#variable_type: Option<super::super::types::Code> = None;
                let mut r#characteristic: Option<Vec<ResearchElementDefinitionCharacteristic>> =
                    None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "ResearchElementDefinition" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"ResearchElementDefinition",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Experimental => {
                                if _ctx.from_json {
                                    let some = r#experimental.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "experimental",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#experimental.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "experimental",
                                        ));
                                    }
                                    r#experimental = Some(map_access.next_value()?);
                                }
                            }
                            Field::ExperimentalPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#experimental.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_experimental",
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
                                        "experimental",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::SubjectCodeableConcept => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subjectCodeableConcept",
                                    ));
                                }
                                r#subject =
                                    Some(ResearchElementDefinitionSubject::CodeableConcept(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::SubjectReference => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subjectReference",
                                    ));
                                }
                                r#subject = Some(ResearchElementDefinitionSubject::Reference(
                                    map_access.next_value()?,
                                ));
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Contact => {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact = Some(map_access.next_value()?);
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Comment => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#comment.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#comment.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    r#comment = Some(map_access.next_value()?);
                                }
                            }
                            Field::CommentPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#comment.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_comment"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "comment",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
                                            "characteristic",
                                        ],
                                    ));
                                }
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
                                if _ctx.from_json {
                                    let some = r#purpose.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("purpose"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#purpose.is_some() {
                                        return Err(serde::de::Error::duplicate_field("purpose"));
                                    }
                                    r#purpose = Some(map_access.next_value()?);
                                }
                            }
                            Field::PurposePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "purpose",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Usage => {
                                if _ctx.from_json {
                                    let some = r#usage.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("usage"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#usage.is_some() {
                                        return Err(serde::de::Error::duplicate_field("usage"));
                                    }
                                    r#usage = Some(map_access.next_value()?);
                                }
                            }
                            Field::UsagePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "usage",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
                                            "characteristic",
                                        ],
                                    ));
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
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
                                if _ctx.from_json {
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
                                } else {
                                    if r#library.is_some() {
                                        return Err(serde::de::Error::duplicate_field("library"));
                                    }
                                    r#library = Some(map_access.next_value()?);
                                }
                            }
                            Field::LibraryPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "library",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
                                            "characteristic",
                                        ],
                                    ));
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::VariableType => {
                                if _ctx.from_json {
                                    let some = r#variable_type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "variableType",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#variable_type.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "variableType",
                                        ));
                                    }
                                    r#variable_type = Some(map_access.next_value()?);
                                }
                            }
                            Field::VariableTypePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#variable_type.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_variableType",
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
                                        "variableType",
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
                                            "experimental",
                                            "subjectCodeableConcept",
                                            "subjectReference",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "comment",
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
                                            "type",
                                            "variableType",
                                            "characteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::Characteristic => {
                                if r#characteristic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "characteristic",
                                    ));
                                }
                                r#characteristic = Some(map_access.next_value()?);
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
                                        "experimental",
                                        "subjectCodeableConcept",
                                        "subjectReference",
                                        "date",
                                        "publisher",
                                        "contact",
                                        "description",
                                        "comment",
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
                                        "type",
                                        "variableType",
                                        "characteristic",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ResearchElementDefinition {
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
                        r#experimental,
                        r#subject,
                        r#date,
                        r#publisher,
                        r#contact: r#contact.unwrap_or(vec![]),
                        r#description,
                        r#comment: r#comment.unwrap_or(vec![]),
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
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#variable_type,
                        r#characteristic: r#characteristic.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
