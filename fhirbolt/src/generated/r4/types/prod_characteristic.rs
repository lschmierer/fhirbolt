// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct ProdCharacteristic {
    pub r#nominal_volume: Option<Box<super::super::types::Quantity>>,
    pub r#weight: Option<Box<super::super::types::Quantity>>,
    pub r#imprint: Vec<super::super::types::String>,
    pub r#image: Vec<Box<super::super::types::Attachment>>,
    pub r#color: Vec<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#width: Option<Box<super::super::types::Quantity>>,
    pub r#height: Option<Box<super::super::types::Quantity>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#scoring: Option<Box<super::super::types::CodeableConcept>>,
    pub r#external_diameter: Option<Box<super::super::types::Quantity>>,
    pub r#depth: Option<Box<super::super::types::Quantity>>,
    pub r#shape: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
}
impl serde::Serialize for ProdCharacteristic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#nominal_volume.as_ref() {
            state.serialize_entry("nominalVolume", some)?;
        }
        if let Some(some) = self.r#weight.as_ref() {
            state.serialize_entry("weight", some)?;
        }
        if !self.r#imprint.is_empty() {
            let values: Vec<_> = self.r#imprint.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("imprint", &values)?;
            }
            let requires_elements = self
                .r#imprint
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
                    .r#imprint
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
                state.serialize_entry("_imprint", &primitive_elements)?;
            }
        }
        if !self.r#image.is_empty() {
            state.serialize_entry("image", &self.r#image)?;
        }
        if !self.r#color.is_empty() {
            let values: Vec<_> = self.r#color.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("color", &values)?;
            }
            let requires_elements = self
                .r#color
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
                    .r#color
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
                state.serialize_entry("_color", &primitive_elements)?;
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#width.as_ref() {
            state.serialize_entry("width", some)?;
        }
        if let Some(some) = self.r#height.as_ref() {
            state.serialize_entry("height", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#scoring.as_ref() {
            state.serialize_entry("scoring", some)?;
        }
        if let Some(some) = self.r#external_diameter.as_ref() {
            state.serialize_entry("externalDiameter", some)?;
        }
        if let Some(some) = self.r#depth.as_ref() {
            state.serialize_entry("depth", some)?;
        }
        if let Some(some) = self.r#shape.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("shape", some)?;
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
                state.serialize_entry("_shape", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        state.end()
    }
}
