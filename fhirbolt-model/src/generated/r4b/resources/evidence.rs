// Generated on 2023-04-05 by fhirbolt-codegen v0.1.0
#[doc = "Citation Resource or display of suggested citation for this evidence."]
#[derive(Debug, Clone, PartialEq)]
pub enum EvidenceCiteAs {
    Reference(Box<super::super::types::Reference>),
    Markdown(Box<super::super::types::Markdown>),
    Invalid,
}
impl Default for EvidenceCiteAs {
    fn default() -> EvidenceCiteAs {
        EvidenceCiteAs::Invalid
    }
}
#[doc = "Evidence variable such as population, exposure, or outcome."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EvidenceVariableDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A text description or summary of the variable."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Footnotes and/or explanatory notes."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "population | subpopulation | exposure | referenceExposure | measuredVariable | confounder."]
    pub r#variable_role: Box<super::super::types::CodeableConcept>,
    #[doc = "Definition of the actual variable related to the statistic(s)."]
    pub r#observed: Option<Box<super::super::types::Reference>>,
    #[doc = "Definition of the intended variable related to the Evidence."]
    pub r#intended: Option<Box<super::super::types::Reference>>,
    #[doc = "Indication of quality of match between intended variable to actual variable."]
    pub r#directness_match: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for EvidenceVariableDefinition {
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
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            state.serialize_entry("variableRole", &self.r#variable_role)?;
            if let Some(some) = self.r#observed.as_ref() {
                state.serialize_entry("observed", some)?;
            }
            if let Some(some) = self.r#intended.as_ref() {
                state.serialize_entry("intended", some)?;
            }
            if let Some(some) = self.r#directness_match.as_ref() {
                state.serialize_entry("directnessMatch", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceVariableDefinition {
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
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "variableRole")]
            VariableRole,
            #[serde(rename = "observed")]
            Observed,
            #[serde(rename = "intended")]
            Intended,
            #[serde(rename = "directnessMatch")]
            DirectnessMatch,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceVariableDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceVariableDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EvidenceVariableDefinition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#variable_role: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#observed: Option<Box<super::super::types::Reference>> = None;
                let mut r#intended: Option<Box<super::super::types::Reference>> = None;
                let mut r#directness_match: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
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
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "note",
                                            "variableRole",
                                            "observed",
                                            "intended",
                                            "directnessMatch",
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
                            Field::VariableRole => {
                                if r#variable_role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("variableRole"));
                                }
                                r#variable_role = Some(map_access.next_value()?);
                            }
                            Field::Observed => {
                                if r#observed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("observed"));
                                }
                                r#observed = Some(map_access.next_value()?);
                            }
                            Field::Intended => {
                                if r#intended.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intended"));
                                }
                                r#intended = Some(map_access.next_value()?);
                            }
                            Field::DirectnessMatch => {
                                if r#directness_match.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "directnessMatch",
                                    ));
                                }
                                r#directness_match = Some(map_access.next_value()?);
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
                                        "description",
                                        "note",
                                        "variableRole",
                                        "observed",
                                        "intended",
                                        "directnessMatch",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceVariableDefinition {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#description,
                        r#note: r#note.unwrap_or(vec![]),
                        r#variable_role: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#variable_role.unwrap_or(Default::default())
                        } else {
                            r#variable_role
                                .ok_or(serde::de::Error::missing_field("variableRole"))?
                        },
                        r#observed,
                        r#intended,
                        r#directness_match,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Number of samples in the statistic."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EvidenceStatisticSampleSize {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Human-readable summary of population sample size."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Footnote or explanatory note about the sample size."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Number of participants in the population."]
    pub r#number_of_studies: Option<super::super::types::UnsignedInt>,
    #[doc = "A human-readable string to clarify or explain concepts about the sample size."]
    pub r#number_of_participants: Option<super::super::types::UnsignedInt>,
    #[doc = "Number of participants with known results for measured variables."]
    pub r#known_data_count: Option<super::super::types::UnsignedInt>,
}
impl serde::ser::Serialize for EvidenceStatisticSampleSize {
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
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number_of_studies.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("numberOfStudies", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_numberOfStudies", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number_of_studies.as_ref() {
                    state.serialize_entry("numberOfStudies", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number_of_participants.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("numberOfParticipants", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_numberOfParticipants", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number_of_participants.as_ref() {
                    state.serialize_entry("numberOfParticipants", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#known_data_count.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("knownDataCount", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_knownDataCount", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#known_data_count.as_ref() {
                    state.serialize_entry("knownDataCount", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceStatisticSampleSize {
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
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "numberOfStudies")]
            NumberOfStudies,
            #[serde(rename = "_numberOfStudies")]
            NumberOfStudiesPrimitiveElement,
            #[serde(rename = "numberOfParticipants")]
            NumberOfParticipants,
            #[serde(rename = "_numberOfParticipants")]
            NumberOfParticipantsPrimitiveElement,
            #[serde(rename = "knownDataCount")]
            KnownDataCount,
            #[serde(rename = "_knownDataCount")]
            KnownDataCountPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceStatisticSampleSize;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceStatisticSampleSize")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<EvidenceStatisticSampleSize, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#number_of_studies: Option<super::super::types::UnsignedInt> = None;
                let mut r#number_of_participants: Option<super::super::types::UnsignedInt> = None;
                let mut r#known_data_count: Option<super::super::types::UnsignedInt> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
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
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "note",
                                            "numberOfStudies",
                                            "numberOfParticipants",
                                            "knownDataCount",
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
                            Field::NumberOfStudies => {
                                if _ctx.from_json {
                                    let some =
                                        r#number_of_studies.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfStudies",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#number_of_studies.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfStudies",
                                        ));
                                    }
                                    r#number_of_studies = Some(map_access.next_value()?);
                                }
                            }
                            Field::NumberOfStudiesPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#number_of_studies.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_numberOfStudies",
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
                                        "numberOfStudies",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "note",
                                            "numberOfStudies",
                                            "numberOfParticipants",
                                            "knownDataCount",
                                        ],
                                    ));
                                }
                            }
                            Field::NumberOfParticipants => {
                                if _ctx.from_json {
                                    let some =
                                        r#number_of_participants.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfParticipants",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#number_of_participants.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfParticipants",
                                        ));
                                    }
                                    r#number_of_participants = Some(map_access.next_value()?);
                                }
                            }
                            Field::NumberOfParticipantsPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#number_of_participants.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_numberOfParticipants",
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
                                        "numberOfParticipants",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "note",
                                            "numberOfStudies",
                                            "numberOfParticipants",
                                            "knownDataCount",
                                        ],
                                    ));
                                }
                            }
                            Field::KnownDataCount => {
                                if _ctx.from_json {
                                    let some = r#known_data_count.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "knownDataCount",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#known_data_count.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "knownDataCount",
                                        ));
                                    }
                                    r#known_data_count = Some(map_access.next_value()?);
                                }
                            }
                            Field::KnownDataCountPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#known_data_count.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_knownDataCount",
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
                                        "knownDataCount",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "note",
                                            "numberOfStudies",
                                            "numberOfParticipants",
                                            "knownDataCount",
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
                                        "description",
                                        "note",
                                        "numberOfStudies",
                                        "numberOfParticipants",
                                        "knownDataCount",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceStatisticSampleSize {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#description,
                        r#note: r#note.unwrap_or(vec![]),
                        r#number_of_studies,
                        r#number_of_participants,
                        r#known_data_count,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A statistical attribute of the statistic such as a measure of heterogeneity."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EvidenceStatisticAttributeEstimate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Human-readable summary of the estimate."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Footnote or explanatory note about the estimate."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "The type of attribute estimate, eg confidence interval or p value."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The singular quantity of the attribute estimate, for attribute estimates represented as single values; also used to report unit of measure."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Use 95 for a 95% confidence interval."]
    pub r#level: Option<super::super::types::Decimal>,
    #[doc = "Lower bound of confidence interval."]
    pub r#range: Option<Box<super::super::types::Range>>,
    #[doc = "A nested attribute estimate; which is the attribute estimate of an attribute estimate."]
    pub r#attribute_estimate: Vec<EvidenceStatisticAttributeEstimate>,
}
impl serde::ser::Serialize for EvidenceStatisticAttributeEstimate {
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
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#level.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("level", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_level", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#level.as_ref() {
                    state.serialize_entry("level", some)?;
                }
            }
            if let Some(some) = self.r#range.as_ref() {
                state.serialize_entry("range", some)?;
            }
            if !self.r#attribute_estimate.is_empty() {
                state.serialize_entry("attributeEstimate", &self.r#attribute_estimate)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceStatisticAttributeEstimate {
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
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "level")]
            Level,
            #[serde(rename = "_level")]
            LevelPrimitiveElement,
            #[serde(rename = "range")]
            Range,
            #[serde(rename = "attributeEstimate")]
            AttributeEstimate,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceStatisticAttributeEstimate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceStatisticAttributeEstimate")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<EvidenceStatisticAttributeEstimate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#level: Option<super::super::types::Decimal> = None;
                let mut r#range: Option<Box<super::super::types::Range>> = None;
                let mut r#attribute_estimate: Option<Vec<EvidenceStatisticAttributeEstimate>> =
                    None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
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
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "note",
                                            "type",
                                            "quantity",
                                            "level",
                                            "range",
                                            "attributeEstimate",
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::Level => {
                                if _ctx.from_json {
                                    let some = r#level.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("level"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#level.is_some() {
                                        return Err(serde::de::Error::duplicate_field("level"));
                                    }
                                    r#level = Some(map_access.next_value()?);
                                }
                            }
                            Field::LevelPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#level.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_level"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "level",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "note",
                                            "type",
                                            "quantity",
                                            "level",
                                            "range",
                                            "attributeEstimate",
                                        ],
                                    ));
                                }
                            }
                            Field::Range => {
                                if r#range.is_some() {
                                    return Err(serde::de::Error::duplicate_field("range"));
                                }
                                r#range = Some(map_access.next_value()?);
                            }
                            Field::AttributeEstimate => {
                                if _ctx.from_json {
                                    if r#attribute_estimate.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "attributeEstimate",
                                        ));
                                    }
                                    r#attribute_estimate = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#attribute_estimate.get_or_insert(Default::default());
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
                                        "extension",
                                        "modifierExtension",
                                        "description",
                                        "note",
                                        "type",
                                        "quantity",
                                        "level",
                                        "range",
                                        "attributeEstimate",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceStatisticAttributeEstimate {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#description,
                        r#note: r#note.unwrap_or(vec![]),
                        r#type,
                        r#quantity,
                        r#level,
                        r#range,
                        r#attribute_estimate: r#attribute_estimate.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A variable adjusted for in the adjusted analysis."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EvidenceStatisticModelCharacteristicVariable {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Description of the variable."]
    pub r#variable_definition: Box<super::super::types::Reference>,
    #[doc = "How the variable is classified for use in adjusted analysis."]
    pub r#handling: Option<super::super::types::Code>,
    #[doc = "Description for grouping of ordinal or polychotomous variables."]
    pub r#value_category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Discrete value for grouping of ordinal or polychotomous variables."]
    pub r#value_quantity: Vec<Box<super::super::types::Quantity>>,
    #[doc = "Range of values for grouping of ordinal or polychotomous variables."]
    pub r#value_range: Vec<Box<super::super::types::Range>>,
}
impl serde::ser::Serialize for EvidenceStatisticModelCharacteristicVariable {
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
            state.serialize_entry("variableDefinition", &self.r#variable_definition)?;
            if _ctx.output_json {
                if let Some(some) = self.r#handling.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("handling", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_handling", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#handling.as_ref() {
                    state.serialize_entry("handling", some)?;
                }
            }
            if !self.r#value_category.is_empty() {
                state.serialize_entry("valueCategory", &self.r#value_category)?;
            }
            if !self.r#value_quantity.is_empty() {
                state.serialize_entry("valueQuantity", &self.r#value_quantity)?;
            }
            if !self.r#value_range.is_empty() {
                state.serialize_entry("valueRange", &self.r#value_range)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceStatisticModelCharacteristicVariable {
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
            #[serde(rename = "variableDefinition")]
            VariableDefinition,
            #[serde(rename = "handling")]
            Handling,
            #[serde(rename = "_handling")]
            HandlingPrimitiveElement,
            #[serde(rename = "valueCategory")]
            ValueCategory,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueRange")]
            ValueRange,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceStatisticModelCharacteristicVariable;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceStatisticModelCharacteristicVariable")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<EvidenceStatisticModelCharacteristicVariable, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#variable_definition: Option<Box<super::super::types::Reference>> = None;
                let mut r#handling: Option<super::super::types::Code> = None;
                let mut r#value_category: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#value_quantity: Option<Vec<Box<super::super::types::Quantity>>> = None;
                let mut r#value_range: Option<Vec<Box<super::super::types::Range>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
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
                            Field::VariableDefinition => {
                                if r#variable_definition.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "variableDefinition",
                                    ));
                                }
                                r#variable_definition = Some(map_access.next_value()?);
                            }
                            Field::Handling => {
                                if _ctx.from_json {
                                    let some = r#handling.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("handling"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#handling.is_some() {
                                        return Err(serde::de::Error::duplicate_field("handling"));
                                    }
                                    r#handling = Some(map_access.next_value()?);
                                }
                            }
                            Field::HandlingPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#handling.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_handling"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "handling",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "variableDefinition",
                                            "handling",
                                            "valueCategory",
                                            "valueQuantity",
                                            "valueRange",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueCategory => {
                                if _ctx.from_json {
                                    if r#value_category.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueCategory",
                                        ));
                                    }
                                    r#value_category = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#value_category.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ValueQuantity => {
                                if _ctx.from_json {
                                    if r#value_quantity.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueQuantity",
                                        ));
                                    }
                                    r#value_quantity = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#value_quantity.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ValueRange => {
                                if _ctx.from_json {
                                    if r#value_range.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueRange",
                                        ));
                                    }
                                    r#value_range = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#value_range.get_or_insert(Default::default());
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
                                        "extension",
                                        "modifierExtension",
                                        "variableDefinition",
                                        "handling",
                                        "valueCategory",
                                        "valueQuantity",
                                        "valueRange",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceStatisticModelCharacteristicVariable {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#variable_definition: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#variable_definition.unwrap_or(Default::default())
                        } else {
                            r#variable_definition
                                .ok_or(serde::de::Error::missing_field("variableDefinition"))?
                        },
                        r#handling,
                        r#value_category: r#value_category.unwrap_or(vec![]),
                        r#value_quantity: r#value_quantity.unwrap_or(vec![]),
                        r#value_range: r#value_range.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A component of the method to generate the statistic."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EvidenceStatisticModelCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Description of a component of the method to generate the statistic."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Further specification of the quantified value of the component of the method to generate the statistic."]
    pub r#value: Option<Box<super::super::types::Quantity>>,
    #[doc = "A variable adjusted for in the adjusted analysis."]
    pub r#variable: Vec<EvidenceStatisticModelCharacteristicVariable>,
    #[doc = "An attribute of the statistic used as a model characteristic."]
    pub r#attribute_estimate: Vec<EvidenceStatisticAttributeEstimate>,
}
impl serde::ser::Serialize for EvidenceStatisticModelCharacteristic {
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
            state.serialize_entry("code", &self.r#code)?;
            if let Some(some) = self.r#value.as_ref() {
                state.serialize_entry("value", some)?;
            }
            if !self.r#variable.is_empty() {
                state.serialize_entry("variable", &self.r#variable)?;
            }
            if !self.r#attribute_estimate.is_empty() {
                state.serialize_entry("attributeEstimate", &self.r#attribute_estimate)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceStatisticModelCharacteristic {
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
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "variable")]
            Variable,
            #[serde(rename = "attributeEstimate")]
            AttributeEstimate,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceStatisticModelCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceStatisticModelCharacteristic")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<EvidenceStatisticModelCharacteristic, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<Box<super::super::types::Quantity>> = None;
                let mut r#variable: Option<Vec<EvidenceStatisticModelCharacteristicVariable>> =
                    None;
                let mut r#attribute_estimate: Option<Vec<EvidenceStatisticAttributeEstimate>> =
                    None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
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
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Value => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                r#value = Some(map_access.next_value()?);
                            }
                            Field::Variable => {
                                if _ctx.from_json {
                                    if r#variable.is_some() {
                                        return Err(serde::de::Error::duplicate_field("variable"));
                                    }
                                    r#variable = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#variable.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::AttributeEstimate => {
                                if _ctx.from_json {
                                    if r#attribute_estimate.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "attributeEstimate",
                                        ));
                                    }
                                    r#attribute_estimate = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#attribute_estimate.get_or_insert(Default::default());
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
                                        "extension",
                                        "modifierExtension",
                                        "code",
                                        "value",
                                        "variable",
                                        "attributeEstimate",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceStatisticModelCharacteristic {
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
                        r#variable: r#variable.unwrap_or(vec![]),
                        r#attribute_estimate: r#attribute_estimate.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Values and parameters for a single statistic."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EvidenceStatistic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A description of the content value of the statistic."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Footnotes and/or explanatory notes."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Type of statistic, eg relative risk."]
    pub r#statistic_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the measured variable is handled categorically, the category element is used to define which category the statistic is reporting."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Statistic value."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The number of events associated with the statistic, where the unit of analysis is different from numberAffected, sampleSize.knownDataCount and sampleSize.numberOfParticipants."]
    pub r#number_of_events: Option<super::super::types::UnsignedInt>,
    #[doc = "The number of participants affected where the unit of analysis is the same as sampleSize.knownDataCount and sampleSize.numberOfParticipants."]
    pub r#number_affected: Option<super::super::types::UnsignedInt>,
    #[doc = "Number of samples in the statistic."]
    pub r#sample_size: Option<EvidenceStatisticSampleSize>,
    #[doc = "A statistical attribute of the statistic such as a measure of heterogeneity."]
    pub r#attribute_estimate: Vec<EvidenceStatisticAttributeEstimate>,
    #[doc = "A component of the method to generate the statistic."]
    pub r#model_characteristic: Vec<EvidenceStatisticModelCharacteristic>,
}
impl serde::ser::Serialize for EvidenceStatistic {
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
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if let Some(some) = self.r#statistic_type.as_ref() {
                state.serialize_entry("statisticType", some)?;
            }
            if let Some(some) = self.r#category.as_ref() {
                state.serialize_entry("category", some)?;
            }
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number_of_events.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("numberOfEvents", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_numberOfEvents", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number_of_events.as_ref() {
                    state.serialize_entry("numberOfEvents", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number_affected.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("numberAffected", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_numberAffected", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number_affected.as_ref() {
                    state.serialize_entry("numberAffected", some)?;
                }
            }
            if let Some(some) = self.r#sample_size.as_ref() {
                state.serialize_entry("sampleSize", some)?;
            }
            if !self.r#attribute_estimate.is_empty() {
                state.serialize_entry("attributeEstimate", &self.r#attribute_estimate)?;
            }
            if !self.r#model_characteristic.is_empty() {
                state.serialize_entry("modelCharacteristic", &self.r#model_characteristic)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceStatistic {
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
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "statisticType")]
            StatisticType,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "numberOfEvents")]
            NumberOfEvents,
            #[serde(rename = "_numberOfEvents")]
            NumberOfEventsPrimitiveElement,
            #[serde(rename = "numberAffected")]
            NumberAffected,
            #[serde(rename = "_numberAffected")]
            NumberAffectedPrimitiveElement,
            #[serde(rename = "sampleSize")]
            SampleSize,
            #[serde(rename = "attributeEstimate")]
            AttributeEstimate,
            #[serde(rename = "modelCharacteristic")]
            ModelCharacteristic,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceStatistic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceStatistic")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EvidenceStatistic, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#statistic_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#number_of_events: Option<super::super::types::UnsignedInt> = None;
                let mut r#number_affected: Option<super::super::types::UnsignedInt> = None;
                let mut r#sample_size: Option<EvidenceStatisticSampleSize> = None;
                let mut r#attribute_estimate: Option<Vec<EvidenceStatisticAttributeEstimate>> =
                    None;
                let mut r#model_characteristic: Option<Vec<EvidenceStatisticModelCharacteristic>> =
                    None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
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
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "note",
                                            "statisticType",
                                            "category",
                                            "quantity",
                                            "numberOfEvents",
                                            "numberAffected",
                                            "sampleSize",
                                            "attributeEstimate",
                                            "modelCharacteristic",
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
                            Field::StatisticType => {
                                if r#statistic_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statisticType"));
                                }
                                r#statistic_type = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::NumberOfEvents => {
                                if _ctx.from_json {
                                    let some = r#number_of_events.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfEvents",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#number_of_events.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfEvents",
                                        ));
                                    }
                                    r#number_of_events = Some(map_access.next_value()?);
                                }
                            }
                            Field::NumberOfEventsPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#number_of_events.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_numberOfEvents",
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
                                        "numberOfEvents",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "note",
                                            "statisticType",
                                            "category",
                                            "quantity",
                                            "numberOfEvents",
                                            "numberAffected",
                                            "sampleSize",
                                            "attributeEstimate",
                                            "modelCharacteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::NumberAffected => {
                                if _ctx.from_json {
                                    let some = r#number_affected.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberAffected",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#number_affected.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberAffected",
                                        ));
                                    }
                                    r#number_affected = Some(map_access.next_value()?);
                                }
                            }
                            Field::NumberAffectedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#number_affected.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_numberAffected",
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
                                        "numberAffected",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "note",
                                            "statisticType",
                                            "category",
                                            "quantity",
                                            "numberOfEvents",
                                            "numberAffected",
                                            "sampleSize",
                                            "attributeEstimate",
                                            "modelCharacteristic",
                                        ],
                                    ));
                                }
                            }
                            Field::SampleSize => {
                                if r#sample_size.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sampleSize"));
                                }
                                r#sample_size = Some(map_access.next_value()?);
                            }
                            Field::AttributeEstimate => {
                                if _ctx.from_json {
                                    if r#attribute_estimate.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "attributeEstimate",
                                        ));
                                    }
                                    r#attribute_estimate = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#attribute_estimate.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModelCharacteristic => {
                                if _ctx.from_json {
                                    if r#model_characteristic.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modelCharacteristic",
                                        ));
                                    }
                                    r#model_characteristic = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#model_characteristic.get_or_insert(Default::default());
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
                                        "extension",
                                        "modifierExtension",
                                        "description",
                                        "note",
                                        "statisticType",
                                        "category",
                                        "quantity",
                                        "numberOfEvents",
                                        "numberAffected",
                                        "sampleSize",
                                        "attributeEstimate",
                                        "modelCharacteristic",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceStatistic {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#description,
                        r#note: r#note.unwrap_or(vec![]),
                        r#statistic_type,
                        r#category,
                        r#quantity,
                        r#number_of_events,
                        r#number_affected,
                        r#sample_size,
                        r#attribute_estimate: r#attribute_estimate.unwrap_or(vec![]),
                        r#model_characteristic: r#model_characteristic.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Assessment of certainty, confidence in the estimates, or quality of the evidence."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EvidenceCertainty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Textual description of certainty."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Footnotes and/or explanatory notes."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Aspect of certainty being rated."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Assessment or judgement of the aspect."]
    pub r#rating: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Individual or group who did the rating."]
    pub r#rater: Option<super::super::types::String>,
    #[doc = "A domain or subdomain of certainty."]
    pub r#subcomponent: Vec<EvidenceCertainty>,
}
impl serde::ser::Serialize for EvidenceCertainty {
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
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#rating.as_ref() {
                state.serialize_entry("rating", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#rater.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("rater", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_rater", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#rater.as_ref() {
                    state.serialize_entry("rater", some)?;
                }
            }
            if !self.r#subcomponent.is_empty() {
                state.serialize_entry("subcomponent", &self.r#subcomponent)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceCertainty {
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
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "rating")]
            Rating,
            #[serde(rename = "rater")]
            Rater,
            #[serde(rename = "_rater")]
            RaterPrimitiveElement,
            #[serde(rename = "subcomponent")]
            Subcomponent,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceCertainty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceCertainty")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EvidenceCertainty, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#rating: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#rater: Option<super::super::types::String> = None;
                let mut r#subcomponent: Option<Vec<EvidenceCertainty>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
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
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "note",
                                            "type",
                                            "rating",
                                            "rater",
                                            "subcomponent",
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Rating => {
                                if r#rating.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rating"));
                                }
                                r#rating = Some(map_access.next_value()?);
                            }
                            Field::Rater => {
                                if _ctx.from_json {
                                    let some = r#rater.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("rater"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#rater.is_some() {
                                        return Err(serde::de::Error::duplicate_field("rater"));
                                    }
                                    r#rater = Some(map_access.next_value()?);
                                }
                            }
                            Field::RaterPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#rater.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_rater"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "rater",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "description",
                                            "note",
                                            "type",
                                            "rating",
                                            "rater",
                                            "subcomponent",
                                        ],
                                    ));
                                }
                            }
                            Field::Subcomponent => {
                                if _ctx.from_json {
                                    if r#subcomponent.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "subcomponent",
                                        ));
                                    }
                                    r#subcomponent = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#subcomponent.get_or_insert(Default::default());
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
                                        "extension",
                                        "modifierExtension",
                                        "description",
                                        "note",
                                        "type",
                                        "rating",
                                        "rater",
                                        "subcomponent",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceCertainty {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#description,
                        r#note: r#note.unwrap_or(vec![]),
                        r#type,
                        r#rating,
                        r#rater,
                        r#subcomponent: r#subcomponent.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The Evidence Resource provides a machine-interpretable expression of an evidence concept including the evidence variables (eg population, exposures/interventions, comparators, outcomes, measured variables, confounding variables), the statistics, and the certainty of this evidence."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Evidence {
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
    #[doc = "An absolute URI that is used to identify this evidence when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this summary is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the summary is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this summary when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the summary when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the summary author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the summary."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "Citation Resource or display of suggested citation for this evidence."]
    pub r#cite_as: Option<EvidenceCiteAs>,
    #[doc = "The status of this summary. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "The date  (and optionally time) when the summary was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the summary changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate evidence instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The name of the organization or individual that published the evidence."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individiual, organization, or device primarily involved in the creation and maintenance of the content."]
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individiual, organization, or device primarily responsible for internal coherence of the content."]
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individiual, organization, or device primarily responsible for review of some aspect of the content."]
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individiual, organization, or device responsible for officially endorsing the content for use in some setting."]
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "Link or citation to artifact associated with the summary."]
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    #[doc = "A free text natural language description of the evidence from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Declarative description of the Evidence."]
    pub r#assertion: Option<super::super::types::Markdown>,
    #[doc = "Footnotes and/or explanatory notes."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Evidence variable such as population, exposure, or outcome."]
    pub r#variable_definition: Vec<EvidenceVariableDefinition>,
    #[doc = "The method to combine studies."]
    pub r#synthesis_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of study that produced this evidence."]
    pub r#study_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Values and parameters for a single statistic."]
    pub r#statistic: Vec<EvidenceStatistic>,
    #[doc = "Assessment of certainty, confidence in the estimates, or quality of the evidence."]
    pub r#certainty: Vec<EvidenceCertainty>,
}
impl crate::AnyResource for Evidence {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4B;
}
impl serde::ser::Serialize for Evidence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Evidence")?;
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
            if let Some(some) = self.r#cite_as.as_ref() {
                match some {
                    EvidenceCiteAs::Reference(ref value) => {
                        state.serialize_entry("citeAsReference", value)?;
                    }
                    EvidenceCiteAs::Markdown(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("citeAsMarkdown", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_citeAsMarkdown", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("citeAsMarkdown", value)?;
                        }
                    }
                    EvidenceCiteAs::Invalid => {
                        return Err(serde::ser::Error::custom("cite_as is invalid"))
                    }
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
            if !self.r#use_context.is_empty() {
                state.serialize_entry("useContext", &self.r#use_context)?;
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
                if let Some(some) = self.r#assertion.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("assertion", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_assertion", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#assertion.as_ref() {
                    state.serialize_entry("assertion", some)?;
                }
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#variable_definition.is_empty() {
                state.serialize_entry("variableDefinition", &self.r#variable_definition)?;
            }
            if let Some(some) = self.r#synthesis_type.as_ref() {
                state.serialize_entry("synthesisType", some)?;
            }
            if let Some(some) = self.r#study_type.as_ref() {
                state.serialize_entry("studyType", some)?;
            }
            if !self.r#statistic.is_empty() {
                state.serialize_entry("statistic", &self.r#statistic)?;
            }
            if !self.r#certainty.is_empty() {
                state.serialize_entry("certainty", &self.r#certainty)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Evidence {
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
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            #[serde(rename = "citeAsReference")]
            CiteAsReference,
            #[serde(rename = "citeAsMarkdown")]
            CiteAsMarkdown,
            #[serde(rename = "_citeAsMarkdown")]
            CiteAsMarkdownPrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "useContext")]
            UseContext,
            #[serde(rename = "approvalDate")]
            ApprovalDate,
            #[serde(rename = "_approvalDate")]
            ApprovalDatePrimitiveElement,
            #[serde(rename = "lastReviewDate")]
            LastReviewDate,
            #[serde(rename = "_lastReviewDate")]
            LastReviewDatePrimitiveElement,
            #[serde(rename = "publisher")]
            Publisher,
            #[serde(rename = "_publisher")]
            PublisherPrimitiveElement,
            #[serde(rename = "contact")]
            Contact,
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
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "assertion")]
            Assertion,
            #[serde(rename = "_assertion")]
            AssertionPrimitiveElement,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "variableDefinition")]
            VariableDefinition,
            #[serde(rename = "synthesisType")]
            SynthesisType,
            #[serde(rename = "studyType")]
            StudyType,
            #[serde(rename = "statistic")]
            Statistic,
            #[serde(rename = "certainty")]
            Certainty,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Evidence;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Evidence")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Evidence, V::Error>
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
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#cite_as: Option<EvidenceCiteAs> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#use_context: Option<Vec<Box<super::super::types::UsageContext>>> = None;
                let mut r#approval_date: Option<super::super::types::Date> = None;
                let mut r#last_review_date: Option<super::super::types::Date> = None;
                let mut r#publisher: Option<super::super::types::String> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#author: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#editor: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#reviewer: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#endorser: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#related_artifact: Option<Vec<Box<super::super::types::RelatedArtifact>>> =
                    None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#assertion: Option<super::super::types::Markdown> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#variable_definition: Option<Vec<EvidenceVariableDefinition>> = None;
                let mut r#synthesis_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#study_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#statistic: Option<Vec<EvidenceStatistic>> = None;
                let mut r#certainty: Option<Vec<EvidenceCertainty>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Evidence" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Evidence",
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
                                        ],
                                    ));
                                }
                            }
                            Field::CiteAsReference => {
                                if r#cite_as.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "citeAsReference",
                                    ));
                                }
                                r#cite_as =
                                    Some(EvidenceCiteAs::Reference(map_access.next_value()?));
                            }
                            Field::CiteAsMarkdown => {
                                if _ctx.from_json {
                                    let r#enum = r#cite_as.get_or_insert(EvidenceCiteAs::Markdown(
                                        Default::default(),
                                    ));
                                    if let EvidenceCiteAs::Markdown(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "citeAsMarkdown",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("citeAs[x]"));
                                    }
                                } else {
                                    if r#cite_as.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "citeAsMarkdown",
                                        ));
                                    }
                                    r#cite_as =
                                        Some(EvidenceCiteAs::Markdown(map_access.next_value()?));
                                }
                            }
                            Field::CiteAsMarkdownPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#cite_as.get_or_insert(EvidenceCiteAs::Markdown(
                                        Default::default(),
                                    ));
                                    if let EvidenceCiteAs::Markdown(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_citeAsMarkdown",
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
                                            "_citeAs[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "citeAsMarkdown",
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
                                        ],
                                    ));
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
                                        ],
                                    ));
                                }
                            }
                            Field::Assertion => {
                                if _ctx.from_json {
                                    let some = r#assertion.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("assertion"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#assertion.is_some() {
                                        return Err(serde::de::Error::duplicate_field("assertion"));
                                    }
                                    r#assertion = Some(map_access.next_value()?);
                                }
                            }
                            Field::AssertionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#assertion.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_assertion",
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
                                        "assertion",
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
                                            "title",
                                            "citeAsReference",
                                            "citeAsMarkdown",
                                            "status",
                                            "date",
                                            "useContext",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "publisher",
                                            "contact",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "relatedArtifact",
                                            "description",
                                            "assertion",
                                            "note",
                                            "variableDefinition",
                                            "synthesisType",
                                            "studyType",
                                            "statistic",
                                            "certainty",
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
                            Field::VariableDefinition => {
                                if _ctx.from_json {
                                    if r#variable_definition.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "variableDefinition",
                                        ));
                                    }
                                    r#variable_definition = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#variable_definition.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::SynthesisType => {
                                if r#synthesis_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("synthesisType"));
                                }
                                r#synthesis_type = Some(map_access.next_value()?);
                            }
                            Field::StudyType => {
                                if r#study_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("studyType"));
                                }
                                r#study_type = Some(map_access.next_value()?);
                            }
                            Field::Statistic => {
                                if _ctx.from_json {
                                    if r#statistic.is_some() {
                                        return Err(serde::de::Error::duplicate_field("statistic"));
                                    }
                                    r#statistic = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#statistic.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Certainty => {
                                if _ctx.from_json {
                                    if r#certainty.is_some() {
                                        return Err(serde::de::Error::duplicate_field("certainty"));
                                    }
                                    r#certainty = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#certainty.get_or_insert(Default::default());
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
                                        "title",
                                        "citeAsReference",
                                        "citeAsMarkdown",
                                        "status",
                                        "date",
                                        "useContext",
                                        "approvalDate",
                                        "lastReviewDate",
                                        "publisher",
                                        "contact",
                                        "author",
                                        "editor",
                                        "reviewer",
                                        "endorser",
                                        "relatedArtifact",
                                        "description",
                                        "assertion",
                                        "note",
                                        "variableDefinition",
                                        "synthesisType",
                                        "studyType",
                                        "statistic",
                                        "certainty",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Evidence {
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
                        r#title,
                        r#cite_as,
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#date,
                        r#use_context: r#use_context.unwrap_or(vec![]),
                        r#approval_date,
                        r#last_review_date,
                        r#publisher,
                        r#contact: r#contact.unwrap_or(vec![]),
                        r#author: r#author.unwrap_or(vec![]),
                        r#editor: r#editor.unwrap_or(vec![]),
                        r#reviewer: r#reviewer.unwrap_or(vec![]),
                        r#endorser: r#endorser.unwrap_or(vec![]),
                        r#related_artifact: r#related_artifact.unwrap_or(vec![]),
                        r#description,
                        r#assertion,
                        r#note: r#note.unwrap_or(vec![]),
                        r#variable_definition: r#variable_definition.unwrap_or(vec![]),
                        r#synthesis_type,
                        r#study_type,
                        r#statistic: r#statistic.unwrap_or(vec![]),
                        r#certainty: r#certainty.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
