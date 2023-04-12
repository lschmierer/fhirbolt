// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Time of product collection."]
#[derive(Debug, Clone, PartialEq)]
pub enum BiologicallyDerivedProductCollectionCollected {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for BiologicallyDerivedProductCollectionCollected {
    fn default() -> BiologicallyDerivedProductCollectionCollected {
        BiologicallyDerivedProductCollectionCollected::Invalid
    }
}
#[doc = "Time of processing."]
#[derive(Debug, Clone, PartialEq)]
pub enum BiologicallyDerivedProductProcessingTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for BiologicallyDerivedProductProcessingTime {
    fn default() -> BiologicallyDerivedProductProcessingTime {
        BiologicallyDerivedProductProcessingTime::Invalid
    }
}
#[doc = "Time of manipulation."]
#[derive(Debug, Clone, PartialEq)]
pub enum BiologicallyDerivedProductManipulationTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for BiologicallyDerivedProductManipulationTime {
    fn default() -> BiologicallyDerivedProductManipulationTime {
        BiologicallyDerivedProductManipulationTime::Invalid
    }
}
#[doc = "How this product was collected."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductCollection {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Healthcare professional who is performing the collection."]
    pub r#collector: Option<Box<super::super::types::Reference>>,
    #[doc = "The patient or entity, such as a hospital or vendor in the case of a processed/manipulated/manufactured product, providing the product."]
    pub r#source: Option<Box<super::super::types::Reference>>,
    #[doc = "Time of product collection."]
    pub r#collected: Option<BiologicallyDerivedProductCollectionCollected>,
}
impl serde::ser::Serialize for BiologicallyDerivedProductCollection {
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
            if let Some(some) = self.r#collector.as_ref() {
                state.serialize_entry("collector", some)?;
            }
            if let Some(some) = self.r#source.as_ref() {
                state.serialize_entry("source", some)?;
            }
            if let Some(some) = self.r#collected.as_ref() {
                match some {
                    BiologicallyDerivedProductCollectionCollected::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("collectedDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_collectedDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("collectedDateTime", value)?;
                        }
                    }
                    BiologicallyDerivedProductCollectionCollected::Period(ref value) => {
                        state.serialize_entry("collectedPeriod", value)?;
                    }
                    BiologicallyDerivedProductCollectionCollected::Invalid => {
                        return Err(serde::ser::Error::custom("collected is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "Any processing of the product during collection that does not change the fundamental nature of the product. For example adding anti-coagulants during the collection of Peripheral Blood Stem Cells."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductProcessing {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Description of of processing."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Procesing code."]
    pub r#procedure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Substance added during processing."]
    pub r#additive: Option<Box<super::super::types::Reference>>,
    #[doc = "Time of processing."]
    pub r#time: Option<BiologicallyDerivedProductProcessingTime>,
}
impl serde::ser::Serialize for BiologicallyDerivedProductProcessing {
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
            if let Some(some) = self.r#procedure.as_ref() {
                state.serialize_entry("procedure", some)?;
            }
            if let Some(some) = self.r#additive.as_ref() {
                state.serialize_entry("additive", some)?;
            }
            if let Some(some) = self.r#time.as_ref() {
                match some {
                    BiologicallyDerivedProductProcessingTime::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("timeDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_timeDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("timeDateTime", value)?;
                        }
                    }
                    BiologicallyDerivedProductProcessingTime::Period(ref value) => {
                        state.serialize_entry("timePeriod", value)?;
                    }
                    BiologicallyDerivedProductProcessingTime::Invalid => {
                        return Err(serde::ser::Error::custom("time is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "Any manipulation of product post-collection that is intended to alter the product.  For example a buffy-coat enrichment or CD8 reduction of Peripheral Blood Stem Cells to make it more suitable for infusion."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductManipulation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Description of manipulation."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Time of manipulation."]
    pub r#time: Option<BiologicallyDerivedProductManipulationTime>,
}
impl serde::ser::Serialize for BiologicallyDerivedProductManipulation {
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
            if let Some(some) = self.r#time.as_ref() {
                match some {
                    BiologicallyDerivedProductManipulationTime::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("timeDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_timeDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("timeDateTime", value)?;
                        }
                    }
                    BiologicallyDerivedProductManipulationTime::Period(ref value) => {
                        state.serialize_entry("timePeriod", value)?;
                    }
                    BiologicallyDerivedProductManipulationTime::Invalid => {
                        return Err(serde::ser::Error::custom("time is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
#[doc = "Product storage."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductStorage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Description of storage."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Storage temperature."]
    pub r#temperature: Option<super::super::types::Decimal>,
    #[doc = "Temperature scale used."]
    pub r#scale: Option<super::super::types::Code>,
    #[doc = "Storage timeperiod."]
    pub r#duration: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for BiologicallyDerivedProductStorage {
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
                if let Some(some) = self.r#temperature.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("temperature", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_temperature", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#temperature.as_ref() {
                    state.serialize_entry("temperature", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#scale.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("scale", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_scale", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#scale.as_ref() {
                    state.serialize_entry("scale", some)?;
                }
            }
            if let Some(some) = self.r#duration.as_ref() {
                state.serialize_entry("duration", some)?;
            }
            state.end()
        })
    }
}
#[doc = "A material substance originating from a biological entity intended to be transplanted or infused\ninto another (possibly the same) biological entity."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProduct {
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
    #[doc = "This records identifiers associated with this biologically derived product instance that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate (e.g. in CDA documents, or in written / printed documentation)."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Broad category of this product."]
    pub r#product_category: Option<super::super::types::Code>,
    #[doc = "A code that identifies the kind of this biologically derived product (SNOMED Ctcode)."]
    pub r#product_code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether the product is currently available."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Procedure request to obtain this biologically derived product."]
    pub r#request: Vec<Box<super::super::types::Reference>>,
    #[doc = "Number of discrete units within this product."]
    pub r#quantity: Option<super::super::types::Integer>,
    #[doc = "Parent product (if any)."]
    pub r#parent: Vec<Box<super::super::types::Reference>>,
    #[doc = "How this product was collected."]
    pub r#collection: Option<BiologicallyDerivedProductCollection>,
    #[doc = "Any processing of the product during collection that does not change the fundamental nature of the product. For example adding anti-coagulants during the collection of Peripheral Blood Stem Cells."]
    pub r#processing: Vec<BiologicallyDerivedProductProcessing>,
    #[doc = "Any manipulation of product post-collection that is intended to alter the product.  For example a buffy-coat enrichment or CD8 reduction of Peripheral Blood Stem Cells to make it more suitable for infusion."]
    pub r#manipulation: Option<BiologicallyDerivedProductManipulation>,
    #[doc = "Product storage."]
    pub r#storage: Vec<BiologicallyDerivedProductStorage>,
}
impl serde::ser::Serialize for BiologicallyDerivedProduct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "BiologicallyDerivedProduct")?;
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
                if let Some(some) = self.r#product_category.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("productCategory", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_productCategory", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#product_category.as_ref() {
                    state.serialize_entry("productCategory", some)?;
                }
            }
            if let Some(some) = self.r#product_code.as_ref() {
                state.serialize_entry("productCode", some)?;
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
            if !self.r#request.is_empty() {
                state.serialize_entry("request", &self.r#request)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#quantity.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("quantity", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_quantity", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#quantity.as_ref() {
                    state.serialize_entry("quantity", some)?;
                }
            }
            if !self.r#parent.is_empty() {
                state.serialize_entry("parent", &self.r#parent)?;
            }
            if let Some(some) = self.r#collection.as_ref() {
                state.serialize_entry("collection", some)?;
            }
            if !self.r#processing.is_empty() {
                state.serialize_entry("processing", &self.r#processing)?;
            }
            if let Some(some) = self.r#manipulation.as_ref() {
                state.serialize_entry("manipulation", some)?;
            }
            if !self.r#storage.is_empty() {
                state.serialize_entry("storage", &self.r#storage)?;
            }
            state.end()
        })
    }
}
