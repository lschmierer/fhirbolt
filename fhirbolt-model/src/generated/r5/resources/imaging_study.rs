// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Indicates who or what performed the series and how they were involved."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingStudySeriesPerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Distinguishes the type of involvement of the performer in the series."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates who or what performed the series."]
    pub r#actor: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImagingStudySeriesPerformer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#function: Default::default(),
            r#actor: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "A single SOP instance within the series, e.g. an image, or presentation state."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingStudySeriesInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The DICOM SOP Instance UID for this image or other DICOM content."]
    pub r#uid: super::super::types::Id,
    #[doc = "DICOM instance  type."]
    pub r#sop_class: Box<super::super::types::Coding>,
    #[doc = "The number of instance in the series."]
    pub r#number: Option<super::super::types::UnsignedInt>,
    #[doc = "The description of the instance."]
    pub r#title: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImagingStudySeriesInstance {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#uid: super::super::types::Id {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#sop_class: Box::new(super::super::types::Coding {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#number: Default::default(),
            r#title: Default::default(),
        }
    }
}
#[doc = "Each study has one or more series of images or other content."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingStudySeries {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The DICOM Series Instance UID for the series."]
    pub r#uid: super::super::types::Id,
    #[doc = "The numeric identifier of this series in the study."]
    pub r#number: Option<super::super::types::UnsignedInt>,
    #[doc = "The distinct modality for this series. This may include both acquisition and non-acquisition modalities."]
    pub r#modality: Box<super::super::types::CodeableConcept>,
    #[doc = "A description of the series."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Number of SOP Instances in the Study. The value given may be larger than the number of instance elements this resource contains due to resource availability, security, or other factors. This element should be present if any instance elements are present."]
    pub r#number_of_instances: Option<super::super::types::UnsignedInt>,
    #[doc = "The network service providing access (e.g., query, view, or retrieval) for this series. See implementation notes for information about using DICOM endpoints. A series-level endpoint, if present, has precedence over a study-level endpoint with the same Endpoint.connectionType."]
    pub r#endpoint: Vec<super::super::types::Reference>,
    #[doc = "The anatomic structures examined. See DICOM Part 16 Annex L (<http://dicom.nema.org/medical/dicom/current/output/chtml/part16/chapter_L.html>) for DICOM to SNOMED-CT mappings. The bodySite may indicate the laterality of body part imaged; if so, it shall be consistent with any content of ImagingStudy.series.laterality."]
    pub r#body_site: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The laterality of the (possibly paired) anatomic structures examined. E.g., the left knee, both lungs, or unpaired abdomen. If present, shall be consistent with any laterality information indicated in ImagingStudy.series.bodySite."]
    pub r#laterality: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specimen imaged, e.g., for whole slide imaging of a biopsy."]
    pub r#specimen: Vec<super::super::types::Reference>,
    #[doc = "The date and time the series was started."]
    pub r#started: Option<super::super::types::DateTime>,
    #[doc = "Indicates who or what performed the series and how they were involved."]
    pub r#performer: Vec<ImagingStudySeriesPerformer>,
    #[doc = "A single SOP instance within the series, e.g. an image, or presentation state."]
    pub r#instance: Vec<ImagingStudySeriesInstance>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImagingStudySeries {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#uid: super::super::types::Id {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#number: Default::default(),
            r#modality: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#description: Default::default(),
            r#number_of_instances: Default::default(),
            r#endpoint: Default::default(),
            r#body_site: Default::default(),
            r#laterality: Default::default(),
            r#specimen: Default::default(),
            r#started: Default::default(),
            r#performer: Default::default(),
            r#instance: Default::default(),
        }
    }
}
#[doc = "Representation of the content produced in a DICOM imaging study. A study comprises a set of series, each of which includes a set of Service-Object Pair Instances (SOP Instances - images or other data) acquired or produced in a common context.  A series is of only one modality (e.g. X-ray, CT, MR, ultrasound), but a study may have multiple series of different modalities."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingStudy {
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
    #[doc = "Identifiers for the ImagingStudy such as DICOM Study Instance UID."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The current state of the ImagingStudy resource. This is not the status of any ServiceRequest or Task resources associated with the ImagingStudy."]
    pub r#status: super::super::types::Code,
    #[doc = "A list of all the distinct values of series.modality. This may include both acquisition and non-acquisition modalities."]
    pub r#modality: Vec<super::super::types::CodeableConcept>,
    #[doc = "The subject, typically a patient, of the imaging study."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The healthcare event (e.g. a patient and healthcare provider interaction) during which this ImagingStudy is made."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Date and time the study started."]
    pub r#started: Option<super::super::types::DateTime>,
    #[doc = "A list of the diagnostic requests that resulted in this imaging study being performed."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "A larger event of which this particular ImagingStudy is a component or step.  For example,  an ImagingStudy as part of a procedure."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "The requesting/referring physician."]
    pub r#referrer: Option<Box<super::super::types::Reference>>,
    #[doc = "The network service providing access (e.g., query, view, or retrieval) for the study. See implementation notes for information about using DICOM endpoints. A study-level endpoint applies to each series in the study, unless overridden by a series-level endpoint with the same Endpoint.connectionType."]
    pub r#endpoint: Vec<super::super::types::Reference>,
    #[doc = "Number of Series in the Study. This value given may be larger than the number of series elements this Resource contains due to resource availability, security, or other factors. This element should be present if any series elements are present."]
    pub r#number_of_series: Option<super::super::types::UnsignedInt>,
    #[doc = "Number of SOP Instances in Study. This value given may be larger than the number of instance elements this resource contains due to resource availability, security, or other factors. This element should be present if any instance elements are present."]
    pub r#number_of_instances: Option<super::super::types::UnsignedInt>,
    #[doc = "This field corresponds to the DICOM Procedure Code Sequence (0008,1032). This is different from the FHIR Procedure resource that may include the ImagingStudy."]
    pub r#procedure: Vec<super::super::types::CodeableReference>,
    #[doc = "The principal physical location where the ImagingStudy was performed."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Description of clinical condition indicating why the ImagingStudy was requested, and/or Indicates another resource whose existence justifies this Study."]
    pub r#reason: Vec<super::super::types::CodeableReference>,
    #[doc = "Per the recommended DICOM mapping, this element is derived from the Study Description attribute (0008,1030). Observations or findings about the imaging study should be recorded in another resource, e.g. Observation, and not in this element."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The Imaging Manager description of the study. Institution-generated description or classification of the Study (component) performed."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Each study has one or more series of images or other content."]
    pub r#series: Vec<ImagingStudySeries>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImagingStudy {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#modality: Default::default(),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#encounter: Default::default(),
            r#started: Default::default(),
            r#based_on: Default::default(),
            r#part_of: Default::default(),
            r#referrer: Default::default(),
            r#endpoint: Default::default(),
            r#number_of_series: Default::default(),
            r#number_of_instances: Default::default(),
            r#procedure: Default::default(),
            r#location: Default::default(),
            r#reason: Default::default(),
            r#note: Default::default(),
            r#description: Default::default(),
            r#series: Default::default(),
        }
    }
}
