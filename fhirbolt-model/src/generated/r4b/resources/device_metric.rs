// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Describes the calibrations that have been performed or that are required to be performed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceMetricCalibration {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Describes the type of the calibration method."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "Describes the state of the calibration."]
    pub r#state: Option<super::super::types::Code>,
    #[doc = "Describes the time last calibration has been performed."]
    pub r#time: Option<super::super::types::Instant>,
}
impl serde::ser::Serialize for DeviceMetricCalibration {
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
                if let Some(some) = self.r#time.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("time", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_time", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#time.as_ref() {
                    state.serialize_entry("time", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Describes a measurement, calculation or setting capability of a medical device."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceMetric {
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
    #[doc = "Unique instance identifiers assigned to a device by the device or gateway software, manufacturers, other organizations or owners. For example: handle ID."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Describes the type of the metric. For example: Heart Rate, PEEP Setting, etc."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Describes the unit that an observed value determined for this metric will have. For example: Percent, Seconds, etc."]
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Describes the link to the  Device that this DeviceMetric belongs to and that contains administrative device information such as manufacturer, serial number, etc."]
    pub r#source: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes the link to the  Device that this DeviceMetric belongs to and that provide information about the location of this DeviceMetric in the containment structure of the parent Device. An example would be a Device that represents a Channel. This reference can be used by a client application to distinguish DeviceMetrics that have the same type, but should be interpreted based on their containment location."]
    pub r#parent: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates current operational state of the device. For example: On, Off, Standby, etc."]
    pub r#operational_status: Option<super::super::types::Code>,
    #[doc = "Describes the color representation for the metric. This is often used to aid clinicians to track and identify parameter types by color. In practice, consider a Patient Monitor that has ECG/HR and Pleth for example; the parameters are displayed in different characteristic colors, such as HR-blue, BP-green, and PR and SpO2- magenta."]
    pub r#color: Option<super::super::types::Code>,
    #[doc = "Indicates the category of the observation generation process. A DeviceMetric can be for example a setting, measurement, or calculation."]
    pub r#category: super::super::types::Code,
    #[doc = "Describes the measurement repetition time. This is not necessarily the same as the update period. The measurement repetition time can range from milliseconds up to hours. An example for a measurement repetition time in the range of milliseconds is the sampling rate of an ECG. An example for a measurement repetition time in the range of hours is a NIBP that is triggered automatically every hour. The update period may be different than the measurement repetition time, if the device does not update the published observed value with the same frequency as it was measured."]
    pub r#measurement_period: Option<Box<super::super::types::Timing>>,
    #[doc = "Describes the calibrations that have been performed or that are required to be performed."]
    pub r#calibration: Vec<DeviceMetricCalibration>,
}
impl serde::ser::Serialize for DeviceMetric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "DeviceMetric")?;
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
            state.serialize_entry("type", &self.r#type)?;
            if let Some(some) = self.r#unit.as_ref() {
                state.serialize_entry("unit", some)?;
            }
            if let Some(some) = self.r#source.as_ref() {
                state.serialize_entry("source", some)?;
            }
            if let Some(some) = self.r#parent.as_ref() {
                state.serialize_entry("parent", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#operational_status.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("operationalStatus", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_operationalStatus", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#operational_status.as_ref() {
                    state.serialize_entry("operationalStatus", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#color.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("color", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_color", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#color.as_ref() {
                    state.serialize_entry("color", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#category.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("category", &some)?;
                }
                if self.r#category.id.is_some() || !self.r#category.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#category.id.as_ref(),
                        extension: &self.r#category.extension,
                    };
                    state.serialize_entry("_category", &primitive_element)?;
                }
            } else {
                state.serialize_entry("category", &self.r#category)?;
            }
            if let Some(some) = self.r#measurement_period.as_ref() {
                state.serialize_entry("measurementPeriod", some)?;
            }
            if !self.r#calibration.is_empty() {
                state.serialize_entry("calibration", &self.r#calibration)?;
            }
            state.end()
        })
    }
}
