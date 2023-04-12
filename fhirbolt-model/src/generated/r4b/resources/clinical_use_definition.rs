// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Timing or duration information, that may be associated with use with the indicated condition e.g. Adult patients suffering from myocardial infarction (from a few days until less than 35 days), ischaemic stroke (from 7 days until less than 6 months)."]
#[derive(Debug, Clone, PartialEq)]
pub enum ClinicalUseDefinitionIndicationDuration {
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for ClinicalUseDefinitionIndicationDuration {
    fn default() -> ClinicalUseDefinitionIndicationDuration {
        ClinicalUseDefinitionIndicationDuration::Invalid
    }
}
#[doc = "The specific medication, food or laboratory test that interacts."]
#[derive(Debug, Clone, PartialEq)]
pub enum ClinicalUseDefinitionInteractionInteractantItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for ClinicalUseDefinitionInteractionInteractantItem {
    fn default() -> ClinicalUseDefinitionInteractionInteractantItem {
        ClinicalUseDefinitionInteractionInteractantItem::Invalid
    }
}
#[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the contraindication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionContraindicationOtherTherapy {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of relationship between the medicinal product indication or contraindication and another therapy."]
    pub r#relationship_type: Box<super::super::types::CodeableConcept>,
    #[doc = "Reference to a specific medication (active substance, medicinal product or class of products) as part of an indication or contraindication."]
    pub r#therapy: Box<super::super::types::CodeableReference>,
}
impl serde::ser::Serialize for ClinicalUseDefinitionContraindicationOtherTherapy {
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
            state.serialize_entry("relationshipType", &self.r#relationship_type)?;
            state.serialize_entry("therapy", &self.r#therapy)?;
            state.end()
        })
    }
}
#[doc = "Specifics for when this is a contraindication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionContraindication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The situation that is being documented as contraindicating against this item."]
    pub r#disease_symptom_procedure: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The status of the disease or symptom for the contraindication, for example \"chronic\" or \"metastatic\"."]
    pub r#disease_status: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "A comorbidity (concurrent condition) or coinfection."]
    pub r#comorbidity: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "The indication which this is a contraidication for."]
    pub r#indication: Vec<Box<super::super::types::Reference>>,
    #[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the contraindication."]
    pub r#other_therapy: Vec<ClinicalUseDefinitionContraindicationOtherTherapy>,
}
impl serde::ser::Serialize for ClinicalUseDefinitionContraindication {
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
            if let Some(some) = self.r#disease_symptom_procedure.as_ref() {
                state.serialize_entry("diseaseSymptomProcedure", some)?;
            }
            if let Some(some) = self.r#disease_status.as_ref() {
                state.serialize_entry("diseaseStatus", some)?;
            }
            if !self.r#comorbidity.is_empty() {
                state.serialize_entry("comorbidity", &self.r#comorbidity)?;
            }
            if !self.r#indication.is_empty() {
                state.serialize_entry("indication", &self.r#indication)?;
            }
            if !self.r#other_therapy.is_empty() {
                state.serialize_entry("otherTherapy", &self.r#other_therapy)?;
            }
            state.end()
        })
    }
}
#[doc = "Specifics for when this is an indication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionIndication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The situation that is being documented as an indicaton for this item."]
    pub r#disease_symptom_procedure: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The status of the disease or symptom for the indication, for example \"chronic\" or \"metastatic\"."]
    pub r#disease_status: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "A comorbidity (concurrent condition) or coinfection as part of the indication."]
    pub r#comorbidity: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "The intended effect, aim or strategy to be achieved."]
    pub r#intended_effect: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Timing or duration information, that may be associated with use with the indicated condition e.g. Adult patients suffering from myocardial infarction (from a few days until less than 35 days), ischaemic stroke (from 7 days until less than 6 months)."]
    pub r#duration: Option<ClinicalUseDefinitionIndicationDuration>,
    #[doc = "An unwanted side effect or negative outcome that may happen if you use the drug (or other subject of this resource) for this indication."]
    pub r#undesirable_effect: Vec<Box<super::super::types::Reference>>,
    #[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the indication."]
    pub r#other_therapy: Vec<ClinicalUseDefinitionContraindicationOtherTherapy>,
}
impl serde::ser::Serialize for ClinicalUseDefinitionIndication {
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
            if let Some(some) = self.r#disease_symptom_procedure.as_ref() {
                state.serialize_entry("diseaseSymptomProcedure", some)?;
            }
            if let Some(some) = self.r#disease_status.as_ref() {
                state.serialize_entry("diseaseStatus", some)?;
            }
            if !self.r#comorbidity.is_empty() {
                state.serialize_entry("comorbidity", &self.r#comorbidity)?;
            }
            if let Some(some) = self.r#intended_effect.as_ref() {
                state.serialize_entry("intendedEffect", some)?;
            }
            if let Some(some) = self.r#duration.as_ref() {
                match some {
                    ClinicalUseDefinitionIndicationDuration::Range(ref value) => {
                        state.serialize_entry("durationRange", value)?;
                    }
                    ClinicalUseDefinitionIndicationDuration::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("durationString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_durationString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("durationString", value)?;
                        }
                    }
                    ClinicalUseDefinitionIndicationDuration::Invalid => {
                        return Err(serde::ser::Error::custom("duration is invalid"))
                    }
                }
            }
            if !self.r#undesirable_effect.is_empty() {
                state.serialize_entry("undesirableEffect", &self.r#undesirable_effect)?;
            }
            if !self.r#other_therapy.is_empty() {
                state.serialize_entry("otherTherapy", &self.r#other_therapy)?;
            }
            state.end()
        })
    }
}
#[doc = "The specific medication, food, substance or laboratory test that interacts."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionInteractionInteractant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specific medication, food or laboratory test that interacts."]
    pub r#item: ClinicalUseDefinitionInteractionInteractantItem,
}
impl serde::ser::Serialize for ClinicalUseDefinitionInteractionInteractant {
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
            match self.r#item {
                ClinicalUseDefinitionInteractionInteractantItem::Reference(ref value) => {
                    state.serialize_entry("itemReference", value)?;
                }
                ClinicalUseDefinitionInteractionInteractantItem::CodeableConcept(ref value) => {
                    state.serialize_entry("itemCodeableConcept", value)?;
                }
                ClinicalUseDefinitionInteractionInteractantItem::Invalid => {
                    return Err(serde::ser::Error::custom("item is a required field"))
                }
            }
            state.end()
        })
    }
}
#[doc = "Specifics for when this is an interaction."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionInteraction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specific medication, food, substance or laboratory test that interacts."]
    pub r#interactant: Vec<ClinicalUseDefinitionInteractionInteractant>,
    #[doc = "The type of the interaction e.g. drug-drug interaction, drug-food interaction, drug-lab test interaction."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The effect of the interaction, for example \"reduced gastric absorption of primary medication\"."]
    pub r#effect: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The incidence of the interaction, e.g. theoretical, observed."]
    pub r#incidence: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Actions for managing the interaction."]
    pub r#management: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClinicalUseDefinitionInteraction {
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
            if !self.r#interactant.is_empty() {
                state.serialize_entry("interactant", &self.r#interactant)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#effect.as_ref() {
                state.serialize_entry("effect", some)?;
            }
            if let Some(some) = self.r#incidence.as_ref() {
                state.serialize_entry("incidence", some)?;
            }
            if !self.r#management.is_empty() {
                state.serialize_entry("management", &self.r#management)?;
            }
            state.end()
        })
    }
}
#[doc = "Describe the possible undesirable effects (negative outcomes) from the use of the medicinal product as treatment."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionUndesirableEffect {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The situation in which the undesirable effect may manifest."]
    pub r#symptom_condition_effect: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "High level classification of the effect."]
    pub r#classification: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "How often the effect is seen."]
    pub r#frequency_of_occurrence: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClinicalUseDefinitionUndesirableEffect {
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
            if let Some(some) = self.r#symptom_condition_effect.as_ref() {
                state.serialize_entry("symptomConditionEffect", some)?;
            }
            if let Some(some) = self.r#classification.as_ref() {
                state.serialize_entry("classification", some)?;
            }
            if let Some(some) = self.r#frequency_of_occurrence.as_ref() {
                state.serialize_entry("frequencyOfOccurrence", some)?;
            }
            state.end()
        })
    }
}
#[doc = "A critical piece of information about environmental, health or physical risks or hazards that serve as caution to the user. For example 'Do not operate heavy machinery', 'May cause drowsiness', or 'Get medical advice/attention if you feel unwell'."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionWarning {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A textual definition of this warning, with formatting."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A coded or unformatted textual definition of this warning."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClinicalUseDefinitionWarning {
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
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            state.end()
        })
    }
}
#[doc = "A single issue - either an indication, contraindication, interaction or an undesirable effect for a medicinal product, medication, device or procedure."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinition {
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
    #[doc = "Business identifier for this issue."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "indication | contraindication | interaction | undesirable-effect | warning."]
    pub r#type: super::super::types::Code,
    #[doc = "A categorisation of the issue, primarily for dividing warnings into subject heading areas such as \"Pregnancy and Lactation\", \"Overdose\", \"Effects on Ability to Drive and Use Machines\"."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The medication or procedure for which this is an indication."]
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    #[doc = "Whether this is a current issue or one that has been retired etc."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specifics for when this is a contraindication."]
    pub r#contraindication: Option<ClinicalUseDefinitionContraindication>,
    #[doc = "Specifics for when this is an indication."]
    pub r#indication: Option<ClinicalUseDefinitionIndication>,
    #[doc = "Specifics for when this is an interaction."]
    pub r#interaction: Option<ClinicalUseDefinitionInteraction>,
    #[doc = "The population group to which this applies."]
    pub r#population: Vec<Box<super::super::types::Reference>>,
    #[doc = "Describe the possible undesirable effects (negative outcomes) from the use of the medicinal product as treatment."]
    pub r#undesirable_effect: Option<ClinicalUseDefinitionUndesirableEffect>,
    #[doc = "A critical piece of information about environmental, health or physical risks or hazards that serve as caution to the user. For example 'Do not operate heavy machinery', 'May cause drowsiness', or 'Get medical advice/attention if you feel unwell'."]
    pub r#warning: Option<ClinicalUseDefinitionWarning>,
}
impl serde::ser::Serialize for ClinicalUseDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ClinicalUseDefinition")?;
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
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if !self.r#subject.is_empty() {
                state.serialize_entry("subject", &self.r#subject)?;
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if let Some(some) = self.r#contraindication.as_ref() {
                state.serialize_entry("contraindication", some)?;
            }
            if let Some(some) = self.r#indication.as_ref() {
                state.serialize_entry("indication", some)?;
            }
            if let Some(some) = self.r#interaction.as_ref() {
                state.serialize_entry("interaction", some)?;
            }
            if !self.r#population.is_empty() {
                state.serialize_entry("population", &self.r#population)?;
            }
            if let Some(some) = self.r#undesirable_effect.as_ref() {
                state.serialize_entry("undesirableEffect", some)?;
            }
            if let Some(some) = self.r#warning.as_ref() {
                state.serialize_entry("warning", some)?;
            }
            state.end()
        })
    }
}
