// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Reference to a specific medication (active substance, medicinal product or class of products) as part of an indication or contraindication."]
#[derive(Debug, Clone, PartialEq)]
pub enum MedicinalProductIndicationOtherTherapyMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicinalProductIndicationOtherTherapyMedication {
    fn default() -> MedicinalProductIndicationOtherTherapyMedication {
        MedicinalProductIndicationOtherTherapyMedication::Invalid
    }
}
#[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the indication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductIndicationOtherTherapy {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of relationship between the medicinal product indication or contraindication and another therapy."]
    pub r#therapy_relationship_type: Box<super::super::types::CodeableConcept>,
    #[doc = "Reference to a specific medication (active substance, medicinal product or class of products) as part of an indication or contraindication."]
    pub r#medication: MedicinalProductIndicationOtherTherapyMedication,
}
impl serde::ser::Serialize for MedicinalProductIndicationOtherTherapy {
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
            state.serialize_entry("therapyRelationshipType", &self.r#therapy_relationship_type)?;
            match self.r#medication {
                MedicinalProductIndicationOtherTherapyMedication::CodeableConcept(ref value) => {
                    state.serialize_entry("medicationCodeableConcept", value)?;
                }
                MedicinalProductIndicationOtherTherapyMedication::Reference(ref value) => {
                    state.serialize_entry("medicationReference", value)?;
                }
                MedicinalProductIndicationOtherTherapyMedication::Invalid => {
                    return Err(serde::ser::Error::custom("medication is a required field"))
                }
            }
            state.end()
        })
    }
}
#[doc = "Indication for the Medicinal Product."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductIndication {
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
    #[doc = "The medication for which this is an indication."]
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    #[doc = "The disease, symptom or procedure that is the indication for treatment."]
    pub r#disease_symptom_procedure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the disease or symptom for which the indication applies."]
    pub r#disease_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Comorbidity (concurrent condition) or co-infection as part of the indication."]
    pub r#comorbidity: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The intended effect, aim or strategy to be achieved by the indication."]
    pub r#intended_effect: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Timing or duration information as part of the indication."]
    pub r#duration: Option<Box<super::super::types::Quantity>>,
    #[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the indication."]
    pub r#other_therapy: Vec<MedicinalProductIndicationOtherTherapy>,
    #[doc = "Describe the undesirable effects of the medicinal product."]
    pub r#undesirable_effect: Vec<Box<super::super::types::Reference>>,
    #[doc = "The population group to which this applies."]
    pub r#population: Vec<Box<super::super::types::Population>>,
}
impl serde::ser::Serialize for MedicinalProductIndication {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "MedicinalProductIndication")?;
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
            if !self.r#subject.is_empty() {
                state.serialize_entry("subject", &self.r#subject)?;
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
                state.serialize_entry("duration", some)?;
            }
            if !self.r#other_therapy.is_empty() {
                state.serialize_entry("otherTherapy", &self.r#other_therapy)?;
            }
            if !self.r#undesirable_effect.is_empty() {
                state.serialize_entry("undesirableEffect", &self.r#undesirable_effect)?;
            }
            if !self.r#population.is_empty() {
                state.serialize_entry("population", &self.r#population)?;
            }
            state.end()
        })
    }
}
