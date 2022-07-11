// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct MedicinalProductPackagedBatchIdentifier {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#outer_packaging: Box<super::super::types::Identifier>,
    pub r#immediate_packaging: Option<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductPackagedPackageItem {
    pub r#physical_characteristics: Option<Box<super::super::types::ProdCharacteristic>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#device: Vec<Box<super::super::types::Reference>>,
    pub r#shelf_life_storage: Vec<Box<super::super::types::ProductShelfLife>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    pub r#material: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#package_item: Vec<MedicinalProductPackagedPackageItem>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#quantity: Box<super::super::types::Quantity>,
    pub r#alternate_material: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#other_characteristics: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#manufactured_item: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductPackaged {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#marketing_authorization: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#marketing_status: Vec<Box<super::super::types::MarketingStatus>>,
    pub r#batch_identifier: Vec<MedicinalProductPackagedBatchIdentifier>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    pub r#package_item: Vec<MedicinalProductPackagedPackageItem>,
    pub r#language: Option<super::super::types::Code>,
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
}
