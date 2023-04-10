// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for ProdCharacteristic Type: The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ProdCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Where applicable, the height can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#height: Option<Box<super::super::types::Quantity>>,
    #[doc = "Where applicable, the width can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#width: Option<Box<super::super::types::Quantity>>,
    #[doc = "Where applicable, the depth can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#depth: Option<Box<super::super::types::Quantity>>,
    #[doc = "Where applicable, the weight can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#weight: Option<Box<super::super::types::Quantity>>,
    #[doc = "Where applicable, the nominal volume can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#nominal_volume: Option<Box<super::super::types::Quantity>>,
    #[doc = "Where applicable, the external diameter can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#external_diameter: Option<Box<super::super::types::Quantity>>,
    #[doc = "Where applicable, the shape can be specified An appropriate controlled vocabulary shall be used The term and the term identifier shall be used."]
    pub r#shape: Option<super::super::types::String>,
    #[doc = "Where applicable, the color can be specified An appropriate controlled vocabulary shall be used The term and the term identifier shall be used."]
    pub r#color: Vec<super::super::types::String>,
    #[doc = "Where applicable, the imprint can be specified as text."]
    pub r#imprint: Vec<super::super::types::String>,
    #[doc = "Where applicable, the image can be provided The format of the image attachment shall be specified by regional implementations."]
    pub r#image: Vec<Box<super::super::types::Attachment>>,
    #[doc = "Where applicable, the scoring can be specified An appropriate controlled vocabulary shall be used The term and the term identifier shall be used."]
    pub r#scoring: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ProdCharacteristic {
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
            if let Some(some) = self.r#height.as_ref() {
                state.serialize_entry("height", some)?;
            }
            if let Some(some) = self.r#width.as_ref() {
                state.serialize_entry("width", some)?;
            }
            if let Some(some) = self.r#depth.as_ref() {
                state.serialize_entry("depth", some)?;
            }
            if let Some(some) = self.r#weight.as_ref() {
                state.serialize_entry("weight", some)?;
            }
            if let Some(some) = self.r#nominal_volume.as_ref() {
                state.serialize_entry("nominalVolume", some)?;
            }
            if let Some(some) = self.r#external_diameter.as_ref() {
                state.serialize_entry("externalDiameter", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#shape.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("shape", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_shape", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#shape.as_ref() {
                    state.serialize_entry("shape", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#color.is_empty() {
                    let values = self
                        .r#color
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("color", &values)?;
                    }
                    let requires_elements = self
                        .r#color
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#color
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_color", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#color.is_empty() {
                    state.serialize_entry("color", &self.r#color)?;
                }
            }
            if _ctx.output_json {
                if !self.r#imprint.is_empty() {
                    let values = self
                        .r#imprint
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("imprint", &values)?;
                    }
                    let requires_elements = self
                        .r#imprint
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#imprint
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_imprint", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#imprint.is_empty() {
                    state.serialize_entry("imprint", &self.r#imprint)?;
                }
            }
            if !self.r#image.is_empty() {
                state.serialize_entry("image", &self.r#image)?;
            }
            if let Some(some) = self.r#scoring.as_ref() {
                state.serialize_entry("scoring", some)?;
            }
            state.end()
        })
    }
}
