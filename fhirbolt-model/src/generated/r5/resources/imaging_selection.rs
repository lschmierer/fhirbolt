// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Selector of the instances – human or machine."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingSelectionPerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Distinguishes the type of involvement of the performer."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Author – human or machine."]
    pub r#actor: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImagingSelectionPerformer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#function: Default::default(),
            r#actor: Default::default(),
        }
    }
}
#[doc = "Each imaging selection instance or frame list might includes an image region, specified by a region type and a set of 2D coordinates.\n       If the parent imagingSelection.instance contains a subset element of type frame, the image region applies to all frames in the subset list."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingSelectionInstanceImageRegion2D {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Specifies the type of image region."]
    pub r#region_type: super::super::types::Code,
    #[doc = "The coordinates describing the image region. Encoded as a set of (column, row) pairs that denote positions in the selected image / frames specified with sub-pixel resolution.\n       The origin at the TLHC of the TLHC pixel is 0.0\\0.0, the BRHC of the TLHC pixel is 1.0\\1.0, and the BRHC of the BRHC pixel is the number of columns\\rows in the image / frames. The values must be within the range 0\\0 to the number of columns\\rows in the image / frames."]
    pub r#coordinate: Vec<super::super::types::Decimal>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImagingSelectionInstanceImageRegion2D {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#region_type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#coordinate: Default::default(),
        }
    }
}
#[doc = "Each imaging selection might includes a 3D image region, specified by a region type and a set of 3D coordinates."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingSelectionInstanceImageRegion3D {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Specifies the type of image region."]
    pub r#region_type: super::super::types::Code,
    #[doc = "The coordinates describing the image region. Encoded as an ordered set of (x,y,z) triplets (in mm and may be negative) that define a region of interest in the patient-relative Reference Coordinate System defined by ImagingSelection.frameOfReferenceUid element."]
    pub r#coordinate: Vec<super::super::types::Decimal>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImagingSelectionInstanceImageRegion3D {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#region_type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#coordinate: Default::default(),
        }
    }
}
#[doc = "Each imaging selection includes one or more selected DICOM SOP instances."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingSelectionInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The SOP Instance UID for the selected DICOM instance."]
    pub r#uid: super::super::types::Id,
    #[doc = "The Instance Number for the selected DICOM instance."]
    pub r#number: Option<super::super::types::UnsignedInt>,
    #[doc = "The SOP Class UID for the selected DICOM instance."]
    pub r#sop_class: Option<Box<super::super::types::Coding>>,
    #[doc = "Selected subset of the SOP Instance. The content and format of the subset item is determined by the SOP Class of the selected instance.\n       May be one of:\n       - A list of frame numbers selected from a multiframe SOP Instance.\n       - A list of Content Item Observation UID values selected from a DICOM SR or other structured document SOP Instance.\n       - A list of segment numbers selected from a segmentation SOP Instance.\n       - A list of Region of Interest (ROI) numbers selected from a radiotherapy structure set SOP Instance."]
    pub r#subset: Vec<super::super::types::String>,
    #[doc = "Each imaging selection instance or frame list might includes an image region, specified by a region type and a set of 2D coordinates.\n       If the parent imagingSelection.instance contains a subset element of type frame, the image region applies to all frames in the subset list."]
    pub r#image_region_2_d: Vec<ImagingSelectionInstanceImageRegion2D>,
    #[doc = "Each imaging selection might includes a 3D image region, specified by a region type and a set of 3D coordinates."]
    pub r#image_region_3_d: Vec<ImagingSelectionInstanceImageRegion3D>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImagingSelectionInstance {
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
            r#sop_class: Default::default(),
            r#subset: Default::default(),
            r#image_region_2_d: Default::default(),
            r#image_region_3_d: Default::default(),
        }
    }
}
#[doc = "A selection of DICOM SOP instances and/or frames within a single Study and Series. This might include additional specifics such as an image region, an Observation UID or a Segmentation Number, allowing linkage to an Observation Resource or transferring this information along with the ImagingStudy Resource."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingSelection {
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
    #[doc = "A unique identifier assigned to this imaging selection."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The current state of the ImagingSelection resource. This is not the status of any ImagingStudy, ServiceRequest, or Task resources associated with the ImagingSelection."]
    pub r#status: super::super::types::Code,
    #[doc = "The patient, or group of patients, location, device, organization, procedure or practitioner this imaging selection is about and into whose or what record the imaging selection is placed."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The date and time this imaging selection was created."]
    pub r#issued: Option<super::super::types::Instant>,
    #[doc = "Selector of the instances – human or machine."]
    pub r#performer: Vec<ImagingSelectionPerformer>,
    #[doc = "A list of the diagnostic requests that resulted in this imaging selection being performed."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "Classifies the imaging selection."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Reason for referencing the selected content."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The Study Instance UID for the DICOM Study from which the images were selected."]
    pub r#study_uid: Option<super::super::types::Id>,
    #[doc = "The imaging study from which the imaging selection is made."]
    pub r#derived_from: Vec<super::super::types::Reference>,
    #[doc = "The network service providing retrieval access to the selected images, frames, etc. See implementation notes for information about using DICOM endpoints."]
    pub r#endpoint: Vec<super::super::types::Reference>,
    #[doc = "The Series Instance UID for the DICOM Series from which the images were selected."]
    pub r#series_uid: Option<super::super::types::Id>,
    #[doc = "The Series Number for the DICOM Series from which the images were selected."]
    pub r#series_number: Option<super::super::types::UnsignedInt>,
    #[doc = "The Frame of Reference UID identifying the coordinate system that conveys spatial and/or temporal information for the selected images or frames."]
    pub r#frame_of_reference_uid: Option<super::super::types::Id>,
    #[doc = "The anatomic structures examined. See DICOM Part 16 Annex L (<http://dicom.nema.org/medical/dicom/current/output/chtml/part16/chapter_L.html>) for DICOM to SNOMED-CT mappings."]
    pub r#body_site: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The actual focus of an observation when it is not the patient of record representing something or someone associated with the patient such as a spouse, parent, fetus, or donor. For example, fetus observations in a mother's record.  The focus of an observation could also be an existing condition,  an intervention, the subject's diet,  another observation of the subject,  or a body structure such as tumor or implanted device.   An example use case would be using the Observation resource to capture whether the mother is trained to change her child's tracheostomy tube. In this example, the child is the patient of record and the mother is the focus."]
    pub r#focus: Vec<super::super::types::Reference>,
    #[doc = "Each imaging selection includes one or more selected DICOM SOP instances."]
    pub r#instance: Vec<ImagingSelectionInstance>,
}
#[allow(clippy::derivable_impls)]
impl Default for ImagingSelection {
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
            r#subject: Default::default(),
            r#issued: Default::default(),
            r#performer: Default::default(),
            r#based_on: Default::default(),
            r#category: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#study_uid: Default::default(),
            r#derived_from: Default::default(),
            r#endpoint: Default::default(),
            r#series_uid: Default::default(),
            r#series_number: Default::default(),
            r#frame_of_reference_uid: Default::default(),
            r#body_site: Default::default(),
            r#focus: Default::default(),
            r#instance: Default::default(),
        }
    }
}
