// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Date/time(s) or duration when the charged service was applied."]
#[derive(Debug, Clone, PartialEq)]
pub enum ChargeItemOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for ChargeItemOccurrence {
    fn default() -> ChargeItemOccurrence {
        ChargeItemOccurrence::Invalid
    }
}
#[doc = "Identifies the device, food, drug or other product being charged either by type code or reference to an instance."]
#[derive(Debug, Clone, PartialEq)]
pub enum ChargeItemProduct {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for ChargeItemProduct {
    fn default() -> ChargeItemProduct {
        ChargeItemProduct::Invalid
    }
}
#[doc = "Indicates who or what performed or participated in the charged service."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChargeItemPerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Describes the type of performance or participation(e.g. primary surgeon, anesthesiologiest, etc.)."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The device, practitioner, etc. who performed or participated in the service."]
    pub r#actor: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for ChargeItemPerformer {
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
            if let Some(some) = self.r#function.as_ref() {
                state.serialize_entry("function", some)?;
            }
            state.serialize_entry("actor", &self.r#actor)?;
            state.end()
        })
    }
}
#[doc = "The resource ChargeItem describes the provision of healthcare provider products for a certain patient, therefore referring not only to the product, but containing in addition details of the provision, like date, time, amounts and participating organizations and persons. Main Usage of the ChargeItem is to enable the billing process and internal cost allocation."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChargeItem {
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
    #[doc = "Identifiers assigned to this event performer or other systems."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "References the (external) source of pricing information, rules of application for the code this ChargeItem uses."]
    pub r#definition_uri: Vec<super::super::types::Uri>,
    #[doc = "References the source of pricing information, rules of application for the code this ChargeItem uses."]
    pub r#definition_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The current state of the ChargeItem."]
    pub r#status: super::super::types::Code,
    #[doc = "ChargeItems can be grouped to larger ChargeItems covering the whole set."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "A code that identifies the charge, like a billing code."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The individual or set of individuals the action is being or was performed on."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The encounter or episode of care that establishes the context for this event."]
    pub r#context: Option<Box<super::super::types::Reference>>,
    #[doc = "Date/time(s) or duration when the charged service was applied."]
    pub r#occurrence: Option<ChargeItemOccurrence>,
    #[doc = "Indicates who or what performed or participated in the charged service."]
    pub r#performer: Vec<ChargeItemPerformer>,
    #[doc = "The organization requesting the service."]
    pub r#performing_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "The organization performing the service."]
    pub r#requesting_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "The financial cost center permits the tracking of charge attribution."]
    pub r#cost_center: Option<Box<super::super::types::Reference>>,
    #[doc = "Quantity of which the charge item has been serviced."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The anatomical location where the related service has been applied."]
    pub r#bodysite: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Factor overriding the factor determined by the rules associated with the code."]
    pub r#factor_override: Option<super::super::types::Decimal>,
    #[doc = "Total price of the charge overriding the list price associated with the code."]
    pub r#price_override: Option<Box<super::super::types::Money>>,
    #[doc = "If the list price or the rule-based factor associated with the code is overridden, this attribute can capture a text to indicate the  reason for this action."]
    pub r#override_reason: Option<super::super::types::String>,
    #[doc = "The device, practitioner, etc. who entered the charge item."]
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    #[doc = "Date the charge item was entered."]
    pub r#entered_date: Option<super::super::types::DateTime>,
    #[doc = "Describes why the event occurred in coded or textual form."]
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicated the rendered service that caused this charge."]
    pub r#service: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies the device, food, drug or other product being charged either by type code or reference to an instance."]
    pub r#product: Option<ChargeItemProduct>,
    #[doc = "Account into which this ChargeItems belongs."]
    pub r#account: Vec<Box<super::super::types::Reference>>,
    #[doc = "Comments made about the event by the performer, subject or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Further information supporting this charge."]
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ChargeItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ChargeItem")?;
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
                if !self.r#definition_uri.is_empty() {
                    let values = self
                        .r#definition_uri
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("definitionUri", &values)?;
                    }
                    let requires_elements = self
                        .r#definition_uri
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#definition_uri
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
                        state.serialize_entry("_definitionUri", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#definition_uri.is_empty() {
                    state.serialize_entry("definitionUri", &self.r#definition_uri)?;
                }
            }
            if _ctx.output_json {
                if !self.r#definition_canonical.is_empty() {
                    let values = self
                        .r#definition_canonical
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("definitionCanonical", &values)?;
                    }
                    let requires_elements = self
                        .r#definition_canonical
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#definition_canonical
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
                        state.serialize_entry("_definitionCanonical", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#definition_canonical.is_empty() {
                    state.serialize_entry("definitionCanonical", &self.r#definition_canonical)?;
                }
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
            if !self.r#part_of.is_empty() {
                state.serialize_entry("partOf", &self.r#part_of)?;
            }
            state.serialize_entry("code", &self.r#code)?;
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#context.as_ref() {
                state.serialize_entry("context", some)?;
            }
            if let Some(some) = self.r#occurrence.as_ref() {
                match some {
                    ChargeItemOccurrence::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("occurrenceDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_occurrenceDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("occurrenceDateTime", value)?;
                        }
                    }
                    ChargeItemOccurrence::Period(ref value) => {
                        state.serialize_entry("occurrencePeriod", value)?;
                    }
                    ChargeItemOccurrence::Timing(ref value) => {
                        state.serialize_entry("occurrenceTiming", value)?;
                    }
                    ChargeItemOccurrence::Invalid => {
                        return Err(serde::ser::Error::custom("occurrence is invalid"))
                    }
                }
            }
            if !self.r#performer.is_empty() {
                state.serialize_entry("performer", &self.r#performer)?;
            }
            if let Some(some) = self.r#performing_organization.as_ref() {
                state.serialize_entry("performingOrganization", some)?;
            }
            if let Some(some) = self.r#requesting_organization.as_ref() {
                state.serialize_entry("requestingOrganization", some)?;
            }
            if let Some(some) = self.r#cost_center.as_ref() {
                state.serialize_entry("costCenter", some)?;
            }
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            if !self.r#bodysite.is_empty() {
                state.serialize_entry("bodysite", &self.r#bodysite)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#factor_override.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("factorOverride", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_factorOverride", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#factor_override.as_ref() {
                    state.serialize_entry("factorOverride", some)?;
                }
            }
            if let Some(some) = self.r#price_override.as_ref() {
                state.serialize_entry("priceOverride", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#override_reason.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("overrideReason", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_overrideReason", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#override_reason.as_ref() {
                    state.serialize_entry("overrideReason", some)?;
                }
            }
            if let Some(some) = self.r#enterer.as_ref() {
                state.serialize_entry("enterer", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#entered_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("enteredDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_enteredDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#entered_date.as_ref() {
                    state.serialize_entry("enteredDate", some)?;
                }
            }
            if !self.r#reason.is_empty() {
                state.serialize_entry("reason", &self.r#reason)?;
            }
            if !self.r#service.is_empty() {
                state.serialize_entry("service", &self.r#service)?;
            }
            if let Some(some) = self.r#product.as_ref() {
                match some {
                    ChargeItemProduct::Reference(ref value) => {
                        state.serialize_entry("productReference", value)?;
                    }
                    ChargeItemProduct::CodeableConcept(ref value) => {
                        state.serialize_entry("productCodeableConcept", value)?;
                    }
                    ChargeItemProduct::Invalid => {
                        return Err(serde::ser::Error::custom("product is invalid"))
                    }
                }
            }
            if !self.r#account.is_empty() {
                state.serialize_entry("account", &self.r#account)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#supporting_information.is_empty() {
                state.serialize_entry("supportingInformation", &self.r#supporting_information)?;
            }
            state.end()
        })
    }
}
