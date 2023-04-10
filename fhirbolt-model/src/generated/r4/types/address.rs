// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world.\n\nNeed to be able to record postal addresses, along with notes about their use."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Address {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The purpose of this address."]
    pub r#use: Option<super::super::types::Code>,
    #[doc = "Distinguishes between physical addresses (those you can visit) and mailing addresses (e.g. PO Boxes and care-of addresses). Most addresses are both."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "Specifies the entire address as it should be displayed e.g. on a postal label. This may be provided instead of or as well as the specific parts."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "This component contains the house number, apartment number, street name, street direction,  P.O. Box number, delivery hints, and similar address information."]
    pub r#line: Vec<super::super::types::String>,
    #[doc = "The name of the city, town, suburb, village or other community or delivery center."]
    pub r#city: Option<super::super::types::String>,
    #[doc = "The name of the administrative area (county)."]
    pub r#district: Option<super::super::types::String>,
    #[doc = "Sub-unit of a country with limited sovereignty in a federally organized country. A code may be used if codes are in common use (e.g. US 2 letter state codes)."]
    pub r#state: Option<super::super::types::String>,
    #[doc = "A postal code designating a region defined by the postal service."]
    pub r#postal_code: Option<super::super::types::String>,
    #[doc = "Country - a nation as commonly understood or generally accepted."]
    pub r#country: Option<super::super::types::String>,
    #[doc = "Time period when address was/is in use."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for Address {
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
            if _ctx.output_json {
                if let Some(some) = self.r#use.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("use", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_use", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#use.as_ref() {
                    state.serialize_entry("use", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("type", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_type", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#type.as_ref() {
                    state.serialize_entry("type", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#line.is_empty() {
                    let values = self
                        .r#line
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("line", &values)?;
                    }
                    let requires_elements = self
                        .r#line
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#line
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
                        state.serialize_entry("_line", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#line.is_empty() {
                    state.serialize_entry("line", &self.r#line)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#city.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("city", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_city", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#city.as_ref() {
                    state.serialize_entry("city", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#district.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("district", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_district", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#district.as_ref() {
                    state.serialize_entry("district", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#state.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("state", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_state", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#state.as_ref() {
                    state.serialize_entry("state", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#postal_code.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("postalCode", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_postalCode", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#postal_code.as_ref() {
                    state.serialize_entry("postalCode", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#country.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("country", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_country", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#country.as_ref() {
                    state.serialize_entry("country", some)?;
                }
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            state.end()
        })
    }
}
