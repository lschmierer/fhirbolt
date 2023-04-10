// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "Date of procedure."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicinalProductAuthorizationProcedureDate {
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
    Invalid,
}
impl Default for MedicinalProductAuthorizationProcedureDate {
    fn default() -> MedicinalProductAuthorizationProcedureDate {
        MedicinalProductAuthorizationProcedureDate::Invalid
    }
}
#[doc = "Authorization in areas within a country."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductAuthorizationJurisdictionalAuthorization {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The assigned number for the marketing authorization."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Country of authorization."]
    pub r#country: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Jurisdiction within a country."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The legal status of supply in a jurisdiction or region."]
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The start and expected end date of the authorization."]
    pub r#validity_period: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for MedicinalProductAuthorizationJurisdictionalAuthorization {
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
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if let Some(some) = self.r#country.as_ref() {
                state.serialize_entry("country", some)?;
            }
            if !self.r#jurisdiction.is_empty() {
                state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
            }
            if let Some(some) = self.r#legal_status_of_supply.as_ref() {
                state.serialize_entry("legalStatusOfSupply", some)?;
            }
            if let Some(some) = self.r#validity_period.as_ref() {
                state.serialize_entry("validityPeriod", some)?;
            }
            state.end()
        })
    }
}
#[doc = "The regulatory procedure for granting or amending a marketing authorization."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductAuthorizationProcedure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifier for this procedure."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Type of procedure."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Date of procedure."]
    pub r#date: Option<MedicinalProductAuthorizationProcedureDate>,
    #[doc = "Applcations submitted to obtain a marketing authorization."]
    pub r#application: Vec<MedicinalProductAuthorizationProcedure>,
}
impl serde::ser::Serialize for MedicinalProductAuthorizationProcedure {
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
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            state.serialize_entry("type", &self.r#type)?;
            if let Some(some) = self.r#date.as_ref() {
                match some {
                    MedicinalProductAuthorizationProcedureDate::Period(ref value) => {
                        state.serialize_entry("datePeriod", value)?;
                    }
                    MedicinalProductAuthorizationProcedureDate::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("dateDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_dateDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("dateDateTime", value)?;
                        }
                    }
                    MedicinalProductAuthorizationProcedureDate::Invalid => {
                        return Err(serde::ser::Error::custom("date is invalid"))
                    }
                }
            }
            if !self.r#application.is_empty() {
                state.serialize_entry("application", &self.r#application)?;
            }
            state.end()
        })
    }
}
#[doc = "The regulatory authorization of a medicinal product."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductAuthorization {
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
    #[doc = "Business identifier for the marketing authorization, as assigned by a regulator."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The medicinal product that is being authorized."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The country in which the marketing authorization has been granted."]
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Jurisdiction within a country."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the marketing authorization."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date at which the given status has become applicable."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "The date when a suspended the marketing or the marketing authorization of the product is anticipated to be restored."]
    pub r#restore_date: Option<super::super::types::DateTime>,
    #[doc = "The beginning of the time period in which the marketing authorization is in the specific status shall be specified A complete date consisting of day, month and year shall be specified using the ISO 8601 date format."]
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    #[doc = "A period of time after authorization before generic product applicatiosn can be submitted."]
    pub r#data_exclusivity_period: Option<Box<super::super::types::Period>>,
    #[doc = "The date when the first authorization was granted by a Medicines Regulatory Agency."]
    pub r#date_of_first_authorization: Option<super::super::types::DateTime>,
    #[doc = "Date of first marketing authorization for a company's new medicinal product in any country in the World."]
    pub r#international_birth_date: Option<super::super::types::DateTime>,
    #[doc = "The legal framework against which this authorization is granted."]
    pub r#legal_basis: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Authorization in areas within a country."]
    pub r#jurisdictional_authorization:
        Vec<MedicinalProductAuthorizationJurisdictionalAuthorization>,
    #[doc = "Marketing Authorization Holder."]
    pub r#holder: Option<Box<super::super::types::Reference>>,
    #[doc = "Medicines Regulatory Agency."]
    pub r#regulator: Option<Box<super::super::types::Reference>>,
    #[doc = "The regulatory procedure for granting or amending a marketing authorization."]
    pub r#procedure: Option<MedicinalProductAuthorizationProcedure>,
}
impl crate::AnyResource for MedicinalProductAuthorization {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for MedicinalProductAuthorization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "MedicinalProductAuthorization")?;
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
            if let Some(some) = self.r#subject.as_ref() {
                state.serialize_entry("subject", some)?;
            }
            if !self.r#country.is_empty() {
                state.serialize_entry("country", &self.r#country)?;
            }
            if !self.r#jurisdiction.is_empty() {
                state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#restore_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("restoreDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_restoreDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#restore_date.as_ref() {
                    state.serialize_entry("restoreDate", some)?;
                }
            }
            if let Some(some) = self.r#validity_period.as_ref() {
                state.serialize_entry("validityPeriod", some)?;
            }
            if let Some(some) = self.r#data_exclusivity_period.as_ref() {
                state.serialize_entry("dataExclusivityPeriod", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date_of_first_authorization.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("dateOfFirstAuthorization", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_dateOfFirstAuthorization", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#date_of_first_authorization.as_ref() {
                    state.serialize_entry("dateOfFirstAuthorization", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#international_birth_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("internationalBirthDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_internationalBirthDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#international_birth_date.as_ref() {
                    state.serialize_entry("internationalBirthDate", some)?;
                }
            }
            if let Some(some) = self.r#legal_basis.as_ref() {
                state.serialize_entry("legalBasis", some)?;
            }
            if !self.r#jurisdictional_authorization.is_empty() {
                state.serialize_entry(
                    "jurisdictionalAuthorization",
                    &self.r#jurisdictional_authorization,
                )?;
            }
            if let Some(some) = self.r#holder.as_ref() {
                state.serialize_entry("holder", some)?;
            }
            if let Some(some) = self.r#regulator.as_ref() {
                state.serialize_entry("regulator", some)?;
            }
            if let Some(some) = self.r#procedure.as_ref() {
                state.serialize_entry("procedure", some)?;
            }
            state.end()
        })
    }
}
