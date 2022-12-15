// Generated on 2022-12-15 by fhirbolt-codegen v0.1.0
#[doc = "This subclause refers to the description of each subunit constituting the SubstanceProtein. A subunit is a linear sequence of amino acids linked through peptide bonds. The Subunit information shall be provided when the finished SubstanceProtein is a complex of multiple sequences; subunits are not used to delineate domains within a single sequence. Subunits are listed in order of decreasing length; sequences of the same length will be ordered by decreasing molecular weight; subunits that have identical sequences will be repeated multiple times."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceProteinSubunit {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Index of primary sequences of amino acids linked through peptide bonds in order of decreasing length. Sequences of the same length will be ordered by molecular weight. Subunits that have identical sequences will be repeated and have sequential subscripts."]
    pub r#subunit: Option<super::super::types::Integer>,
    #[doc = "The sequence information shall be provided enumerating the amino acids from N- to C-terminal end using standard single-letter amino acid codes. Uppercase shall be used for L-amino acids and lowercase for D-amino acids. Transcribed SubstanceProteins will always be described using the translated sequence; for synthetic peptide containing amino acids that are not represented with a single letter code an X should be used within the sequence. The modified amino acids will be distinguished by their position in the sequence."]
    pub r#sequence: Option<super::super::types::String>,
    #[doc = "Length of linear sequences of amino acids contained in the subunit."]
    pub r#length: Option<super::super::types::Integer>,
    #[doc = "The sequence information shall be provided enumerating the amino acids from N- to C-terminal end using standard single-letter amino acid codes. Uppercase shall be used for L-amino acids and lowercase for D-amino acids. Transcribed SubstanceProteins will always be described using the translated sequence; for synthetic peptide containing amino acids that are not represented with a single letter code an X should be used within the sequence. The modified amino acids will be distinguished by their position in the sequence."]
    pub r#sequence_attachment: Option<Box<super::super::types::Attachment>>,
    #[doc = "Unique identifier for molecular fragment modification based on the ISO 11238 Substance ID."]
    pub r#n_terminal_modification_id: Option<Box<super::super::types::Identifier>>,
    #[doc = "The name of the fragment modified at the N-terminal of the SubstanceProtein shall be specified."]
    pub r#n_terminal_modification: Option<super::super::types::String>,
    #[doc = "Unique identifier for molecular fragment modification based on the ISO 11238 Substance ID."]
    pub r#c_terminal_modification_id: Option<Box<super::super::types::Identifier>>,
    #[doc = "The modification at the C-terminal shall be specified."]
    pub r#c_terminal_modification: Option<super::super::types::String>,
}
impl crate::AnyResource for SubstanceProteinSubunit {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for SubstanceProteinSubunit {
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
            if _ctx.output_json {
                if let Some(some) = self.r#subunit.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("subunit", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_subunit", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#subunit.as_ref() {
                    state.serialize_entry("subunit", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#sequence.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sequence", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sequence", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#sequence.as_ref() {
                    state.serialize_entry("sequence", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#length.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("length", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_length", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#length.as_ref() {
                    state.serialize_entry("length", some)?;
                }
            }
            if let Some(some) = self.r#sequence_attachment.as_ref() {
                state.serialize_entry("sequenceAttachment", some)?;
            }
            if let Some(some) = self.r#n_terminal_modification_id.as_ref() {
                state.serialize_entry("nTerminalModificationId", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#n_terminal_modification.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("nTerminalModification", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_nTerminalModification", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#n_terminal_modification.as_ref() {
                    state.serialize_entry("nTerminalModification", some)?;
                }
            }
            if let Some(some) = self.r#c_terminal_modification_id.as_ref() {
                state.serialize_entry("cTerminalModificationId", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#c_terminal_modification.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("cTerminalModification", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_cTerminalModification", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#c_terminal_modification.as_ref() {
                    state.serialize_entry("cTerminalModification", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceProteinSubunit {
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
            #[serde(rename = "subunit")]
            Subunit,
            #[serde(rename = "_subunit")]
            SubunitPrimitiveElement,
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "length")]
            Length,
            #[serde(rename = "_length")]
            LengthPrimitiveElement,
            #[serde(rename = "sequenceAttachment")]
            SequenceAttachment,
            #[serde(rename = "nTerminalModificationId")]
            NTerminalModificationId,
            #[serde(rename = "nTerminalModification")]
            NTerminalModification,
            #[serde(rename = "_nTerminalModification")]
            NTerminalModificationPrimitiveElement,
            #[serde(rename = "cTerminalModificationId")]
            CTerminalModificationId,
            #[serde(rename = "cTerminalModification")]
            CTerminalModification,
            #[serde(rename = "_cTerminalModification")]
            CTerminalModificationPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceProteinSubunit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceProteinSubunit")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceProteinSubunit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#subunit: Option<super::super::types::Integer> = None;
                let mut r#sequence: Option<super::super::types::String> = None;
                let mut r#length: Option<super::super::types::Integer> = None;
                let mut r#sequence_attachment: Option<Box<super::super::types::Attachment>> = None;
                let mut r#n_terminal_modification_id: Option<Box<super::super::types::Identifier>> =
                    None;
                let mut r#n_terminal_modification: Option<super::super::types::String> = None;
                let mut r#c_terminal_modification_id: Option<Box<super::super::types::Identifier>> =
                    None;
                let mut r#c_terminal_modification: Option<super::super::types::String> = None;
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
                            Field::Subunit => {
                                let some = r#subunit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subunit"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SubunitPrimitiveElement => {
                                let some = r#subunit.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_subunit"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Sequence => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SequencePrimitiveElement => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Length => {
                                let some = r#length.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("length"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::LengthPrimitiveElement => {
                                let some = r#length.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_length"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::SequenceAttachment => {
                                if r#sequence_attachment.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "sequenceAttachment",
                                    ));
                                }
                                r#sequence_attachment = Some(map_access.next_value()?);
                            }
                            Field::NTerminalModificationId => {
                                if r#n_terminal_modification_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "nTerminalModificationId",
                                    ));
                                }
                                r#n_terminal_modification_id = Some(map_access.next_value()?);
                            }
                            Field::NTerminalModification => {
                                let some =
                                    r#n_terminal_modification.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "nTerminalModification",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::NTerminalModificationPrimitiveElement => {
                                let some =
                                    r#n_terminal_modification.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_nTerminalModification",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::CTerminalModificationId => {
                                if r#c_terminal_modification_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "cTerminalModificationId",
                                    ));
                                }
                                r#c_terminal_modification_id = Some(map_access.next_value()?);
                            }
                            Field::CTerminalModification => {
                                let some =
                                    r#c_terminal_modification.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "cTerminalModification",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::CTerminalModificationPrimitiveElement => {
                                let some =
                                    r#c_terminal_modification.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_cTerminalModification",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
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
                                        "subunit",
                                        "sequence",
                                        "length",
                                        "sequenceAttachment",
                                        "nTerminalModificationId",
                                        "nTerminalModification",
                                        "cTerminalModificationId",
                                        "cTerminalModification",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceProteinSubunit {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#subunit,
                        r#sequence,
                        r#length,
                        r#sequence_attachment,
                        r#n_terminal_modification_id,
                        r#n_terminal_modification,
                        r#c_terminal_modification_id,
                        r#c_terminal_modification,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A SubstanceProtein is defined as a single unit of a linear amino acid sequence, or a combination of subunits that are either covalently linked or have a defined invariant stoichiometric relationship. This includes all synthetic, recombinant and purified SubstanceProteins of defined sequence, whether the use is therapeutic or prophylactic. This set of elements will be used to describe albumins, coagulation factors, cytokines, growth factors, peptide/SubstanceProtein hormones, enzymes, toxins, toxoids, recombinant vaccines, and immunomodulators."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceProtein {
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
    #[doc = "The SubstanceProtein descriptive elements will only be used when a complete or partial amino acid sequence is available or derivable from a nucleic acid sequence."]
    pub r#sequence_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Number of linear sequences of amino acids linked through peptide bonds. The number of subunits constituting the SubstanceProtein shall be described. It is possible that the number of subunits can be variable."]
    pub r#number_of_subunits: Option<super::super::types::Integer>,
    #[doc = "The disulphide bond between two cysteine residues either on the same subunit or on two different subunits shall be described. The position of the disulfide bonds in the SubstanceProtein shall be listed in increasing order of subunit number and position within subunit followed by the abbreviation of the amino acids involved. The disulfide linkage positions shall actually contain the amino acid Cysteine at the respective positions."]
    pub r#disulfide_linkage: Vec<super::super::types::String>,
    #[doc = "This subclause refers to the description of each subunit constituting the SubstanceProtein. A subunit is a linear sequence of amino acids linked through peptide bonds. The Subunit information shall be provided when the finished SubstanceProtein is a complex of multiple sequences; subunits are not used to delineate domains within a single sequence. Subunits are listed in order of decreasing length; sequences of the same length will be ordered by decreasing molecular weight; subunits that have identical sequences will be repeated multiple times."]
    pub r#subunit: Vec<SubstanceProteinSubunit>,
}
impl serde::ser::Serialize for SubstanceProtein {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "SubstanceProtein")?;
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
            if let Some(some) = self.r#sequence_type.as_ref() {
                state.serialize_entry("sequenceType", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number_of_subunits.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("numberOfSubunits", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_numberOfSubunits", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number_of_subunits.as_ref() {
                    state.serialize_entry("numberOfSubunits", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#disulfide_linkage.is_empty() {
                    let values = self
                        .r#disulfide_linkage
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("disulfideLinkage", &values)?;
                    }
                    let requires_elements = self
                        .r#disulfide_linkage
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#disulfide_linkage
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
                        state.serialize_entry("_disulfideLinkage", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#disulfide_linkage.is_empty() {
                    state.serialize_entry("disulfideLinkage", &self.r#disulfide_linkage)?;
                }
            }
            if !self.r#subunit.is_empty() {
                state.serialize_entry("subunit", &self.r#subunit)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceProtein {
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
            #[serde(rename = "sequenceType")]
            SequenceType,
            #[serde(rename = "numberOfSubunits")]
            NumberOfSubunits,
            #[serde(rename = "_numberOfSubunits")]
            NumberOfSubunitsPrimitiveElement,
            #[serde(rename = "disulfideLinkage")]
            DisulfideLinkage,
            #[serde(rename = "_disulfideLinkage")]
            DisulfideLinkagePrimitiveElement,
            #[serde(rename = "subunit")]
            Subunit,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceProtein;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceProtein")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceProtein, V::Error>
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
                let mut r#sequence_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#number_of_subunits: Option<super::super::types::Integer> = None;
                let mut r#disulfide_linkage: Option<Vec<super::super::types::String>> = None;
                let mut r#subunit: Option<Vec<SubstanceProteinSubunit>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "SubstanceProtein" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"SubstanceProtein",
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
                            Field::SequenceType => {
                                if r#sequence_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequenceType"));
                                }
                                r#sequence_type = Some(map_access.next_value()?);
                            }
                            Field::NumberOfSubunits => {
                                let some = r#number_of_subunits.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "numberOfSubunits",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::NumberOfSubunitsPrimitiveElement => {
                                let some = r#number_of_subunits.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_numberOfSubunits",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::DisulfideLinkage => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#disulfide_linkage.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "disulfideLinkage",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::DisulfideLinkagePrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#disulfide_linkage.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_disulfideLinkage",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::Subunit => {
                                if r#subunit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subunit"));
                                }
                                r#subunit = Some(map_access.next_value()?);
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
                                        "sequenceType",
                                        "numberOfSubunits",
                                        "disulfideLinkage",
                                        "subunit",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceProtein {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence_type,
                        r#number_of_subunits,
                        r#disulfide_linkage: r#disulfide_linkage.unwrap_or(vec![]),
                        r#subunit: r#subunit.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
