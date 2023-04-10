// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "A sequence that is used as a reference to describe variants that are present in a sequence analyzed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MolecularSequenceReferenceSeq {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Structural unit composed of a nucleic acid molecule which controls its own replication through the interaction of specific proteins at one or more origins of replication ([SO:0000340](<http://www.sequenceontology.org/browser/current_svn/term/SO:0000340>))."]
    pub r#chromosome: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The Genome Build used for reference, following GRCh build versions e.g. 'GRCh 37'.  Version number must be included if a versioned release of a primary build was used."]
    pub r#genome_build: Option<super::super::types::String>,
    #[doc = "A relative reference to a DNA strand based on gene orientation. The strand that contains the open reading frame of the gene is the \"sense\" strand, and the opposite complementary strand is the \"antisense\" strand."]
    pub r#orientation: Option<super::super::types::Code>,
    #[doc = "Reference identifier of reference sequence submitted to NCBI. It must match the type in the MolecularSequence.type field. For example, the prefix, “NG_” identifies reference sequence for genes, “NM_” for messenger RNA transcripts, and “NP_” for amino acid sequences."]
    pub r#reference_seq_id: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A pointer to another MolecularSequence entity as reference sequence."]
    pub r#reference_seq_pointer: Option<Box<super::super::types::Reference>>,
    #[doc = "A string like \"ACGT\"."]
    pub r#reference_seq_string: Option<super::super::types::String>,
    #[doc = "An absolute reference to a strand. The Watson strand is the strand whose 5'-end is on the short arm of the chromosome, and the Crick strand as the one whose 5'-end is on the long arm."]
    pub r#strand: Option<super::super::types::Code>,
    #[doc = "Start position of the window on the reference sequence. If the coordinate system is either 0-based or 1-based, then start position is inclusive."]
    pub r#window_start: Option<super::super::types::Integer>,
    #[doc = "End position of the window on the reference sequence. If the coordinate system is 0-based then end is exclusive and does not include the last position. If the coordinate system is 1-base, then end is inclusive and includes the last position."]
    pub r#window_end: Option<super::super::types::Integer>,
}
impl serde::ser::Serialize for MolecularSequenceReferenceSeq {
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
            if let Some(some) = self.r#chromosome.as_ref() {
                state.serialize_entry("chromosome", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#genome_build.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("genomeBuild", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_genomeBuild", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#genome_build.as_ref() {
                    state.serialize_entry("genomeBuild", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#orientation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("orientation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_orientation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#orientation.as_ref() {
                    state.serialize_entry("orientation", some)?;
                }
            }
            if let Some(some) = self.r#reference_seq_id.as_ref() {
                state.serialize_entry("referenceSeqId", some)?;
            }
            if let Some(some) = self.r#reference_seq_pointer.as_ref() {
                state.serialize_entry("referenceSeqPointer", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#reference_seq_string.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("referenceSeqString", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_referenceSeqString", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#reference_seq_string.as_ref() {
                    state.serialize_entry("referenceSeqString", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#strand.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("strand", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_strand", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#strand.as_ref() {
                    state.serialize_entry("strand", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#window_start.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("windowStart", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_windowStart", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#window_start.as_ref() {
                    state.serialize_entry("windowStart", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#window_end.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("windowEnd", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_windowEnd", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#window_end.as_ref() {
                    state.serialize_entry("windowEnd", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "The definition of variant here originates from Sequence ontology ([variant_of](<http://www.sequenceontology.org/browser/current_svn/term/variant_of>)). This element can represent amino acid or nucleic sequence change(including insertion,deletion,SNP,etc.)  It can represent some complex mutation or segment variation with the assist of CIGAR string."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MolecularSequenceVariant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Start position of the variant on the  reference sequence. If the coordinate system is either 0-based or 1-based, then start position is inclusive."]
    pub r#start: Option<super::super::types::Integer>,
    #[doc = "End position of the variant on the reference sequence. If the coordinate system is 0-based then end is exclusive and does not include the last position. If the coordinate system is 1-base, then end is inclusive and includes the last position."]
    pub r#end: Option<super::super::types::Integer>,
    #[doc = "An allele is one of a set of coexisting sequence variants of a gene ([SO:0001023](<http://www.sequenceontology.org/browser/current_svn/term/SO:0001023>)).  Nucleotide(s)/amino acids from start position of sequence to stop position of sequence on the positive (+) strand of the observed  sequence. When the sequence  type is DNA, it should be the sequence on the positive (+) strand. This will lay in the range between variant.start and variant.end."]
    pub r#observed_allele: Option<super::super::types::String>,
    #[doc = "An allele is one of a set of coexisting sequence variants of a gene ([SO:0001023](<http://www.sequenceontology.org/browser/current_svn/term/SO:0001023>)). Nucleotide(s)/amino acids from start position of sequence to stop position of sequence on the positive (+) strand of the reference sequence. When the sequence  type is DNA, it should be the sequence on the positive (+) strand. This will lay in the range between variant.start and variant.end."]
    pub r#reference_allele: Option<super::super::types::String>,
    #[doc = "Extended CIGAR string for aligning the sequence with reference bases. See detailed documentation [here](<http://support.illumina.com/help/SequencingAnalysisWorkflow/Content/Vault/Informatics/Sequencing_Analysis/CASAVA/swSEQ_mCA_ExtendedCIGARFormat.htm>)."]
    pub r#cigar: Option<super::super::types::String>,
    #[doc = "A pointer to an Observation containing variant information."]
    pub r#variant_pointer: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MolecularSequenceVariant {
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
                if let Some(some) = self.r#start.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("start", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_start", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#start.as_ref() {
                    state.serialize_entry("start", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#end.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("end", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_end", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#end.as_ref() {
                    state.serialize_entry("end", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#observed_allele.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("observedAllele", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_observedAllele", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#observed_allele.as_ref() {
                    state.serialize_entry("observedAllele", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#reference_allele.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("referenceAllele", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_referenceAllele", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#reference_allele.as_ref() {
                    state.serialize_entry("referenceAllele", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#cigar.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("cigar", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_cigar", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#cigar.as_ref() {
                    state.serialize_entry("cigar", some)?;
                }
            }
            if let Some(some) = self.r#variant_pointer.as_ref() {
                state.serialize_entry("variantPointer", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Receiver Operator Characteristic (ROC) Curve  to give sensitivity/specificity tradeoff."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MolecularSequenceQualityRoc {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Invidual data point representing the GQ (genotype quality) score threshold."]
    pub r#score: Vec<super::super::types::Integer>,
    #[doc = "The number of true positives if the GQ score threshold was set to \"score\" field value."]
    pub r#num_tp: Vec<super::super::types::Integer>,
    #[doc = "The number of false positives if the GQ score threshold was set to \"score\" field value."]
    pub r#num_fp: Vec<super::super::types::Integer>,
    #[doc = "The number of false negatives if the GQ score threshold was set to \"score\" field value."]
    pub r#num_fn: Vec<super::super::types::Integer>,
    #[doc = "Calculated precision if the GQ score threshold was set to \"score\" field value."]
    pub r#precision: Vec<super::super::types::Decimal>,
    #[doc = "Calculated sensitivity if the GQ score threshold was set to \"score\" field value."]
    pub r#sensitivity: Vec<super::super::types::Decimal>,
    #[doc = "Calculated fScore if the GQ score threshold was set to \"score\" field value."]
    pub r#f_measure: Vec<super::super::types::Decimal>,
}
impl serde::ser::Serialize for MolecularSequenceQualityRoc {
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
                if !self.r#score.is_empty() {
                    let values = self
                        .r#score
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("score", &values)?;
                    }
                    let requires_elements = self
                        .r#score
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#score
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
                        state.serialize_entry("_score", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#score.is_empty() {
                    state.serialize_entry("score", &self.r#score)?;
                }
            }
            if _ctx.output_json {
                if !self.r#num_tp.is_empty() {
                    let values = self
                        .r#num_tp
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("numTP", &values)?;
                    }
                    let requires_elements = self
                        .r#num_tp
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#num_tp
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
                        state.serialize_entry("_numTP", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#num_tp.is_empty() {
                    state.serialize_entry("numTP", &self.r#num_tp)?;
                }
            }
            if _ctx.output_json {
                if !self.r#num_fp.is_empty() {
                    let values = self
                        .r#num_fp
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("numFP", &values)?;
                    }
                    let requires_elements = self
                        .r#num_fp
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#num_fp
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
                        state.serialize_entry("_numFP", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#num_fp.is_empty() {
                    state.serialize_entry("numFP", &self.r#num_fp)?;
                }
            }
            if _ctx.output_json {
                if !self.r#num_fn.is_empty() {
                    let values = self
                        .r#num_fn
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("numFN", &values)?;
                    }
                    let requires_elements = self
                        .r#num_fn
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#num_fn
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
                        state.serialize_entry("_numFN", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#num_fn.is_empty() {
                    state.serialize_entry("numFN", &self.r#num_fn)?;
                }
            }
            if _ctx.output_json {
                if !self.r#precision.is_empty() {
                    let values = self
                        .r#precision
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| {
                            v.as_ref()
                                .map(|some| {
                                    some.parse::<serde_json::Number>().map_err(|_| {
                                        serde::ser::Error::custom("error serializing decimal")
                                    })
                                })
                                .transpose()
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("precision", &values)?;
                    }
                    let requires_elements = self
                        .r#precision
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#precision
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
                        state.serialize_entry("_precision", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#precision.is_empty() {
                    state.serialize_entry("precision", &self.r#precision)?;
                }
            }
            if _ctx.output_json {
                if !self.r#sensitivity.is_empty() {
                    let values = self
                        .r#sensitivity
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| {
                            v.as_ref()
                                .map(|some| {
                                    some.parse::<serde_json::Number>().map_err(|_| {
                                        serde::ser::Error::custom("error serializing decimal")
                                    })
                                })
                                .transpose()
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("sensitivity", &values)?;
                    }
                    let requires_elements = self
                        .r#sensitivity
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#sensitivity
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
                        state.serialize_entry("_sensitivity", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#sensitivity.is_empty() {
                    state.serialize_entry("sensitivity", &self.r#sensitivity)?;
                }
            }
            if _ctx.output_json {
                if !self.r#f_measure.is_empty() {
                    let values = self
                        .r#f_measure
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| {
                            v.as_ref()
                                .map(|some| {
                                    some.parse::<serde_json::Number>().map_err(|_| {
                                        serde::ser::Error::custom("error serializing decimal")
                                    })
                                })
                                .transpose()
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("fMeasure", &values)?;
                    }
                    let requires_elements = self
                        .r#f_measure
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#f_measure
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
                        state.serialize_entry("_fMeasure", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#f_measure.is_empty() {
                    state.serialize_entry("fMeasure", &self.r#f_measure)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "An experimental feature attribute that defines the quality of the feature in a quantitative way, such as a phred quality score ([SO:0001686](<http://www.sequenceontology.org/browser/current_svn/term/SO:0001686>))."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MolecularSequenceQuality {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "INDEL / SNP / Undefined variant."]
    pub r#type: super::super::types::Code,
    #[doc = "Gold standard sequence used for comparing against."]
    pub r#standard_sequence: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Start position of the sequence. If the coordinate system is either 0-based or 1-based, then start position is inclusive."]
    pub r#start: Option<super::super::types::Integer>,
    #[doc = "End position of the sequence. If the coordinate system is 0-based then end is exclusive and does not include the last position. If the coordinate system is 1-base, then end is inclusive and includes the last position."]
    pub r#end: Option<super::super::types::Integer>,
    #[doc = "The score of an experimentally derived feature such as a p-value ([SO:0001685](<http://www.sequenceontology.org/browser/current_svn/term/SO:0001685>))."]
    pub r#score: Option<Box<super::super::types::Quantity>>,
    #[doc = "Which method is used to get sequence quality."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "True positives, from the perspective of the truth data, i.e. the number of sites in the Truth Call Set for which there are paths through the Query Call Set that are consistent with all of the alleles at this site, and for which there is an accurate genotype call for the event."]
    pub r#truth_tp: Option<super::super::types::Decimal>,
    #[doc = "True positives, from the perspective of the query data, i.e. the number of sites in the Query Call Set for which there are paths through the Truth Call Set that are consistent with all of the alleles at this site, and for which there is an accurate genotype call for the event."]
    pub r#query_tp: Option<super::super::types::Decimal>,
    #[doc = "False negatives, i.e. the number of sites in the Truth Call Set for which there is no path through the Query Call Set that is consistent with all of the alleles at this site, or sites for which there is an inaccurate genotype call for the event. Sites with correct variant but incorrect genotype are counted here."]
    pub r#truth_fn: Option<super::super::types::Decimal>,
    #[doc = "False positives, i.e. the number of sites in the Query Call Set for which there is no path through the Truth Call Set that is consistent with this site. Sites with correct variant but incorrect genotype are counted here."]
    pub r#query_fp: Option<super::super::types::Decimal>,
    #[doc = "The number of false positives where the non-REF alleles in the Truth and Query Call Sets match (i.e. cases where the truth is 1/1 and the query is 0/1 or similar)."]
    pub r#gt_fp: Option<super::super::types::Decimal>,
    #[doc = "QUERY.TP / (QUERY.TP + QUERY.FP)."]
    pub r#precision: Option<super::super::types::Decimal>,
    #[doc = "TRUTH.TP / (TRUTH.TP + TRUTH.FN)."]
    pub r#recall: Option<super::super::types::Decimal>,
    #[doc = "Harmonic mean of Recall and Precision, computed as: 2 * precision * recall / (precision + recall)."]
    pub r#f_score: Option<super::super::types::Decimal>,
    #[doc = "Receiver Operator Characteristic (ROC) Curve  to give sensitivity/specificity tradeoff."]
    pub r#roc: Option<MolecularSequenceQualityRoc>,
}
impl serde::ser::Serialize for MolecularSequenceQuality {
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
            if let Some(some) = self.r#standard_sequence.as_ref() {
                state.serialize_entry("standardSequence", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#start.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("start", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_start", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#start.as_ref() {
                    state.serialize_entry("start", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#end.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("end", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_end", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#end.as_ref() {
                    state.serialize_entry("end", some)?;
                }
            }
            if let Some(some) = self.r#score.as_ref() {
                state.serialize_entry("score", some)?;
            }
            if let Some(some) = self.r#method.as_ref() {
                state.serialize_entry("method", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#truth_tp.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("truthTP", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_truthTP", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#truth_tp.as_ref() {
                    state.serialize_entry("truthTP", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#query_tp.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("queryTP", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_queryTP", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#query_tp.as_ref() {
                    state.serialize_entry("queryTP", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#truth_fn.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("truthFN", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_truthFN", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#truth_fn.as_ref() {
                    state.serialize_entry("truthFN", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#query_fp.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("queryFP", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_queryFP", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#query_fp.as_ref() {
                    state.serialize_entry("queryFP", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#gt_fp.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("gtFP", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_gtFP", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#gt_fp.as_ref() {
                    state.serialize_entry("gtFP", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#precision.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("precision", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_precision", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#precision.as_ref() {
                    state.serialize_entry("precision", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#recall.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("recall", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_recall", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#recall.as_ref() {
                    state.serialize_entry("recall", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#f_score.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("fScore", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_fScore", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#f_score.as_ref() {
                    state.serialize_entry("fScore", some)?;
                }
            }
            if let Some(some) = self.r#roc.as_ref() {
                state.serialize_entry("roc", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Configurations of the external repository. The repository shall store target's observedSeq or records related with target's observedSeq."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MolecularSequenceRepository {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Click and see / RESTful API / Need login to see / RESTful API with authentication / Other ways to see resource."]
    pub r#type: super::super::types::Code,
    #[doc = "URI of an external repository which contains further details about the genetics data."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "URI of an external repository which contains further details about the genetics data."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Id of the variant in this external repository. The server will understand how to use this id to call for more info about datasets in external repository."]
    pub r#dataset_id: Option<super::super::types::String>,
    #[doc = "Id of the variantset in this external repository. The server will understand how to use this id to call for more info about variantsets in external repository."]
    pub r#variantset_id: Option<super::super::types::String>,
    #[doc = "Id of the read in this external repository."]
    pub r#readset_id: Option<super::super::types::String>,
}
impl serde::ser::Serialize for MolecularSequenceRepository {
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
                if let Some(some) = self.r#dataset_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("datasetId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_datasetId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#dataset_id.as_ref() {
                    state.serialize_entry("datasetId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#variantset_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("variantsetId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_variantsetId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#variantset_id.as_ref() {
                    state.serialize_entry("variantsetId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#readset_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("readsetId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_readsetId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#readset_id.as_ref() {
                    state.serialize_entry("readsetId", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Structural variant outer."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MolecularSequenceStructureVariantOuter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Structural variant outer start. If the coordinate system is either 0-based or 1-based, then start position is inclusive."]
    pub r#start: Option<super::super::types::Integer>,
    #[doc = "Structural variant outer end. If the coordinate system is 0-based then end is exclusive and does not include the last position. If the coordinate system is 1-base, then end is inclusive and includes the last position."]
    pub r#end: Option<super::super::types::Integer>,
}
impl serde::ser::Serialize for MolecularSequenceStructureVariantOuter {
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
                if let Some(some) = self.r#start.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("start", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_start", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#start.as_ref() {
                    state.serialize_entry("start", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#end.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("end", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_end", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#end.as_ref() {
                    state.serialize_entry("end", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Structural variant inner."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MolecularSequenceStructureVariantInner {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Structural variant inner start. If the coordinate system is either 0-based or 1-based, then start position is inclusive."]
    pub r#start: Option<super::super::types::Integer>,
    #[doc = "Structural variant inner end. If the coordinate system is 0-based then end is exclusive and does not include the last position. If the coordinate system is 1-base, then end is inclusive and includes the last position."]
    pub r#end: Option<super::super::types::Integer>,
}
impl serde::ser::Serialize for MolecularSequenceStructureVariantInner {
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
                if let Some(some) = self.r#start.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("start", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_start", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#start.as_ref() {
                    state.serialize_entry("start", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#end.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("end", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_end", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#end.as_ref() {
                    state.serialize_entry("end", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Information about chromosome structure variation."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MolecularSequenceStructureVariant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Information about chromosome structure variation DNA change type."]
    pub r#variant_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to indicate if the outer and inner start-end values have the same meaning."]
    pub r#exact: Option<super::super::types::Boolean>,
    #[doc = "Length of the variant chromosome."]
    pub r#length: Option<super::super::types::Integer>,
    #[doc = "Structural variant outer."]
    pub r#outer: Option<MolecularSequenceStructureVariantOuter>,
    #[doc = "Structural variant inner."]
    pub r#inner: Option<MolecularSequenceStructureVariantInner>,
}
impl serde::ser::Serialize for MolecularSequenceStructureVariant {
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
            if let Some(some) = self.r#variant_type.as_ref() {
                state.serialize_entry("variantType", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#exact.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("exact", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_exact", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#exact.as_ref() {
                    state.serialize_entry("exact", some)?;
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
            if let Some(some) = self.r#outer.as_ref() {
                state.serialize_entry("outer", some)?;
            }
            if let Some(some) = self.r#inner.as_ref() {
                state.serialize_entry("inner", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Raw data describing a biological sequence."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MolecularSequence {
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
    #[doc = "A unique identifier for this particular sequence instance. This is a FHIR-defined id."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Amino Acid Sequence/ DNA Sequence / RNA Sequence."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "Whether the sequence is numbered starting at 0 (0-based numbering or coordinates, inclusive start, exclusive end) or starting at 1 (1-based numbering, inclusive start and inclusive end)."]
    pub r#coordinate_system: super::super::types::Integer,
    #[doc = "The patient whose sequencing results are described by this resource."]
    pub r#patient: Option<Box<super::super::types::Reference>>,
    #[doc = "Specimen used for sequencing."]
    pub r#specimen: Option<Box<super::super::types::Reference>>,
    #[doc = "The method for sequencing, for example, chip information."]
    pub r#device: Option<Box<super::super::types::Reference>>,
    #[doc = "The organization or lab that should be responsible for this result."]
    pub r#performer: Option<Box<super::super::types::Reference>>,
    #[doc = "The number of copies of the sequence of interest. (RNASeq)."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "A sequence that is used as a reference to describe variants that are present in a sequence analyzed."]
    pub r#reference_seq: Option<MolecularSequenceReferenceSeq>,
    #[doc = "The definition of variant here originates from Sequence ontology ([variant_of](<http://www.sequenceontology.org/browser/current_svn/term/variant_of>)). This element can represent amino acid or nucleic sequence change(including insertion,deletion,SNP,etc.)  It can represent some complex mutation or segment variation with the assist of CIGAR string."]
    pub r#variant: Vec<MolecularSequenceVariant>,
    #[doc = "Sequence that was observed. It is the result marked by referenceSeq along with variant records on referenceSeq. This shall start from referenceSeq.windowStart and end by referenceSeq.windowEnd."]
    pub r#observed_seq: Option<super::super::types::String>,
    #[doc = "An experimental feature attribute that defines the quality of the feature in a quantitative way, such as a phred quality score ([SO:0001686](<http://www.sequenceontology.org/browser/current_svn/term/SO:0001686>))."]
    pub r#quality: Vec<MolecularSequenceQuality>,
    #[doc = "Coverage (read depth or depth) is the average number of reads representing a given nucleotide in the reconstructed sequence."]
    pub r#read_coverage: Option<super::super::types::Integer>,
    #[doc = "Configurations of the external repository. The repository shall store target's observedSeq or records related with target's observedSeq."]
    pub r#repository: Vec<MolecularSequenceRepository>,
    #[doc = "Pointer to next atomic sequence which at most contains one variant."]
    pub r#pointer: Vec<Box<super::super::types::Reference>>,
    #[doc = "Information about chromosome structure variation."]
    pub r#structure_variant: Vec<MolecularSequenceStructureVariant>,
}
impl crate::AnyResource for MolecularSequence {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for MolecularSequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "MolecularSequence")?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#coordinate_system.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("coordinateSystem", &some)?;
                }
                if self.r#coordinate_system.id.is_some()
                    || !self.r#coordinate_system.extension.is_empty()
                {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#coordinate_system.id.as_ref(),
                        extension: &self.r#coordinate_system.extension,
                    };
                    state.serialize_entry("_coordinateSystem", &primitive_element)?;
                }
            } else {
                state.serialize_entry("coordinateSystem", &self.r#coordinate_system)?;
            }
            if let Some(some) = self.r#patient.as_ref() {
                state.serialize_entry("patient", some)?;
            }
            if let Some(some) = self.r#specimen.as_ref() {
                state.serialize_entry("specimen", some)?;
            }
            if let Some(some) = self.r#device.as_ref() {
                state.serialize_entry("device", some)?;
            }
            if let Some(some) = self.r#performer.as_ref() {
                state.serialize_entry("performer", some)?;
            }
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            if let Some(some) = self.r#reference_seq.as_ref() {
                state.serialize_entry("referenceSeq", some)?;
            }
            if !self.r#variant.is_empty() {
                state.serialize_entry("variant", &self.r#variant)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#observed_seq.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("observedSeq", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_observedSeq", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#observed_seq.as_ref() {
                    state.serialize_entry("observedSeq", some)?;
                }
            }
            if !self.r#quality.is_empty() {
                state.serialize_entry("quality", &self.r#quality)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#read_coverage.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("readCoverage", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_readCoverage", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#read_coverage.as_ref() {
                    state.serialize_entry("readCoverage", some)?;
                }
            }
            if !self.r#repository.is_empty() {
                state.serialize_entry("repository", &self.r#repository)?;
            }
            if !self.r#pointer.is_empty() {
                state.serialize_entry("pointer", &self.r#pointer)?;
            }
            if !self.r#structure_variant.is_empty() {
                state.serialize_entry("structureVariant", &self.r#structure_variant)?;
            }
            state.end()
        })
    }
}
