// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "Information about the primary source(s) involved in validation."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VerificationResultPrimarySource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Reference to the primary source."]
    pub r#who: Option<Box<super::super::types::Reference>>,
    #[doc = "Type of primary source (License Board; Primary Education; Continuing Education; Postal Service; Relationship owner; Registration Authority; legal source; issuing source; authoritative source)."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Method for communicating with the primary source (manual; API; Push)."]
    pub r#communication_method: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Status of the validation of the target against the primary source (successful; failed; unknown)."]
    pub r#validation_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the target was validated against the primary source."]
    pub r#validation_date: Option<super::super::types::DateTime>,
    #[doc = "Ability of the primary source to push updates/alerts (yes; no; undetermined)."]
    pub r#can_push_updates: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Type of alerts/updates the primary source can send (specific requested changes; any changes; as defined by source)."]
    pub r#push_type_available: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for VerificationResultPrimarySource {
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
            if let Some(some) = self.r#who.as_ref() {
                state.serialize_entry("who", some)?;
            }
            if !self.r#type.is_empty() {
                state.serialize_entry("type", &self.r#type)?;
            }
            if !self.r#communication_method.is_empty() {
                state.serialize_entry("communicationMethod", &self.r#communication_method)?;
            }
            if let Some(some) = self.r#validation_status.as_ref() {
                state.serialize_entry("validationStatus", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#validation_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("validationDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_validationDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#validation_date.as_ref() {
                    state.serialize_entry("validationDate", some)?;
                }
            }
            if let Some(some) = self.r#can_push_updates.as_ref() {
                state.serialize_entry("canPushUpdates", some)?;
            }
            if !self.r#push_type_available.is_empty() {
                state.serialize_entry("pushTypeAvailable", &self.r#push_type_available)?;
            }
            state.end()
        })
    }
}
#[doc = "Information about the entity attesting to information."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VerificationResultAttestation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The individual or organization attesting to information."]
    pub r#who: Option<Box<super::super::types::Reference>>,
    #[doc = "When the who is asserting on behalf of another (organization or individual)."]
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    #[doc = "The method by which attested information was submitted/retrieved (manual; API; Push)."]
    pub r#communication_method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date the information was attested to."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "A digital identity certificate associated with the attestation source."]
    pub r#source_identity_certificate: Option<super::super::types::String>,
    #[doc = "A digital identity certificate associated with the proxy entity submitting attested information on behalf of the attestation source."]
    pub r#proxy_identity_certificate: Option<super::super::types::String>,
    #[doc = "Signed assertion by the proxy entity indicating that they have the right to submit attested information on behalf of the attestation source."]
    pub r#proxy_signature: Option<Box<super::super::types::Signature>>,
    #[doc = "Signed assertion by the attestation source that they have attested to the information."]
    pub r#source_signature: Option<Box<super::super::types::Signature>>,
}
impl serde::ser::Serialize for VerificationResultAttestation {
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
            if let Some(some) = self.r#who.as_ref() {
                state.serialize_entry("who", some)?;
            }
            if let Some(some) = self.r#on_behalf_of.as_ref() {
                state.serialize_entry("onBehalfOf", some)?;
            }
            if let Some(some) = self.r#communication_method.as_ref() {
                state.serialize_entry("communicationMethod", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#source_identity_certificate.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sourceIdentityCertificate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sourceIdentityCertificate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#source_identity_certificate.as_ref() {
                    state.serialize_entry("sourceIdentityCertificate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#proxy_identity_certificate.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("proxyIdentityCertificate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_proxyIdentityCertificate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#proxy_identity_certificate.as_ref() {
                    state.serialize_entry("proxyIdentityCertificate", some)?;
                }
            }
            if let Some(some) = self.r#proxy_signature.as_ref() {
                state.serialize_entry("proxySignature", some)?;
            }
            if let Some(some) = self.r#source_signature.as_ref() {
                state.serialize_entry("sourceSignature", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Information about the entity validating information."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VerificationResultValidator {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Reference to the organization validating information."]
    pub r#organization: Box<super::super::types::Reference>,
    #[doc = "A digital identity certificate associated with the validator."]
    pub r#identity_certificate: Option<super::super::types::String>,
    #[doc = "Signed assertion by the validator that they have validated the information."]
    pub r#attestation_signature: Option<Box<super::super::types::Signature>>,
}
impl serde::ser::Serialize for VerificationResultValidator {
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
            state.serialize_entry("organization", &self.r#organization)?;
            if _ctx.output_json {
                if let Some(some) = self.r#identity_certificate.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("identityCertificate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_identityCertificate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#identity_certificate.as_ref() {
                    state.serialize_entry("identityCertificate", some)?;
                }
            }
            if let Some(some) = self.r#attestation_signature.as_ref() {
                state.serialize_entry("attestationSignature", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Describes validation requirements, source(s), status and dates for one or more elements."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VerificationResult {
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
    #[doc = "A resource that was validated."]
    pub r#target: Vec<Box<super::super::types::Reference>>,
    #[doc = "The fhirpath location(s) within the resource that was validated."]
    pub r#target_location: Vec<super::super::types::String>,
    #[doc = "The frequency with which the target must be validated (none; initial; periodic)."]
    pub r#need: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The validation status of the target (attested; validated; in process; requires revalidation; validation failed; revalidation failed)."]
    pub r#status: super::super::types::Code,
    #[doc = "When the validation status was updated."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "What the target is validated against (nothing; primary source; multiple sources)."]
    pub r#validation_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The primary process by which the target is validated (edit check; value set; primary source; multiple sources; standalone; in context)."]
    pub r#validation_process: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Frequency of revalidation."]
    pub r#frequency: Option<Box<super::super::types::Timing>>,
    #[doc = "The date/time validation was last completed (including failed validations)."]
    pub r#last_performed: Option<super::super::types::DateTime>,
    #[doc = "The date when target is next validated, if appropriate."]
    pub r#next_scheduled: Option<super::super::types::Date>,
    #[doc = "The result if validation fails (fatal; warning; record only; none)."]
    pub r#failure_action: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Information about the primary source(s) involved in validation."]
    pub r#primary_source: Vec<VerificationResultPrimarySource>,
    #[doc = "Information about the entity attesting to information."]
    pub r#attestation: Option<VerificationResultAttestation>,
    #[doc = "Information about the entity validating information."]
    pub r#validator: Vec<VerificationResultValidator>,
}
impl crate::AnyResource for VerificationResult {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for VerificationResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "VerificationResult")?;
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
            if !self.r#target.is_empty() {
                state.serialize_entry("target", &self.r#target)?;
            }
            if _ctx.output_json {
                if !self.r#target_location.is_empty() {
                    let values = self
                        .r#target_location
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("targetLocation", &values)?;
                    }
                    let requires_elements = self
                        .r#target_location
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#target_location
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
                        state.serialize_entry("_targetLocation", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#target_location.is_empty() {
                    state.serialize_entry("targetLocation", &self.r#target_location)?;
                }
            }
            if let Some(some) = self.r#need.as_ref() {
                state.serialize_entry("need", some)?;
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
            if let Some(some) = self.r#validation_type.as_ref() {
                state.serialize_entry("validationType", some)?;
            }
            if !self.r#validation_process.is_empty() {
                state.serialize_entry("validationProcess", &self.r#validation_process)?;
            }
            if let Some(some) = self.r#frequency.as_ref() {
                state.serialize_entry("frequency", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#last_performed.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("lastPerformed", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lastPerformed", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#last_performed.as_ref() {
                    state.serialize_entry("lastPerformed", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#next_scheduled.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("nextScheduled", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_nextScheduled", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#next_scheduled.as_ref() {
                    state.serialize_entry("nextScheduled", some)?;
                }
            }
            if let Some(some) = self.r#failure_action.as_ref() {
                state.serialize_entry("failureAction", some)?;
            }
            if !self.r#primary_source.is_empty() {
                state.serialize_entry("primarySource", &self.r#primary_source)?;
            }
            if let Some(some) = self.r#attestation.as_ref() {
                state.serialize_entry("attestation", some)?;
            }
            if !self.r#validator.is_empty() {
                state.serialize_entry("validator", &self.r#validator)?;
            }
            state.end()
        })
    }
}
