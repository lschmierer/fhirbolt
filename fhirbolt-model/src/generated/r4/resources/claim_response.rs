// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The date or dates when the service or product was supplied, performed or completed."]
#[derive(Debug, Clone, PartialEq)]
pub enum ClaimResponseAddItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ClaimResponseAddItemServiced {
    fn default() -> ClaimResponseAddItemServiced {
        ClaimResponseAddItemServiced::Invalid
    }
}
#[doc = "Where the product or service was provided."]
#[derive(Debug, Clone, PartialEq)]
pub enum ClaimResponseAddItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ClaimResponseAddItemLocation {
    fn default() -> ClaimResponseAddItemLocation {
        ClaimResponseAddItemLocation::Invalid
    }
}
#[doc = "If this item is a group then the values here are a summary of the adjudication of the detail items. If this item is a simple product or service then this is the result of the adjudication of this item."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponseItemAdjudication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code to indicate the information type of this adjudication record. Information types may include the value submitted, maximum values or percentages allowed or payable under the plan, amounts that: the patient is responsible for in aggregate or pertaining to this item; amounts paid by other coverages; and, the benefit payable for this item."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "A code supporting the understanding of the adjudication result and explaining variance from expected amount."]
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Monetary amount associated with the category."]
    pub r#amount: Option<Box<super::super::types::Money>>,
    #[doc = "A non-monetary value associated with the category. Mutually exclusive to the amount element above."]
    pub r#value: Option<super::super::types::Decimal>,
}
impl serde::ser::Serialize for ClaimResponseItemAdjudication {
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
            state.serialize_entry("category", &self.r#category)?;
            if let Some(some) = self.r#reason.as_ref() {
                state.serialize_entry("reason", some)?;
            }
            if let Some(some) = self.r#amount.as_ref() {
                state.serialize_entry("amount", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("value", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_value", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#value.as_ref() {
                    state.serialize_entry("value", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "A sub-detail adjudication of a simple product or service."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponseItemDetailSubDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely reference the claim sub-detail entry."]
    pub r#sub_detail_sequence: super::super::types::PositiveInt,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
}
impl serde::ser::Serialize for ClaimResponseItemDetailSubDetail {
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
                if let Some(some) = self.r#sub_detail_sequence.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("subDetailSequence", &some)?;
                }
                if self.r#sub_detail_sequence.id.is_some()
                    || !self.r#sub_detail_sequence.extension.is_empty()
                {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#sub_detail_sequence.id.as_ref(),
                        extension: &self.r#sub_detail_sequence.extension,
                    };
                    state.serialize_entry("_subDetailSequence", &primitive_element)?;
                }
            } else {
                state.serialize_entry("subDetailSequence", &self.r#sub_detail_sequence)?;
            }
            if _ctx.output_json {
                if !self.r#note_number.is_empty() {
                    let values = self
                        .r#note_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("noteNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#note_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#note_number
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
                        state.serialize_entry("_noteNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#note_number.is_empty() {
                    state.serialize_entry("noteNumber", &self.r#note_number)?;
                }
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            state.end()
        })
    }
}
#[doc = "A claim detail. Either a simple (a product or service) or a 'group' of sub-details which are simple items."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponseItemDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely reference the claim detail entry."]
    pub r#detail_sequence: super::super::types::PositiveInt,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    #[doc = "A sub-detail adjudication of a simple product or service."]
    pub r#sub_detail: Vec<ClaimResponseItemDetailSubDetail>,
}
impl serde::ser::Serialize for ClaimResponseItemDetail {
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
                if let Some(some) = self.r#detail_sequence.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("detailSequence", &some)?;
                }
                if self.r#detail_sequence.id.is_some()
                    || !self.r#detail_sequence.extension.is_empty()
                {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#detail_sequence.id.as_ref(),
                        extension: &self.r#detail_sequence.extension,
                    };
                    state.serialize_entry("_detailSequence", &primitive_element)?;
                }
            } else {
                state.serialize_entry("detailSequence", &self.r#detail_sequence)?;
            }
            if _ctx.output_json {
                if !self.r#note_number.is_empty() {
                    let values = self
                        .r#note_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("noteNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#note_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#note_number
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
                        state.serialize_entry("_noteNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#note_number.is_empty() {
                    state.serialize_entry("noteNumber", &self.r#note_number)?;
                }
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            if !self.r#sub_detail.is_empty() {
                state.serialize_entry("subDetail", &self.r#sub_detail)?;
            }
            state.end()
        })
    }
}
#[doc = "A claim line. Either a simple (a product or service) or a 'group' of details which can also be a simple items or groups of sub-details."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponseItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely reference the claim item entries."]
    pub r#item_sequence: super::super::types::PositiveInt,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "If this item is a group then the values here are a summary of the adjudication of the detail items. If this item is a simple product or service then this is the result of the adjudication of this item."]
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    #[doc = "A claim detail. Either a simple (a product or service) or a 'group' of sub-details which are simple items."]
    pub r#detail: Vec<ClaimResponseItemDetail>,
}
impl serde::ser::Serialize for ClaimResponseItem {
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
                if let Some(some) = self.r#item_sequence.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("itemSequence", &some)?;
                }
                if self.r#item_sequence.id.is_some() || !self.r#item_sequence.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#item_sequence.id.as_ref(),
                        extension: &self.r#item_sequence.extension,
                    };
                    state.serialize_entry("_itemSequence", &primitive_element)?;
                }
            } else {
                state.serialize_entry("itemSequence", &self.r#item_sequence)?;
            }
            if _ctx.output_json {
                if !self.r#note_number.is_empty() {
                    let values = self
                        .r#note_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("noteNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#note_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#note_number
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
                        state.serialize_entry("_noteNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#note_number.is_empty() {
                    state.serialize_entry("noteNumber", &self.r#note_number)?;
                }
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            if !self.r#detail.is_empty() {
                state.serialize_entry("detail", &self.r#detail)?;
            }
            state.end()
        })
    }
}
#[doc = "The third-tier service adjudications for payor added services."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponseAddItemDetailSubDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
}
impl serde::ser::Serialize for ClaimResponseAddItemDetailSubDetail {
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
            state.serialize_entry("productOrService", &self.r#product_or_service)?;
            if !self.r#modifier.is_empty() {
                state.serialize_entry("modifier", &self.r#modifier)?;
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
            if let Some(some) = self.r#net.as_ref() {
                state.serialize_entry("net", some)?;
            }
            if _ctx.output_json {
                if !self.r#note_number.is_empty() {
                    let values = self
                        .r#note_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("noteNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#note_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#note_number
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
                        state.serialize_entry("_noteNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#note_number.is_empty() {
                    state.serialize_entry("noteNumber", &self.r#note_number)?;
                }
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            state.end()
        })
    }
}
#[doc = "The second-tier service adjudications for payor added services."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponseAddItemDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    #[doc = "The third-tier service adjudications for payor added services."]
    pub r#sub_detail: Vec<ClaimResponseAddItemDetailSubDetail>,
}
impl serde::ser::Serialize for ClaimResponseAddItemDetail {
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
            state.serialize_entry("productOrService", &self.r#product_or_service)?;
            if !self.r#modifier.is_empty() {
                state.serialize_entry("modifier", &self.r#modifier)?;
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
            if let Some(some) = self.r#net.as_ref() {
                state.serialize_entry("net", some)?;
            }
            if _ctx.output_json {
                if !self.r#note_number.is_empty() {
                    let values = self
                        .r#note_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("noteNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#note_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#note_number
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
                        state.serialize_entry("_noteNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#note_number.is_empty() {
                    state.serialize_entry("noteNumber", &self.r#note_number)?;
                }
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            if !self.r#sub_detail.is_empty() {
                state.serialize_entry("subDetail", &self.r#sub_detail)?;
            }
            state.end()
        })
    }
}
#[doc = "The first-tier service adjudications for payor added product or service lines."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponseAddItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Claim items which this service line is intended to replace."]
    pub r#item_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The sequence number of the details within the claim item which this line is intended to replace."]
    pub r#detail_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The sequence number of the sub-details within the details within the claim item which this line is intended to replace."]
    pub r#subdetail_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The providers who are authorized for the services rendered to the patient."]
    pub r#provider: Vec<Box<super::super::types::Reference>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date or dates when the service or product was supplied, performed or completed."]
    pub r#serviced: Option<ClaimResponseAddItemServiced>,
    #[doc = "Where the product or service was provided."]
    pub r#location: Option<ClaimResponseAddItemLocation>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Physical service site on the patient (limb, tooth, etc.)."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A region or surface of the bodySite, e.g. limb region or tooth surface(s)."]
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    #[doc = "The second-tier service adjudications for payor added services."]
    pub r#detail: Vec<ClaimResponseAddItemDetail>,
}
impl serde::ser::Serialize for ClaimResponseAddItem {
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
                if !self.r#item_sequence.is_empty() {
                    let values = self
                        .r#item_sequence
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("itemSequence", &values)?;
                    }
                    let requires_elements = self
                        .r#item_sequence
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#item_sequence
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
                        state.serialize_entry("_itemSequence", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#item_sequence.is_empty() {
                    state.serialize_entry("itemSequence", &self.r#item_sequence)?;
                }
            }
            if _ctx.output_json {
                if !self.r#detail_sequence.is_empty() {
                    let values = self
                        .r#detail_sequence
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("detailSequence", &values)?;
                    }
                    let requires_elements = self
                        .r#detail_sequence
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#detail_sequence
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
                        state.serialize_entry("_detailSequence", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#detail_sequence.is_empty() {
                    state.serialize_entry("detailSequence", &self.r#detail_sequence)?;
                }
            }
            if _ctx.output_json {
                if !self.r#subdetail_sequence.is_empty() {
                    let values = self
                        .r#subdetail_sequence
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("subdetailSequence", &values)?;
                    }
                    let requires_elements = self
                        .r#subdetail_sequence
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#subdetail_sequence
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
                        state.serialize_entry("_subdetailSequence", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#subdetail_sequence.is_empty() {
                    state.serialize_entry("subdetailSequence", &self.r#subdetail_sequence)?;
                }
            }
            if !self.r#provider.is_empty() {
                state.serialize_entry("provider", &self.r#provider)?;
            }
            state.serialize_entry("productOrService", &self.r#product_or_service)?;
            if !self.r#modifier.is_empty() {
                state.serialize_entry("modifier", &self.r#modifier)?;
            }
            if !self.r#program_code.is_empty() {
                state.serialize_entry("programCode", &self.r#program_code)?;
            }
            if let Some(some) = self.r#serviced.as_ref() {
                match some {
                    ClaimResponseAddItemServiced::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("servicedDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_servicedDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("servicedDate", value)?;
                        }
                    }
                    ClaimResponseAddItemServiced::Period(ref value) => {
                        state.serialize_entry("servicedPeriod", value)?;
                    }
                    ClaimResponseAddItemServiced::Invalid => {
                        return Err(serde::ser::Error::custom("serviced is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#location.as_ref() {
                match some {
                    ClaimResponseAddItemLocation::CodeableConcept(ref value) => {
                        state.serialize_entry("locationCodeableConcept", value)?;
                    }
                    ClaimResponseAddItemLocation::Address(ref value) => {
                        state.serialize_entry("locationAddress", value)?;
                    }
                    ClaimResponseAddItemLocation::Reference(ref value) => {
                        state.serialize_entry("locationReference", value)?;
                    }
                    ClaimResponseAddItemLocation::Invalid => {
                        return Err(serde::ser::Error::custom("location is invalid"))
                    }
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
            if let Some(some) = self.r#net.as_ref() {
                state.serialize_entry("net", some)?;
            }
            if let Some(some) = self.r#body_site.as_ref() {
                state.serialize_entry("bodySite", some)?;
            }
            if !self.r#sub_site.is_empty() {
                state.serialize_entry("subSite", &self.r#sub_site)?;
            }
            if _ctx.output_json {
                if !self.r#note_number.is_empty() {
                    let values = self
                        .r#note_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("noteNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#note_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#note_number
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
                        state.serialize_entry("_noteNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#note_number.is_empty() {
                    state.serialize_entry("noteNumber", &self.r#note_number)?;
                }
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            if !self.r#detail.is_empty() {
                state.serialize_entry("detail", &self.r#detail)?;
            }
            state.end()
        })
    }
}
#[doc = "Categorized monetary totals for the adjudication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponseTotal {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code to indicate the information type of this adjudication record. Information types may include: the value submitted, maximum values or percentages allowed or payable under the plan, amounts that the patient is responsible for in aggregate or pertaining to this item, amounts paid by other coverages, and the benefit payable for this item."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "Monetary total amount associated with the category."]
    pub r#amount: Box<super::super::types::Money>,
}
impl serde::ser::Serialize for ClaimResponseTotal {
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
            state.serialize_entry("category", &self.r#category)?;
            state.serialize_entry("amount", &self.r#amount)?;
            state.end()
        })
    }
}
#[doc = "Payment details for the adjudication of the claim."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponsePayment {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Whether this represents partial or complete payment of the benefits payable."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Total amount of all adjustments to this payment included in this transaction which are not related to this claim's adjudication."]
    pub r#adjustment: Option<Box<super::super::types::Money>>,
    #[doc = "Reason for the payment adjustment."]
    pub r#adjustment_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Estimated date the payment will be issued or the actual issue date of payment."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "Benefits payable less any payment adjustment."]
    pub r#amount: Box<super::super::types::Money>,
    #[doc = "Issuer's unique identifier for the payment instrument."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
}
impl serde::ser::Serialize for ClaimResponsePayment {
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
            if let Some(some) = self.r#adjustment.as_ref() {
                state.serialize_entry("adjustment", some)?;
            }
            if let Some(some) = self.r#adjustment_reason.as_ref() {
                state.serialize_entry("adjustmentReason", some)?;
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
            state.serialize_entry("amount", &self.r#amount)?;
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            state.end()
        })
    }
}
#[doc = "A note that describes or explains adjudication results in a human readable form."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponseProcessNote {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify a note entry."]
    pub r#number: Option<super::super::types::PositiveInt>,
    #[doc = "The business purpose of the note text."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "The explanation or description associated with the processing."]
    pub r#text: super::super::types::String,
    #[doc = "A code to define the language used in the text of the note."]
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClaimResponseProcessNote {
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
                if let Some(some) = self.r#number.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("number", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_number", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number.as_ref() {
                    state.serialize_entry("number", some)?;
                }
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
                if let Some(some) = self.r#text.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("text", &some)?;
                }
                if self.r#text.id.is_some() || !self.r#text.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#text.id.as_ref(),
                        extension: &self.r#text.extension,
                    };
                    state.serialize_entry("_text", &primitive_element)?;
                }
            } else {
                state.serialize_entry("text", &self.r#text)?;
            }
            if let Some(some) = self.r#language.as_ref() {
                state.serialize_entry("language", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Financial instruments for reimbursement for the health care products and services specified on the claim."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponseInsurance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify insurance entries and provide a sequence of coverages to convey coordination of benefit order."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "A flag to indicate that this Coverage is to be used for adjudication of this claim when set to true."]
    pub r#focal: super::super::types::Boolean,
    #[doc = "Reference to the insurance card level information contained in the Coverage resource. The coverage issuing insurer will use these details to locate the patient's actual coverage within the insurer's information system."]
    pub r#coverage: Box<super::super::types::Reference>,
    #[doc = "A business agreement number established between the provider and the insurer for special business processing purposes."]
    pub r#business_arrangement: Option<super::super::types::String>,
    #[doc = "The result of the adjudication of the line items for the Coverage specified in this insurance."]
    pub r#claim_response: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ClaimResponseInsurance {
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
                if let Some(some) = self.r#sequence.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sequence", &some)?;
                }
                if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#sequence.id.as_ref(),
                        extension: &self.r#sequence.extension,
                    };
                    state.serialize_entry("_sequence", &primitive_element)?;
                }
            } else {
                state.serialize_entry("sequence", &self.r#sequence)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#focal.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("focal", &some)?;
                }
                if self.r#focal.id.is_some() || !self.r#focal.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#focal.id.as_ref(),
                        extension: &self.r#focal.extension,
                    };
                    state.serialize_entry("_focal", &primitive_element)?;
                }
            } else {
                state.serialize_entry("focal", &self.r#focal)?;
            }
            state.serialize_entry("coverage", &self.r#coverage)?;
            if _ctx.output_json {
                if let Some(some) = self.r#business_arrangement.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("businessArrangement", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_businessArrangement", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#business_arrangement.as_ref() {
                    state.serialize_entry("businessArrangement", some)?;
                }
            }
            if let Some(some) = self.r#claim_response.as_ref() {
                state.serialize_entry("claimResponse", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Errors encountered during the processing of the adjudication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponseError {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The sequence number of the line item submitted which contains the error. This value is omitted when the error occurs outside of the item structure."]
    pub r#item_sequence: Option<super::super::types::PositiveInt>,
    #[doc = "The sequence number of the detail within the line item submitted which contains the error. This value is omitted when the error occurs outside of the item structure."]
    pub r#detail_sequence: Option<super::super::types::PositiveInt>,
    #[doc = "The sequence number of the sub-detail within the detail within the line item submitted which contains the error. This value is omitted when the error occurs outside of the item structure."]
    pub r#sub_detail_sequence: Option<super::super::types::PositiveInt>,
    #[doc = "An error code, from a specified code system, which details why the claim could not be adjudicated."]
    pub r#code: Box<super::super::types::CodeableConcept>,
}
impl serde::ser::Serialize for ClaimResponseError {
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
                if let Some(some) = self.r#item_sequence.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("itemSequence", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_itemSequence", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#item_sequence.as_ref() {
                    state.serialize_entry("itemSequence", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#detail_sequence.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("detailSequence", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_detailSequence", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#detail_sequence.as_ref() {
                    state.serialize_entry("detailSequence", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#sub_detail_sequence.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("subDetailSequence", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_subDetailSequence", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#sub_detail_sequence.as_ref() {
                    state.serialize_entry("subDetailSequence", some)?;
                }
            }
            state.serialize_entry("code", &self.r#code)?;
            state.end()
        })
    }
}
#[doc = "This resource provides the adjudication details from the processing of a Claim resource."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClaimResponse {
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
    #[doc = "A unique identifier assigned to this claim response."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "A finer grained suite of claim type codes which may convey additional information such as Inpatient vs Outpatient and/or a specialty service."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A finer grained suite of claim type codes which may convey additional information such as Inpatient vs Outpatient and/or a specialty service."]
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code to indicate whether the nature of the request is: to request adjudication of products and services previously rendered; or requesting authorization and adjudication for provision in the future; or requesting the non-binding adjudication of the listed products and services which could be provided in the future."]
    pub r#use: super::super::types::Code,
    #[doc = "The party to whom the professional services and/or products have been supplied or are being considered and for whom actual for facast reimbursement is sought."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The date this resource was created."]
    pub r#created: super::super::types::DateTime,
    #[doc = "The party responsible for authorization, adjudication and reimbursement."]
    pub r#insurer: Box<super::super::types::Reference>,
    #[doc = "The provider which is responsible for the claim, predetermination or preauthorization."]
    pub r#requestor: Option<Box<super::super::types::Reference>>,
    #[doc = "Original request resource reference."]
    pub r#request: Option<Box<super::super::types::Reference>>,
    #[doc = "The outcome of the claim, predetermination, or preauthorization processing."]
    pub r#outcome: super::super::types::Code,
    #[doc = "A human readable description of the status of the adjudication."]
    pub r#disposition: Option<super::super::types::String>,
    #[doc = "Reference from the Insurer which is used in later communications which refers to this adjudication."]
    pub r#pre_auth_ref: Option<super::super::types::String>,
    #[doc = "The time frame during which this authorization is effective."]
    pub r#pre_auth_period: Option<Box<super::super::types::Period>>,
    #[doc = "Type of Party to be reimbursed: subscriber, provider, other."]
    pub r#payee_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A claim line. Either a simple (a product or service) or a 'group' of details which can also be a simple items or groups of sub-details."]
    pub r#item: Vec<ClaimResponseItem>,
    #[doc = "The first-tier service adjudications for payor added product or service lines."]
    pub r#add_item: Vec<ClaimResponseAddItem>,
    #[doc = "The adjudication results which are presented at the header level rather than at the line-item or add-item levels."]
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    #[doc = "Categorized monetary totals for the adjudication."]
    pub r#total: Vec<ClaimResponseTotal>,
    #[doc = "Payment details for the adjudication of the claim."]
    pub r#payment: Option<ClaimResponsePayment>,
    #[doc = "A code, used only on a response to a preauthorization, to indicate whether the benefits payable have been reserved and for whom."]
    pub r#funds_reserve: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code for the form to be used for printing the content."]
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The actual form, by reference or inclusion, for printing the content or an EOB."]
    pub r#form: Option<Box<super::super::types::Attachment>>,
    #[doc = "A note that describes or explains adjudication results in a human readable form."]
    pub r#process_note: Vec<ClaimResponseProcessNote>,
    #[doc = "Request for additional supporting or authorizing information."]
    pub r#communication_request: Vec<Box<super::super::types::Reference>>,
    #[doc = "Financial instruments for reimbursement for the health care products and services specified on the claim."]
    pub r#insurance: Vec<ClaimResponseInsurance>,
    #[doc = "Errors encountered during the processing of the adjudication."]
    pub r#error: Vec<ClaimResponseError>,
}
impl serde::ser::Serialize for ClaimResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ClaimResponse")?;
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
            state.serialize_entry("type", &self.r#type)?;
            if let Some(some) = self.r#sub_type.as_ref() {
                state.serialize_entry("subType", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#use.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("use", &some)?;
                }
                if self.r#use.id.is_some() || !self.r#use.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#use.id.as_ref(),
                        extension: &self.r#use.extension,
                    };
                    state.serialize_entry("_use", &primitive_element)?;
                }
            } else {
                state.serialize_entry("use", &self.r#use)?;
            }
            state.serialize_entry("patient", &self.r#patient)?;
            if _ctx.output_json {
                if let Some(some) = self.r#created.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("created", &some)?;
                }
                if self.r#created.id.is_some() || !self.r#created.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#created.id.as_ref(),
                        extension: &self.r#created.extension,
                    };
                    state.serialize_entry("_created", &primitive_element)?;
                }
            } else {
                state.serialize_entry("created", &self.r#created)?;
            }
            state.serialize_entry("insurer", &self.r#insurer)?;
            if let Some(some) = self.r#requestor.as_ref() {
                state.serialize_entry("requestor", some)?;
            }
            if let Some(some) = self.r#request.as_ref() {
                state.serialize_entry("request", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#outcome.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("outcome", &some)?;
                }
                if self.r#outcome.id.is_some() || !self.r#outcome.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#outcome.id.as_ref(),
                        extension: &self.r#outcome.extension,
                    };
                    state.serialize_entry("_outcome", &primitive_element)?;
                }
            } else {
                state.serialize_entry("outcome", &self.r#outcome)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#disposition.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("disposition", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_disposition", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#disposition.as_ref() {
                    state.serialize_entry("disposition", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#pre_auth_ref.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("preAuthRef", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_preAuthRef", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#pre_auth_ref.as_ref() {
                    state.serialize_entry("preAuthRef", some)?;
                }
            }
            if let Some(some) = self.r#pre_auth_period.as_ref() {
                state.serialize_entry("preAuthPeriod", some)?;
            }
            if let Some(some) = self.r#payee_type.as_ref() {
                state.serialize_entry("payeeType", some)?;
            }
            if !self.r#item.is_empty() {
                state.serialize_entry("item", &self.r#item)?;
            }
            if !self.r#add_item.is_empty() {
                state.serialize_entry("addItem", &self.r#add_item)?;
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            if !self.r#total.is_empty() {
                state.serialize_entry("total", &self.r#total)?;
            }
            if let Some(some) = self.r#payment.as_ref() {
                state.serialize_entry("payment", some)?;
            }
            if let Some(some) = self.r#funds_reserve.as_ref() {
                state.serialize_entry("fundsReserve", some)?;
            }
            if let Some(some) = self.r#form_code.as_ref() {
                state.serialize_entry("formCode", some)?;
            }
            if let Some(some) = self.r#form.as_ref() {
                state.serialize_entry("form", some)?;
            }
            if !self.r#process_note.is_empty() {
                state.serialize_entry("processNote", &self.r#process_note)?;
            }
            if !self.r#communication_request.is_empty() {
                state.serialize_entry("communicationRequest", &self.r#communication_request)?;
            }
            if !self.r#insurance.is_empty() {
                state.serialize_entry("insurance", &self.r#insurance)?;
            }
            if !self.r#error.is_empty() {
                state.serialize_entry("error", &self.r#error)?;
            }
            state.end()
        })
    }
}
