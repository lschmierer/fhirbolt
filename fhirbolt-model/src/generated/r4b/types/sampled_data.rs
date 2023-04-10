// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.\n\nThere is a need for a concise way to handle the data produced by devices that sample a physical state at a high frequency."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SampledData {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The base quantity that a measured value of zero represents. In addition, this provides the units of the entire measurement series."]
    pub r#origin: Box<super::super::types::Quantity>,
    #[doc = "The length of time between sampling times, measured in milliseconds."]
    pub r#period: super::super::types::Decimal,
    #[doc = "A correction factor that is applied to the sampled data points before they are added to the origin."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The lower limit of detection of the measured points. This is needed if any of the data points have the value \"L\" (lower than detection limit)."]
    pub r#lower_limit: Option<super::super::types::Decimal>,
    #[doc = "The upper limit of detection of the measured points. This is needed if any of the data points have the value \"U\" (higher than detection limit)."]
    pub r#upper_limit: Option<super::super::types::Decimal>,
    #[doc = "The number of sample points at each time point. If this value is greater than one, then the dimensions will be interlaced - all the sample points for a point in time will be recorded at once."]
    pub r#dimensions: super::super::types::PositiveInt,
    #[doc = "A series of data points which are decimal values separated by a single space (character u20). The special values \"E\" (error), \"L\" (below detection limit) and \"U\" (above detection limit) can also be used in place of a decimal value."]
    pub r#data: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SampledData {
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
            state.serialize_entry("origin", &self.r#origin)?;
            if _ctx.output_json {
                if let Some(some) = self.r#period.value.as_ref() {
                    let some = some
                        .parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                    state.serialize_entry("period", &some)?;
                }
                if self.r#period.id.is_some() || !self.r#period.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#period.id.as_ref(),
                        extension: &self.r#period.extension,
                    };
                    state.serialize_entry("_period", &primitive_element)?;
                }
            } else {
                state.serialize_entry("period", &self.r#period)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#factor.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("factor", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_factor", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#factor.as_ref() {
                    state.serialize_entry("factor", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#lower_limit.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("lowerLimit", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lowerLimit", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#lower_limit.as_ref() {
                    state.serialize_entry("lowerLimit", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#upper_limit.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("upperLimit", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_upperLimit", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#upper_limit.as_ref() {
                    state.serialize_entry("upperLimit", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#dimensions.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("dimensions", &some)?;
                }
                if self.r#dimensions.id.is_some() || !self.r#dimensions.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#dimensions.id.as_ref(),
                        extension: &self.r#dimensions.extension,
                    };
                    state.serialize_entry("_dimensions", &primitive_element)?;
                }
            } else {
                state.serialize_entry("dimensions", &self.r#dimensions)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#data.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("data", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_data", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#data.as_ref() {
                    state.serialize_entry("data", some)?;
                }
            }
            state.end()
        })
    }
}
