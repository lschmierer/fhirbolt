// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicinalProductSpecialDesignationIndication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct MedicinalProductSpecialDesignation {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#intended_use: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#species: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#id: Option<std::string::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#indication: Option<MedicinalProductSpecialDesignationIndication>,
}
impl serde::Serialize for MedicinalProductSpecialDesignation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#intended_use.as_ref() {
            state.serialize_entry("intendedUse", some)?;
        }
        if let Some(some) = self.r#status.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if let Some(some) = self.r#species.as_ref() {
            state.serialize_entry("species", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("date", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_date", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if let Some(some) = self.r#indication.as_ref() {
            match some {
                MedicinalProductSpecialDesignationIndication::CodeableConcept(ref value) => {
                    state.serialize_entry("indicationCodeableConcept", value)?;
                }
                MedicinalProductSpecialDesignationIndication::Reference(ref value) => {
                    state.serialize_entry("indicationReference", value)?;
                }
            }
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicinalProductNameNamePart {
    pub r#type: Box<super::super::types::Coding>,
    pub r#part: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
impl serde::Serialize for MedicinalProductNameNamePart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("type", &self.r#type)?;
        {
            if let Some(some) = self.r#part.value.as_ref() {
                state.serialize_entry("part", some)?;
            }
            if self.r#part.id.is_some() || !self.r#part.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#part.id,
                    extension: &self.r#part.extension,
                };
                state.serialize_entry("_part", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicinalProductNameCountryLanguage {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#jurisdiction: Option<Box<super::super::types::CodeableConcept>>,
    pub r#language: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#country: Box<super::super::types::CodeableConcept>,
}
impl serde::Serialize for MedicinalProductNameCountryLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#jurisdiction.as_ref() {
            state.serialize_entry("jurisdiction", some)?;
        }
        state.serialize_entry("language", &self.r#language)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.serialize_entry("country", &self.r#country)?;
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicinalProductName {
    pub r#name_part: Vec<MedicinalProductNameNamePart>,
    pub r#product_name: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#country_language: Vec<MedicinalProductNameCountryLanguage>,
}
impl serde::Serialize for MedicinalProductName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#name_part.is_empty() {
            state.serialize_entry("namePart", &self.r#name_part)?;
        }
        {
            if let Some(some) = self.r#product_name.value.as_ref() {
                state.serialize_entry("productName", some)?;
            }
            if self.r#product_name.id.is_some() || !self.r#product_name.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#product_name.id,
                    extension: &self.r#product_name.extension,
                };
                state.serialize_entry("_productName", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#country_language.is_empty() {
            state.serialize_entry("countryLanguage", &self.r#country_language)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicinalProductManufacturingBusinessOperation {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#confidentiality_indicator: Option<Box<super::super::types::CodeableConcept>>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    pub r#operation_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#effective_date: Option<super::super::types::DateTime>,
    pub r#id: Option<std::string::String>,
    pub r#authorisation_reference_number: Option<Box<super::super::types::Identifier>>,
    pub r#regulator: Option<Box<super::super::types::Reference>>,
}
impl serde::Serialize for MedicinalProductManufacturingBusinessOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#confidentiality_indicator.as_ref() {
            state.serialize_entry("confidentialityIndicator", some)?;
        }
        if !self.r#manufacturer.is_empty() {
            state.serialize_entry("manufacturer", &self.r#manufacturer)?;
        }
        if let Some(some) = self.r#operation_type.as_ref() {
            state.serialize_entry("operationType", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#effective_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("effectiveDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_effectiveDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#authorisation_reference_number.as_ref() {
            state.serialize_entry("authorisationReferenceNumber", some)?;
        }
        if let Some(some) = self.r#regulator.as_ref() {
            state.serialize_entry("regulator", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicinalProduct {
    pub r#special_measures: Vec<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#paediatric_use_indicator: Option<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#product_classification: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#attached_document: Vec<Box<super::super::types::Reference>>,
    pub r#special_designation: Vec<MedicinalProductSpecialDesignation>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#clinical_trial: Vec<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#marketing_status: Vec<Box<super::super::types::MarketingStatus>>,
    pub r#packaged_medicinal_product: Vec<Box<super::super::types::Reference>>,
    pub r#master_file: Vec<Box<super::super::types::Reference>>,
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
    pub r#name: Vec<MedicinalProductName>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#manufacturing_business_operation: Vec<MedicinalProductManufacturingBusinessOperation>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#combined_pharmaceutical_dose_form: Option<Box<super::super::types::CodeableConcept>>,
    pub r#additional_monitoring_indicator: Option<Box<super::super::types::CodeableConcept>>,
    pub r#pharmaceutical_product: Vec<Box<super::super::types::Reference>>,
    pub r#contact: Vec<Box<super::super::types::Reference>>,
    pub r#cross_reference: Vec<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#domain: Option<Box<super::super::types::Coding>>,
}
impl serde::Serialize for MedicinalProduct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProduct")?;
        if !self.r#special_measures.is_empty() {
            let values: Vec<_> = self.r#special_measures.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("specialMeasures", &values)?;
            }
            let requires_elements = self
                .r#special_measures
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_elements: Vec<_> = self
                    .r#special_measures
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(PrimtiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_specialMeasures", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#language.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("language", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#paediatric_use_indicator.as_ref() {
            state.serialize_entry("paediatricUseIndicator", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if !self.r#product_classification.is_empty() {
            state.serialize_entry("productClassification", &self.r#product_classification)?;
        }
        if !self.r#attached_document.is_empty() {
            state.serialize_entry("attachedDocument", &self.r#attached_document)?;
        }
        if !self.r#special_designation.is_empty() {
            state.serialize_entry("specialDesignation", &self.r#special_designation)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#clinical_trial.is_empty() {
            state.serialize_entry("clinicalTrial", &self.r#clinical_trial)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_implicitRules", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if !self.r#marketing_status.is_empty() {
            state.serialize_entry("marketingStatus", &self.r#marketing_status)?;
        }
        if !self.r#packaged_medicinal_product.is_empty() {
            state.serialize_entry(
                "packagedMedicinalProduct",
                &self.r#packaged_medicinal_product,
            )?;
        }
        if !self.r#master_file.is_empty() {
            state.serialize_entry("masterFile", &self.r#master_file)?;
        }
        if let Some(some) = self.r#legal_status_of_supply.as_ref() {
            state.serialize_entry("legalStatusOfSupply", some)?;
        }
        if !self.r#name.is_empty() {
            state.serialize_entry("name", &self.r#name)?;
        }
        if !self.r#contained.is_empty() {
            state.serialize_entry("contained", &self.r#contained)?;
        }
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if !self.r#manufacturing_business_operation.is_empty() {
            state.serialize_entry(
                "manufacturingBusinessOperation",
                &self.r#manufacturing_business_operation,
            )?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#combined_pharmaceutical_dose_form.as_ref() {
            state.serialize_entry("combinedPharmaceuticalDoseForm", some)?;
        }
        if let Some(some) = self.r#additional_monitoring_indicator.as_ref() {
            state.serialize_entry("additionalMonitoringIndicator", some)?;
        }
        if !self.r#pharmaceutical_product.is_empty() {
            state.serialize_entry("pharmaceuticalProduct", &self.r#pharmaceutical_product)?;
        }
        if !self.r#contact.is_empty() {
            state.serialize_entry("contact", &self.r#contact)?;
        }
        if !self.r#cross_reference.is_empty() {
            state.serialize_entry("crossReference", &self.r#cross_reference)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#domain.as_ref() {
            state.serialize_entry("domain", some)?;
        }
        state.end()
    }
}
