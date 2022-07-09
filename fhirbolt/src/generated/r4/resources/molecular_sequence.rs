// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct MolecularSequenceReferenceSeq {
    pub r#reference_seq_pointer: Option<Box<super::super::types::Reference>>,
    pub r#window_end: Option<super::super::types::Integer>,
    pub r#id: Option<std::string::String>,
    pub r#orientation: Option<super::super::types::Code>,
    pub r#reference_seq_id: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reference_seq_string: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#chromosome: Option<Box<super::super::types::CodeableConcept>>,
    pub r#strand: Option<super::super::types::Code>,
    pub r#genome_build: Option<super::super::types::String>,
    pub r#window_start: Option<super::super::types::Integer>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceQualityRoc {
    pub r#f_measure: Vec<super::super::types::Decimal>,
    pub r#num_fn: Vec<super::super::types::Integer>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#num_fp: Vec<super::super::types::Integer>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#num_tp: Vec<super::super::types::Integer>,
    pub r#precision: Vec<super::super::types::Decimal>,
    pub r#sensitivity: Vec<super::super::types::Decimal>,
    pub r#score: Vec<super::super::types::Integer>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceQuality {
    pub r#query_fp: Option<super::super::types::Decimal>,
    pub r#recall: Option<super::super::types::Decimal>,
    pub r#gt_fp: Option<super::super::types::Decimal>,
    pub r#query_tp: Option<super::super::types::Decimal>,
    pub r#roc: Option<MolecularSequenceQualityRoc>,
    pub r#type: super::super::types::Code,
    pub r#f_score: Option<super::super::types::Decimal>,
    pub r#truth_fn: Option<super::super::types::Decimal>,
    pub r#start: Option<super::super::types::Integer>,
    pub r#truth_tp: Option<super::super::types::Decimal>,
    pub r#standard_sequence: Option<Box<super::super::types::CodeableConcept>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#end: Option<super::super::types::Integer>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#score: Option<Box<super::super::types::Quantity>>,
    pub r#precision: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceStructureVariantOuter {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#start: Option<super::super::types::Integer>,
    pub r#end: Option<super::super::types::Integer>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceStructureVariantInner {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#start: Option<super::super::types::Integer>,
    pub r#end: Option<super::super::types::Integer>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceStructureVariant {
    pub r#outer: Option<MolecularSequenceStructureVariantOuter>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#variant_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#exact: Option<super::super::types::Boolean>,
    pub r#length: Option<super::super::types::Integer>,
    pub r#id: Option<std::string::String>,
    pub r#inner: Option<MolecularSequenceStructureVariantInner>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceRepository {
    pub r#type: super::super::types::Code,
    pub r#url: Option<super::super::types::Uri>,
    pub r#variantset_id: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#dataset_id: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#readset_id: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceVariant {
    pub r#start: Option<super::super::types::Integer>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference_allele: Option<super::super::types::String>,
    pub r#observed_allele: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#cigar: Option<super::super::types::String>,
    pub r#end: Option<super::super::types::Integer>,
    pub r#variant_pointer: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequence {
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#pointer: Vec<Box<super::super::types::Reference>>,
    pub r#reference_seq: Option<MolecularSequenceReferenceSeq>,
    pub r#observed_seq: Option<super::super::types::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#coordinate_system: super::super::types::Integer,
    pub r#patient: Option<Box<super::super::types::Reference>>,
    pub r#quality: Vec<MolecularSequenceQuality>,
    pub r#read_coverage: Option<super::super::types::Integer>,
    pub r#structure_variant: Vec<MolecularSequenceStructureVariant>,
    pub r#device: Option<Box<super::super::types::Reference>>,
    pub r#type: Option<super::super::types::Code>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#repository: Vec<MolecularSequenceRepository>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#variant: Vec<MolecularSequenceVariant>,
    pub r#language: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#specimen: Option<Box<super::super::types::Reference>>,
}
