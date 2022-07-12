// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum SpecimenDefinitionTypeTestedContainerMinimumVolume {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct SpecimenDefinitionTypeTestedHandling {
    pub r#id: Option<std::string::String>,
    pub r#temperature_qualifier: Option<Box<super::super::types::CodeableConcept>>,
    pub r#max_duration: Option<Box<super::super::types::Duration>>,
    pub r#instruction: Option<super::super::types::String>,
    pub r#temperature_range: Option<Box<super::super::types::Range>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
impl serde::Serialize for SpecimenDefinitionTypeTestedHandling {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#temperature_qualifier.as_ref() {
            state.serialize_entry("temperatureQualifier", some)?;
        }
        if let Some(some) = self.r#max_duration.as_ref() {
            state.serialize_entry("maxDuration", some)?;
        }
        if let Some(some) = self.r#instruction.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("instruction", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_instruction", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#temperature_range.as_ref() {
            state.serialize_entry("temperatureRange", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct SpecimenDefinitionTypeTestedContainerAdditive {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#additive: SpecimenDefinitionTypeTestedContainerAdditiveAdditive,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
impl serde::Serialize for SpecimenDefinitionTypeTestedContainerAdditive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        match self.r#additive {
            SpecimenDefinitionTypeTestedContainerAdditiveAdditive::CodeableConcept(ref value) => {
                state.serialize_entry("additiveCodeableConcept", value)?;
            }
            SpecimenDefinitionTypeTestedContainerAdditiveAdditive::Reference(ref value) => {
                state.serialize_entry("additiveReference", value)?;
            }
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct SpecimenDefinitionTypeTestedContainer {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#cap: Option<Box<super::super::types::CodeableConcept>>,
    pub r#additive: Vec<SpecimenDefinitionTypeTestedContainerAdditive>,
    pub r#description: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#minimum_volume: Option<SpecimenDefinitionTypeTestedContainerMinimumVolume>,
    pub r#material: Option<Box<super::super::types::CodeableConcept>>,
    pub r#capacity: Option<Box<super::super::types::Quantity>>,
    pub r#preparation: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
}
impl serde::Serialize for SpecimenDefinitionTypeTestedContainer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#cap.as_ref() {
            state.serialize_entry("cap", some)?;
        }
        if !self.r#additive.is_empty() {
            state.serialize_entry("additive", &self.r#additive)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#minimum_volume.as_ref() {
            match some {
                SpecimenDefinitionTypeTestedContainerMinimumVolume::Quantity(ref value) => {
                    state.serialize_entry("minimumVolumeQuantity", value)?;
                }
                SpecimenDefinitionTypeTestedContainerMinimumVolume::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("minimumVolumeString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_minimumVolumeString", &primitive_element)?;
                    }
                }
            }
        }
        if let Some(some) = self.r#material.as_ref() {
            state.serialize_entry("material", some)?;
        }
        if let Some(some) = self.r#capacity.as_ref() {
            state.serialize_entry("capacity", some)?;
        }
        if let Some(some) = self.r#preparation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("preparation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_preparation", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct SpecimenDefinitionTypeTested {
    pub r#preference: super::super::types::Code,
    pub r#handling: Vec<SpecimenDefinitionTypeTestedHandling>,
    pub r#is_derived: Option<super::super::types::Boolean>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#requirement: Option<super::super::types::String>,
    pub r#retention_time: Option<Box<super::super::types::Duration>>,
    pub r#rejection_criterion: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#container: Option<SpecimenDefinitionTypeTestedContainer>,
}
impl serde::Serialize for SpecimenDefinitionTypeTested {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        {
            if let Some(some) = self.r#preference.value.as_ref() {
                state.serialize_entry("preference", some)?;
            }
            if self.r#preference.id.is_some() || !self.r#preference.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#preference.id,
                    extension: &self.r#preference.extension,
                };
                state.serialize_entry("_preference", &primitive_element)?;
            }
        }
        if !self.r#handling.is_empty() {
            state.serialize_entry("handling", &self.r#handling)?;
        }
        if let Some(some) = self.r#is_derived.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isDerived", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isDerived", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#requirement.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("requirement", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_requirement", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#retention_time.as_ref() {
            state.serialize_entry("retentionTime", some)?;
        }
        if !self.r#rejection_criterion.is_empty() {
            state.serialize_entry("rejectionCriterion", &self.r#rejection_criterion)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#container.as_ref() {
            state.serialize_entry("container", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct SpecimenDefinition {
    pub r#type_tested: Vec<SpecimenDefinitionTypeTested>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#type_collected: Option<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#patient_preparation: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#time_aspect: Option<super::super::types::String>,
    pub r#collection: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::Serialize for SpecimenDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SpecimenDefinition")?;
        if !self.r#type_tested.is_empty() {
            state.serialize_entry("typeTested", &self.r#type_tested)?;
        }
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        if !self.r#contained.is_empty() {
            state.serialize_entry("contained", &self.r#contained)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if let Some(some) = self.r#type_collected.as_ref() {
            state.serialize_entry("typeCollected", some)?;
        }
        if let Some(some) = self.r#language.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("language", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_implicitRules", &primitive_element)?;
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#patient_preparation.is_empty() {
            state.serialize_entry("patientPreparation", &self.r#patient_preparation)?;
        }
        if let Some(some) = self.r#time_aspect.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("timeAspect", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_timeAspect", &primitive_element)?;
            }
        }
        if !self.r#collection.is_empty() {
            state.serialize_entry("collection", &self.r#collection)?;
        }
        state.end()
    }
}
