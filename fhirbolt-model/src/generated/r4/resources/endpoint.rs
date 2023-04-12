// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The technical details of an endpoint that can be used for electronic services, such as for web services providing XDS.b or a REST endpoint for another FHIR server. This may include any security context information."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Endpoint {
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
    #[doc = "Identifier for the organization that is used to identify the endpoint across multiple disparate systems."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "active | suspended | error | off | test."]
    pub r#status: super::super::types::Code,
    #[doc = "A coded value that represents the technical details of the usage of this endpoint, such as what WSDLs should be used in what way. (e.g. XDS.b/DICOM/cds-hook)."]
    pub r#connection_type: Box<super::super::types::Coding>,
    #[doc = "A friendly name that this endpoint can be referred to with."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The organization that manages this endpoint (even if technically another organization is hosting this in the cloud, it is the organization associated with the data)."]
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Contact details for a human to contact about the subscription. The primary use of this for system administrator troubleshooting."]
    pub r#contact: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "The interval during which the endpoint is expected to be operational."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The payload type describes the acceptable content that can be communicated on the endpoint."]
    pub r#payload_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The mime type to send the payload in - e.g. application/fhir+xml, application/fhir+json. If the mime type is not specified, then the sender could send any content (including no content depending on the connectionType)."]
    pub r#payload_mime_type: Vec<super::super::types::Code>,
    #[doc = "The uri that describes the actual end-point to connect to."]
    pub r#address: super::super::types::Url,
    #[doc = "Additional headers / information to send as part of the notification."]
    pub r#header: Vec<super::super::types::String>,
}
impl serde::ser::Serialize for Endpoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Endpoint")?;
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
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            state.serialize_entry("connectionType", &self.r#connection_type)?;
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
            if let Some(some) = self.r#managing_organization.as_ref() {
                state.serialize_entry("managingOrganization", some)?;
            }
            if !self.r#contact.is_empty() {
                state.serialize_entry("contact", &self.r#contact)?;
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            if !self.r#payload_type.is_empty() {
                state.serialize_entry("payloadType", &self.r#payload_type)?;
            }
            if _ctx.output_json {
                if !self.r#payload_mime_type.is_empty() {
                    let values = self
                        .r#payload_mime_type
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("payloadMimeType", &values)?;
                    }
                    let requires_elements = self
                        .r#payload_mime_type
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#payload_mime_type
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
                        state.serialize_entry("_payloadMimeType", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#payload_mime_type.is_empty() {
                    state.serialize_entry("payloadMimeType", &self.r#payload_mime_type)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#address.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("address", &some)?;
                }
                if self.r#address.id.is_some() || !self.r#address.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#address.id.as_ref(),
                        extension: &self.r#address.extension,
                    };
                    state.serialize_entry("_address", &primitive_element)?;
                }
            } else {
                state.serialize_entry("address", &self.r#address)?;
            }
            if _ctx.output_json {
                if !self.r#header.is_empty() {
                    let values = self
                        .r#header
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("header", &values)?;
                    }
                    let requires_elements = self
                        .r#header
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#header
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
                        state.serialize_entry("_header", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#header.is_empty() {
                    state.serialize_entry("header", &self.r#header)?;
                }
            }
            state.end()
        })
    }
}
