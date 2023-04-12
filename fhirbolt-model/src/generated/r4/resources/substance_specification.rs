// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Quantitative value for this moiety."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceSpecificationMoietyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SubstanceSpecificationMoietyAmount {
    fn default() -> SubstanceSpecificationMoietyAmount {
        SubstanceSpecificationMoietyAmount::Invalid
    }
}
#[doc = "A substance upon which a defining property depends (e.g. for solubility: in water, in alcohol)."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceSpecificationPropertyDefiningSubstance {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for SubstanceSpecificationPropertyDefiningSubstance {
    fn default() -> SubstanceSpecificationPropertyDefiningSubstance {
        SubstanceSpecificationPropertyDefiningSubstance::Invalid
    }
}
#[doc = "Quantitative value for this property."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceSpecificationPropertyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SubstanceSpecificationPropertyAmount {
    fn default() -> SubstanceSpecificationPropertyAmount {
        SubstanceSpecificationPropertyAmount::Invalid
    }
}
#[doc = "A pointer to another substance, as a resource or just a representational code."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceSpecificationRelationshipSubstance {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for SubstanceSpecificationRelationshipSubstance {
    fn default() -> SubstanceSpecificationRelationshipSubstance {
        SubstanceSpecificationRelationshipSubstance::Invalid
    }
}
#[doc = "A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceSpecificationRelationshipAmount {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SubstanceSpecificationRelationshipAmount {
    fn default() -> SubstanceSpecificationRelationshipAmount {
        SubstanceSpecificationRelationshipAmount::Invalid
    }
}
#[doc = "Moiety, for structural modifications."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationMoiety {
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
    #[doc = "Molecular formula."]
    pub r#molecular_formula: Option<super::super::types::String>,
    #[doc = "Quantitative value for this moiety."]
    pub r#amount: Option<SubstanceSpecificationMoietyAmount>,
}
impl serde::ser::Serialize for SubstanceSpecificationMoiety {
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
                    SubstanceSpecificationMoietyAmount::Quantity(ref value) => {
                        state.serialize_entry("amountQuantity", value)?;
                    }
                    SubstanceSpecificationMoietyAmount::String(ref value) => {
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
                    SubstanceSpecificationMoietyAmount::Invalid => {
                        return Err(serde::ser::Error::custom("amount is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "General specifications for this substance, including how it is related to other substances."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A category for this property, e.g. Physical, Chemical, Enzymatic."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Property type e.g. viscosity, pH, isoelectric point."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Parameters that were used in the measurement of a property (e.g. for viscosity: measured at 20C with a pH of 7.1)."]
    pub r#parameters: Option<super::super::types::String>,
    #[doc = "A substance upon which a defining property depends (e.g. for solubility: in water, in alcohol)."]
    pub r#defining_substance: Option<SubstanceSpecificationPropertyDefiningSubstance>,
    #[doc = "Quantitative value for this property."]
    pub r#amount: Option<SubstanceSpecificationPropertyAmount>,
}
impl serde::ser::Serialize for SubstanceSpecificationProperty {
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
            if let Some(some) = self.r#category.as_ref() {
                state.serialize_entry("category", some)?;
            }
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#parameters.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("parameters", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_parameters", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#parameters.as_ref() {
                    state.serialize_entry("parameters", some)?;
                }
            }
            if let Some(some) = self.r#defining_substance.as_ref() {
                match some {
                    SubstanceSpecificationPropertyDefiningSubstance::Reference(ref value) => {
                        state.serialize_entry("definingSubstanceReference", value)?;
                    }
                    SubstanceSpecificationPropertyDefiningSubstance::CodeableConcept(ref value) => {
                        state.serialize_entry("definingSubstanceCodeableConcept", value)?;
                    }
                    SubstanceSpecificationPropertyDefiningSubstance::Invalid => {
                        return Err(serde::ser::Error::custom("defining_substance is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#amount.as_ref() {
                match some {
                    SubstanceSpecificationPropertyAmount::Quantity(ref value) => {
                        state.serialize_entry("amountQuantity", value)?;
                    }
                    SubstanceSpecificationPropertyAmount::String(ref value) => {
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
                    SubstanceSpecificationPropertyAmount::Invalid => {
                        return Err(serde::ser::Error::custom("amount is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationStructureIsotopeMolecularWeight {
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
    pub r#amount: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for SubstanceSpecificationStructureIsotopeMolecularWeight {
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
            if let Some(some) = self.r#amount.as_ref() {
                state.serialize_entry("amount", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Applicable for single substances that contain a radionuclide or a non-natural isotopic ratio."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationStructureIsotope {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Substance identifier for each non-natural or radioisotope."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Substance name for each non-natural or radioisotope."]
    pub r#name: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of isotopic substitution present in a single substance."]
    pub r#substitution: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Half life - for a non-natural nuclide."]
    pub r#half_life: Option<Box<super::super::types::Quantity>>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
}
impl serde::ser::Serialize for SubstanceSpecificationStructureIsotope {
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
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            if let Some(some) = self.r#name.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if let Some(some) = self.r#substitution.as_ref() {
                state.serialize_entry("substitution", some)?;
            }
            if let Some(some) = self.r#half_life.as_ref() {
                state.serialize_entry("halfLife", some)?;
            }
            if let Some(some) = self.r#molecular_weight.as_ref() {
                state.serialize_entry("molecularWeight", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Molecular structural representation."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationStructureRepresentation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of structure (e.g. Full, Partial, Representative)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The structural representation as text string in a format e.g. InChI, SMILES, MOLFILE, CDX."]
    pub r#representation: Option<super::super::types::String>,
    #[doc = "An attached file with the structural representation."]
    pub r#attachment: Option<Box<super::super::types::Attachment>>,
}
impl serde::ser::Serialize for SubstanceSpecificationStructureRepresentation {
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
            if let Some(some) = self.r#attachment.as_ref() {
                state.serialize_entry("attachment", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Structural information."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationStructure {
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
    #[doc = "Molecular formula."]
    pub r#molecular_formula: Option<super::super::types::String>,
    #[doc = "Specified per moiety according to the Hill system, i.e. first C, then H, then alphabetical, each moiety separated by a dot."]
    pub r#molecular_formula_by_moiety: Option<super::super::types::String>,
    #[doc = "Applicable for single substances that contain a radionuclide or a non-natural isotopic ratio."]
    pub r#isotope: Vec<SubstanceSpecificationStructureIsotope>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
    #[doc = "Molecular structural representation."]
    pub r#representation: Vec<SubstanceSpecificationStructureRepresentation>,
}
impl serde::ser::Serialize for SubstanceSpecificationStructure {
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
            if !self.r#isotope.is_empty() {
                state.serialize_entry("isotope", &self.r#isotope)?;
            }
            if let Some(some) = self.r#molecular_weight.as_ref() {
                state.serialize_entry("molecularWeight", some)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
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
pub struct SubstanceSpecificationCode {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specific code."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Status of the code assignment."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date at which the code status is changed as part of the terminology maintenance."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "Any comment can be provided in this field, if necessary."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceSpecificationCode {
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
            if _ctx.output_json {
                if let Some(some) = self.r#comment.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("comment", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_comment", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#comment.as_ref() {
                    state.serialize_entry("comment", some)?;
                }
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
pub struct SubstanceSpecificationNameOfficial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Which authority uses this official name."]
    pub r#authority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the official name."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Date of official name change."]
    pub r#date: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for SubstanceSpecificationNameOfficial {
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
pub struct SubstanceSpecificationName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The actual name."]
    pub r#name: super::super::types::String,
    #[doc = "Name type."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the name."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If this is the preferred name for this substance."]
    pub r#preferred: Option<super::super::types::Boolean>,
    #[doc = "Language of the name."]
    pub r#language: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The use context of this name for example if there is a different name a drug active ingredient as opposed to a food colour additive."]
    pub r#domain: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The jurisdiction where this name applies."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A synonym of this name."]
    pub r#synonym: Vec<SubstanceSpecificationName>,
    #[doc = "A translation for this name."]
    pub r#translation: Vec<SubstanceSpecificationName>,
    #[doc = "Details of the official nature of this name."]
    pub r#official: Vec<SubstanceSpecificationNameOfficial>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceSpecificationName {
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
pub struct SubstanceSpecificationRelationship {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A pointer to another substance, as a resource or just a representational code."]
    pub r#substance: Option<SubstanceSpecificationRelationshipSubstance>,
    #[doc = "For example \"salt to parent\", \"active moiety\", \"starting material\"."]
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible substance relationships."]
    pub r#is_defining: Option<super::super::types::Boolean>,
    #[doc = "A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other."]
    pub r#amount: Option<SubstanceSpecificationRelationshipAmount>,
    #[doc = "For use when the numeric."]
    pub r#amount_ratio_low_limit: Option<Box<super::super::types::Ratio>>,
    #[doc = "An operator for the amount, for example \"average\", \"approximately\", \"less than\"."]
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceSpecificationRelationship {
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
            if let Some(some) = self.r#substance.as_ref() {
                match some {
                    SubstanceSpecificationRelationshipSubstance::Reference(ref value) => {
                        state.serialize_entry("substanceReference", value)?;
                    }
                    SubstanceSpecificationRelationshipSubstance::CodeableConcept(ref value) => {
                        state.serialize_entry("substanceCodeableConcept", value)?;
                    }
                    SubstanceSpecificationRelationshipSubstance::Invalid => {
                        return Err(serde::ser::Error::custom("substance is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#relationship.as_ref() {
                state.serialize_entry("relationship", some)?;
            }
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
                    SubstanceSpecificationRelationshipAmount::Quantity(ref value) => {
                        state.serialize_entry("amountQuantity", value)?;
                    }
                    SubstanceSpecificationRelationshipAmount::Range(ref value) => {
                        state.serialize_entry("amountRange", value)?;
                    }
                    SubstanceSpecificationRelationshipAmount::Ratio(ref value) => {
                        state.serialize_entry("amountRatio", value)?;
                    }
                    SubstanceSpecificationRelationshipAmount::String(ref value) => {
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
                    SubstanceSpecificationRelationshipAmount::Invalid => {
                        return Err(serde::ser::Error::custom("amount is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#amount_ratio_low_limit.as_ref() {
                state.serialize_entry("amountRatioLowLimit", some)?;
            }
            if let Some(some) = self.r#amount_type.as_ref() {
                state.serialize_entry("amountType", some)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
#[doc = "The detailed description of a substance, typically at a level beyond what is used for prescribing."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceSpecification {
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
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "High level categorization, e.g. polymer or nucleic acid."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Status of substance within the catalogue e.g. approved."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If the substance applies to only human or veterinary use."]
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Textual description of the substance."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
    #[doc = "Textual comment about this record of a substance."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "Moiety, for structural modifications."]
    pub r#moiety: Vec<SubstanceSpecificationMoiety>,
    #[doc = "General specifications for this substance, including how it is related to other substances."]
    pub r#property: Vec<SubstanceSpecificationProperty>,
    #[doc = "General information detailing this substance."]
    pub r#reference_information: Option<Box<super::super::types::Reference>>,
    #[doc = "Structural information."]
    pub r#structure: Option<SubstanceSpecificationStructure>,
    #[doc = "Codes associated with the substance."]
    pub r#code: Vec<SubstanceSpecificationCode>,
    #[doc = "Names applicable to this substance."]
    pub r#name: Vec<SubstanceSpecificationName>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Vec<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    #[doc = "A link between this substance and another, with details of the relationship."]
    pub r#relationship: Vec<SubstanceSpecificationRelationship>,
    #[doc = "Data items specific to nucleic acids."]
    pub r#nucleic_acid: Option<Box<super::super::types::Reference>>,
    #[doc = "Data items specific to polymers."]
    pub r#polymer: Option<Box<super::super::types::Reference>>,
    #[doc = "Data items specific to proteins."]
    pub r#protein: Option<Box<super::super::types::Reference>>,
    #[doc = "Material or taxonomic/anatomical source for the substance."]
    pub r#source_material: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceSpecification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "SubstanceSpecification")?;
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
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if let Some(some) = self.r#domain.as_ref() {
                state.serialize_entry("domain", some)?;
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
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#comment.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("comment", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_comment", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#comment.as_ref() {
                    state.serialize_entry("comment", some)?;
                }
            }
            if !self.r#moiety.is_empty() {
                state.serialize_entry("moiety", &self.r#moiety)?;
            }
            if !self.r#property.is_empty() {
                state.serialize_entry("property", &self.r#property)?;
            }
            if let Some(some) = self.r#reference_information.as_ref() {
                state.serialize_entry("referenceInformation", some)?;
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
            if !self.r#molecular_weight.is_empty() {
                state.serialize_entry("molecularWeight", &self.r#molecular_weight)?;
            }
            if !self.r#relationship.is_empty() {
                state.serialize_entry("relationship", &self.r#relationship)?;
            }
            if let Some(some) = self.r#nucleic_acid.as_ref() {
                state.serialize_entry("nucleicAcid", some)?;
            }
            if let Some(some) = self.r#polymer.as_ref() {
                state.serialize_entry("polymer", some)?;
            }
            if let Some(some) = self.r#protein.as_ref() {
                state.serialize_entry("protein", some)?;
            }
            if let Some(some) = self.r#source_material.as_ref() {
                state.serialize_entry("sourceMaterial", some)?;
            }
            state.end()
        })
    }
}
