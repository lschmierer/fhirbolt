// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "A sequence that is used as a reference to describe variants that are present in a sequence analyzed."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceReferenceSeq {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
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
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequenceReferenceSeq {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#chromosome: Default::default(),
            r#genome_build: Default::default(),
            r#orientation: Default::default(),
            r#reference_seq_id: Default::default(),
            r#reference_seq_pointer: Default::default(),
            r#reference_seq_string: Default::default(),
            r#strand: Default::default(),
            r#window_start: Default::default(),
            r#window_end: Default::default(),
        }
    }
}
#[doc = "The definition of variant here originates from Sequence ontology ([variant_of](<http://www.sequenceontology.org/browser/current_svn/term/variant_of>)). This element can represent amino acid or nucleic sequence change(including insertion,deletion,SNP,etc.)  It can represent some complex mutation or segment variation with the assist of CIGAR string."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceVariant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
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
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequenceVariant {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#start: Default::default(),
            r#end: Default::default(),
            r#observed_allele: Default::default(),
            r#reference_allele: Default::default(),
            r#cigar: Default::default(),
            r#variant_pointer: Default::default(),
        }
    }
}
#[doc = "Receiver Operator Characteristic (ROC) Curve  to give sensitivity/specificity tradeoff."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceQualityRoc {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
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
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequenceQualityRoc {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#score: Default::default(),
            r#num_tp: Default::default(),
            r#num_fp: Default::default(),
            r#num_fn: Default::default(),
            r#precision: Default::default(),
            r#sensitivity: Default::default(),
            r#f_measure: Default::default(),
        }
    }
}
#[doc = "An experimental feature attribute that defines the quality of the feature in a quantitative way, such as a phred quality score ([SO:0001686](<http://www.sequenceontology.org/browser/current_svn/term/SO:0001686>))."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceQuality {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
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
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequenceQuality {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#standard_sequence: Default::default(),
            r#start: Default::default(),
            r#end: Default::default(),
            r#score: Default::default(),
            r#method: Default::default(),
            r#truth_tp: Default::default(),
            r#query_tp: Default::default(),
            r#truth_fn: Default::default(),
            r#query_fp: Default::default(),
            r#gt_fp: Default::default(),
            r#precision: Default::default(),
            r#recall: Default::default(),
            r#f_score: Default::default(),
            r#roc: Default::default(),
        }
    }
}
#[doc = "Configurations of the external repository. The repository shall store target's observedSeq or records related with target's observedSeq."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceRepository {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
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
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequenceRepository {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#url: Default::default(),
            r#name: Default::default(),
            r#dataset_id: Default::default(),
            r#variantset_id: Default::default(),
            r#readset_id: Default::default(),
        }
    }
}
#[doc = "Structural variant outer."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceStructureVariantOuter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Structural variant outer start. If the coordinate system is either 0-based or 1-based, then start position is inclusive."]
    pub r#start: Option<super::super::types::Integer>,
    #[doc = "Structural variant outer end. If the coordinate system is 0-based then end is exclusive and does not include the last position. If the coordinate system is 1-base, then end is inclusive and includes the last position."]
    pub r#end: Option<super::super::types::Integer>,
}
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequenceStructureVariantOuter {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#start: Default::default(),
            r#end: Default::default(),
        }
    }
}
#[doc = "Structural variant inner."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceStructureVariantInner {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Structural variant inner start. If the coordinate system is either 0-based or 1-based, then start position is inclusive."]
    pub r#start: Option<super::super::types::Integer>,
    #[doc = "Structural variant inner end. If the coordinate system is 0-based then end is exclusive and does not include the last position. If the coordinate system is 1-base, then end is inclusive and includes the last position."]
    pub r#end: Option<super::super::types::Integer>,
}
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequenceStructureVariantInner {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#start: Default::default(),
            r#end: Default::default(),
        }
    }
}
#[doc = "Information about chromosome structure variation."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceStructureVariant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
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
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequenceStructureVariant {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#variant_type: Default::default(),
            r#exact: Default::default(),
            r#length: Default::default(),
            r#outer: Default::default(),
            r#inner: Default::default(),
        }
    }
}
#[doc = "Raw data describing a biological sequence."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequence {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<Box<super::super::types::Id>>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A unique identifier for this particular sequence instance. This is a FHIR-defined id."]
    pub r#identifier: Vec<super::super::types::Identifier>,
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
    pub r#pointer: Vec<super::super::types::Reference>,
    #[doc = "Information about chromosome structure variation."]
    pub r#structure_variant: Vec<MolecularSequenceStructureVariant>,
}
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequence {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#text: Default::default(),
            r#contained: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#type: Default::default(),
            r#coordinate_system: super::super::types::Integer {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#patient: Default::default(),
            r#specimen: Default::default(),
            r#device: Default::default(),
            r#performer: Default::default(),
            r#quantity: Default::default(),
            r#reference_seq: Default::default(),
            r#variant: Default::default(),
            r#observed_seq: Default::default(),
            r#quality: Default::default(),
            r#read_coverage: Default::default(),
            r#repository: Default::default(),
            r#pointer: Default::default(),
            r#structure_variant: Default::default(),
        }
    }
}
