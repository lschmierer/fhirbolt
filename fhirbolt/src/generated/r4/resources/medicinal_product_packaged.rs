// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct MedicinalProductPackagedPackageItem {
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#other_characteristics: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#manufactured_item: Vec<Box<super::super::types::Reference>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#alternate_material: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#shelf_life_storage: Vec<Box<super::super::types::ProductShelfLife>>,
    pub r#quantity: Box<super::super::types::Quantity>,
    pub r#id: Option<std::string::String>,
    pub r#device: Vec<Box<super::super::types::Reference>>,
    pub r#material: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#package_item: Vec<MedicinalProductPackagedPackageItem>,
    pub r#physical_characteristics: Option<Box<super::super::types::ProdCharacteristic>>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
}
impl serde::Serialize for MedicinalProductPackagedPackageItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if !self.r#other_characteristics.is_empty() {
            state.serialize_entry("otherCharacteristics", &self.r#other_characteristics)?;
        }
        if !self.r#manufactured_item.is_empty() {
            state.serialize_entry("manufacturedItem", &self.r#manufactured_item)?;
        }
        state.serialize_entry("type", &self.r#type)?;
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#alternate_material.is_empty() {
            state.serialize_entry("alternateMaterial", &self.r#alternate_material)?;
        }
        if !self.r#shelf_life_storage.is_empty() {
            state.serialize_entry("shelfLifeStorage", &self.r#shelf_life_storage)?;
        }
        state.serialize_entry("quantity", &self.r#quantity)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#device.is_empty() {
            state.serialize_entry("device", &self.r#device)?;
        }
        if !self.r#material.is_empty() {
            state.serialize_entry("material", &self.r#material)?;
        }
        if !self.r#package_item.is_empty() {
            state.serialize_entry("packageItem", &self.r#package_item)?;
        }
        if let Some(some) = self.r#physical_characteristics.as_ref() {
            state.serialize_entry("physicalCharacteristics", some)?;
        }
        if !self.r#manufacturer.is_empty() {
            state.serialize_entry("manufacturer", &self.r#manufacturer)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicinalProductPackagedBatchIdentifier {
    pub r#outer_packaging: Box<super::super::types::Identifier>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#immediate_packaging: Option<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
impl serde::Serialize for MedicinalProductPackagedBatchIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("outerPackaging", &self.r#outer_packaging)?;
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#immediate_packaging.as_ref() {
            state.serialize_entry("immediatePackaging", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct MedicinalProductPackaged {
    pub r#package_item: Vec<MedicinalProductPackagedPackageItem>,
    pub r#batch_identifier: Vec<MedicinalProductPackagedBatchIdentifier>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#id: Option<std::string::String>,
    pub r#marketing_authorization: Option<Box<super::super::types::Reference>>,
    pub r#marketing_status: Vec<Box<super::super::types::MarketingStatus>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
}
impl serde::Serialize for MedicinalProductPackaged {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProductPackaged")?;
        if !self.r#package_item.is_empty() {
            state.serialize_entry("packageItem", &self.r#package_item)?;
        }
        if !self.r#batch_identifier.is_empty() {
            state.serialize_entry("batchIdentifier", &self.r#batch_identifier)?;
        }
        if !self.r#contained.is_empty() {
            state.serialize_entry("contained", &self.r#contained)?;
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#marketing_authorization.as_ref() {
            state.serialize_entry("marketingAuthorization", some)?;
        }
        if !self.r#marketing_status.is_empty() {
            state.serialize_entry("marketingStatus", &self.r#marketing_status)?;
        }
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if !self.r#manufacturer.is_empty() {
            state.serialize_entry("manufacturer", &self.r#manufacturer)?;
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if let Some(some) = self.r#legal_status_of_supply.as_ref() {
            state.serialize_entry("legalStatusOfSupply", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
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
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if !self.r#subject.is_empty() {
            state.serialize_entry("subject", &self.r#subject)?;
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
        state.end()
    }
}
