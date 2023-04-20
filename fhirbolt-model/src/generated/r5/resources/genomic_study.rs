// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "The analysis event or other GenomicStudy that generated this input file."]
#[derive(Debug, Clone, PartialEq)]
pub enum GenomicStudyAnalysisInputGeneratedBy {
    Identifier(Box<super::super::types::Identifier>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for GenomicStudyAnalysisInputGeneratedBy {
    fn default() -> GenomicStudyAnalysisInputGeneratedBy {
        GenomicStudyAnalysisInputGeneratedBy::Invalid
    }
}
#[doc = "Inputs for the analysis event."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GenomicStudyAnalysisInput {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "File containing input data."]
    pub r#file: Option<Box<super::super::types::Reference>>,
    #[doc = "Type of input data, e.g., BAM, CRAM, or FASTA."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The analysis event or other GenomicStudy that generated this input file."]
    pub r#generated_by: Option<GenomicStudyAnalysisInputGeneratedBy>,
}
#[doc = "Outputs for the analysis event."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GenomicStudyAnalysisOutput {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "File containing output data."]
    pub r#file: Option<Box<super::super::types::Reference>>,
    #[doc = "Type of output data, e.g., VCF, MAF, or BAM."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[doc = "Performer for the analysis event."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GenomicStudyAnalysisPerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The organization, healthcare professional, or others who participated in performing this analysis."]
    pub r#actor: Option<Box<super::super::types::Reference>>,
    #[doc = "Role of the actor for this analysis."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
}
#[doc = "Devices used for the analysis (e.g., instruments, software), with settings and parameters."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GenomicStudyAnalysisDevice {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Device used for the analysis."]
    pub r#device: Option<Box<super::super::types::Reference>>,
    #[doc = "Specific function for the device used for the analysis."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
}
#[doc = "The details about a specific analysis that was performed in this GenomicStudy."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GenomicStudyAnalysis {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifiers for the analysis event."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Type of the methods used in the analysis, e.g., Fluorescence in situ hybridization (FISH), Karyotyping, or Microsatellite instability testing (MSI)."]
    pub r#method_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Type of the genomic changes studied in the analysis, e.g., DNA, RNA, or amino acid change."]
    pub r#change_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The reference genome build that is used in this analysis."]
    pub r#genome_build: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The defined protocol that describes the analysis."]
    pub r#instantiates_canonical: Option<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol that describes the analysis."]
    pub r#instantiates_uri: Option<super::super::types::Uri>,
    #[doc = "Name of the analysis event (human friendly)."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The focus of a genomic analysis when it is not the patient of record representing something or someone associated with the patient such as a spouse, parent, child, or sibling. For example, in trio testing, the GenomicStudy.subject would be the child (proband) and the GenomicStudy.analysis.focus of a specific analysis would be the parent."]
    pub r#focus: Vec<Box<super::super::types::Reference>>,
    #[doc = "The specimen used in the analysis event."]
    pub r#specimen: Vec<Box<super::super::types::Reference>>,
    #[doc = "The date of the analysis event."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Any notes capture with the analysis event."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "The protocol that was performed for the analysis event."]
    pub r#protocol_performed: Option<Box<super::super::types::Reference>>,
    #[doc = "The genomic regions to be studied in the analysis (BED file)."]
    pub r#regions_studied: Vec<Box<super::super::types::Reference>>,
    #[doc = "Genomic regions actually called in the analysis event (BED file)."]
    pub r#regions_called: Vec<Box<super::super::types::Reference>>,
    #[doc = "Inputs for the analysis event."]
    pub r#input: Vec<GenomicStudyAnalysisInput>,
    #[doc = "Outputs for the analysis event."]
    pub r#output: Vec<GenomicStudyAnalysisOutput>,
    #[doc = "Performer for the analysis event."]
    pub r#performer: Vec<GenomicStudyAnalysisPerformer>,
    #[doc = "Devices used for the analysis (e.g., instruments, software), with settings and parameters."]
    pub r#device: Vec<GenomicStudyAnalysisDevice>,
}
#[doc = "A set of analyses performed to analyze and generate genomic data."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GenomicStudy {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifiers for this genomic study."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The status of the genomic study."]
    pub r#status: super::super::types::Code,
    #[doc = "The type of the study, e.g., Familial variant segregation, Functional variation detection, or Gene expression profiling."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The primary subject of the genomic study."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The healthcare event with which this genomics study is associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "When the genomic study was started."]
    pub r#start_date: Option<super::super::types::DateTime>,
    #[doc = "Event resources that the genomic study is based on."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "Healthcare professional who requested or referred the genomic study."]
    pub r#referrer: Option<Box<super::super::types::Reference>>,
    #[doc = "Healthcare professionals who interpreted the genomic study."]
    pub r#interpreter: Vec<Box<super::super::types::Reference>>,
    #[doc = "Why the genomic study was performed."]
    pub r#reason: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "The defined protocol that describes the study."]
    pub r#instantiates_canonical: Option<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol that describes the study."]
    pub r#instantiates_uri: Option<super::super::types::Uri>,
    #[doc = "Comments related to the genomic study."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Description of the genomic study."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The details about a specific analysis that was performed in this GenomicStudy."]
    pub r#analysis: Vec<GenomicStudyAnalysis>,
}
