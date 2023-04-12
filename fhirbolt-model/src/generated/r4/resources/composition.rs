// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The target composition/document of this relationship."]
#[derive(Debug, Clone, PartialEq)]
pub enum CompositionRelatesToTarget {
    Identifier(Box<super::super::types::Identifier>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for CompositionRelatesToTarget {
    fn default() -> CompositionRelatesToTarget {
        CompositionRelatesToTarget::Invalid
    }
}
#[doc = "A participant who has attested to the accuracy of the composition/document."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompositionAttester {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of attestation the authenticator offers."]
    pub r#mode: super::super::types::Code,
    #[doc = "When the composition was attested by the party."]
    pub r#time: Option<super::super::types::DateTime>,
    #[doc = "Who attested the composition in the specified way."]
    pub r#party: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for CompositionAttester {
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
                if let Some(some) = self.r#mode.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("mode", &some)?;
                }
                if self.r#mode.id.is_some() || !self.r#mode.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#mode.id.as_ref(),
                        extension: &self.r#mode.extension,
                    };
                    state.serialize_entry("_mode", &primitive_element)?;
                }
            } else {
                state.serialize_entry("mode", &self.r#mode)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#time.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("time", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_time", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#time.as_ref() {
                    state.serialize_entry("time", some)?;
                }
            }
            if let Some(some) = self.r#party.as_ref() {
                state.serialize_entry("party", some)?;
            }
            state.end()
        })
    }
}
#[doc = "Relationships that this composition has with other compositions or documents that already exist."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompositionRelatesTo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of relationship that this composition has with anther composition or document."]
    pub r#code: super::super::types::Code,
    #[doc = "The target composition/document of this relationship."]
    pub r#target: CompositionRelatesToTarget,
}
impl serde::ser::Serialize for CompositionRelatesTo {
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
                if let Some(some) = self.r#code.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("code", &some)?;
                }
                if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#code.id.as_ref(),
                        extension: &self.r#code.extension,
                    };
                    state.serialize_entry("_code", &primitive_element)?;
                }
            } else {
                state.serialize_entry("code", &self.r#code)?;
            }
            match self.r#target {
                CompositionRelatesToTarget::Identifier(ref value) => {
                    state.serialize_entry("targetIdentifier", value)?;
                }
                CompositionRelatesToTarget::Reference(ref value) => {
                    state.serialize_entry("targetReference", value)?;
                }
                CompositionRelatesToTarget::Invalid => {
                    return Err(serde::ser::Error::custom("target is a required field"))
                }
            }
            state.end()
        })
    }
}
#[doc = "The clinical service, such as a colonoscopy or an appendectomy, being documented."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompositionEvent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "This list of codes represents the main clinical acts, such as a colonoscopy or an appendectomy, being documented. In some cases, the event is inherent in the typeCode, such as a \"History and Physical Report\" in which the procedure being documented is necessarily a \"History and Physical\" act."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The period of time covered by the documentation. There is no assertion that the documentation is a complete representation for this period, only that it documents events during this time."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The description and/or reference of the event(s) being documented. For example, this could be used to document such a colonoscopy or an appendectomy."]
    pub r#detail: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for CompositionEvent {
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
            if !self.r#code.is_empty() {
                state.serialize_entry("code", &self.r#code)?;
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            if !self.r#detail.is_empty() {
                state.serialize_entry("detail", &self.r#detail)?;
            }
            state.end()
        })
    }
}
#[doc = "The root of the sections that make up the composition."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompositionSection {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The label for this particular section.  This will be part of the rendered content for the document, and is often used to build a table of contents."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "A code identifying the kind of content contained within the section. This must be consistent with the section title."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies who is responsible for the information in this section, not necessarily who typed it in."]
    pub r#author: Vec<Box<super::super::types::Reference>>,
    #[doc = "The actual focus of the section when it is not the subject of the composition, but instead represents something or someone associated with the subject such as (for a patient subject) a spouse, parent, fetus, or donor. If not focus is specified, the focus is assumed to be focus of the parent section, or, for a section in the Composition itself, the subject of the composition. Sections with a focus SHALL only include resources where the logical subject (patient, subject, focus, etc.) matches the section focus, or the resources have no logical subject (few resources)."]
    pub r#focus: Option<Box<super::super::types::Reference>>,
    #[doc = "A human-readable narrative that contains the attested content of the section, used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "How the entry list was prepared - whether it is a working list that is suitable for being maintained on an ongoing basis, or if it represents a snapshot of a list of items from another source, or whether it is a prepared list where items may be marked as added, modified or deleted."]
    pub r#mode: Option<super::super::types::Code>,
    #[doc = "Specifies the order applied to the items in the section entries."]
    pub r#ordered_by: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A reference to the actual resource from which the narrative in the section is derived."]
    pub r#entry: Vec<Box<super::super::types::Reference>>,
    #[doc = "If the section is empty, why the list is empty. An empty section typically has some text explaining the empty reason."]
    pub r#empty_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A nested sub-section within this section."]
    pub r#section: Vec<CompositionSection>,
}
impl serde::ser::Serialize for CompositionSection {
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
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if !self.r#author.is_empty() {
                state.serialize_entry("author", &self.r#author)?;
            }
            if let Some(some) = self.r#focus.as_ref() {
                state.serialize_entry("focus", some)?;
            }
            if let Some(some) = self.r#text.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#mode.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("mode", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_mode", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#mode.as_ref() {
                    state.serialize_entry("mode", some)?;
                }
            }
            if let Some(some) = self.r#ordered_by.as_ref() {
                state.serialize_entry("orderedBy", some)?;
            }
            if !self.r#entry.is_empty() {
                state.serialize_entry("entry", &self.r#entry)?;
            }
            if let Some(some) = self.r#empty_reason.as_ref() {
                state.serialize_entry("emptyReason", some)?;
            }
            if !self.r#section.is_empty() {
                state.serialize_entry("section", &self.r#section)?;
            }
            state.end()
        })
    }
}
#[doc = "A set of healthcare-related information that is assembled together into a single logical package that provides a single coherent statement of meaning, establishes its own context and that has clinical attestation with regard to who is making the statement. A Composition defines the structure and narrative content necessary for a document. However, a Composition alone does not constitute a document. Rather, the Composition must be the first entry in a Bundle where Bundle.type=document, and any other resources referenced from Composition must be included as subsequent entries in the Bundle (for example Patient, Practitioner, Encounter, etc.).\n\nTo support documents, and also to capture the EN13606 notion of an attested commit to the patient EHR, and to allow a set of disparate resources at the information/engineering level to be gathered into a clinical statement."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Composition {
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
    #[doc = "A version-independent identifier for the Composition. This identifier stays constant as the composition is changed over time."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The workflow/clinical status of this composition. The status is a marker for the clinical standing of the document."]
    pub r#status: super::super::types::Code,
    #[doc = "Specifies the particular kind of composition (e.g. History and Physical, Discharge Summary, Progress Note). This usually equates to the purpose of making the composition."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A categorization for the type of the composition - helps for indexing and searching. This may be implied by or derived from the code specified in the Composition Type."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Who or what the composition is about. The composition can be about a person, (patient or healthcare practitioner), a device (e.g. a machine) or even a group of subjects (such as a document about a herd of livestock, or a set of patients that share a common exposure)."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes the clinical encounter or type of care this documentation is associated with."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The composition editing time, when the composition was last logically changed by the author."]
    pub r#date: super::super::types::DateTime,
    #[doc = "Identifies who is responsible for the information in the composition, not necessarily who typed it in."]
    pub r#author: Vec<Box<super::super::types::Reference>>,
    #[doc = "Official human-readable label for the composition."]
    pub r#title: super::super::types::String,
    #[doc = "The code specifying the level of confidentiality of the Composition."]
    pub r#confidentiality: Option<super::super::types::Code>,
    #[doc = "A participant who has attested to the accuracy of the composition/document."]
    pub r#attester: Vec<CompositionAttester>,
    #[doc = "Identifies the organization or group who is responsible for ongoing maintenance of and access to the composition/document information."]
    pub r#custodian: Option<Box<super::super::types::Reference>>,
    #[doc = "Relationships that this composition has with other compositions or documents that already exist."]
    pub r#relates_to: Vec<CompositionRelatesTo>,
    #[doc = "The clinical service, such as a colonoscopy or an appendectomy, being documented."]
    pub r#event: Vec<CompositionEvent>,
    #[doc = "The root of the sections that make up the composition."]
    pub r#section: Vec<CompositionSection>,
}
impl serde::ser::Serialize for Composition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Composition")?;
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
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
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
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if let Some(some) = self.r#subject.as_ref() {
                state.serialize_entry("subject", some)?;
            }
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("date", &some)?;
                }
                if self.r#date.id.is_some() || !self.r#date.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#date.id.as_ref(),
                        extension: &self.r#date.extension,
                    };
                    state.serialize_entry("_date", &primitive_element)?;
                }
            } else {
                state.serialize_entry("date", &self.r#date)?;
            }
            if !self.r#author.is_empty() {
                state.serialize_entry("author", &self.r#author)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#title.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("title", &some)?;
                }
                if self.r#title.id.is_some() || !self.r#title.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#title.id.as_ref(),
                        extension: &self.r#title.extension,
                    };
                    state.serialize_entry("_title", &primitive_element)?;
                }
            } else {
                state.serialize_entry("title", &self.r#title)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#confidentiality.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("confidentiality", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_confidentiality", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#confidentiality.as_ref() {
                    state.serialize_entry("confidentiality", some)?;
                }
            }
            if !self.r#attester.is_empty() {
                state.serialize_entry("attester", &self.r#attester)?;
            }
            if let Some(some) = self.r#custodian.as_ref() {
                state.serialize_entry("custodian", some)?;
            }
            if !self.r#relates_to.is_empty() {
                state.serialize_entry("relatesTo", &self.r#relates_to)?;
            }
            if !self.r#event.is_empty() {
                state.serialize_entry("event", &self.r#event)?;
            }
            if !self.r#section.is_empty() {
                state.serialize_entry("section", &self.r#section)?;
            }
            state.end()
        })
    }
}
