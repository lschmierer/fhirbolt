// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "The minimum volume to be conditioned in the container."]
#[derive(Debug, Clone, PartialEq)]
pub enum SpecimenDefinitionTypeTestedContainerMinimumVolume {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SpecimenDefinitionTypeTestedContainerMinimumVolume {
    fn default() -> SpecimenDefinitionTypeTestedContainerMinimumVolume {
        SpecimenDefinitionTypeTestedContainerMinimumVolume::Invalid
    }
}
#[doc = "Substance introduced in the kind of container to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
#[derive(Debug, Clone, PartialEq)]
pub enum SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
    fn default() -> SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
        SpecimenDefinitionTypeTestedContainerAdditiveAdditive::Invalid
    }
}
#[doc = "Substance introduced in the kind of container to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpecimenDefinitionTypeTestedContainerAdditive {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Substance introduced in the kind of container to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
    pub r#additive: SpecimenDefinitionTypeTestedContainerAdditiveAdditive,
}
impl serde::ser::Serialize for SpecimenDefinitionTypeTestedContainerAdditive {
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
            match self.r#additive {
                SpecimenDefinitionTypeTestedContainerAdditiveAdditive::CodeableConcept(
                    ref value,
                ) => {
                    state.serialize_entry("additiveCodeableConcept", value)?;
                }
                SpecimenDefinitionTypeTestedContainerAdditiveAdditive::Reference(ref value) => {
                    state.serialize_entry("additiveReference", value)?;
                }
                SpecimenDefinitionTypeTestedContainerAdditiveAdditive::Invalid => {
                    return Err(serde::ser::Error::custom("additive is a required field"))
                }
            }
            state.end()
        })
    }
}
#[doc = "The specimen's container."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpecimenDefinitionTypeTestedContainer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of material of the container."]
    pub r#material: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of container used to contain this kind of specimen."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Color of container cap."]
    pub r#cap: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The textual description of the kind of container."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The capacity (volume or other measure) of this kind of container."]
    pub r#capacity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The minimum volume to be conditioned in the container."]
    pub r#minimum_volume: Option<SpecimenDefinitionTypeTestedContainerMinimumVolume>,
    #[doc = "Substance introduced in the kind of container to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
    pub r#additive: Vec<SpecimenDefinitionTypeTestedContainerAdditive>,
    #[doc = "Special processing that should be applied to the container for this kind of specimen."]
    pub r#preparation: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SpecimenDefinitionTypeTestedContainer {
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
            if let Some(some) = self.r#material.as_ref() {
                state.serialize_entry("material", some)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#cap.as_ref() {
                state.serialize_entry("cap", some)?;
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
            if let Some(some) = self.r#capacity.as_ref() {
                state.serialize_entry("capacity", some)?;
            }
            if let Some(some) = self.r#minimum_volume.as_ref() {
                match some {
                    SpecimenDefinitionTypeTestedContainerMinimumVolume::Quantity(ref value) => {
                        state.serialize_entry("minimumVolumeQuantity", value)?;
                    }
                    SpecimenDefinitionTypeTestedContainerMinimumVolume::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("minimumVolumeString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_minimumVolumeString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("minimumVolumeString", value)?;
                        }
                    }
                    SpecimenDefinitionTypeTestedContainerMinimumVolume::Invalid => {
                        return Err(serde::ser::Error::custom("minimum_volume is invalid"))
                    }
                }
            }
            if !self.r#additive.is_empty() {
                state.serialize_entry("additive", &self.r#additive)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#preparation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("preparation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_preparation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#preparation.as_ref() {
                    state.serialize_entry("preparation", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Set of instructions for preservation/transport of the specimen at a defined temperature interval, prior the testing process."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpecimenDefinitionTypeTestedHandling {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "It qualifies the interval of temperature, which characterizes an occurrence of handling. Conditions that are not related to temperature may be handled in the instruction element."]
    pub r#temperature_qualifier: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The temperature interval for this set of handling instructions."]
    pub r#temperature_range: Option<Box<super::super::types::Range>>,
    #[doc = "The maximum time interval of preservation of the specimen with these conditions."]
    pub r#max_duration: Option<Box<super::super::types::Duration>>,
    #[doc = "Additional textual instructions for the preservation or transport of the specimen. For instance, 'Protect from light exposure'."]
    pub r#instruction: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SpecimenDefinitionTypeTestedHandling {
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
            if let Some(some) = self.r#temperature_qualifier.as_ref() {
                state.serialize_entry("temperatureQualifier", some)?;
            }
            if let Some(some) = self.r#temperature_range.as_ref() {
                state.serialize_entry("temperatureRange", some)?;
            }
            if let Some(some) = self.r#max_duration.as_ref() {
                state.serialize_entry("maxDuration", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#instruction.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("instruction", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_instruction", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#instruction.as_ref() {
                    state.serialize_entry("instruction", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Specimen conditioned in a container as expected by the testing laboratory."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpecimenDefinitionTypeTested {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Primary of secondary specimen."]
    pub r#is_derived: Option<super::super::types::Boolean>,
    #[doc = "The kind of specimen conditioned for testing expected by lab."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The preference for this type of conditioned specimen."]
    pub r#preference: super::super::types::Code,
    #[doc = "The specimen's container."]
    pub r#container: Option<SpecimenDefinitionTypeTestedContainer>,
    #[doc = "Requirements for delivery and special handling of this kind of conditioned specimen."]
    pub r#requirement: Option<super::super::types::String>,
    #[doc = "The usual time that a specimen of this kind is retained after the ordered tests are completed, for the purpose of additional testing."]
    pub r#retention_time: Option<Box<super::super::types::Duration>>,
    #[doc = "Criterion for rejection of the specimen in its container by the laboratory."]
    pub r#rejection_criterion: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Set of instructions for preservation/transport of the specimen at a defined temperature interval, prior the testing process."]
    pub r#handling: Vec<SpecimenDefinitionTypeTestedHandling>,
}
impl serde::ser::Serialize for SpecimenDefinitionTypeTested {
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
                if let Some(some) = self.r#is_derived.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("isDerived", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_isDerived", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#is_derived.as_ref() {
                    state.serialize_entry("isDerived", some)?;
                }
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#preference.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("preference", &some)?;
                }
                if self.r#preference.id.is_some() || !self.r#preference.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#preference.id.as_ref(),
                        extension: &self.r#preference.extension,
                    };
                    state.serialize_entry("_preference", &primitive_element)?;
                }
            } else {
                state.serialize_entry("preference", &self.r#preference)?;
            }
            if let Some(some) = self.r#container.as_ref() {
                state.serialize_entry("container", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#requirement.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("requirement", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_requirement", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#requirement.as_ref() {
                    state.serialize_entry("requirement", some)?;
                }
            }
            if let Some(some) = self.r#retention_time.as_ref() {
                state.serialize_entry("retentionTime", some)?;
            }
            if !self.r#rejection_criterion.is_empty() {
                state.serialize_entry("rejectionCriterion", &self.r#rejection_criterion)?;
            }
            if !self.r#handling.is_empty() {
                state.serialize_entry("handling", &self.r#handling)?;
            }
            state.end()
        })
    }
}
#[doc = "A kind of specimen with associated set of requirements."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpecimenDefinition {
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
    #[doc = "A business identifier associated with the kind of specimen."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The kind of material to be collected."]
    pub r#type_collected: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Preparation of the patient for specimen collection."]
    pub r#patient_preparation: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Time aspect of specimen collection (duration or offset)."]
    pub r#time_aspect: Option<super::super::types::String>,
    #[doc = "The action to be performed for collecting the specimen."]
    pub r#collection: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specimen conditioned in a container as expected by the testing laboratory."]
    pub r#type_tested: Vec<SpecimenDefinitionTypeTested>,
}
impl crate::AnyResource for SpecimenDefinition {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4B;
}
impl serde::ser::Serialize for SpecimenDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "SpecimenDefinition")?;
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
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            if let Some(some) = self.r#type_collected.as_ref() {
                state.serialize_entry("typeCollected", some)?;
            }
            if !self.r#patient_preparation.is_empty() {
                state.serialize_entry("patientPreparation", &self.r#patient_preparation)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#time_aspect.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("timeAspect", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_timeAspect", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#time_aspect.as_ref() {
                    state.serialize_entry("timeAspect", some)?;
                }
            }
            if !self.r#collection.is_empty() {
                state.serialize_entry("collection", &self.r#collection)?;
            }
            if !self.r#type_tested.is_empty() {
                state.serialize_entry("typeTested", &self.r#type_tested)?;
            }
            state.end()
        })
    }
}
