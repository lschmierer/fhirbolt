// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ServiceRequestQuantity {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
    Range(Box<super::super::types::Range>),
}
impl Default for ServiceRequestQuantity {
    fn default() -> ServiceRequestQuantity {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ServiceRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
}
impl Default for ServiceRequestOccurrence {
    fn default() -> ServiceRequestOccurrence {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ServiceRequestAsNeeded {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
impl Default for ServiceRequestAsNeeded {
    fn default() -> ServiceRequestAsNeeded {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct ServiceRequest {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#replaces: Vec<Box<super::super::types::Reference>>,
    pub r#requisition: Option<Box<super::super::types::Identifier>>,
    pub r#status: super::super::types::Code,
    pub r#intent: super::super::types::Code,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#order_detail: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<ServiceRequestQuantity>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#occurrence: Option<ServiceRequestOccurrence>,
    pub r#as_needed: Option<ServiceRequestAsNeeded>,
    pub r#authored_on: Option<super::super::types::DateTime>,
    pub r#requester: Option<Box<super::super::types::Reference>>,
    pub r#performer_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    pub r#location_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#location_reference: Vec<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#insurance: Vec<Box<super::super::types::Reference>>,
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    pub r#specimen: Vec<Box<super::super::types::Reference>>,
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#patient_instruction: Option<super::super::types::String>,
    pub r#relevant_history: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ServiceRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ServiceRequest")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
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
        if !self.r#instantiates_canonical.is_empty() {
            let values: Vec<_> = self
                .r#instantiates_canonical
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiatesCanonical", &values)?;
            }
            let requires_elements = self
                .r#instantiates_canonical
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
                    .r#instantiates_canonical
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
                state.serialize_entry("_instantiatesCanonical", &primitive_elements)?;
            }
        }
        if !self.r#instantiates_uri.is_empty() {
            let values: Vec<_> = self.r#instantiates_uri.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiatesUri", &values)?;
            }
            let requires_elements = self
                .r#instantiates_uri
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
                    .r#instantiates_uri
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
                state.serialize_entry("_instantiatesUri", &primitive_elements)?;
            }
        }
        if !self.r#based_on.is_empty() {
            state.serialize_entry("basedOn", &self.r#based_on)?;
        }
        if !self.r#replaces.is_empty() {
            state.serialize_entry("replaces", &self.r#replaces)?;
        }
        if let Some(some) = self.r#requisition.as_ref() {
            state.serialize_entry("requisition", some)?;
        }
        if let Some(some) = self.r#status.value.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if let Some(some) = self.r#intent.value.as_ref() {
            state.serialize_entry("intent", some)?;
        }
        if self.r#intent.id.is_some() || !self.r#intent.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#intent.id,
                extension: &self.r#intent.extension,
            };
            state.serialize_entry("_intent", &primitive_element)?;
        }
        if !self.r#category.is_empty() {
            state.serialize_entry("category", &self.r#category)?;
        }
        if let Some(some) = self.r#priority.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("priority", some)?;
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
                state.serialize_entry("_priority", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#do_not_perform.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("doNotPerform", some)?;
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
                state.serialize_entry("_doNotPerform", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if !self.r#order_detail.is_empty() {
            state.serialize_entry("orderDetail", &self.r#order_detail)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            match some {
                ServiceRequestQuantity::Quantity(ref value) => {
                    state.serialize_entry("quantityQuantity", value)?;
                }
                ServiceRequestQuantity::Ratio(ref value) => {
                    state.serialize_entry("quantityRatio", value)?;
                }
                ServiceRequestQuantity::Range(ref value) => {
                    state.serialize_entry("quantityRange", value)?;
                }
            }
        }
        state.serialize_entry("subject", &self.r#subject)?;
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#occurrence.as_ref() {
            match some {
                ServiceRequestOccurrence::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("occurrenceDateTime", some)?;
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
                        state.serialize_entry("_occurrenceDateTime", &primitive_element)?;
                    }
                }
                ServiceRequestOccurrence::Period(ref value) => {
                    state.serialize_entry("occurrencePeriod", value)?;
                }
                ServiceRequestOccurrence::Timing(ref value) => {
                    state.serialize_entry("occurrenceTiming", value)?;
                }
            }
        }
        if let Some(some) = self.r#as_needed.as_ref() {
            match some {
                ServiceRequestAsNeeded::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("asNeededBoolean", some)?;
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
                        state.serialize_entry("_asNeededBoolean", &primitive_element)?;
                    }
                }
                ServiceRequestAsNeeded::CodeableConcept(ref value) => {
                    state.serialize_entry("asNeededCodeableConcept", value)?;
                }
            }
        }
        if let Some(some) = self.r#authored_on.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("authoredOn", some)?;
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
                state.serialize_entry("_authoredOn", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#requester.as_ref() {
            state.serialize_entry("requester", some)?;
        }
        if let Some(some) = self.r#performer_type.as_ref() {
            state.serialize_entry("performerType", some)?;
        }
        if !self.r#performer.is_empty() {
            state.serialize_entry("performer", &self.r#performer)?;
        }
        if !self.r#location_code.is_empty() {
            state.serialize_entry("locationCode", &self.r#location_code)?;
        }
        if !self.r#location_reference.is_empty() {
            state.serialize_entry("locationReference", &self.r#location_reference)?;
        }
        if !self.r#reason_code.is_empty() {
            state.serialize_entry("reasonCode", &self.r#reason_code)?;
        }
        if !self.r#reason_reference.is_empty() {
            state.serialize_entry("reasonReference", &self.r#reason_reference)?;
        }
        if !self.r#insurance.is_empty() {
            state.serialize_entry("insurance", &self.r#insurance)?;
        }
        if !self.r#supporting_info.is_empty() {
            state.serialize_entry("supportingInfo", &self.r#supporting_info)?;
        }
        if !self.r#specimen.is_empty() {
            state.serialize_entry("specimen", &self.r#specimen)?;
        }
        if !self.r#body_site.is_empty() {
            state.serialize_entry("bodySite", &self.r#body_site)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if let Some(some) = self.r#patient_instruction.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("patientInstruction", some)?;
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
                state.serialize_entry("_patientInstruction", &primitive_element)?;
            }
        }
        if !self.r#relevant_history.is_empty() {
            state.serialize_entry("relevantHistory", &self.r#relevant_history)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ServiceRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ServiceRequest")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ServiceRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#instantiates_canonical: Option<Vec<super::super::types::Canonical>> =
                    None;
                let mut r#instantiates_uri: Option<Vec<super::super::types::Uri>> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#replaces: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#requisition: Option<Box<super::super::types::Identifier>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#intent: Option<super::super::types::Code> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#priority: Option<super::super::types::Code> = None;
                let mut r#do_not_perform: Option<super::super::types::Boolean> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#order_detail: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#quantity: Option<ServiceRequestQuantity> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#occurrence: Option<ServiceRequestOccurrence> = None;
                let mut r#as_needed: Option<ServiceRequestAsNeeded> = None;
                let mut r#authored_on: Option<super::super::types::DateTime> = None;
                let mut r#requester: Option<Box<super::super::types::Reference>> = None;
                let mut r#performer_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#performer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#location_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#location_reference: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#insurance: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#supporting_info: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#specimen: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#body_site: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#patient_instruction: Option<super::super::types::String> = None;
                let mut r#relevant_history: Option<Vec<Box<super::super::types::Reference>>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "meta" => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        "implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_implicitRules" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#language.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_language"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "text" => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        "contained" => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "instantiatesCanonical" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#instantiates_canonical
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatesCanonical",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_instantiatesCanonical" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let elements: Vec<PrimtiveElement> = map_access.next_value()?;
                            let vec = r#instantiates_canonical
                                .get_or_insert(Vec::with_capacity(elements.len()));
                            if vec.len() != elements.len() {
                                return Err(serde::de::Error::invalid_length(
                                    elements.len(),
                                    &"primitive values length",
                                ));
                            }
                            if vec
                                .iter()
                                .any(|e| e.id.is_some() || !e.extension.is_empty())
                            {
                                return Err(serde::de::Error::duplicate_field(
                                    "_instantiatesCanonical",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "instantiatesUri" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#instantiates_uri.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("instantiatesUri"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_instantiatesUri" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let elements: Vec<PrimtiveElement> = map_access.next_value()?;
                            let vec = r#instantiates_uri
                                .get_or_insert(Vec::with_capacity(elements.len()));
                            if vec.len() != elements.len() {
                                return Err(serde::de::Error::invalid_length(
                                    elements.len(),
                                    &"primitive values length",
                                ));
                            }
                            if vec
                                .iter()
                                .any(|e| e.id.is_some() || !e.extension.is_empty())
                            {
                                return Err(serde::de::Error::duplicate_field("_instantiatesUri"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "basedOn" => {
                            if r#based_on.is_some() {
                                return Err(serde::de::Error::duplicate_field("basedOn"));
                            }
                            r#based_on = Some(map_access.next_value()?);
                        }
                        "replaces" => {
                            if r#replaces.is_some() {
                                return Err(serde::de::Error::duplicate_field("replaces"));
                            }
                            r#replaces = Some(map_access.next_value()?);
                        }
                        "requisition" => {
                            if r#requisition.is_some() {
                                return Err(serde::de::Error::duplicate_field("requisition"));
                            }
                            r#requisition = Some(map_access.next_value()?);
                        }
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_status"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "intent" => {
                            let some = r#intent.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("intent"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_intent" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#intent.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_intent"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "category" => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "priority" => {
                            let some = r#priority.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_priority" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#priority.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_priority"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "doNotPerform" => {
                            let some = r#do_not_perform.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("doNotPerform"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_doNotPerform" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#do_not_perform.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_doNotPerform"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "code" => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        "orderDetail" => {
                            if r#order_detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderDetail"));
                            }
                            r#order_detail = Some(map_access.next_value()?);
                        }
                        "quantityQuantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantityQuantity"));
                            }
                            r#quantity =
                                Some(ServiceRequestQuantity::Quantity(map_access.next_value()?));
                        }
                        "quantityRatio" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantityRatio"));
                            }
                            r#quantity =
                                Some(ServiceRequestQuantity::Ratio(map_access.next_value()?));
                        }
                        "quantityRange" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantityRange"));
                            }
                            r#quantity =
                                Some(ServiceRequestQuantity::Range(map_access.next_value()?));
                        }
                        "subject" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        "encounter" => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(map_access.next_value()?);
                        }
                        "occurrenceDateTime" => {
                            let r#enum = r#occurrence.get_or_insert(
                                ServiceRequestOccurrence::DateTime(Default::default()),
                            );
                            if let ServiceRequestOccurrence::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("occurrence[x]"));
                            }
                        }
                        "_occurrenceDateTime" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#occurrence.get_or_insert(
                                ServiceRequestOccurrence::DateTime(Default::default()),
                            );
                            if let ServiceRequestOccurrence::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_occurrenceDateTime",
                                    ));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_occurrence[x]"));
                            }
                        }
                        "occurrencePeriod" => {
                            if r#occurrence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrencePeriod"));
                            }
                            r#occurrence =
                                Some(ServiceRequestOccurrence::Period(map_access.next_value()?));
                        }
                        "occurrenceTiming" => {
                            if r#occurrence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrenceTiming"));
                            }
                            r#occurrence =
                                Some(ServiceRequestOccurrence::Timing(map_access.next_value()?));
                        }
                        "asNeededBoolean" => {
                            let r#enum = r#as_needed
                                .get_or_insert(ServiceRequestAsNeeded::Boolean(Default::default()));
                            if let ServiceRequestAsNeeded::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "asNeededBoolean",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("asNeeded[x]"));
                            }
                        }
                        "_asNeededBoolean" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#as_needed
                                .get_or_insert(ServiceRequestAsNeeded::Boolean(Default::default()));
                            if let ServiceRequestAsNeeded::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_asNeededBoolean",
                                    ));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_asNeeded[x]"));
                            }
                        }
                        "asNeededCodeableConcept" => {
                            if r#as_needed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "asNeededCodeableConcept",
                                ));
                            }
                            r#as_needed = Some(ServiceRequestAsNeeded::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "authoredOn" => {
                            let some = r#authored_on.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("authoredOn"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_authoredOn" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#authored_on.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_authoredOn"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "requester" => {
                            if r#requester.is_some() {
                                return Err(serde::de::Error::duplicate_field("requester"));
                            }
                            r#requester = Some(map_access.next_value()?);
                        }
                        "performerType" => {
                            if r#performer_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("performerType"));
                            }
                            r#performer_type = Some(map_access.next_value()?);
                        }
                        "performer" => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(map_access.next_value()?);
                        }
                        "locationCode" => {
                            if r#location_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationCode"));
                            }
                            r#location_code = Some(map_access.next_value()?);
                        }
                        "locationReference" => {
                            if r#location_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationReference"));
                            }
                            r#location_reference = Some(map_access.next_value()?);
                        }
                        "reasonCode" => {
                            if r#reason_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonCode"));
                            }
                            r#reason_code = Some(map_access.next_value()?);
                        }
                        "reasonReference" => {
                            if r#reason_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonReference"));
                            }
                            r#reason_reference = Some(map_access.next_value()?);
                        }
                        "insurance" => {
                            if r#insurance.is_some() {
                                return Err(serde::de::Error::duplicate_field("insurance"));
                            }
                            r#insurance = Some(map_access.next_value()?);
                        }
                        "supportingInfo" => {
                            if r#supporting_info.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportingInfo"));
                            }
                            r#supporting_info = Some(map_access.next_value()?);
                        }
                        "specimen" => {
                            if r#specimen.is_some() {
                                return Err(serde::de::Error::duplicate_field("specimen"));
                            }
                            r#specimen = Some(map_access.next_value()?);
                        }
                        "bodySite" => {
                            if r#body_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodySite"));
                            }
                            r#body_site = Some(map_access.next_value()?);
                        }
                        "note" => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        "patientInstruction" => {
                            let some = r#patient_instruction.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patientInstruction",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_patientInstruction" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#patient_instruction.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_patientInstruction",
                                ));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "relevantHistory" => {
                            if r#relevant_history.is_some() {
                                return Err(serde::de::Error::duplicate_field("relevantHistory"));
                            }
                            r#relevant_history = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "meta",
                                    "implicit_rules",
                                    "language",
                                    "text",
                                    "contained",
                                    "extension",
                                    "modifier_extension",
                                    "identifier",
                                    "instantiates_canonical",
                                    "instantiates_uri",
                                    "based_on",
                                    "replaces",
                                    "requisition",
                                    "status",
                                    "intent",
                                    "category",
                                    "priority",
                                    "do_not_perform",
                                    "code",
                                    "order_detail",
                                    "quantity",
                                    "subject",
                                    "encounter",
                                    "occurrence",
                                    "as_needed",
                                    "authored_on",
                                    "requester",
                                    "performer_type",
                                    "performer",
                                    "location_code",
                                    "location_reference",
                                    "reason_code",
                                    "reason_reference",
                                    "insurance",
                                    "supporting_info",
                                    "specimen",
                                    "body_site",
                                    "note",
                                    "patient_instruction",
                                    "relevant_history",
                                ],
                            ))
                        }
                    }
                }
                Ok(ServiceRequest {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#instantiates_canonical: r#instantiates_canonical.unwrap_or(vec![]),
                    r#instantiates_uri: r#instantiates_uri.unwrap_or(vec![]),
                    r#based_on: r#based_on.unwrap_or(vec![]),
                    r#replaces: r#replaces.unwrap_or(vec![]),
                    r#requisition,
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#intent: r#intent.ok_or(serde::de::Error::missing_field("intent"))?,
                    r#category: r#category.unwrap_or(vec![]),
                    r#priority,
                    r#do_not_perform,
                    r#code,
                    r#order_detail: r#order_detail.unwrap_or(vec![]),
                    r#quantity,
                    r#subject: r#subject.ok_or(serde::de::Error::missing_field("subject"))?,
                    r#encounter,
                    r#occurrence,
                    r#as_needed,
                    r#authored_on,
                    r#requester,
                    r#performer_type,
                    r#performer: r#performer.unwrap_or(vec![]),
                    r#location_code: r#location_code.unwrap_or(vec![]),
                    r#location_reference: r#location_reference.unwrap_or(vec![]),
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                    r#insurance: r#insurance.unwrap_or(vec![]),
                    r#supporting_info: r#supporting_info.unwrap_or(vec![]),
                    r#specimen: r#specimen.unwrap_or(vec![]),
                    r#body_site: r#body_site.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#patient_instruction,
                    r#relevant_history: r#relevant_history.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
