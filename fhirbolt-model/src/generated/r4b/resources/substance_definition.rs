// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Quantitative value for this moiety."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceDefinitionMoietyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SubstanceDefinitionMoietyAmount {
    fn default() -> SubstanceDefinitionMoietyAmount {
        SubstanceDefinitionMoietyAmount::Invalid
    }
}
#[doc = "A value for the property."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceDefinitionPropertyValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Date(Box<super::super::types::Date>),
    Boolean(Box<super::super::types::Boolean>),
    Attachment(Box<super::super::types::Attachment>),
    Invalid,
}
impl Default for SubstanceDefinitionPropertyValue {
    fn default() -> SubstanceDefinitionPropertyValue {
        SubstanceDefinitionPropertyValue::Invalid
    }
}
#[doc = "A pointer to another substance, as a resource or just a representational code."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceDefinitionRelationshipSubstanceDefinition {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for SubstanceDefinitionRelationshipSubstanceDefinition {
    fn default() -> SubstanceDefinitionRelationshipSubstanceDefinition {
        SubstanceDefinitionRelationshipSubstanceDefinition::Invalid
    }
}
#[doc = "A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceDefinitionRelationshipAmount {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SubstanceDefinitionRelationshipAmount {
    fn default() -> SubstanceDefinitionRelationshipAmount {
        SubstanceDefinitionRelationshipAmount::Invalid
    }
}
#[doc = "Moiety, for structural modifications."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionMoiety {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Role that the moiety is playing."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifier by which this moiety substance is known."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Textual name for this moiety substance."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Stereochemistry type."]
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Optical activity type."]
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Molecular formula for this moiety of this substance, typically using the Hill system."]
    pub r#molecular_formula: Option<super::super::types::String>,
    #[doc = "Quantitative value for this moiety."]
    pub r#amount: Option<SubstanceDefinitionMoietyAmount>,
    #[doc = "The measurement type of the quantitative value. In capturing the actual relative amounts of substances or molecular fragments it may be necessary to indicate whether the amount refers to, for example, a mole ratio or weight ratio."]
    pub r#measurement_type: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubstanceDefinitionMoiety {
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
            if let Some(some) = self.r#role.as_ref() {
                state.serialize_entry("role", some)?;
            }
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("name", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_name", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#name.as_ref() {
                    state.serialize_entry("name", some)?;
                }
            }
            if let Some(some) = self.r#stereochemistry.as_ref() {
                state.serialize_entry("stereochemistry", some)?;
            }
            if let Some(some) = self.r#optical_activity.as_ref() {
                state.serialize_entry("opticalActivity", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#molecular_formula.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("molecularFormula", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_molecularFormula", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#molecular_formula.as_ref() {
                    state.serialize_entry("molecularFormula", some)?;
                }
            }
            if let Some(some) = self.r#amount.as_ref() {
                match some {
                    SubstanceDefinitionMoietyAmount::Quantity(ref value) => {
                        state.serialize_entry("amountQuantity", value)?;
                    }
                    SubstanceDefinitionMoietyAmount::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("amountString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_amountString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("amountString", value)?;
                        }
                    }
                    SubstanceDefinitionMoietyAmount::Invalid => {
                        return Err(serde::ser::Error::custom("amount is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#measurement_type.as_ref() {
                state.serialize_entry("measurementType", some)?;
            }
            state.end()
        })
    }
}
#[doc = "General specifications for this substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code expressing the type of property."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A value for the property."]
    pub r#value: Option<SubstanceDefinitionPropertyValue>,
}
impl serde::ser::Serialize for SubstanceDefinitionProperty {
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
            state.serialize_entry("type", &self.r#type)?;
            if let Some(some) = self.r#value.as_ref() {
                match some {
                    SubstanceDefinitionPropertyValue::CodeableConcept(ref value) => {
                        state.serialize_entry("valueCodeableConcept", value)?;
                    }
                    SubstanceDefinitionPropertyValue::Quantity(ref value) => {
                        state.serialize_entry("valueQuantity", value)?;
                    }
                    SubstanceDefinitionPropertyValue::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueDate", value)?;
                        }
                    }
                    SubstanceDefinitionPropertyValue::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueBoolean", value)?;
                        }
                    }
                    SubstanceDefinitionPropertyValue::Attachment(ref value) => {
                        state.serialize_entry("valueAttachment", value)?;
                    }
                    SubstanceDefinitionPropertyValue::Invalid => {
                        return Err(serde::ser::Error::custom("value is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionMolecularWeight {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The method by which the molecular weight was determined."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Type of molecular weight such as exact, average (also known as. number average), weight average."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to capture quantitative values for a variety of elements. If only limits are given, the arithmetic mean would be the average. If only a single definite value for a given element is given, it would be captured in this field."]
    pub r#amount: Box<super::super::types::Quantity>,
}
impl serde::ser::Serialize for SubstanceDefinitionMolecularWeight {
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
            if let Some(some) = self.r#method.as_ref() {
                state.serialize_entry("method", some)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            state.serialize_entry("amount", &self.r#amount)?;
            state.end()
        })
    }
}
#[doc = "A depiction of the structure or characterization of the substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionStructureRepresentation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of structural representation (e.g. full, partial)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The structural representation or characterization as a text string in a standard format."]
    pub r#representation: Option<super::super::types::String>,
    #[doc = "The format of the representation e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB, mmCIF. The logical content type rather than the physical file format of a document."]
    pub r#format: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An attached file with the structural representation or characterization e.g. a molecular structure graphic of the substance, a JCAMP or AnIML file."]
    pub r#document: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceDefinitionStructureRepresentation {
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
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#representation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("representation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_representation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#representation.as_ref() {
                    state.serialize_entry("representation", some)?;
                }
            }
            if let Some(some) = self.r#format.as_ref() {
                state.serialize_entry("format", some)?;
            }
            if let Some(some) = self.r#document.as_ref() {
                state.serialize_entry("document", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Structural information."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionStructure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Stereochemistry type."]
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Optical activity type."]
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Molecular formula of this substance, typically using the Hill system."]
    pub r#molecular_formula: Option<super::super::types::String>,
    #[doc = "Specified per moiety according to the Hill system, i.e. first C, then H, then alphabetical, each moiety separated by a dot."]
    pub r#molecular_formula_by_moiety: Option<super::super::types::String>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Option<SubstanceDefinitionMolecularWeight>,
    #[doc = "The method used to elucidate the structure or characterization of the drug substance. Examples: X-ray, HPLC, NMR, Peptide mapping, Ligand binding assay."]
    pub r#technique: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The source of information about the structure."]
    pub r#source_document: Vec<Box<super::super::types::Reference>>,
    #[doc = "A depiction of the structure or characterization of the substance."]
    pub r#representation: Vec<SubstanceDefinitionStructureRepresentation>,
}
impl serde::ser::Serialize for SubstanceDefinitionStructure {
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
            if let Some(some) = self.r#stereochemistry.as_ref() {
                state.serialize_entry("stereochemistry", some)?;
            }
            if let Some(some) = self.r#optical_activity.as_ref() {
                state.serialize_entry("opticalActivity", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#molecular_formula.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("molecularFormula", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_molecularFormula", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#molecular_formula.as_ref() {
                    state.serialize_entry("molecularFormula", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#molecular_formula_by_moiety.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("molecularFormulaByMoiety", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_molecularFormulaByMoiety", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#molecular_formula_by_moiety.as_ref() {
                    state.serialize_entry("molecularFormulaByMoiety", some)?;
                }
            }
            if let Some(some) = self.r#molecular_weight.as_ref() {
                state.serialize_entry("molecularWeight", some)?;
            }
            if !self.r#technique.is_empty() {
                state.serialize_entry("technique", &self.r#technique)?;
            }
            if !self.r#source_document.is_empty() {
                state.serialize_entry("sourceDocument", &self.r#source_document)?;
            }
            if !self.r#representation.is_empty() {
                state.serialize_entry("representation", &self.r#representation)?;
            }
            state.end()
        })
    }
}
#[doc = "Codes associated with the substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionCode {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specific code."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Status of the code assignment, for example 'provisional', 'approved'."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date at which the code status was changed as part of the terminology maintenance."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "Any comment can be provided in this field, if necessary."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceDefinitionCode {
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
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("statusDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_statusDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status_date.as_ref() {
                    state.serialize_entry("statusDate", some)?;
                }
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
#[doc = "Details of the official nature of this name."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionNameOfficial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Which authority uses this official name."]
    pub r#authority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the official name, for example 'draft', 'active', 'retired'."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Date of the official name change."]
    pub r#date: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for SubstanceDefinitionNameOfficial {
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
            if let Some(some) = self.r#authority.as_ref() {
                state.serialize_entry("authority", some)?;
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("date", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_date", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#date.as_ref() {
                    state.serialize_entry("date", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Names applicable to this substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The actual name."]
    pub r#name: super::super::types::String,
    #[doc = "Name type, for example 'systematic',  'scientific, 'brand'."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the name, for example 'current', 'proposed'."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If this is the preferred name for this substance."]
    pub r#preferred: Option<super::super::types::Boolean>,
    #[doc = "Human language that the name is written in."]
    pub r#language: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The use context of this name for example if there is a different name a drug active ingredient as opposed to a food colour additive."]
    pub r#domain: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The jurisdiction where this name applies."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A synonym of this particular name, by which the substance is also known."]
    pub r#synonym: Vec<SubstanceDefinitionName>,
    #[doc = "A translation for this name into another human language."]
    pub r#translation: Vec<SubstanceDefinitionName>,
    #[doc = "Details of the official nature of this name."]
    pub r#official: Vec<SubstanceDefinitionNameOfficial>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceDefinitionName {
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
                if let Some(some) = self.r#name.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#name.id.as_ref(),
                        extension: &self.r#name.extension,
                    };
                    state.serialize_entry("_name", &primitive_element)?;
                }
            } else {
                state.serialize_entry("name", &self.r#name)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#preferred.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("preferred", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_preferred", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#preferred.as_ref() {
                    state.serialize_entry("preferred", some)?;
                }
            }
            if !self.r#language.is_empty() {
                state.serialize_entry("language", &self.r#language)?;
            }
            if !self.r#domain.is_empty() {
                state.serialize_entry("domain", &self.r#domain)?;
            }
            if !self.r#jurisdiction.is_empty() {
                state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
            }
            if !self.r#synonym.is_empty() {
                state.serialize_entry("synonym", &self.r#synonym)?;
            }
            if !self.r#translation.is_empty() {
                state.serialize_entry("translation", &self.r#translation)?;
            }
            if !self.r#official.is_empty() {
                state.serialize_entry("official", &self.r#official)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
#[doc = "A link between this substance and another, with details of the relationship."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionRelationship {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A pointer to another substance, as a resource or just a representational code."]
    pub r#substance_definition: Option<SubstanceDefinitionRelationshipSubstanceDefinition>,
    #[doc = "For example \"salt to parent\", \"active moiety\", \"starting material\", \"polymorph\", \"impurity of\"."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible substance relationships."]
    pub r#is_defining: Option<super::super::types::Boolean>,
    #[doc = "A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other."]
    pub r#amount: Option<SubstanceDefinitionRelationshipAmount>,
    #[doc = "For use when the numeric has an uncertain range."]
    pub r#ratio_high_limit_amount: Option<Box<super::super::types::Ratio>>,
    #[doc = "An operator for the amount, for example \"average\", \"approximately\", \"less than\"."]
    pub r#comparator: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceDefinitionRelationship {
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
            if let Some(some) = self.r#substance_definition.as_ref() {
                match some {
                    SubstanceDefinitionRelationshipSubstanceDefinition::Reference(ref value) => {
                        state.serialize_entry("substanceDefinitionReference", value)?;
                    }
                    SubstanceDefinitionRelationshipSubstanceDefinition::CodeableConcept(
                        ref value,
                    ) => {
                        state.serialize_entry("substanceDefinitionCodeableConcept", value)?;
                    }
                    SubstanceDefinitionRelationshipSubstanceDefinition::Invalid => {
                        return Err(serde::ser::Error::custom("substance_definition is invalid"))
                    }
                }
            }
            state.serialize_entry("type", &self.r#type)?;
            if _ctx.output_json {
                if let Some(some) = self.r#is_defining.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("isDefining", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_isDefining", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#is_defining.as_ref() {
                    state.serialize_entry("isDefining", some)?;
                }
            }
            if let Some(some) = self.r#amount.as_ref() {
                match some {
                    SubstanceDefinitionRelationshipAmount::Quantity(ref value) => {
                        state.serialize_entry("amountQuantity", value)?;
                    }
                    SubstanceDefinitionRelationshipAmount::Ratio(ref value) => {
                        state.serialize_entry("amountRatio", value)?;
                    }
                    SubstanceDefinitionRelationshipAmount::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("amountString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_amountString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("amountString", value)?;
                        }
                    }
                    SubstanceDefinitionRelationshipAmount::Invalid => {
                        return Err(serde::ser::Error::custom("amount is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#ratio_high_limit_amount.as_ref() {
                state.serialize_entry("ratioHighLimitAmount", some)?;
            }
            if let Some(some) = self.r#comparator.as_ref() {
                state.serialize_entry("comparator", some)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
#[doc = "Material or taxonomic/anatomical source for the substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionSourceMaterial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A classification that provides the origin of the raw material. Example: cat hair would be an Animal source type."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The genus of an organism, typically referring to the Latin epithet of the genus element of the plant/animal scientific name."]
    pub r#genus: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The species of an organism, typically referring to the Latin epithet of the species of the plant/animal."]
    pub r#species: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An anatomical origin of the source material within an organism."]
    pub r#part: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The country or countries where the material is harvested."]
    pub r#country_of_origin: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubstanceDefinitionSourceMaterial {
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
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#genus.as_ref() {
                state.serialize_entry("genus", some)?;
            }
            if let Some(some) = self.r#species.as_ref() {
                state.serialize_entry("species", some)?;
            }
            if let Some(some) = self.r#part.as_ref() {
                state.serialize_entry("part", some)?;
            }
            if !self.r#country_of_origin.is_empty() {
                state.serialize_entry("countryOfOrigin", &self.r#country_of_origin)?;
            }
            state.end()
        })
    }
}
#[doc = "The detailed description of a substance, typically at a level beyond what is used for prescribing."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinition {
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
    #[doc = "Identifier by which this substance is known."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A business level version identifier of the substance."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Status of substance within the catalogue e.g. active, retired."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A high level categorization, e.g. polymer or nucleic acid, or food, chemical, biological, or a lower level such as the general types of polymer (linear or branch chain) or type of impurity (process related or contaminant)."]
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "If the substance applies to human or veterinary use."]
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The quality standard, established benchmark, to which substance complies (e.g. USP/NF, Ph. Eur, JP, BP, Company Standard)."]
    pub r#grade: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Textual description of the substance."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Supporting literature."]
    pub r#information_source: Vec<Box<super::super::types::Reference>>,
    #[doc = "Textual comment about the substance's catalogue or registry record."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "The entity that creates, makes, produces or fabricates the substance. This is a set of potential manufacturers but is not necessarily comprehensive."]
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    #[doc = "An entity that is the source for the substance. It may be different from the manufacturer. Supplier is synonymous to a distributor."]
    pub r#supplier: Vec<Box<super::super::types::Reference>>,
    #[doc = "Moiety, for structural modifications."]
    pub r#moiety: Vec<SubstanceDefinitionMoiety>,
    #[doc = "General specifications for this substance."]
    pub r#property: Vec<SubstanceDefinitionProperty>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Vec<SubstanceDefinitionMolecularWeight>,
    #[doc = "Structural information."]
    pub r#structure: Option<SubstanceDefinitionStructure>,
    #[doc = "Codes associated with the substance."]
    pub r#code: Vec<SubstanceDefinitionCode>,
    #[doc = "Names applicable to this substance."]
    pub r#name: Vec<SubstanceDefinitionName>,
    #[doc = "A link between this substance and another, with details of the relationship."]
    pub r#relationship: Vec<SubstanceDefinitionRelationship>,
    #[doc = "Material or taxonomic/anatomical source for the substance."]
    pub r#source_material: Option<SubstanceDefinitionSourceMaterial>,
}
impl serde::ser::Serialize for SubstanceDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "SubstanceDefinition")?;
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
                if let Some(some) = self.r#version.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("version", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_version", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#version.as_ref() {
                    state.serialize_entry("version", some)?;
                }
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if !self.r#classification.is_empty() {
                state.serialize_entry("classification", &self.r#classification)?;
            }
            if let Some(some) = self.r#domain.as_ref() {
                state.serialize_entry("domain", some)?;
            }
            if !self.r#grade.is_empty() {
                state.serialize_entry("grade", &self.r#grade)?;
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
            if !self.r#information_source.is_empty() {
                state.serialize_entry("informationSource", &self.r#information_source)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#manufacturer.is_empty() {
                state.serialize_entry("manufacturer", &self.r#manufacturer)?;
            }
            if !self.r#supplier.is_empty() {
                state.serialize_entry("supplier", &self.r#supplier)?;
            }
            if !self.r#moiety.is_empty() {
                state.serialize_entry("moiety", &self.r#moiety)?;
            }
            if !self.r#property.is_empty() {
                state.serialize_entry("property", &self.r#property)?;
            }
            if !self.r#molecular_weight.is_empty() {
                state.serialize_entry("molecularWeight", &self.r#molecular_weight)?;
            }
            if let Some(some) = self.r#structure.as_ref() {
                state.serialize_entry("structure", some)?;
            }
            if !self.r#code.is_empty() {
                state.serialize_entry("code", &self.r#code)?;
            }
            if !self.r#name.is_empty() {
                state.serialize_entry("name", &self.r#name)?;
            }
            if !self.r#relationship.is_empty() {
                state.serialize_entry("relationship", &self.r#relationship)?;
            }
            if let Some(some) = self.r#source_material.as_ref() {
                state.serialize_entry("sourceMaterial", some)?;
            }
            state.end()
        })
    }
}
