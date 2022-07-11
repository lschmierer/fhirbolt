// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct MeasureReportGroupStratifierStratumComponent {
    pub r#value: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#code: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct MeasureReportGroupStratifierStratumPopulation {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#count: Option<super::super::types::Integer>,
    pub r#subject_results: Option<Box<super::super::types::Reference>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct MeasureReportGroupStratifierStratum {
    pub r#value: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#measure_score: Option<Box<super::super::types::Quantity>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#component: Vec<MeasureReportGroupStratifierStratumComponent>,
    pub r#population: Vec<MeasureReportGroupStratifierStratumPopulation>,
}
#[derive(Debug, Clone)]
pub struct MeasureReportGroupStratifier {
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#stratum: Vec<MeasureReportGroupStratifierStratum>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MeasureReportGroupPopulation {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#count: Option<super::super::types::Integer>,
    pub r#id: Option<std::string::String>,
    pub r#subject_results: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MeasureReportGroup {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#stratifier: Vec<MeasureReportGroupStratifier>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#measure_score: Option<Box<super::super::types::Quantity>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#population: Vec<MeasureReportGroupPopulation>,
}
#[derive(Debug, Clone)]
pub struct MeasureReport {
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#reporter: Option<Box<super::super::types::Reference>>,
    pub r#group: Vec<MeasureReportGroup>,
    pub r#period: Box<super::super::types::Period>,
    pub r#status: super::super::types::Code,
    pub r#language: Option<super::super::types::Code>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#type: super::super::types::Code,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#evaluated_resource: Vec<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#improvement_notation: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#measure: super::super::types::Canonical,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
