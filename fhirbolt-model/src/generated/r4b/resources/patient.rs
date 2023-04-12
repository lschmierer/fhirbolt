// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Indicates if the individual is deceased or not."]
#[derive(Debug, Clone, PartialEq)]
pub enum PatientDeceased {
    Boolean(Box<super::super::types::Boolean>),
    DateTime(Box<super::super::types::DateTime>),
    Invalid,
}
impl Default for PatientDeceased {
    fn default() -> PatientDeceased {
        PatientDeceased::Invalid
    }
}
#[doc = "Indicates whether the patient is part of a multiple (boolean) or indicates the actual birth order (integer)."]
#[derive(Debug, Clone, PartialEq)]
pub enum PatientMultipleBirth {
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Invalid,
}
impl Default for PatientMultipleBirth {
    fn default() -> PatientMultipleBirth {
        PatientMultipleBirth::Invalid
    }
}
#[doc = "A contact party (e.g. guardian, partner, friend) for the patient."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PatientContact {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The nature of the relationship between the patient and the contact person."]
    pub r#relationship: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A name associated with the contact person."]
    pub r#name: Option<Box<super::super::types::HumanName>>,
    #[doc = "A contact detail for the person, e.g. a telephone number or an email address."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "Address for the contact person."]
    pub r#address: Option<Box<super::super::types::Address>>,
    #[doc = "Administrative Gender - the gender that the contact person is considered to have for administration and record keeping purposes."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "Organization on behalf of which the contact is acting or for which the contact is working."]
    pub r#organization: Option<Box<super::super::types::Reference>>,
    #[doc = "The period during which this contact person or organization is valid to be contacted relating to this patient."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for PatientContact {
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
            if !self.r#relationship.is_empty() {
                state.serialize_entry("relationship", &self.r#relationship)?;
            }
            if let Some(some) = self.r#name.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if !self.r#telecom.is_empty() {
                state.serialize_entry("telecom", &self.r#telecom)?;
            }
            if let Some(some) = self.r#address.as_ref() {
                state.serialize_entry("address", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#gender.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("gender", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_gender", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#gender.as_ref() {
                    state.serialize_entry("gender", some)?;
                }
            }
            if let Some(some) = self.r#organization.as_ref() {
                state.serialize_entry("organization", some)?;
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            state.end()
        })
    }
}
#[doc = "A language which may be used to communicate with the patient about his or her health."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PatientCommunication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The ISO-639-1 alpha 2 code in lower case for the language, optionally followed by a hyphen and the ISO-3166-1 alpha 2 code for the region in upper case; e.g. \"en\" for English, or \"en-US\" for American English versus \"en-EN\" for England English."]
    pub r#language: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates whether or not the patient prefers this language (over other languages he masters up a certain level)."]
    pub r#preferred: Option<super::super::types::Boolean>,
}
impl serde::ser::Serialize for PatientCommunication {
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
            state.serialize_entry("language", &self.r#language)?;
            if _ctx.output_json {
                if let Some(some) = self.r#preferred.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("preferred", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_preferred", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#preferred.as_ref() {
                    state.serialize_entry("preferred", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Link to another patient resource that concerns the same actual patient."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PatientLink {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The other patient resource that the link refers to."]
    pub r#other: Box<super::super::types::Reference>,
    #[doc = "The type of link between this patient resource and another patient resource."]
    pub r#type: super::super::types::Code,
}
impl serde::ser::Serialize for PatientLink {
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
            state.serialize_entry("other", &self.r#other)?;
            if _ctx.output_json {
                if let Some(some) = self.r#type.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("type", &some)?;
                }
                if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#type.id.as_ref(),
                        extension: &self.r#type.extension,
                    };
                    state.serialize_entry("_type", &primitive_element)?;
                }
            } else {
                state.serialize_entry("type", &self.r#type)?;
            }
            state.end()
        })
    }
}
#[doc = "Demographics and other administrative information about an individual or animal receiving care or other health-related services.\n\nTracking patient is the center of the healthcare process."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Patient {
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
    #[doc = "An identifier for this patient."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Whether this patient record is in active use. \nMany systems use this property to mark as non-current patients, such as those that have not been seen for a period of time based on an organization's business rules.\n\nIt is often used to filter patient lists to exclude inactive patients\n\nDeceased patients may also be marked as inactive for the same reasons, but may be active for some time after death."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "A name associated with the individual."]
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    #[doc = "A contact detail (e.g. a telephone number or an email address) by which the individual may be contacted."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "Administrative Gender - the gender that the patient is considered to have for administration and record keeping purposes."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "The date of birth for the individual."]
    pub r#birth_date: Option<super::super::types::Date>,
    #[doc = "Indicates if the individual is deceased or not."]
    pub r#deceased: Option<PatientDeceased>,
    #[doc = "An address for the individual."]
    pub r#address: Vec<Box<super::super::types::Address>>,
    #[doc = "This field contains a patient's most recent marital (civil) status."]
    pub r#marital_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates whether the patient is part of a multiple (boolean) or indicates the actual birth order (integer)."]
    pub r#multiple_birth: Option<PatientMultipleBirth>,
    #[doc = "Image of the patient."]
    pub r#photo: Vec<Box<super::super::types::Attachment>>,
    #[doc = "A contact party (e.g. guardian, partner, friend) for the patient."]
    pub r#contact: Vec<PatientContact>,
    #[doc = "A language which may be used to communicate with the patient about his or her health."]
    pub r#communication: Vec<PatientCommunication>,
    #[doc = "Patient's nominated care provider."]
    pub r#general_practitioner: Vec<Box<super::super::types::Reference>>,
    #[doc = "Organization that is the custodian of the patient record."]
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Link to another patient resource that concerns the same actual patient."]
    pub r#link: Vec<PatientLink>,
}
impl serde::ser::Serialize for Patient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Patient")?;
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
                if let Some(some) = self.r#active.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("active", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_active", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#active.as_ref() {
                    state.serialize_entry("active", some)?;
                }
            }
            if !self.r#name.is_empty() {
                state.serialize_entry("name", &self.r#name)?;
            }
            if !self.r#telecom.is_empty() {
                state.serialize_entry("telecom", &self.r#telecom)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#gender.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("gender", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_gender", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#gender.as_ref() {
                    state.serialize_entry("gender", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#birth_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("birthDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_birthDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#birth_date.as_ref() {
                    state.serialize_entry("birthDate", some)?;
                }
            }
            if let Some(some) = self.r#deceased.as_ref() {
                match some {
                    PatientDeceased::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("deceasedBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_deceasedBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("deceasedBoolean", value)?;
                        }
                    }
                    PatientDeceased::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("deceasedDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_deceasedDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("deceasedDateTime", value)?;
                        }
                    }
                    PatientDeceased::Invalid => {
                        return Err(serde::ser::Error::custom("deceased is invalid"))
                    }
                }
            }
            if !self.r#address.is_empty() {
                state.serialize_entry("address", &self.r#address)?;
            }
            if let Some(some) = self.r#marital_status.as_ref() {
                state.serialize_entry("maritalStatus", some)?;
            }
            if let Some(some) = self.r#multiple_birth.as_ref() {
                match some {
                    PatientMultipleBirth::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("multipleBirthBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_multipleBirthBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("multipleBirthBoolean", value)?;
                        }
                    }
                    PatientMultipleBirth::Integer(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("multipleBirthInteger", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_multipleBirthInteger", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("multipleBirthInteger", value)?;
                        }
                    }
                    PatientMultipleBirth::Invalid => {
                        return Err(serde::ser::Error::custom("multiple_birth is invalid"))
                    }
                }
            }
            if !self.r#photo.is_empty() {
                state.serialize_entry("photo", &self.r#photo)?;
            }
            if !self.r#contact.is_empty() {
                state.serialize_entry("contact", &self.r#contact)?;
            }
            if !self.r#communication.is_empty() {
                state.serialize_entry("communication", &self.r#communication)?;
            }
            if !self.r#general_practitioner.is_empty() {
                state.serialize_entry("generalPractitioner", &self.r#general_practitioner)?;
            }
            if let Some(some) = self.r#managing_organization.as_ref() {
                state.serialize_entry("managingOrganization", some)?;
            }
            if !self.r#link.is_empty() {
                state.serialize_entry("link", &self.r#link)?;
            }
            state.end()
        })
    }
}
