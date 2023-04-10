// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "Narrows the range of legal concerns to focus on the achievement of specific contractual objectives."]
#[derive(Debug, Clone, PartialEq)]
pub enum ContractTopic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractTopic {
    fn default() -> ContractTopic {
        ContractTopic::Invalid
    }
}
#[doc = "The entity that the term applies to."]
#[derive(Debug, Clone, PartialEq)]
pub enum ContractTermTopic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractTermTopic {
    fn default() -> ContractTermTopic {
        ContractTermTopic::Invalid
    }
}
#[doc = "Response to an offer clause or question text,  which enables selection of values to be agreed to, e.g., the period of participation, the date of occupancy of a rental, warrently duration, or whether biospecimen may be used for further research."]
#[derive(Debug, Clone, PartialEq)]
pub enum ContractTermOfferAnswerValue {
    Boolean(Box<super::super::types::Boolean>),
    Decimal(Box<super::super::types::Decimal>),
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Uri(Box<super::super::types::Uri>),
    Attachment(Box<super::super::types::Attachment>),
    Coding(Box<super::super::types::Coding>),
    Quantity(Box<super::super::types::Quantity>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractTermOfferAnswerValue {
    fn default() -> ContractTermOfferAnswerValue {
        ContractTermOfferAnswerValue::Invalid
    }
}
#[doc = "Specific type of Contract Valued Item that may be priced."]
#[derive(Debug, Clone, PartialEq)]
pub enum ContractTermAssetValuedItemEntity {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractTermAssetValuedItemEntity {
    fn default() -> ContractTermAssetValuedItemEntity {
        ContractTermAssetValuedItemEntity::Invalid
    }
}
#[doc = "When action happens."]
#[derive(Debug, Clone, PartialEq)]
pub enum ContractTermActionOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for ContractTermActionOccurrence {
    fn default() -> ContractTermActionOccurrence {
        ContractTermActionOccurrence::Invalid
    }
}
#[doc = "Human readable rendering of this Contract in a format and representation intended to enhance comprehension and ensure understandability."]
#[derive(Debug, Clone, PartialEq)]
pub enum ContractFriendlyContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractFriendlyContent {
    fn default() -> ContractFriendlyContent {
        ContractFriendlyContent::Invalid
    }
}
#[doc = "Contract legal text in human renderable form."]
#[derive(Debug, Clone, PartialEq)]
pub enum ContractLegalContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractLegalContent {
    fn default() -> ContractLegalContent {
        ContractLegalContent::Invalid
    }
}
#[doc = "Computable Contract conveyed using a policy rule language (e.g. XACML, DKAL, SecPal)."]
#[derive(Debug, Clone, PartialEq)]
pub enum ContractRuleContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractRuleContent {
    fn default() -> ContractRuleContent {
        ContractRuleContent::Invalid
    }
}
#[doc = "Legally binding Contract: This is the signed and legally recognized representation of the Contract, which is considered the \"source of truth\" and which would be the basis for legal action related to enforcement of this Contract."]
#[derive(Debug, Clone, PartialEq)]
pub enum ContractLegallyBinding {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractLegallyBinding {
    fn default() -> ContractLegallyBinding {
        ContractLegallyBinding::Invalid
    }
}
#[doc = "Precusory content developed with a focus and intent of supporting the formation a Contract instance, which may be associated with and transformable into a Contract."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractContentDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Precusory content structure and use, i.e., a boilerplate, template, application for a contract such as an insurance policy or benefits under a program, e.g., workers compensation."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Detailed Precusory content type."]
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The  individual or organization that published the Contract precursor content."]
    pub r#publisher: Option<Box<super::super::types::Reference>>,
    #[doc = "The date (and optionally time) when the contract was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the contract changes."]
    pub r#publication_date: Option<super::super::types::DateTime>,
    #[doc = "amended | appended | cancelled | disputed | entered-in-error | executable | executed | negotiable | offered | policy | rejected | renewed | revoked | resolved | terminated."]
    pub r#publication_status: super::super::types::Code,
    #[doc = "A copyright statement relating to Contract precursor content. Copyright statements are generally legal restrictions on the use and publishing of the Contract precursor content."]
    pub r#copyright: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for ContractContentDefinition {
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
            state.serialize_entry("type", &self.r#type)?;
            if let Some(some) = self.r#sub_type.as_ref() {
                state.serialize_entry("subType", some)?;
            }
            if let Some(some) = self.r#publisher.as_ref() {
                state.serialize_entry("publisher", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#publication_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("publicationDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_publicationDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#publication_date.as_ref() {
                    state.serialize_entry("publicationDate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#publication_status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("publicationStatus", &some)?;
                }
                if self.r#publication_status.id.is_some()
                    || !self.r#publication_status.extension.is_empty()
                {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#publication_status.id.as_ref(),
                        extension: &self.r#publication_status.extension,
                    };
                    state.serialize_entry("_publicationStatus", &primitive_element)?;
                }
            } else {
                state.serialize_entry("publicationStatus", &self.r#publication_status)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#copyright.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("copyright", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_copyright", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#copyright.as_ref() {
                    state.serialize_entry("copyright", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Security labels that protect the handling of information about the term and its elements, which may be specifically identified.."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractTermSecurityLabel {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Number used to link this term or term element to the applicable Security Label."]
    pub r#number: Vec<super::super::types::UnsignedInt>,
    #[doc = "Security label privacy tag that species the level of confidentiality protection required for this term and/or term elements."]
    pub r#classification: Box<super::super::types::Coding>,
    #[doc = "Security label privacy tag that species the applicable privacy and security policies governing this term and/or term elements."]
    pub r#category: Vec<Box<super::super::types::Coding>>,
    #[doc = "Security label privacy tag that species the manner in which term and/or term elements are to be protected."]
    pub r#control: Vec<Box<super::super::types::Coding>>,
}
impl serde::ser::Serialize for ContractTermSecurityLabel {
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
                if !self.r#number.is_empty() {
                    let values = self
                        .r#number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("number", &values)?;
                    }
                    let requires_elements = self
                        .r#number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#number
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
                        state.serialize_entry("_number", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#number.is_empty() {
                    state.serialize_entry("number", &self.r#number)?;
                }
            }
            state.serialize_entry("classification", &self.r#classification)?;
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if !self.r#control.is_empty() {
                state.serialize_entry("control", &self.r#control)?;
            }
            state.end()
        })
    }
}
#[doc = "Offer Recipient."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractTermOfferParty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Participant in the offer."]
    pub r#reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "How the party participates in the offer."]
    pub r#role: Box<super::super::types::CodeableConcept>,
}
impl serde::ser::Serialize for ContractTermOfferParty {
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
            if !self.r#reference.is_empty() {
                state.serialize_entry("reference", &self.r#reference)?;
            }
            state.serialize_entry("role", &self.r#role)?;
            state.end()
        })
    }
}
#[doc = "Response to offer text."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractTermOfferAnswer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Response to an offer clause or question text,  which enables selection of values to be agreed to, e.g., the period of participation, the date of occupancy of a rental, warrently duration, or whether biospecimen may be used for further research."]
    pub r#value: ContractTermOfferAnswerValue,
}
impl serde::ser::Serialize for ContractTermOfferAnswer {
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
            match self.r#value {
                ContractTermOfferAnswerValue::Boolean(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueBoolean", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueBoolean", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueBoolean", value)?;
                    }
                }
                ContractTermOfferAnswerValue::Decimal(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = some.parse::<serde_json::Number>().map_err(|_| {
                                serde::ser::Error::custom("error serializing decimal")
                            })?;
                            state.serialize_entry("valueDecimal", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueDecimal", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueDecimal", value)?;
                    }
                }
                ContractTermOfferAnswerValue::Integer(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueInteger", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueInteger", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueInteger", value)?;
                    }
                }
                ContractTermOfferAnswerValue::Date(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueDate", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueDate", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueDate", value)?;
                    }
                }
                ContractTermOfferAnswerValue::DateTime(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueDateTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueDateTime", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueDateTime", value)?;
                    }
                }
                ContractTermOfferAnswerValue::Time(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueTime", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueTime", value)?;
                    }
                }
                ContractTermOfferAnswerValue::String(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueString", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueString", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueString", value)?;
                    }
                }
                ContractTermOfferAnswerValue::Uri(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueUri", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueUri", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueUri", value)?;
                    }
                }
                ContractTermOfferAnswerValue::Attachment(ref value) => {
                    state.serialize_entry("valueAttachment", value)?;
                }
                ContractTermOfferAnswerValue::Coding(ref value) => {
                    state.serialize_entry("valueCoding", value)?;
                }
                ContractTermOfferAnswerValue::Quantity(ref value) => {
                    state.serialize_entry("valueQuantity", value)?;
                }
                ContractTermOfferAnswerValue::Reference(ref value) => {
                    state.serialize_entry("valueReference", value)?;
                }
                ContractTermOfferAnswerValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is a required field"))
                }
            }
            state.end()
        })
    }
}
#[doc = "The matter of concern in the context of this provision of the agrement."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractTermOffer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Unique identifier for this particular Contract Provision."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Offer Recipient."]
    pub r#party: Vec<ContractTermOfferParty>,
    #[doc = "The owner of an asset has the residual control rights over the asset: the right to decide all usages of the asset in any way not inconsistent with a prior contract, custom, or law (Hart, 1995, p. 30)."]
    pub r#topic: Option<Box<super::super::types::Reference>>,
    #[doc = "Type of Contract Provision such as specific requirements, purposes for actions, obligations, prohibitions, e.g. life time maximum benefit."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Type of choice made by accepting party with respect to an offer made by an offeror/ grantee."]
    pub r#decision: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "How the decision about a Contract was conveyed."]
    pub r#decision_mode: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Response to offer text."]
    pub r#answer: Vec<ContractTermOfferAnswer>,
    #[doc = "Human readable form of this Contract Offer."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "The id of the clause or question text of the offer in the referenced questionnaire/response."]
    pub r#link_id: Vec<super::super::types::String>,
    #[doc = "Security labels that protects the offer."]
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
}
impl serde::ser::Serialize for ContractTermOffer {
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
            if !self.r#party.is_empty() {
                state.serialize_entry("party", &self.r#party)?;
            }
            if let Some(some) = self.r#topic.as_ref() {
                state.serialize_entry("topic", some)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#decision.as_ref() {
                state.serialize_entry("decision", some)?;
            }
            if !self.r#decision_mode.is_empty() {
                state.serialize_entry("decisionMode", &self.r#decision_mode)?;
            }
            if !self.r#answer.is_empty() {
                state.serialize_entry("answer", &self.r#answer)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#link_id.is_empty() {
                    let values = self
                        .r#link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("linkId", &values)?;
                    }
                    let requires_elements = self
                        .r#link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#link_id
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
                        state.serialize_entry("_linkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#link_id.is_empty() {
                    state.serialize_entry("linkId", &self.r#link_id)?;
                }
            }
            if _ctx.output_json {
                if !self.r#security_label_number.is_empty() {
                    let values = self
                        .r#security_label_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("securityLabelNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#security_label_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#security_label_number
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
                        state.serialize_entry("_securityLabelNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#security_label_number.is_empty() {
                    state.serialize_entry("securityLabelNumber", &self.r#security_label_number)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Circumstance of the asset."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractTermAssetContext {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Asset context reference may include the creator, custodian, or owning Person or Organization (e.g., bank, repository),  location held, e.g., building,  jurisdiction."]
    pub r#reference: Option<Box<super::super::types::Reference>>,
    #[doc = "Coded representation of the context generally or of the Referenced entity, such as the asset holder type or location."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Context description."]
    pub r#text: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ContractTermAssetContext {
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
            if let Some(some) = self.r#reference.as_ref() {
                state.serialize_entry("reference", some)?;
            }
            if !self.r#code.is_empty() {
                state.serialize_entry("code", &self.r#code)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Contract Valued Item List."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractTermAssetValuedItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Specific type of Contract Valued Item that may be priced."]
    pub r#entity: Option<ContractTermAssetValuedItemEntity>,
    #[doc = "Identifies a Contract Valued Item instance."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Indicates the time during which this Contract ValuedItem information is effective."]
    pub r#effective_time: Option<super::super::types::DateTime>,
    #[doc = "Specifies the units by which the Contract Valued Item is measured or counted, and quantifies the countable or measurable Contract Valued Item instances."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "A Contract Valued Item unit valuation measure."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of the Contract Valued Item delivered. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "An amount that expresses the weighting (based on difficulty, cost and/or resource intensiveness) associated with the Contract Valued Item delivered. The concept of Points allows for assignment of point values for a Contract Valued Item, such that a monetary amount can be assigned to each point."]
    pub r#points: Option<super::super::types::Decimal>,
    #[doc = "Expresses the product of the Contract Valued Item unitQuantity and the unitPriceAmt. For example, the formula: unit Quantity * unit Price (Cost per Point) * factor Number  * points = net Amount. Quantity, factor and points are assumed to be 1 if not supplied."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Terms of valuation."]
    pub r#payment: Option<super::super::types::String>,
    #[doc = "When payment is due."]
    pub r#payment_date: Option<super::super::types::DateTime>,
    #[doc = "Who will make payment."]
    pub r#responsible: Option<Box<super::super::types::Reference>>,
    #[doc = "Who will receive payment."]
    pub r#recipient: Option<Box<super::super::types::Reference>>,
    #[doc = "Id  of the clause or question text related to the context of this valuedItem in the referenced form or QuestionnaireResponse."]
    pub r#link_id: Vec<super::super::types::String>,
    #[doc = "A set of security labels that define which terms are controlled by this condition."]
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
}
impl serde::ser::Serialize for ContractTermAssetValuedItem {
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
            if let Some(some) = self.r#entity.as_ref() {
                match some {
                    ContractTermAssetValuedItemEntity::CodeableConcept(ref value) => {
                        state.serialize_entry("entityCodeableConcept", value)?;
                    }
                    ContractTermAssetValuedItemEntity::Reference(ref value) => {
                        state.serialize_entry("entityReference", value)?;
                    }
                    ContractTermAssetValuedItemEntity::Invalid => {
                        return Err(serde::ser::Error::custom("entity is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#effective_time.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("effectiveTime", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_effectiveTime", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#effective_time.as_ref() {
                    state.serialize_entry("effectiveTime", some)?;
                }
            }
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            if let Some(some) = self.r#unit_price.as_ref() {
                state.serialize_entry("unitPrice", some)?;
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
                if let Some(some) = self.r#points.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("points", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_points", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#points.as_ref() {
                    state.serialize_entry("points", some)?;
                }
            }
            if let Some(some) = self.r#net.as_ref() {
                state.serialize_entry("net", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#payment.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("payment", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_payment", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#payment.as_ref() {
                    state.serialize_entry("payment", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#payment_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("paymentDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_paymentDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#payment_date.as_ref() {
                    state.serialize_entry("paymentDate", some)?;
                }
            }
            if let Some(some) = self.r#responsible.as_ref() {
                state.serialize_entry("responsible", some)?;
            }
            if let Some(some) = self.r#recipient.as_ref() {
                state.serialize_entry("recipient", some)?;
            }
            if _ctx.output_json {
                if !self.r#link_id.is_empty() {
                    let values = self
                        .r#link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("linkId", &values)?;
                    }
                    let requires_elements = self
                        .r#link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#link_id
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
                        state.serialize_entry("_linkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#link_id.is_empty() {
                    state.serialize_entry("linkId", &self.r#link_id)?;
                }
            }
            if _ctx.output_json {
                if !self.r#security_label_number.is_empty() {
                    let values = self
                        .r#security_label_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("securityLabelNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#security_label_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#security_label_number
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
                        state.serialize_entry("_securityLabelNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#security_label_number.is_empty() {
                    state.serialize_entry("securityLabelNumber", &self.r#security_label_number)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Contract Term Asset List."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractTermAsset {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Differentiates the kind of the asset ."]
    pub r#scope: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Target entity type about which the term may be concerned."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Associated entities."]
    pub r#type_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "May be a subtype or part of an offered asset."]
    pub r#subtype: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specifies the applicability of the term to an asset resource instance, and instances it refers to orinstances that refer to it, and/or are owned by the offeree."]
    pub r#relationship: Option<Box<super::super::types::Coding>>,
    #[doc = "Circumstance of the asset."]
    pub r#context: Vec<ContractTermAssetContext>,
    #[doc = "Description of the quality and completeness of the asset that imay be a factor in its valuation."]
    pub r#condition: Option<super::super::types::String>,
    #[doc = "Type of Asset availability for use or ownership."]
    pub r#period_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Asset relevant contractual time period."]
    pub r#period: Vec<Box<super::super::types::Period>>,
    #[doc = "Time period of asset use."]
    pub r#use_period: Vec<Box<super::super::types::Period>>,
    #[doc = "Clause or question text (Prose Object) concerning the asset in a linked form, such as a QuestionnaireResponse used in the formation of the contract."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "Id [identifier??] of the clause or question text about the asset in the referenced form or QuestionnaireResponse."]
    pub r#link_id: Vec<super::super::types::String>,
    #[doc = "Response to assets."]
    pub r#answer: Vec<ContractTermOfferAnswer>,
    #[doc = "Security labels that protects the asset."]
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
    #[doc = "Contract Valued Item List."]
    pub r#valued_item: Vec<ContractTermAssetValuedItem>,
}
impl serde::ser::Serialize for ContractTermAsset {
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
            if let Some(some) = self.r#scope.as_ref() {
                state.serialize_entry("scope", some)?;
            }
            if !self.r#type.is_empty() {
                state.serialize_entry("type", &self.r#type)?;
            }
            if !self.r#type_reference.is_empty() {
                state.serialize_entry("typeReference", &self.r#type_reference)?;
            }
            if !self.r#subtype.is_empty() {
                state.serialize_entry("subtype", &self.r#subtype)?;
            }
            if let Some(some) = self.r#relationship.as_ref() {
                state.serialize_entry("relationship", some)?;
            }
            if !self.r#context.is_empty() {
                state.serialize_entry("context", &self.r#context)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#condition.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("condition", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_condition", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#condition.as_ref() {
                    state.serialize_entry("condition", some)?;
                }
            }
            if !self.r#period_type.is_empty() {
                state.serialize_entry("periodType", &self.r#period_type)?;
            }
            if !self.r#period.is_empty() {
                state.serialize_entry("period", &self.r#period)?;
            }
            if !self.r#use_period.is_empty() {
                state.serialize_entry("usePeriod", &self.r#use_period)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#link_id.is_empty() {
                    let values = self
                        .r#link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("linkId", &values)?;
                    }
                    let requires_elements = self
                        .r#link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#link_id
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
                        state.serialize_entry("_linkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#link_id.is_empty() {
                    state.serialize_entry("linkId", &self.r#link_id)?;
                }
            }
            if !self.r#answer.is_empty() {
                state.serialize_entry("answer", &self.r#answer)?;
            }
            if _ctx.output_json {
                if !self.r#security_label_number.is_empty() {
                    let values = self
                        .r#security_label_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("securityLabelNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#security_label_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#security_label_number
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
                        state.serialize_entry("_securityLabelNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#security_label_number.is_empty() {
                    state.serialize_entry("securityLabelNumber", &self.r#security_label_number)?;
                }
            }
            if !self.r#valued_item.is_empty() {
                state.serialize_entry("valuedItem", &self.r#valued_item)?;
            }
            state.end()
        })
    }
}
#[doc = "Entity of the action."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractTermActionSubject {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The entity the action is performed or not performed on or for."]
    pub r#reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Role type of agent assigned roles in this Contract."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ContractTermActionSubject {
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
            if !self.r#reference.is_empty() {
                state.serialize_entry("reference", &self.r#reference)?;
            }
            if let Some(some) = self.r#role.as_ref() {
                state.serialize_entry("role", some)?;
            }
            state.end()
        })
    }
}
#[doc = "An actor taking a role in an activity for which it can be assigned some degree of responsibility for the activity taking place."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractTermAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "True if the term prohibits the  action."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "Activity or service obligation to be done or not done, performed or not performed, effectuated or not by this Contract term."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Entity of the action."]
    pub r#subject: Vec<ContractTermActionSubject>,
    #[doc = "Reason or purpose for the action stipulated by this Contract Provision."]
    pub r#intent: Box<super::super::types::CodeableConcept>,
    #[doc = "Id [identifier??] of the clause or question text related to this action in the referenced form or QuestionnaireResponse."]
    pub r#link_id: Vec<super::super::types::String>,
    #[doc = "Current state of the term action."]
    pub r#status: Box<super::super::types::CodeableConcept>,
    #[doc = "Encounter or Episode with primary association to specified term activity."]
    pub r#context: Option<Box<super::super::types::Reference>>,
    #[doc = "Id [identifier??] of the clause or question text related to the requester of this action in the referenced form or QuestionnaireResponse."]
    pub r#context_link_id: Vec<super::super::types::String>,
    #[doc = "When action happens."]
    pub r#occurrence: Option<ContractTermActionOccurrence>,
    #[doc = "Who or what initiated the action and has responsibility for its activation."]
    pub r#requester: Vec<Box<super::super::types::Reference>>,
    #[doc = "Id [identifier??] of the clause or question text related to the requester of this action in the referenced form or QuestionnaireResponse."]
    pub r#requester_link_id: Vec<super::super::types::String>,
    #[doc = "The type of individual that is desired or required to perform or not perform the action."]
    pub r#performer_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of role or competency of an individual desired or required to perform or not perform the action."]
    pub r#performer_role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates who or what is being asked to perform (or not perform) the ction."]
    pub r#performer: Option<Box<super::super::types::Reference>>,
    #[doc = "Id [identifier??] of the clause or question text related to the reason type or reference of this  action in the referenced form or QuestionnaireResponse."]
    pub r#performer_link_id: Vec<super::super::types::String>,
    #[doc = "Rationale for the action to be performed or not performed. Describes why the action is permitted or prohibited."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates another resource whose existence justifies permitting or not permitting this action."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Describes why the action is to be performed or not performed in textual form."]
    pub r#reason: Vec<super::super::types::String>,
    #[doc = "Id [identifier??] of the clause or question text related to the reason type or reference of this  action in the referenced form or QuestionnaireResponse."]
    pub r#reason_link_id: Vec<super::super::types::String>,
    #[doc = "Comments made about the term action made by the requester, performer, subject or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Security labels that protects the action."]
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
}
impl serde::ser::Serialize for ContractTermAction {
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
                if let Some(some) = self.r#do_not_perform.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("doNotPerform", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_doNotPerform", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#do_not_perform.as_ref() {
                    state.serialize_entry("doNotPerform", some)?;
                }
            }
            state.serialize_entry("type", &self.r#type)?;
            if !self.r#subject.is_empty() {
                state.serialize_entry("subject", &self.r#subject)?;
            }
            state.serialize_entry("intent", &self.r#intent)?;
            if _ctx.output_json {
                if !self.r#link_id.is_empty() {
                    let values = self
                        .r#link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("linkId", &values)?;
                    }
                    let requires_elements = self
                        .r#link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#link_id
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
                        state.serialize_entry("_linkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#link_id.is_empty() {
                    state.serialize_entry("linkId", &self.r#link_id)?;
                }
            }
            state.serialize_entry("status", &self.r#status)?;
            if let Some(some) = self.r#context.as_ref() {
                state.serialize_entry("context", some)?;
            }
            if _ctx.output_json {
                if !self.r#context_link_id.is_empty() {
                    let values = self
                        .r#context_link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("contextLinkId", &values)?;
                    }
                    let requires_elements = self
                        .r#context_link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#context_link_id
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
                        state.serialize_entry("_contextLinkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#context_link_id.is_empty() {
                    state.serialize_entry("contextLinkId", &self.r#context_link_id)?;
                }
            }
            if let Some(some) = self.r#occurrence.as_ref() {
                match some {
                    ContractTermActionOccurrence::DateTime(ref value) => {
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
                    ContractTermActionOccurrence::Period(ref value) => {
                        state.serialize_entry("occurrencePeriod", value)?;
                    }
                    ContractTermActionOccurrence::Timing(ref value) => {
                        state.serialize_entry("occurrenceTiming", value)?;
                    }
                    ContractTermActionOccurrence::Invalid => {
                        return Err(serde::ser::Error::custom("occurrence is invalid"))
                    }
                }
            }
            if !self.r#requester.is_empty() {
                state.serialize_entry("requester", &self.r#requester)?;
            }
            if _ctx.output_json {
                if !self.r#requester_link_id.is_empty() {
                    let values = self
                        .r#requester_link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("requesterLinkId", &values)?;
                    }
                    let requires_elements = self
                        .r#requester_link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#requester_link_id
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
                        state.serialize_entry("_requesterLinkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#requester_link_id.is_empty() {
                    state.serialize_entry("requesterLinkId", &self.r#requester_link_id)?;
                }
            }
            if !self.r#performer_type.is_empty() {
                state.serialize_entry("performerType", &self.r#performer_type)?;
            }
            if let Some(some) = self.r#performer_role.as_ref() {
                state.serialize_entry("performerRole", some)?;
            }
            if let Some(some) = self.r#performer.as_ref() {
                state.serialize_entry("performer", some)?;
            }
            if _ctx.output_json {
                if !self.r#performer_link_id.is_empty() {
                    let values = self
                        .r#performer_link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("performerLinkId", &values)?;
                    }
                    let requires_elements = self
                        .r#performer_link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#performer_link_id
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
                        state.serialize_entry("_performerLinkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#performer_link_id.is_empty() {
                    state.serialize_entry("performerLinkId", &self.r#performer_link_id)?;
                }
            }
            if !self.r#reason_code.is_empty() {
                state.serialize_entry("reasonCode", &self.r#reason_code)?;
            }
            if !self.r#reason_reference.is_empty() {
                state.serialize_entry("reasonReference", &self.r#reason_reference)?;
            }
            if _ctx.output_json {
                if !self.r#reason.is_empty() {
                    let values = self
                        .r#reason
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("reason", &values)?;
                    }
                    let requires_elements = self
                        .r#reason
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#reason
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
                        state.serialize_entry("_reason", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#reason.is_empty() {
                    state.serialize_entry("reason", &self.r#reason)?;
                }
            }
            if _ctx.output_json {
                if !self.r#reason_link_id.is_empty() {
                    let values = self
                        .r#reason_link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("reasonLinkId", &values)?;
                    }
                    let requires_elements = self
                        .r#reason_link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#reason_link_id
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
                        state.serialize_entry("_reasonLinkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#reason_link_id.is_empty() {
                    state.serialize_entry("reasonLinkId", &self.r#reason_link_id)?;
                }
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if _ctx.output_json {
                if !self.r#security_label_number.is_empty() {
                    let values = self
                        .r#security_label_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("securityLabelNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#security_label_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#security_label_number
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
                        state.serialize_entry("_securityLabelNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#security_label_number.is_empty() {
                    state.serialize_entry("securityLabelNumber", &self.r#security_label_number)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "One or more Contract Provisions, which may be related and conveyed as a group, and may contain nested groups."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractTerm {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Unique identifier for this particular Contract Provision."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "When this Contract Provision was issued."]
    pub r#issued: Option<super::super::types::DateTime>,
    #[doc = "Relevant time or time-period when this Contract Provision is applicable."]
    pub r#applies: Option<Box<super::super::types::Period>>,
    #[doc = "The entity that the term applies to."]
    pub r#topic: Option<ContractTermTopic>,
    #[doc = "A legal clause or condition contained within a contract that requires one or both parties to perform a particular requirement by some specified time or prevents one or both parties from performing a particular requirement by some specified time."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A specialized legal clause or condition based on overarching contract type."]
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Statement of a provision in a policy or a contract."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "Security labels that protect the handling of information about the term and its elements, which may be specifically identified.."]
    pub r#security_label: Vec<ContractTermSecurityLabel>,
    #[doc = "The matter of concern in the context of this provision of the agrement."]
    pub r#offer: ContractTermOffer,
    #[doc = "Contract Term Asset List."]
    pub r#asset: Vec<ContractTermAsset>,
    #[doc = "An actor taking a role in an activity for which it can be assigned some degree of responsibility for the activity taking place."]
    pub r#action: Vec<ContractTermAction>,
    #[doc = "Nested group of Contract Provisions."]
    pub r#group: Vec<ContractTerm>,
}
impl serde::ser::Serialize for ContractTerm {
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
            if _ctx.output_json {
                if let Some(some) = self.r#issued.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("issued", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_issued", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#issued.as_ref() {
                    state.serialize_entry("issued", some)?;
                }
            }
            if let Some(some) = self.r#applies.as_ref() {
                state.serialize_entry("applies", some)?;
            }
            if let Some(some) = self.r#topic.as_ref() {
                match some {
                    ContractTermTopic::CodeableConcept(ref value) => {
                        state.serialize_entry("topicCodeableConcept", value)?;
                    }
                    ContractTermTopic::Reference(ref value) => {
                        state.serialize_entry("topicReference", value)?;
                    }
                    ContractTermTopic::Invalid => {
                        return Err(serde::ser::Error::custom("topic is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#sub_type.as_ref() {
                state.serialize_entry("subType", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            if !self.r#security_label.is_empty() {
                state.serialize_entry("securityLabel", &self.r#security_label)?;
            }
            state.serialize_entry("offer", &self.r#offer)?;
            if !self.r#asset.is_empty() {
                state.serialize_entry("asset", &self.r#asset)?;
            }
            if !self.r#action.is_empty() {
                state.serialize_entry("action", &self.r#action)?;
            }
            if !self.r#group.is_empty() {
                state.serialize_entry("group", &self.r#group)?;
            }
            state.end()
        })
    }
}
#[doc = "Parties with legal standing in the Contract, including the principal parties, the grantor(s) and grantee(s), which are any person or organization bound by the contract, and any ancillary parties, which facilitate the execution of the contract such as a notary or witness."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractSigner {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Role of this Contract signer, e.g. notary, grantee."]
    pub r#type: Box<super::super::types::Coding>,
    #[doc = "Party which is a signator to this Contract."]
    pub r#party: Box<super::super::types::Reference>,
    #[doc = "Legally binding Contract DSIG signature contents in Base64."]
    pub r#signature: Vec<Box<super::super::types::Signature>>,
}
impl serde::ser::Serialize for ContractSigner {
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
            state.serialize_entry("type", &self.r#type)?;
            state.serialize_entry("party", &self.r#party)?;
            if !self.r#signature.is_empty() {
                state.serialize_entry("signature", &self.r#signature)?;
            }
            state.end()
        })
    }
}
#[doc = "The \"patient friendly language\" versionof the Contract in whole or in parts. \"Patient friendly language\" means the representation of the Contract and Contract Provisions in a manner that is readily accessible and understandable by a layperson in accordance with best practices for communication styles that ensure that those agreeing to or signing the Contract understand the roles, actions, obligations, responsibilities, and implication of the agreement."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractFriendly {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Human readable rendering of this Contract in a format and representation intended to enhance comprehension and ensure understandability."]
    pub r#content: ContractFriendlyContent,
}
impl serde::ser::Serialize for ContractFriendly {
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
            match self.r#content {
                ContractFriendlyContent::Attachment(ref value) => {
                    state.serialize_entry("contentAttachment", value)?;
                }
                ContractFriendlyContent::Reference(ref value) => {
                    state.serialize_entry("contentReference", value)?;
                }
                ContractFriendlyContent::Invalid => {
                    return Err(serde::ser::Error::custom("content is a required field"))
                }
            }
            state.end()
        })
    }
}
#[doc = "List of Legal expressions or representations of this Contract."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractLegal {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Contract legal text in human renderable form."]
    pub r#content: ContractLegalContent,
}
impl serde::ser::Serialize for ContractLegal {
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
            match self.r#content {
                ContractLegalContent::Attachment(ref value) => {
                    state.serialize_entry("contentAttachment", value)?;
                }
                ContractLegalContent::Reference(ref value) => {
                    state.serialize_entry("contentReference", value)?;
                }
                ContractLegalContent::Invalid => {
                    return Err(serde::ser::Error::custom("content is a required field"))
                }
            }
            state.end()
        })
    }
}
#[doc = "List of Computable Policy Rule Language Representations of this Contract."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractRule {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Computable Contract conveyed using a policy rule language (e.g. XACML, DKAL, SecPal)."]
    pub r#content: ContractRuleContent,
}
impl serde::ser::Serialize for ContractRule {
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
            match self.r#content {
                ContractRuleContent::Attachment(ref value) => {
                    state.serialize_entry("contentAttachment", value)?;
                }
                ContractRuleContent::Reference(ref value) => {
                    state.serialize_entry("contentReference", value)?;
                }
                ContractRuleContent::Invalid => {
                    return Err(serde::ser::Error::custom("content is a required field"))
                }
            }
            state.end()
        })
    }
}
#[doc = "Legally enforceable, formally recorded unilateral or bilateral directive i.e., a policy or agreement."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Contract {
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
    #[doc = "Unique identifier for this Contract or a derivative that references a Source Contract."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Canonical identifier for this contract, represented as a URI (globally unique)."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "An edition identifier used for business purposes to label business significant variants."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "The status of the resource instance."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Legal states of the formation of a legal instrument, which is a formally executed written document that can be formally attributed to its author, records and formally expresses a legally enforceable act, process, or contractual duty, obligation, or right, and therefore evidences that act, process, or agreement."]
    pub r#legal_state: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The URL pointing to a FHIR-defined Contract Definition that is adhered to in whole or part by this Contract."]
    pub r#instantiates_canonical: Option<Box<super::super::types::Reference>>,
    #[doc = "The URL pointing to an externally maintained definition that is adhered to in whole or in part by this Contract."]
    pub r#instantiates_uri: Option<super::super::types::Uri>,
    #[doc = "The minimal content derived from the basal information source at a specific stage in its lifecycle."]
    pub r#content_derivative: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When this  Contract was issued."]
    pub r#issued: Option<super::super::types::DateTime>,
    #[doc = "Relevant time or time-period when this Contract is applicable."]
    pub r#applies: Option<Box<super::super::types::Period>>,
    #[doc = "Event resulting in discontinuation or termination of this Contract instance by one or more parties to the contract."]
    pub r#expiration_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The target entity impacted by or of interest to parties to the agreement."]
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    #[doc = "A formally or informally recognized grouping of people, principals, organizations, or jurisdictions formed for the purpose of achieving some form of collective action such as the promulgation, administration and enforcement of contracts and policies."]
    pub r#authority: Vec<Box<super::super::types::Reference>>,
    #[doc = "Recognized governance framework or system operating with a circumscribed scope in accordance with specified principles, policies, processes or procedures for managing rights, actions, or behaviors of parties or principals relative to resources."]
    pub r#domain: Vec<Box<super::super::types::Reference>>,
    #[doc = "Sites in which the contract is complied with,  exercised, or in force."]
    pub r#site: Vec<Box<super::super::types::Reference>>,
    #[doc = "A natural language name identifying this Contract definition, derivative, or instance in any legal state. Provides additional information about its content. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for this Contract definition, derivative, or instance in any legal state.t giving additional information about its content."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "An explanatory or alternate user-friendly title for this Contract definition, derivative, or instance in any legal state.t giving additional information about its content."]
    pub r#subtitle: Option<super::super::types::String>,
    #[doc = "Alternative representation of the title for this Contract definition, derivative, or instance in any legal state., e.g., a domain specific contract number related to legislation."]
    pub r#alias: Vec<super::super::types::String>,
    #[doc = "The individual or organization that authored the Contract definition, derivative, or instance in any legal state."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "A selector of legal concerns for this Contract definition, derivative, or instance in any legal state."]
    pub r#scope: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Narrows the range of legal concerns to focus on the achievement of specific contractual objectives."]
    pub r#topic: Option<ContractTopic>,
    #[doc = "A high-level category for the legal instrument, whether constructed as a Contract definition, derivative, or instance in any legal state.  Provides additional information about its content within the context of the Contract's scope to distinguish the kinds of systems that would be interested in the contract."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Sub-category for the Contract that distinguishes the kinds of systems that would be interested in the Contract within the context of the Contract's scope."]
    pub r#sub_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Precusory content developed with a focus and intent of supporting the formation a Contract instance, which may be associated with and transformable into a Contract."]
    pub r#content_definition: Option<ContractContentDefinition>,
    #[doc = "One or more Contract Provisions, which may be related and conveyed as a group, and may contain nested groups."]
    pub r#term: Vec<ContractTerm>,
    #[doc = "Information that may be needed by/relevant to the performer in their execution of this term action."]
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    #[doc = "Links to Provenance records for past versions of this Contract definition, derivative, or instance, which identify key state transitions or updates that are likely to be relevant to a user looking at the current version of the Contract.  The Provence.entity indicates the target that was changed in the update. <http://build.fhir.org/provenance>-definitions.html#Provenance.entity."]
    pub r#relevant_history: Vec<Box<super::super::types::Reference>>,
    #[doc = "Parties with legal standing in the Contract, including the principal parties, the grantor(s) and grantee(s), which are any person or organization bound by the contract, and any ancillary parties, which facilitate the execution of the contract such as a notary or witness."]
    pub r#signer: Vec<ContractSigner>,
    #[doc = "The \"patient friendly language\" versionof the Contract in whole or in parts. \"Patient friendly language\" means the representation of the Contract and Contract Provisions in a manner that is readily accessible and understandable by a layperson in accordance with best practices for communication styles that ensure that those agreeing to or signing the Contract understand the roles, actions, obligations, responsibilities, and implication of the agreement."]
    pub r#friendly: Vec<ContractFriendly>,
    #[doc = "List of Legal expressions or representations of this Contract."]
    pub r#legal: Vec<ContractLegal>,
    #[doc = "List of Computable Policy Rule Language Representations of this Contract."]
    pub r#rule: Vec<ContractRule>,
    #[doc = "Legally binding Contract: This is the signed and legally recognized representation of the Contract, which is considered the \"source of truth\" and which would be the basis for legal action related to enforcement of this Contract."]
    pub r#legally_binding: Option<ContractLegallyBinding>,
}
impl crate::AnyResource for Contract {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for Contract {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Contract")?;
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
                if let Some(some) = self.r#url.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("url", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_url", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#url.as_ref() {
                    state.serialize_entry("url", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#version.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("version", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_version", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#version.as_ref() {
                    state.serialize_entry("version", some)?;
                }
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
            if let Some(some) = self.r#legal_state.as_ref() {
                state.serialize_entry("legalState", some)?;
            }
            if let Some(some) = self.r#instantiates_canonical.as_ref() {
                state.serialize_entry("instantiatesCanonical", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#instantiates_uri.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("instantiatesUri", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_instantiatesUri", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#instantiates_uri.as_ref() {
                    state.serialize_entry("instantiatesUri", some)?;
                }
            }
            if let Some(some) = self.r#content_derivative.as_ref() {
                state.serialize_entry("contentDerivative", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#issued.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("issued", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_issued", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#issued.as_ref() {
                    state.serialize_entry("issued", some)?;
                }
            }
            if let Some(some) = self.r#applies.as_ref() {
                state.serialize_entry("applies", some)?;
            }
            if let Some(some) = self.r#expiration_type.as_ref() {
                state.serialize_entry("expirationType", some)?;
            }
            if !self.r#subject.is_empty() {
                state.serialize_entry("subject", &self.r#subject)?;
            }
            if !self.r#authority.is_empty() {
                state.serialize_entry("authority", &self.r#authority)?;
            }
            if !self.r#domain.is_empty() {
                state.serialize_entry("domain", &self.r#domain)?;
            }
            if !self.r#site.is_empty() {
                state.serialize_entry("site", &self.r#site)?;
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
                if let Some(some) = self.r#title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("title", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_title", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#title.as_ref() {
                    state.serialize_entry("title", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#subtitle.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("subtitle", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_subtitle", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#subtitle.as_ref() {
                    state.serialize_entry("subtitle", some)?;
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
            if let Some(some) = self.r#author.as_ref() {
                state.serialize_entry("author", some)?;
            }
            if let Some(some) = self.r#scope.as_ref() {
                state.serialize_entry("scope", some)?;
            }
            if let Some(some) = self.r#topic.as_ref() {
                match some {
                    ContractTopic::CodeableConcept(ref value) => {
                        state.serialize_entry("topicCodeableConcept", value)?;
                    }
                    ContractTopic::Reference(ref value) => {
                        state.serialize_entry("topicReference", value)?;
                    }
                    ContractTopic::Invalid => {
                        return Err(serde::ser::Error::custom("topic is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if !self.r#sub_type.is_empty() {
                state.serialize_entry("subType", &self.r#sub_type)?;
            }
            if let Some(some) = self.r#content_definition.as_ref() {
                state.serialize_entry("contentDefinition", some)?;
            }
            if !self.r#term.is_empty() {
                state.serialize_entry("term", &self.r#term)?;
            }
            if !self.r#supporting_info.is_empty() {
                state.serialize_entry("supportingInfo", &self.r#supporting_info)?;
            }
            if !self.r#relevant_history.is_empty() {
                state.serialize_entry("relevantHistory", &self.r#relevant_history)?;
            }
            if !self.r#signer.is_empty() {
                state.serialize_entry("signer", &self.r#signer)?;
            }
            if !self.r#friendly.is_empty() {
                state.serialize_entry("friendly", &self.r#friendly)?;
            }
            if !self.r#legal.is_empty() {
                state.serialize_entry("legal", &self.r#legal)?;
            }
            if !self.r#rule.is_empty() {
                state.serialize_entry("rule", &self.r#rule)?;
            }
            if let Some(some) = self.r#legally_binding.as_ref() {
                match some {
                    ContractLegallyBinding::Attachment(ref value) => {
                        state.serialize_entry("legallyBindingAttachment", value)?;
                    }
                    ContractLegallyBinding::Reference(ref value) => {
                        state.serialize_entry("legallyBindingReference", value)?;
                    }
                    ContractLegallyBinding::Invalid => {
                        return Err(serde::ser::Error::custom("legally_binding is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
