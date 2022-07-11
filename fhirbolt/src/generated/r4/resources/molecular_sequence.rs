// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct MolecularSequenceQualityRoc {
    pub r#num_fp: Vec<super::super::types::Integer>,
    pub r#precision: Vec<super::super::types::Decimal>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#sensitivity: Vec<super::super::types::Decimal>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#score: Vec<super::super::types::Integer>,
    pub r#num_tp: Vec<super::super::types::Integer>,
    pub r#num_fn: Vec<super::super::types::Integer>,
    pub r#f_measure: Vec<super::super::types::Decimal>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceQuality {
    pub r#score: Option<Box<super::super::types::Quantity>>,
    pub r#query_tp: Option<super::super::types::Decimal>,
    pub r#precision: Option<super::super::types::Decimal>,
    pub r#roc: Option<MolecularSequenceQualityRoc>,
    pub r#standard_sequence: Option<Box<super::super::types::CodeableConcept>>,
    pub r#gt_fp: Option<super::super::types::Decimal>,
    pub r#truth_fn: Option<super::super::types::Decimal>,
    pub r#truth_tp: Option<super::super::types::Decimal>,
    pub r#start: Option<super::super::types::Integer>,
    pub r#end: Option<super::super::types::Integer>,
    pub r#f_score: Option<super::super::types::Decimal>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#recall: Option<super::super::types::Decimal>,
    pub r#type: super::super::types::Code,
    pub r#query_fp: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceStructureVariantInner {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#start: Option<super::super::types::Integer>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#end: Option<super::super::types::Integer>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceStructureVariantOuter {
    pub r#end: Option<super::super::types::Integer>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#start: Option<super::super::types::Integer>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceStructureVariant {
    pub r#length: Option<super::super::types::Integer>,
    pub r#inner: Option<MolecularSequenceStructureVariantInner>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#exact: Option<super::super::types::Boolean>,
    pub r#outer: Option<MolecularSequenceStructureVariantOuter>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#variant_type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceReferenceSeq {
    pub r#window_end: Option<super::super::types::Integer>,
    pub r#chromosome: Option<Box<super::super::types::CodeableConcept>>,
    pub r#window_start: Option<super::super::types::Integer>,
    pub r#orientation: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#strand: Option<super::super::types::Code>,
    pub r#reference_seq_id: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference_seq_pointer: Option<Box<super::super::types::Reference>>,
    pub r#genome_build: Option<super::super::types::String>,
    pub r#reference_seq_string: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceRepository {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#variantset_id: Option<super::super::types::String>,
    pub r#readset_id: Option<super::super::types::String>,
    pub r#dataset_id: Option<super::super::types::String>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequenceVariant {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#variant_pointer: Option<Box<super::super::types::Reference>>,
    pub r#end: Option<super::super::types::Integer>,
    pub r#start: Option<super::super::types::Integer>,
    pub r#reference_allele: Option<super::super::types::String>,
    pub r#observed_allele: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#cigar: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct MolecularSequence {
    pub r#observed_seq: Option<super::super::types::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#quality: Vec<MolecularSequenceQuality>,
    pub r#device: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#patient: Option<Box<super::super::types::Reference>>,
    pub r#type: Option<super::super::types::Code>,
    pub r#read_coverage: Option<super::super::types::Integer>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#pointer: Vec<Box<super::super::types::Reference>>,
    pub r#structure_variant: Vec<MolecularSequenceStructureVariant>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference_seq: Option<MolecularSequenceReferenceSeq>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#specimen: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#repository: Vec<MolecularSequenceRepository>,
    pub r#coordinate_system: super::super::types::Integer,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#variant: Vec<MolecularSequenceVariant>,
}
