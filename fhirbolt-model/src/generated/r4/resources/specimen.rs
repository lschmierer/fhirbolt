// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Time when specimen was collected from subject - the physiologically relevant time."]
#[derive(Debug, Clone, PartialEq)]
pub enum SpecimenCollectionCollected {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for SpecimenCollectionCollected {
    fn default() -> SpecimenCollectionCollected {
        SpecimenCollectionCollected::Invalid
    }
}
#[doc = "Abstinence or reduction from some or all food, drink, or both, for a period of time prior to sample collection."]
#[derive(Debug, Clone, PartialEq)]
pub enum SpecimenCollectionFastingStatus {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Duration(Box<super::super::types::Duration>),
    Invalid,
}
impl Default for SpecimenCollectionFastingStatus {
    fn default() -> SpecimenCollectionFastingStatus {
        SpecimenCollectionFastingStatus::Invalid
    }
}
#[doc = "A record of the time or period when the specimen processing occurred.  For example the time of sample fixation or the period of time the sample was in formalin."]
#[derive(Debug, Clone, PartialEq)]
pub enum SpecimenProcessingTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for SpecimenProcessingTime {
    fn default() -> SpecimenProcessingTime {
        SpecimenProcessingTime::Invalid
    }
}
#[doc = "Introduced substance to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
#[derive(Debug, Clone, PartialEq)]
pub enum SpecimenContainerAdditive {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for SpecimenContainerAdditive {
    fn default() -> SpecimenContainerAdditive {
        SpecimenContainerAdditive::Invalid
    }
}
#[doc = "Details concerning the specimen collection."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpecimenCollection {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Person who collected the specimen."]
    pub r#collector: Option<Box<super::super::types::Reference>>,
    #[doc = "Time when specimen was collected from subject - the physiologically relevant time."]
    pub r#collected: Option<SpecimenCollectionCollected>,
    #[doc = "The span of time over which the collection of a specimen occurred."]
    pub r#duration: Option<Box<super::super::types::Duration>>,
    #[doc = "The quantity of specimen collected; for instance the volume of a blood sample, or the physical measurement of an anatomic pathology sample."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "A coded value specifying the technique that is used to perform the procedure."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Anatomical location from which the specimen was collected (if subject is a patient). This is the target site.  This element is not used for environmental specimens."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Abstinence or reduction from some or all food, drink, or both, for a period of time prior to sample collection."]
    pub r#fasting_status: Option<SpecimenCollectionFastingStatus>,
}
impl serde::ser::Serialize for SpecimenCollection {
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
            if let Some(some) = self.r#collector.as_ref() {
                state.serialize_entry("collector", some)?;
            }
            if let Some(some) = self.r#collected.as_ref() {
                match some {
                    SpecimenCollectionCollected::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("collectedDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_collectedDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("collectedDateTime", value)?;
                        }
                    }
                    SpecimenCollectionCollected::Period(ref value) => {
                        state.serialize_entry("collectedPeriod", value)?;
                    }
                    SpecimenCollectionCollected::Invalid => {
                        return Err(serde::ser::Error::custom("collected is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#duration.as_ref() {
                state.serialize_entry("duration", some)?;
            }
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            if let Some(some) = self.r#method.as_ref() {
                state.serialize_entry("method", some)?;
            }
            if let Some(some) = self.r#body_site.as_ref() {
                state.serialize_entry("bodySite", some)?;
            }
            if let Some(some) = self.r#fasting_status.as_ref() {
                match some {
                    SpecimenCollectionFastingStatus::CodeableConcept(ref value) => {
                        state.serialize_entry("fastingStatusCodeableConcept", value)?;
                    }
                    SpecimenCollectionFastingStatus::Duration(ref value) => {
                        state.serialize_entry("fastingStatusDuration", value)?;
                    }
                    SpecimenCollectionFastingStatus::Invalid => {
                        return Err(serde::ser::Error::custom("fasting_status is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "Details concerning processing and processing steps for the specimen."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpecimenProcessing {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Textual description of procedure."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "A coded value specifying the procedure used to process the specimen."]
    pub r#procedure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Material used in the processing step."]
    pub r#additive: Vec<Box<super::super::types::Reference>>,
    #[doc = "A record of the time or period when the specimen processing occurred.  For example the time of sample fixation or the period of time the sample was in formalin."]
    pub r#time: Option<SpecimenProcessingTime>,
}
impl serde::ser::Serialize for SpecimenProcessing {
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
            if let Some(some) = self.r#procedure.as_ref() {
                state.serialize_entry("procedure", some)?;
            }
            if !self.r#additive.is_empty() {
                state.serialize_entry("additive", &self.r#additive)?;
            }
            if let Some(some) = self.r#time.as_ref() {
                match some {
                    SpecimenProcessingTime::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("timeDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_timeDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("timeDateTime", value)?;
                        }
                    }
                    SpecimenProcessingTime::Period(ref value) => {
                        state.serialize_entry("timePeriod", value)?;
                    }
                    SpecimenProcessingTime::Invalid => {
                        return Err(serde::ser::Error::custom("time is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "The container holding the specimen.  The recursive nature of containers; i.e. blood in tube in tray in rack is not addressed here."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpecimenContainer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Id for container. There may be multiple; a manufacturer's bar code, lab assigned identifier, etc. The container ID may differ from the specimen id in some circumstances."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Textual description of the container."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The type of container associated with the specimen (e.g. slide, aliquot, etc.)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The capacity (volume or other measure) the container may contain."]
    pub r#capacity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The quantity of specimen in the container; may be volume, dimensions, or other appropriate measurements, depending on the specimen type."]
    pub r#specimen_quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Introduced substance to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
    pub r#additive: Option<SpecimenContainerAdditive>,
}
impl serde::ser::Serialize for SpecimenContainer {
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
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
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
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#capacity.as_ref() {
                state.serialize_entry("capacity", some)?;
            }
            if let Some(some) = self.r#specimen_quantity.as_ref() {
                state.serialize_entry("specimenQuantity", some)?;
            }
            if let Some(some) = self.r#additive.as_ref() {
                match some {
                    SpecimenContainerAdditive::CodeableConcept(ref value) => {
                        state.serialize_entry("additiveCodeableConcept", value)?;
                    }
                    SpecimenContainerAdditive::Reference(ref value) => {
                        state.serialize_entry("additiveReference", value)?;
                    }
                    SpecimenContainerAdditive::Invalid => {
                        return Err(serde::ser::Error::custom("additive is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "A sample to be used for analysis."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Specimen {
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
    #[doc = "Id for specimen."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier assigned by the lab when accessioning specimen(s). This is not necessarily the same as the specimen identifier, depending on local lab procedures."]
    pub r#accession_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The availability of the specimen."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The kind of material that forms the specimen."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Where the specimen came from. This may be from patient(s), from a location (e.g., the source of an environmental sample), or a sampling of a substance or a device."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Time when specimen was received for processing or testing."]
    pub r#received_time: Option<super::super::types::DateTime>,
    #[doc = "Reference to the parent (source) specimen which is used when the specimen was either derived from or a component of another specimen."]
    pub r#parent: Vec<Box<super::super::types::Reference>>,
    #[doc = "Details concerning a service request that required a specimen to be collected."]
    pub r#request: Vec<Box<super::super::types::Reference>>,
    #[doc = "Details concerning the specimen collection."]
    pub r#collection: Option<SpecimenCollection>,
    #[doc = "Details concerning processing and processing steps for the specimen."]
    pub r#processing: Vec<SpecimenProcessing>,
    #[doc = "The container holding the specimen.  The recursive nature of containers; i.e. blood in tube in tray in rack is not addressed here."]
    pub r#container: Vec<SpecimenContainer>,
    #[doc = "A mode or state of being that describes the nature of the specimen."]
    pub r#condition: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "To communicate any details or issues about the specimen or during the specimen collection. (for example: broken vial, sent with patient, frozen)."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for Specimen {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Specimen")?;
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
            if let Some(some) = self.r#accession_identifier.as_ref() {
                state.serialize_entry("accessionIdentifier", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("status", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_status", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status.as_ref() {
                    state.serialize_entry("status", some)?;
                }
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#subject.as_ref() {
                state.serialize_entry("subject", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#received_time.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("receivedTime", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_receivedTime", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#received_time.as_ref() {
                    state.serialize_entry("receivedTime", some)?;
                }
            }
            if !self.r#parent.is_empty() {
                state.serialize_entry("parent", &self.r#parent)?;
            }
            if !self.r#request.is_empty() {
                state.serialize_entry("request", &self.r#request)?;
            }
            if let Some(some) = self.r#collection.as_ref() {
                state.serialize_entry("collection", some)?;
            }
            if !self.r#processing.is_empty() {
                state.serialize_entry("processing", &self.r#processing)?;
            }
            if !self.r#container.is_empty() {
                state.serialize_entry("container", &self.r#container)?;
            }
            if !self.r#condition.is_empty() {
                state.serialize_entry("condition", &self.r#condition)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            state.end()
        })
    }
}
