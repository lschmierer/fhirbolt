// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
#[doc = "The time or time-period the observed values are related to. When the subject of the report is a patient, this is usually either the time of the procedure or of specimen collection(s), but very often the source of the date/time is not known, only the date/time itself."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DiagnosticReportEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "This backbone element contains supporting information that was used in the creation of the report not included in the results already included in the report."]
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticReportSupportingInfo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The code value for the role of the supporting information in the diagnostic report."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The reference for the supporting information in the diagnostic report."]
    pub r#reference: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for DiagnosticReportSupportingInfo {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#reference: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "A list of key images or data associated with this report. The images or data are generally created during the diagnostic process, and may be directly of the patient, or of treated specimens (i.e. slides of interest)."]
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticReportMedia {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A comment about the image or data. Typically, this is used to provide an explanation for why the image or data is included, or to draw the viewer's attention to important features."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "Reference to the image or data source."]
    pub r#link: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for DiagnosticReportMedia {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#comment: Default::default(),
            r#link: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "The findings and interpretation of diagnostic tests performed on patients, groups of patients, products, substances, devices, and locations, and/or specimens derived from these. The report includes clinical context such as requesting provider information, and some mix of atomic results, images, textual and coded interpretations, and formatted representation of diagnostic reports. The report also includes non-clinical context such as batch analysis and stability reporting of products and substances.\n\nTo support reporting for any diagnostic report into a clinical data repository."]
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticReport {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifiers assigned to this report by the performer or other systems."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Details concerning a service requested."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "The status of the diagnostic report."]
    pub r#status: super::super::types::Code,
    #[doc = "A code that classifies the clinical discipline, department or diagnostic service that created the report (e.g. cardiology, biochemistry, hematology, MRI). This is used for searching, sorting and display purposes."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "A code or name that describes this diagnostic report."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The subject of the report. Usually, but not always, this is a patient. However, diagnostic services also perform analyses on specimens collected from a variety of other sources."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The healthcare event  (e.g. a patient and healthcare provider interaction) which this DiagnosticReport is about."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The time or time-period the observed values are related to. When the subject of the report is a patient, this is usually either the time of the procedure or of specimen collection(s), but very often the source of the date/time is not known, only the date/time itself."]
    pub r#effective: Option<DiagnosticReportEffective>,
    #[doc = "The date and time that this version of the report was made available to providers, typically after the report was reviewed and verified."]
    pub r#issued: Option<super::super::types::Instant>,
    #[doc = "The diagnostic service that is responsible for issuing the report."]
    pub r#performer: Vec<super::super::types::Reference>,
    #[doc = "The practitioner or organization that is responsible for the report's conclusions and interpretations."]
    pub r#results_interpreter: Vec<super::super::types::Reference>,
    #[doc = "Details about the specimens on which this diagnostic report is based."]
    pub r#specimen: Vec<super::super::types::Reference>,
    #[doc = "[Observations](https://hl7.org/FHIR/observation.html))  that are part of this diagnostic report."]
    pub r#result: Vec<super::super::types::Reference>,
    #[doc = "Comments about the diagnostic report."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "One or more links to full details of any study performed during the diagnostic investigation. An ImagingStudy might comprise a set of radiologic images obtained via a procedure that are analyzed as a group. Typically, this is imaging performed by DICOM enabled modalities, but this is not required. A fully enabled PACS viewer can use this information to provide views of the source images. A GenomicStudy might comprise one or more analyses, each serving a specific purpose. These analyses may vary in method (e.g., karyotyping, CNV, or SNV detection), performer, software, devices used, or regions targeted."]
    pub r#study: Vec<super::super::types::Reference>,
    #[doc = "This backbone element contains supporting information that was used in the creation of the report not included in the results already included in the report."]
    pub r#supporting_info: Vec<DiagnosticReportSupportingInfo>,
    #[doc = "A list of key images or data associated with this report. The images or data are generally created during the diagnostic process, and may be directly of the patient, or of treated specimens (i.e. slides of interest)."]
    pub r#media: Vec<DiagnosticReportMedia>,
    #[doc = "Reference to a Composition resource instance that provides structure for organizing the contents of the DiagnosticReport."]
    pub r#composition: Option<Box<super::super::types::Reference>>,
    #[doc = "Concise and clinically contextualized summary conclusion (interpretation/impression) of the diagnostic report."]
    pub r#conclusion: Option<super::super::types::Markdown>,
    #[doc = "One or more codes that represent the summary conclusion (interpretation/impression) of the diagnostic report."]
    pub r#conclusion_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Rich text representation of the entire result as issued by the diagnostic service. Multiple formats are allowed but they SHALL be semantically equivalent."]
    pub r#presented_form: Vec<super::super::types::Attachment>,
}
#[allow(clippy::derivable_impls)]
impl Default for DiagnosticReport {
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
            r#based_on: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#category: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#subject: Default::default(),
            r#encounter: Default::default(),
            r#effective: Default::default(),
            r#issued: Default::default(),
            r#performer: Default::default(),
            r#results_interpreter: Default::default(),
            r#specimen: Default::default(),
            r#result: Default::default(),
            r#note: Default::default(),
            r#study: Default::default(),
            r#supporting_info: Default::default(),
            r#media: Default::default(),
            r#composition: Default::default(),
            r#conclusion: Default::default(),
            r#conclusion_code: Default::default(),
            r#presented_form: Default::default(),
        }
    }
}
