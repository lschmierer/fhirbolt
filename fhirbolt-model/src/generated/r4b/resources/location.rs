// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The absolute geographic location of the Location, expressed using the WGS84 datum (This is the same co-ordinate system used in KML)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LocationPosition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Longitude. The value domain and the interpretation are the same as for the text of the longitude element in KML (see notes below)."]
    pub r#longitude: super::super::types::Decimal,
    #[doc = "Latitude. The value domain and the interpretation are the same as for the text of the latitude element in KML (see notes below)."]
    pub r#latitude: super::super::types::Decimal,
    #[doc = "Altitude. The value domain and the interpretation are the same as for the text of the altitude element in KML (see notes below)."]
    pub r#altitude: Option<super::super::types::Decimal>,
}
impl serde::ser::Serialize for LocationPosition {
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
                if let Some(some) = self.r#longitude.value.as_ref() {
                    let some = some
                        .parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                    state.serialize_entry("longitude", &some)?;
                }
                if self.r#longitude.id.is_some() || !self.r#longitude.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#longitude.id.as_ref(),
                        extension: &self.r#longitude.extension,
                    };
                    state.serialize_entry("_longitude", &primitive_element)?;
                }
            } else {
                state.serialize_entry("longitude", &self.r#longitude)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#latitude.value.as_ref() {
                    let some = some
                        .parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                    state.serialize_entry("latitude", &some)?;
                }
                if self.r#latitude.id.is_some() || !self.r#latitude.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#latitude.id.as_ref(),
                        extension: &self.r#latitude.extension,
                    };
                    state.serialize_entry("_latitude", &primitive_element)?;
                }
            } else {
                state.serialize_entry("latitude", &self.r#latitude)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#altitude.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("altitude", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_altitude", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#altitude.as_ref() {
                    state.serialize_entry("altitude", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "What days/times during a week is this location usually open."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LocationHoursOfOperation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates which days of the week are available between the start and end Times."]
    pub r#days_of_week: Vec<super::super::types::Code>,
    #[doc = "The Location is open all day."]
    pub r#all_day: Option<super::super::types::Boolean>,
    #[doc = "Time that the Location opens."]
    pub r#opening_time: Option<super::super::types::Time>,
    #[doc = "Time that the Location closes."]
    pub r#closing_time: Option<super::super::types::Time>,
}
impl serde::ser::Serialize for LocationHoursOfOperation {
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
                if !self.r#days_of_week.is_empty() {
                    let values = self
                        .r#days_of_week
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("daysOfWeek", &values)?;
                    }
                    let requires_elements = self
                        .r#days_of_week
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#days_of_week
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
                        state.serialize_entry("_daysOfWeek", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#days_of_week.is_empty() {
                    state.serialize_entry("daysOfWeek", &self.r#days_of_week)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#all_day.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("allDay", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_allDay", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#all_day.as_ref() {
                    state.serialize_entry("allDay", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#opening_time.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("openingTime", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_openingTime", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#opening_time.as_ref() {
                    state.serialize_entry("openingTime", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#closing_time.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("closingTime", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_closingTime", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#closing_time.as_ref() {
                    state.serialize_entry("closingTime", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Details and position information for a physical place where services are provided and resources and participants may be stored, found, contained, or accommodated."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Location {
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
    #[doc = "Unique code or number identifying the location to its users."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The status property covers the general availability of the resource, not the current value which may be covered by the operationStatus, or by a schedule/slots if they are configured for the location."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The operational status covers operation values most relevant to beds (but can also apply to rooms/units/chairs/etc. such as an isolation unit/dialysis chair). This typically covers concepts such as contamination, housekeeping, and other activities like maintenance."]
    pub r#operational_status: Option<Box<super::super::types::Coding>>,
    #[doc = "Name of the location as used by humans. Does not need to be unique."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A list of alternate names that the location is known as, or was known as, in the past."]
    pub r#alias: Vec<super::super::types::String>,
    #[doc = "Description of the Location, which helps in finding or referencing the place."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Indicates whether a resource instance represents a specific location or a class of locations."]
    pub r#mode: Option<super::super::types::Code>,
    #[doc = "Indicates the type of function performed at the location."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The contact details of communication devices available at the location. This can include phone numbers, fax numbers, mobile numbers, email addresses and web sites."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "Physical location."]
    pub r#address: Option<Box<super::super::types::Address>>,
    #[doc = "Physical form of the location, e.g. building, room, vehicle, road."]
    pub r#physical_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The absolute geographic location of the Location, expressed using the WGS84 datum (This is the same co-ordinate system used in KML)."]
    pub r#position: Option<LocationPosition>,
    #[doc = "The organization responsible for the provisioning and upkeep of the location."]
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Another Location of which this Location is physically a part of."]
    pub r#part_of: Option<Box<super::super::types::Reference>>,
    #[doc = "What days/times during a week is this location usually open."]
    pub r#hours_of_operation: Vec<LocationHoursOfOperation>,
    #[doc = "A description of when the locations opening ours are different to normal, e.g. public holiday availability. Succinctly describing all possible exceptions to normal site availability as detailed in the opening hours Times."]
    pub r#availability_exceptions: Option<super::super::types::String>,
    #[doc = "Technical endpoints providing access to services operated for the location."]
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Location")?;
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
                if let Some(some) = self.r#status.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("status", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_status", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status.as_ref() {
                    state.serialize_entry("status", some)?;
                }
            }
            if let Some(some) = self.r#operational_status.as_ref() {
                state.serialize_entry("operationalStatus", some)?;
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
            if _ctx.output_json {
                if !self.r#alias.is_empty() {
                    let values = self
                        .r#alias
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("alias", &values)?;
                    }
                    let requires_elements = self
                        .r#alias
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#alias
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
                        state.serialize_entry("_alias", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#alias.is_empty() {
                    state.serialize_entry("alias", &self.r#alias)?;
                }
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
            if _ctx.output_json {
                if let Some(some) = self.r#mode.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("mode", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_mode", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#mode.as_ref() {
                    state.serialize_entry("mode", some)?;
                }
            }
            if !self.r#type.is_empty() {
                state.serialize_entry("type", &self.r#type)?;
            }
            if !self.r#telecom.is_empty() {
                state.serialize_entry("telecom", &self.r#telecom)?;
            }
            if let Some(some) = self.r#address.as_ref() {
                state.serialize_entry("address", some)?;
            }
            if let Some(some) = self.r#physical_type.as_ref() {
                state.serialize_entry("physicalType", some)?;
            }
            if let Some(some) = self.r#position.as_ref() {
                state.serialize_entry("position", some)?;
            }
            if let Some(some) = self.r#managing_organization.as_ref() {
                state.serialize_entry("managingOrganization", some)?;
            }
            if let Some(some) = self.r#part_of.as_ref() {
                state.serialize_entry("partOf", some)?;
            }
            if !self.r#hours_of_operation.is_empty() {
                state.serialize_entry("hoursOfOperation", &self.r#hours_of_operation)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#availability_exceptions.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("availabilityExceptions", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_availabilityExceptions", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#availability_exceptions.as_ref() {
                    state.serialize_entry("availabilityExceptions", some)?;
                }
            }
            if !self.r#endpoint.is_empty() {
                state.serialize_entry("endpoint", &self.r#endpoint)?;
            }
            state.end()
        })
    }
}
