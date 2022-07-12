// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum GuidanceResponseModule {
    Uri(Box<super::super::types::Uri>),
    Canonical(Box<super::super::types::Canonical>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct GuidanceResponse {
    pub r#occurrence_date_time: Option<super::super::types::DateTime>,
    pub r#id: Option<std::string::String>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#result: Option<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#status: super::super::types::Code,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#request_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#module: GuidanceResponseModule,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#evaluation_message: Vec<Box<super::super::types::Reference>>,
    pub r#output_parameters: Option<Box<super::super::types::Reference>>,
    pub r#data_requirement: Vec<Box<super::super::types::DataRequirement>>,
}
impl serde::Serialize for GuidanceResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "GuidanceResponse")?;
        if let Some(some) = self.r#occurrence_date_time.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("occurrenceDateTime", some)?;
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
                state.serialize_entry("_occurrenceDateTime", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#reason_reference.is_empty() {
            state.serialize_entry("reasonReference", &self.r#reason_reference)?;
        }
        if let Some(some) = self.r#result.as_ref() {
            state.serialize_entry("result", some)?;
        }
        if !self.r#reason_code.is_empty() {
            state.serialize_entry("reasonCode", &self.r#reason_code)?;
        }
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        {
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
        }
        if let Some(some) = self.r#subject.as_ref() {
            state.serialize_entry("subject", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#request_identifier.as_ref() {
            state.serialize_entry("requestIdentifier", some)?;
        }
        match self.r#module {
            GuidanceResponseModule::Uri(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("moduleUri", some)?;
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
                    state.serialize_entry("_moduleUri", &primitive_element)?;
                }
            }
            GuidanceResponseModule::Canonical(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("moduleCanonical", some)?;
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
                    state.serialize_entry("_moduleCanonical", &primitive_element)?;
                }
            }
            GuidanceResponseModule::CodeableConcept(ref value) => {
                state.serialize_entry("moduleCodeableConcept", value)?;
            }
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if let Some(some) = self.r#performer.as_ref() {
            state.serialize_entry("performer", some)?;
        }
        if !self.r#contained.is_empty() {
            state.serialize_entry("contained", &self.r#contained)?;
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
        if !self.r#evaluation_message.is_empty() {
            state.serialize_entry("evaluationMessage", &self.r#evaluation_message)?;
        }
        if let Some(some) = self.r#output_parameters.as_ref() {
            state.serialize_entry("outputParameters", some)?;
        }
        if !self.r#data_requirement.is_empty() {
            state.serialize_entry("dataRequirement", &self.r#data_requirement)?;
        }
        state.end()
    }
}
