// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterialFractionDescription {
    pub r#fraction: Option<super::super::types::String>,
    pub r#material_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
impl serde::Serialize for SubstanceSourceMaterialFractionDescription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#fraction.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("fraction", some)?;
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
                state.serialize_entry("_fraction", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#material_type.as_ref() {
            state.serialize_entry("materialType", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterialOrganismHybrid {
    pub r#hybrid_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#maternal_organism_name: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#paternal_organism_name: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#maternal_organism_id: Option<super::super::types::String>,
    pub r#paternal_organism_id: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
}
impl serde::Serialize for SubstanceSourceMaterialOrganismHybrid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#hybrid_type.as_ref() {
            state.serialize_entry("hybridType", some)?;
        }
        if let Some(some) = self.r#maternal_organism_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("maternalOrganismName", some)?;
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
                state.serialize_entry("_maternalOrganismName", &primitive_element)?;
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#paternal_organism_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("paternalOrganismName", some)?;
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
                state.serialize_entry("_paternalOrganismName", &primitive_element)?;
            }
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#maternal_organism_id.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("maternalOrganismId", some)?;
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
                state.serialize_entry("_maternalOrganismId", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#paternal_organism_id.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("paternalOrganismId", some)?;
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
                state.serialize_entry("_paternalOrganismId", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterialOrganismAuthor {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#author_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#author_description: Option<super::super::types::String>,
}
impl serde::Serialize for SubstanceSourceMaterialOrganismAuthor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#author_type.as_ref() {
            state.serialize_entry("authorType", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#author_description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("authorDescription", some)?;
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
                state.serialize_entry("_authorDescription", &primitive_element)?;
            }
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterialOrganismOrganismGeneral {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#kingdom: Option<Box<super::super::types::CodeableConcept>>,
    pub r#class: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#order: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#phylum: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::Serialize for SubstanceSourceMaterialOrganismOrganismGeneral {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#kingdom.as_ref() {
            state.serialize_entry("kingdom", some)?;
        }
        if let Some(some) = self.r#class.as_ref() {
            state.serialize_entry("class", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#order.as_ref() {
            state.serialize_entry("order", some)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#phylum.as_ref() {
            state.serialize_entry("phylum", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterialOrganism {
    pub r#hybrid: Option<SubstanceSourceMaterialOrganismHybrid>,
    pub r#id: Option<std::string::String>,
    pub r#intraspecific_description: Option<super::super::types::String>,
    pub r#species: Option<Box<super::super::types::CodeableConcept>>,
    pub r#author: Vec<SubstanceSourceMaterialOrganismAuthor>,
    pub r#organism_general: Option<SubstanceSourceMaterialOrganismOrganismGeneral>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#family: Option<Box<super::super::types::CodeableConcept>>,
    pub r#genus: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#intraspecific_type: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::Serialize for SubstanceSourceMaterialOrganism {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#hybrid.as_ref() {
            state.serialize_entry("hybrid", some)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#intraspecific_description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("intraspecificDescription", some)?;
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
                state.serialize_entry("_intraspecificDescription", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#species.as_ref() {
            state.serialize_entry("species", some)?;
        }
        if !self.r#author.is_empty() {
            state.serialize_entry("author", &self.r#author)?;
        }
        if let Some(some) = self.r#organism_general.as_ref() {
            state.serialize_entry("organismGeneral", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#family.as_ref() {
            state.serialize_entry("family", some)?;
        }
        if let Some(some) = self.r#genus.as_ref() {
            state.serialize_entry("genus", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#intraspecific_type.as_ref() {
            state.serialize_entry("intraspecificType", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterialPartDescription {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#part: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#part_location: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::Serialize for SubstanceSourceMaterialPartDescription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#part.as_ref() {
            state.serialize_entry("part", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#part_location.as_ref() {
            state.serialize_entry("partLocation", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct SubstanceSourceMaterial {
    pub r#language: Option<super::super::types::Code>,
    pub r#geographical_location: Vec<super::super::types::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#development_stage: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source_material_class: Option<Box<super::super::types::CodeableConcept>>,
    pub r#fraction_description: Vec<SubstanceSourceMaterialFractionDescription>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#organism_id: Option<Box<super::super::types::Identifier>>,
    pub r#organism: Option<SubstanceSourceMaterialOrganism>,
    pub r#organism_name: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#source_material_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#parent_substance_name: Vec<super::super::types::String>,
    pub r#parent_substance_id: Vec<Box<super::super::types::Identifier>>,
    pub r#source_material_state: Option<Box<super::super::types::CodeableConcept>>,
    pub r#country_of_origin: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#part_description: Vec<SubstanceSourceMaterialPartDescription>,
}
impl serde::Serialize for SubstanceSourceMaterial {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SubstanceSourceMaterial")?;
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
        if !self.r#geographical_location.is_empty() {
            let values: Vec<_> = self
                .r#geographical_location
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("geographicalLocation", &values)?;
            }
            let requires_elements = self
                .r#geographical_location
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_elements: Vec<_> = self
                    .r#geographical_location
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(PrimtiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_geographicalLocation", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#development_stage.as_ref() {
            state.serialize_entry("developmentStage", some)?;
        }
        if let Some(some) = self.r#source_material_class.as_ref() {
            state.serialize_entry("sourceMaterialClass", some)?;
        }
        if !self.r#fraction_description.is_empty() {
            state.serialize_entry("fractionDescription", &self.r#fraction_description)?;
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
        if let Some(some) = self.r#organism_id.as_ref() {
            state.serialize_entry("organismId", some)?;
        }
        if let Some(some) = self.r#organism.as_ref() {
            state.serialize_entry("organism", some)?;
        }
        if let Some(some) = self.r#organism_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("organismName", some)?;
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
                state.serialize_entry("_organismName", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#source_material_type.as_ref() {
            state.serialize_entry("sourceMaterialType", some)?;
        }
        if !self.r#contained.is_empty() {
            state.serialize_entry("contained", &self.r#contained)?;
        }
        if !self.r#parent_substance_name.is_empty() {
            let values: Vec<_> = self
                .r#parent_substance_name
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("parentSubstanceName", &values)?;
            }
            let requires_elements = self
                .r#parent_substance_name
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_elements: Vec<_> = self
                    .r#parent_substance_name
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(PrimtiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_parentSubstanceName", &primitive_elements)?;
            }
        }
        if !self.r#parent_substance_id.is_empty() {
            state.serialize_entry("parentSubstanceId", &self.r#parent_substance_id)?;
        }
        if let Some(some) = self.r#source_material_state.as_ref() {
            state.serialize_entry("sourceMaterialState", some)?;
        }
        if !self.r#country_of_origin.is_empty() {
            state.serialize_entry("countryOfOrigin", &self.r#country_of_origin)?;
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if !self.r#part_description.is_empty() {
            state.serialize_entry("partDescription", &self.r#part_description)?;
        }
        state.end()
    }
}
