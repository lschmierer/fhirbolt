// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Characteristics e.g. a products onset of action."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductPharmaceuticalCharacteristics {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A coded characteristic."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The status of characteristic e.g. assigned or pending."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for MedicinalProductPharmaceuticalCharacteristics {
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
            state.serialize_entry("code", &self.r#code)?;
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            state.end()
        })
    }
}
#[doc = "A species specific time during which consumption of animal product is not appropriate."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Coded expression for the type of tissue for which the withdrawal period applues, e.g. meat, milk."]
    pub r#tissue: Box<super::super::types::CodeableConcept>,
    #[doc = "A value for the time."]
    pub r#value: Box<super::super::types::Quantity>,
    #[doc = "Extra information about the withdrawal period."]
    pub r#supporting_information: Option<super::super::types::String>,
}
impl serde::ser::Serialize
    for MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod
{
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
            state.serialize_entry("tissue", &self.r#tissue)?;
            state.serialize_entry("value", &self.r#value)?;
            if _ctx.output_json {
                if let Some(some) = self.r#supporting_information.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("supportingInformation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_supportingInformation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#supporting_information.as_ref() {
                    state.serialize_entry("supportingInformation", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "A species for which this route applies."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Coded expression for the species."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "A species specific time during which consumption of animal product is not appropriate."]
    pub r#withdrawal_period:
        Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod>,
}
impl serde::ser::Serialize for MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies {
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
            state.serialize_entry("code", &self.r#code)?;
            if !self.r#withdrawal_period.is_empty() {
                state.serialize_entry("withdrawalPeriod", &self.r#withdrawal_period)?;
            }
            state.end()
        })
    }
}
#[doc = "The path by which the pharmaceutical product is taken into or makes contact with the body."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministration {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Coded expression for the route."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The first dose (dose quantity) administered in humans can be specified, for a product under investigation, using a numerical value and its unit of measurement."]
    pub r#first_dose: Option<Box<super::super::types::Quantity>>,
    #[doc = "The maximum single dose that can be administered as per the protocol of a clinical trial can be specified using a numerical value and its unit of measurement."]
    pub r#max_single_dose: Option<Box<super::super::types::Quantity>>,
    #[doc = "The maximum dose per day (maximum dose quantity to be administered in any one 24-h period) that can be administered as per the protocol referenced in the clinical trial authorisation."]
    pub r#max_dose_per_day: Option<Box<super::super::types::Quantity>>,
    #[doc = "The maximum dose per treatment period that can be administered as per the protocol referenced in the clinical trial authorisation."]
    pub r#max_dose_per_treatment_period: Option<Box<super::super::types::Ratio>>,
    #[doc = "The maximum treatment period during which an Investigational Medicinal Product can be administered as per the protocol referenced in the clinical trial authorisation."]
    pub r#max_treatment_period: Option<Box<super::super::types::Duration>>,
    #[doc = "A species for which this route applies."]
    pub r#target_species: Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies>,
}
impl serde::ser::Serialize for MedicinalProductPharmaceuticalRouteOfAdministration {
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
            state.serialize_entry("code", &self.r#code)?;
            if let Some(some) = self.r#first_dose.as_ref() {
                state.serialize_entry("firstDose", some)?;
            }
            if let Some(some) = self.r#max_single_dose.as_ref() {
                state.serialize_entry("maxSingleDose", some)?;
            }
            if let Some(some) = self.r#max_dose_per_day.as_ref() {
                state.serialize_entry("maxDosePerDay", some)?;
            }
            if let Some(some) = self.r#max_dose_per_treatment_period.as_ref() {
                state.serialize_entry("maxDosePerTreatmentPeriod", some)?;
            }
            if let Some(some) = self.r#max_treatment_period.as_ref() {
                state.serialize_entry("maxTreatmentPeriod", some)?;
            }
            if !self.r#target_species.is_empty() {
                state.serialize_entry("targetSpecies", &self.r#target_species)?;
            }
            state.end()
        })
    }
}
#[doc = "A pharmaceutical product described in terms of its composition and dose form."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MedicinalProductPharmaceutical {
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
    #[doc = "An identifier for the pharmaceutical medicinal product."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The administrable dose form, after necessary reconstitution."]
    pub r#administrable_dose_form: Box<super::super::types::CodeableConcept>,
    #[doc = "Todo."]
    pub r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Ingredient."]
    pub r#ingredient: Vec<Box<super::super::types::Reference>>,
    #[doc = "Accompanying device."]
    pub r#device: Vec<Box<super::super::types::Reference>>,
    #[doc = "Characteristics e.g. a products onset of action."]
    pub r#characteristics: Vec<MedicinalProductPharmaceuticalCharacteristics>,
    #[doc = "The path by which the pharmaceutical product is taken into or makes contact with the body."]
    pub r#route_of_administration: Vec<MedicinalProductPharmaceuticalRouteOfAdministration>,
}
impl serde::ser::Serialize for MedicinalProductPharmaceutical {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "MedicinalProductPharmaceutical")?;
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
            state.serialize_entry("administrableDoseForm", &self.r#administrable_dose_form)?;
            if let Some(some) = self.r#unit_of_presentation.as_ref() {
                state.serialize_entry("unitOfPresentation", some)?;
            }
            if !self.r#ingredient.is_empty() {
                state.serialize_entry("ingredient", &self.r#ingredient)?;
            }
            if !self.r#device.is_empty() {
                state.serialize_entry("device", &self.r#device)?;
            }
            if !self.r#characteristics.is_empty() {
                state.serialize_entry("characteristics", &self.r#characteristics)?;
            }
            if !self.r#route_of_administration.is_empty() {
                state.serialize_entry("routeOfAdministration", &self.r#route_of_administration)?;
            }
            state.end()
        })
    }
}
