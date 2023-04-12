// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Indicates who or what performed the series and how they were involved."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ImagingStudySeriesPerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Distinguishes the type of involvement of the performer in the series."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates who or what performed the series."]
    pub r#actor: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for ImagingStudySeriesPerformer {
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
            if let Some(some) = self.r#function.as_ref() {
                state.serialize_entry("function", some)?;
            }
            state.serialize_entry("actor", &self.r#actor)?;
            state.end()
        })
    }
}
#[doc = "A single SOP instance within the series, e.g. an image, or presentation state."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ImagingStudySeriesInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The DICOM SOP Instance UID for this image or other DICOM content."]
    pub r#uid: super::super::types::Id,
    #[doc = "DICOM instance  type."]
    pub r#sop_class: Box<super::super::types::Coding>,
    #[doc = "The number of instance in the series."]
    pub r#number: Option<super::super::types::UnsignedInt>,
    #[doc = "The description of the instance."]
    pub r#title: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ImagingStudySeriesInstance {
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
                if let Some(some) = self.r#uid.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("uid", &some)?;
                }
                if self.r#uid.id.is_some() || !self.r#uid.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#uid.id.as_ref(),
                        extension: &self.r#uid.extension,
                    };
                    state.serialize_entry("_uid", &primitive_element)?;
                }
            } else {
                state.serialize_entry("uid", &self.r#uid)?;
            }
            state.serialize_entry("sopClass", &self.r#sop_class)?;
            if _ctx.output_json {
                if let Some(some) = self.r#number.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("number", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_number", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number.as_ref() {
                    state.serialize_entry("number", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("title", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_title", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#title.as_ref() {
                    state.serialize_entry("title", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Each study has one or more series of images or other content."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ImagingStudySeries {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The DICOM Series Instance UID for the series."]
    pub r#uid: super::super::types::Id,
    #[doc = "The numeric identifier of this series in the study."]
    pub r#number: Option<super::super::types::UnsignedInt>,
    #[doc = "The modality of this series sequence."]
    pub r#modality: Box<super::super::types::Coding>,
    #[doc = "A description of the series."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Number of SOP Instances in the Study. The value given may be larger than the number of instance elements this resource contains due to resource availability, security, or other factors. This element should be present if any instance elements are present."]
    pub r#number_of_instances: Option<super::super::types::UnsignedInt>,
    #[doc = "The network service providing access (e.g., query, view, or retrieval) for this series. See implementation notes for information about using DICOM endpoints. A series-level endpoint, if present, has precedence over a study-level endpoint with the same Endpoint.connectionType."]
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    #[doc = "The anatomic structures examined. See DICOM Part 16 Annex L (<http://dicom.nema.org/medical/dicom/current/output/chtml/part16/chapter_L.html>) for DICOM to SNOMED-CT mappings. The bodySite may indicate the laterality of body part imaged; if so, it shall be consistent with any content of ImagingStudy.series.laterality."]
    pub r#body_site: Option<Box<super::super::types::Coding>>,
    #[doc = "The laterality of the (possibly paired) anatomic structures examined. E.g., the left knee, both lungs, or unpaired abdomen. If present, shall be consistent with any laterality information indicated in ImagingStudy.series.bodySite."]
    pub r#laterality: Option<Box<super::super::types::Coding>>,
    #[doc = "The specimen imaged, e.g., for whole slide imaging of a biopsy."]
    pub r#specimen: Vec<Box<super::super::types::Reference>>,
    #[doc = "The date and time the series was started."]
    pub r#started: Option<super::super::types::DateTime>,
    #[doc = "Indicates who or what performed the series and how they were involved."]
    pub r#performer: Vec<ImagingStudySeriesPerformer>,
    #[doc = "A single SOP instance within the series, e.g. an image, or presentation state."]
    pub r#instance: Vec<ImagingStudySeriesInstance>,
}
impl serde::ser::Serialize for ImagingStudySeries {
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
                if let Some(some) = self.r#uid.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("uid", &some)?;
                }
                if self.r#uid.id.is_some() || !self.r#uid.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#uid.id.as_ref(),
                        extension: &self.r#uid.extension,
                    };
                    state.serialize_entry("_uid", &primitive_element)?;
                }
            } else {
                state.serialize_entry("uid", &self.r#uid)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("number", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_number", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number.as_ref() {
                    state.serialize_entry("number", some)?;
                }
            }
            state.serialize_entry("modality", &self.r#modality)?;
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number_of_instances.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("numberOfInstances", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_numberOfInstances", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number_of_instances.as_ref() {
                    state.serialize_entry("numberOfInstances", some)?;
                }
            }
            if !self.r#endpoint.is_empty() {
                state.serialize_entry("endpoint", &self.r#endpoint)?;
            }
            if let Some(some) = self.r#body_site.as_ref() {
                state.serialize_entry("bodySite", some)?;
            }
            if let Some(some) = self.r#laterality.as_ref() {
                state.serialize_entry("laterality", some)?;
            }
            if !self.r#specimen.is_empty() {
                state.serialize_entry("specimen", &self.r#specimen)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#started.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("started", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_started", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#started.as_ref() {
                    state.serialize_entry("started", some)?;
                }
            }
            if !self.r#performer.is_empty() {
                state.serialize_entry("performer", &self.r#performer)?;
            }
            if !self.r#instance.is_empty() {
                state.serialize_entry("instance", &self.r#instance)?;
            }
            state.end()
        })
    }
}
#[doc = "Representation of the content produced in a DICOM imaging study. A study comprises a set of series, each of which includes a set of Service-Object Pair Instances (SOP Instances - images or other data) acquired or produced in a common context.  A series is of only one modality (e.g. X-ray, CT, MR, ultrasound), but a study may have multiple series of different modalities."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ImagingStudy {
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
    #[doc = "Identifiers for the ImagingStudy such as DICOM Study Instance UID, and Accession Number."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The current state of the ImagingStudy."]
    pub r#status: super::super::types::Code,
    #[doc = "A list of all the series.modality values that are actual acquisition modalities, i.e. those in the DICOM Context Group 29 (value set OID 1.2.840.10008.6.1.19)."]
    pub r#modality: Vec<Box<super::super::types::Coding>>,
    #[doc = "The subject, typically a patient, of the imaging study."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The healthcare event (e.g. a patient and healthcare provider interaction) during which this ImagingStudy is made."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Date and time the study started."]
    pub r#started: Option<super::super::types::DateTime>,
    #[doc = "A list of the diagnostic requests that resulted in this imaging study being performed."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "The requesting/referring physician."]
    pub r#referrer: Option<Box<super::super::types::Reference>>,
    #[doc = "Who read the study and interpreted the images or other content."]
    pub r#interpreter: Vec<Box<super::super::types::Reference>>,
    #[doc = "The network service providing access (e.g., query, view, or retrieval) for the study. See implementation notes for information about using DICOM endpoints. A study-level endpoint applies to each series in the study, unless overridden by a series-level endpoint with the same Endpoint.connectionType."]
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    #[doc = "Number of Series in the Study. This value given may be larger than the number of series elements this Resource contains due to resource availability, security, or other factors. This element should be present if any series elements are present."]
    pub r#number_of_series: Option<super::super::types::UnsignedInt>,
    #[doc = "Number of SOP Instances in Study. This value given may be larger than the number of instance elements this resource contains due to resource availability, security, or other factors. This element should be present if any instance elements are present."]
    pub r#number_of_instances: Option<super::super::types::UnsignedInt>,
    #[doc = "The procedure which this ImagingStudy was part of."]
    pub r#procedure_reference: Option<Box<super::super::types::Reference>>,
    #[doc = "The code for the performed procedure type."]
    pub r#procedure_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The principal physical location where the ImagingStudy was performed."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Description of clinical condition indicating why the ImagingStudy was requested."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates another resource whose existence justifies this Study."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Per the recommended DICOM mapping, this element is derived from the Study Description attribute (0008,1030). Observations or findings about the imaging study should be recorded in another resource, e.g. Observation, and not in this element."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "The Imaging Manager description of the study. Institution-generated description or classification of the Study (component) performed."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Each study has one or more series of images or other content."]
    pub r#series: Vec<ImagingStudySeries>,
}
impl serde::ser::Serialize for ImagingStudy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ImagingStudy")?;
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
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            if !self.r#modality.is_empty() {
                state.serialize_entry("modality", &self.r#modality)?;
            }
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#started.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("started", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_started", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#started.as_ref() {
                    state.serialize_entry("started", some)?;
                }
            }
            if !self.r#based_on.is_empty() {
                state.serialize_entry("basedOn", &self.r#based_on)?;
            }
            if let Some(some) = self.r#referrer.as_ref() {
                state.serialize_entry("referrer", some)?;
            }
            if !self.r#interpreter.is_empty() {
                state.serialize_entry("interpreter", &self.r#interpreter)?;
            }
            if !self.r#endpoint.is_empty() {
                state.serialize_entry("endpoint", &self.r#endpoint)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number_of_series.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("numberOfSeries", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_numberOfSeries", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number_of_series.as_ref() {
                    state.serialize_entry("numberOfSeries", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number_of_instances.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("numberOfInstances", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_numberOfInstances", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number_of_instances.as_ref() {
                    state.serialize_entry("numberOfInstances", some)?;
                }
            }
            if let Some(some) = self.r#procedure_reference.as_ref() {
                state.serialize_entry("procedureReference", some)?;
            }
            if !self.r#procedure_code.is_empty() {
                state.serialize_entry("procedureCode", &self.r#procedure_code)?;
            }
            if let Some(some) = self.r#location.as_ref() {
                state.serialize_entry("location", some)?;
            }
            if !self.r#reason_code.is_empty() {
                state.serialize_entry("reasonCode", &self.r#reason_code)?;
            }
            if !self.r#reason_reference.is_empty() {
                state.serialize_entry("reasonReference", &self.r#reason_reference)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            if !self.r#series.is_empty() {
                state.serialize_entry("series", &self.r#series)?;
            }
            state.end()
        })
    }
}
