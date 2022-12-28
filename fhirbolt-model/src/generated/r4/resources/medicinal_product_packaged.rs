// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "Batch numbering."]
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductPackagedBatchIdentifier {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number appearing on the outer packaging of a specific batch."]
    pub r#outer_packaging: Box<super::super::types::Identifier>,
    #[doc = "A number appearing on the immediate packaging (and not the outer packaging)."]
    pub r#immediate_packaging: Option<Box<super::super::types::Identifier>>,
}
impl serde::ser::Serialize for MedicinalProductPackagedBatchIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            state.serialize_entry("outerPackaging", &self.r#outer_packaging)?;
            if let Some(some) = self.r#immediate_packaging.as_ref() {
                state.serialize_entry("immediatePackaging", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductPackagedBatchIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "outerPackaging")]
            OuterPackaging,
            #[serde(rename = "immediatePackaging")]
            ImmediatePackaging,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductPackagedBatchIdentifier;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductPackagedBatchIdentifier")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductPackagedBatchIdentifier, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#outer_packaging: Option<Box<super::super::types::Identifier>> = None;
                let mut r#immediate_packaging: Option<Box<super::super::types::Identifier>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::OuterPackaging => {
                                if r#outer_packaging.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "outerPackaging",
                                    ));
                                }
                                r#outer_packaging = Some(map_access.next_value()?);
                            }
                            Field::ImmediatePackaging => {
                                if r#immediate_packaging.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "immediatePackaging",
                                    ));
                                }
                                r#immediate_packaging = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "outerPackaging",
                                        "immediatePackaging",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MedicinalProductPackagedBatchIdentifier {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#outer_packaging: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#outer_packaging.unwrap_or(Default::default())
                        } else {
                            r#outer_packaging
                                .ok_or(serde::de::Error::missing_field("outerPackaging"))?
                        },
                        r#immediate_packaging,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A packaging item, as a contained for medicine, possibly with other packaging items within."]
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductPackagedPackageItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Including possibly Data Carrier Identifier."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The physical type of the container of the medicine."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The quantity of this package in the medicinal product, at the current level of packaging. The outermost is always 1."]
    pub r#quantity: Box<super::super::types::Quantity>,
    #[doc = "Material type of the package item."]
    pub r#material: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A possible alternate material for the packaging."]
    pub r#alternate_material: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A device accompanying a medicinal product."]
    pub r#device: Vec<Box<super::super::types::Reference>>,
    #[doc = "The manufactured item as contained in the packaged medicinal product."]
    pub r#manufactured_item: Vec<Box<super::super::types::Reference>>,
    #[doc = "Allows containers within containers."]
    pub r#package_item: Vec<MedicinalProductPackagedPackageItem>,
    #[doc = "Dimensions, color etc."]
    pub r#physical_characteristics: Option<Box<super::super::types::ProdCharacteristic>>,
    #[doc = "Other codeable characteristics."]
    pub r#other_characteristics: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Shelf Life and storage information."]
    pub r#shelf_life_storage: Vec<Box<super::super::types::ProductShelfLife>>,
    #[doc = "Manufacturer of this Package Item."]
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MedicinalProductPackagedPackageItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            state.serialize_entry("type", &self.r#type)?;
            state.serialize_entry("quantity", &self.r#quantity)?;
            if !self.r#material.is_empty() {
                state.serialize_entry("material", &self.r#material)?;
            }
            if !self.r#alternate_material.is_empty() {
                state.serialize_entry("alternateMaterial", &self.r#alternate_material)?;
            }
            if !self.r#device.is_empty() {
                state.serialize_entry("device", &self.r#device)?;
            }
            if !self.r#manufactured_item.is_empty() {
                state.serialize_entry("manufacturedItem", &self.r#manufactured_item)?;
            }
            if !self.r#package_item.is_empty() {
                state.serialize_entry("packageItem", &self.r#package_item)?;
            }
            if let Some(some) = self.r#physical_characteristics.as_ref() {
                state.serialize_entry("physicalCharacteristics", some)?;
            }
            if !self.r#other_characteristics.is_empty() {
                state.serialize_entry("otherCharacteristics", &self.r#other_characteristics)?;
            }
            if !self.r#shelf_life_storage.is_empty() {
                state.serialize_entry("shelfLifeStorage", &self.r#shelf_life_storage)?;
            }
            if !self.r#manufacturer.is_empty() {
                state.serialize_entry("manufacturer", &self.r#manufacturer)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductPackagedPackageItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "material")]
            Material,
            #[serde(rename = "alternateMaterial")]
            AlternateMaterial,
            #[serde(rename = "device")]
            Device,
            #[serde(rename = "manufacturedItem")]
            ManufacturedItem,
            #[serde(rename = "packageItem")]
            PackageItem,
            #[serde(rename = "physicalCharacteristics")]
            PhysicalCharacteristics,
            #[serde(rename = "otherCharacteristics")]
            OtherCharacteristics,
            #[serde(rename = "shelfLifeStorage")]
            ShelfLifeStorage,
            #[serde(rename = "manufacturer")]
            Manufacturer,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductPackagedPackageItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductPackagedPackageItem")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductPackagedPackageItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#material: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#alternate_material: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#device: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#manufactured_item: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#package_item: Option<Vec<MedicinalProductPackagedPackageItem>> = None;
                let mut r#physical_characteristics: Option<
                    Box<super::super::types::ProdCharacteristic>,
                > = None;
                let mut r#other_characteristics: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#shelf_life_storage: Option<
                    Vec<Box<super::super::types::ProductShelfLife>>,
                > = None;
                let mut r#manufacturer: Option<Vec<Box<super::super::types::Reference>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::Material => {
                                if r#material.is_some() {
                                    return Err(serde::de::Error::duplicate_field("material"));
                                }
                                r#material = Some(map_access.next_value()?);
                            }
                            Field::AlternateMaterial => {
                                if r#alternate_material.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "alternateMaterial",
                                    ));
                                }
                                r#alternate_material = Some(map_access.next_value()?);
                            }
                            Field::Device => {
                                if r#device.is_some() {
                                    return Err(serde::de::Error::duplicate_field("device"));
                                }
                                r#device = Some(map_access.next_value()?);
                            }
                            Field::ManufacturedItem => {
                                if r#manufactured_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "manufacturedItem",
                                    ));
                                }
                                r#manufactured_item = Some(map_access.next_value()?);
                            }
                            Field::PackageItem => {
                                if r#package_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("packageItem"));
                                }
                                r#package_item = Some(map_access.next_value()?);
                            }
                            Field::PhysicalCharacteristics => {
                                if r#physical_characteristics.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "physicalCharacteristics",
                                    ));
                                }
                                r#physical_characteristics = Some(map_access.next_value()?);
                            }
                            Field::OtherCharacteristics => {
                                if r#other_characteristics.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "otherCharacteristics",
                                    ));
                                }
                                r#other_characteristics = Some(map_access.next_value()?);
                            }
                            Field::ShelfLifeStorage => {
                                if r#shelf_life_storage.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "shelfLifeStorage",
                                    ));
                                }
                                r#shelf_life_storage = Some(map_access.next_value()?);
                            }
                            Field::Manufacturer => {
                                if r#manufacturer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manufacturer"));
                                }
                                r#manufacturer = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "identifier",
                                        "type",
                                        "quantity",
                                        "material",
                                        "alternateMaterial",
                                        "device",
                                        "manufacturedItem",
                                        "packageItem",
                                        "physicalCharacteristics",
                                        "otherCharacteristics",
                                        "shelfLifeStorage",
                                        "manufacturer",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MedicinalProductPackagedPackageItem {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#quantity: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#quantity.unwrap_or(Default::default())
                        } else {
                            r#quantity.ok_or(serde::de::Error::missing_field("quantity"))?
                        },
                        r#material: r#material.unwrap_or(vec![]),
                        r#alternate_material: r#alternate_material.unwrap_or(vec![]),
                        r#device: r#device.unwrap_or(vec![]),
                        r#manufactured_item: r#manufactured_item.unwrap_or(vec![]),
                        r#package_item: r#package_item.unwrap_or(vec![]),
                        r#physical_characteristics,
                        r#other_characteristics: r#other_characteristics.unwrap_or(vec![]),
                        r#shelf_life_storage: r#shelf_life_storage.unwrap_or(vec![]),
                        r#manufacturer: r#manufacturer.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A medicinal product in a container or package."]
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductPackaged {
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
    #[doc = "Unique identifier."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The product with this is a pack for."]
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    #[doc = "Textual description."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The legal status of supply of the medicinal product as classified by the regulator."]
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Marketing information."]
    pub r#marketing_status: Vec<Box<super::super::types::MarketingStatus>>,
    #[doc = "Manufacturer of this Package Item."]
    pub r#marketing_authorization: Option<Box<super::super::types::Reference>>,
    #[doc = "Manufacturer of this Package Item."]
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    #[doc = "Batch numbering."]
    pub r#batch_identifier: Vec<MedicinalProductPackagedBatchIdentifier>,
    #[doc = "A packaging item, as a contained for medicine, possibly with other packaging items within."]
    pub r#package_item: Vec<MedicinalProductPackagedPackageItem>,
}
impl crate::AnyResource for MedicinalProductPackaged {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for MedicinalProductPackaged {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "MedicinalProductPackaged")?;
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
            if !self.r#subject.is_empty() {
                state.serialize_entry("subject", &self.r#subject)?;
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
            if let Some(some) = self.r#legal_status_of_supply.as_ref() {
                state.serialize_entry("legalStatusOfSupply", some)?;
            }
            if !self.r#marketing_status.is_empty() {
                state.serialize_entry("marketingStatus", &self.r#marketing_status)?;
            }
            if let Some(some) = self.r#marketing_authorization.as_ref() {
                state.serialize_entry("marketingAuthorization", some)?;
            }
            if !self.r#manufacturer.is_empty() {
                state.serialize_entry("manufacturer", &self.r#manufacturer)?;
            }
            if !self.r#batch_identifier.is_empty() {
                state.serialize_entry("batchIdentifier", &self.r#batch_identifier)?;
            }
            if !self.r#package_item.is_empty() {
                state.serialize_entry("packageItem", &self.r#package_item)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductPackaged {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "resourceType")]
            ResourceType,
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "meta")]
            Meta,
            #[serde(rename = "implicitRules")]
            ImplicitRules,
            #[serde(rename = "_implicitRules")]
            ImplicitRulesPrimitiveElement,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "_language")]
            LanguagePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "contained")]
            Contained,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "legalStatusOfSupply")]
            LegalStatusOfSupply,
            #[serde(rename = "marketingStatus")]
            MarketingStatus,
            #[serde(rename = "marketingAuthorization")]
            MarketingAuthorization,
            #[serde(rename = "manufacturer")]
            Manufacturer,
            #[serde(rename = "batchIdentifier")]
            BatchIdentifier,
            #[serde(rename = "packageItem")]
            PackageItem,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductPackaged;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductPackaged")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicinalProductPackaged, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#subject: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#legal_status_of_supply: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#marketing_status: Option<Vec<Box<super::super::types::MarketingStatus>>> =
                    None;
                let mut r#marketing_authorization: Option<Box<super::super::types::Reference>> =
                    None;
                let mut r#manufacturer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#batch_identifier: Option<Vec<MedicinalProductPackagedBatchIdentifier>> =
                    None;
                let mut r#package_item: Option<Vec<MedicinalProductPackagedPackageItem>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "MedicinalProductPackaged" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"MedicinalProductPackaged",
                                    ));
                                }
                            }
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Meta => {
                                if r#meta.is_some() {
                                    return Err(serde::de::Error::duplicate_field("meta"));
                                }
                                r#meta = Some(map_access.next_value()?);
                            }
                            Field::ImplicitRules => {
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#implicit_rules.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    r#implicit_rules = Some(map_access.next_value()?);
                                }
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_implicitRules",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "implicitRules",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "subject",
                                            "description",
                                            "legalStatusOfSupply",
                                            "marketingStatus",
                                            "marketingAuthorization",
                                            "manufacturer",
                                            "batchIdentifier",
                                            "packageItem",
                                        ],
                                    ));
                                }
                            }
                            Field::Language => {
                                if _ctx.from_json {
                                    let some = r#language.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#language.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    r#language = Some(map_access.next_value()?);
                                }
                            }
                            Field::LanguagePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#language.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_language"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "language",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "subject",
                                            "description",
                                            "legalStatusOfSupply",
                                            "marketingStatus",
                                            "marketingAuthorization",
                                            "manufacturer",
                                            "batchIdentifier",
                                            "packageItem",
                                        ],
                                    ));
                                }
                            }
                            Field::Text => {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value()?);
                            }
                            Field::Contained => {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "description",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "subject",
                                            "description",
                                            "legalStatusOfSupply",
                                            "marketingStatus",
                                            "marketingAuthorization",
                                            "manufacturer",
                                            "batchIdentifier",
                                            "packageItem",
                                        ],
                                    ));
                                }
                            }
                            Field::LegalStatusOfSupply => {
                                if r#legal_status_of_supply.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "legalStatusOfSupply",
                                    ));
                                }
                                r#legal_status_of_supply = Some(map_access.next_value()?);
                            }
                            Field::MarketingStatus => {
                                if r#marketing_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "marketingStatus",
                                    ));
                                }
                                r#marketing_status = Some(map_access.next_value()?);
                            }
                            Field::MarketingAuthorization => {
                                if r#marketing_authorization.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "marketingAuthorization",
                                    ));
                                }
                                r#marketing_authorization = Some(map_access.next_value()?);
                            }
                            Field::Manufacturer => {
                                if r#manufacturer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manufacturer"));
                                }
                                r#manufacturer = Some(map_access.next_value()?);
                            }
                            Field::BatchIdentifier => {
                                if r#batch_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "batchIdentifier",
                                    ));
                                }
                                r#batch_identifier = Some(map_access.next_value()?);
                            }
                            Field::PackageItem => {
                                if r#package_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("packageItem"));
                                }
                                r#package_item = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "meta",
                                        "implicitRules",
                                        "language",
                                        "text",
                                        "contained",
                                        "extension",
                                        "modifierExtension",
                                        "identifier",
                                        "subject",
                                        "description",
                                        "legalStatusOfSupply",
                                        "marketingStatus",
                                        "marketingAuthorization",
                                        "manufacturer",
                                        "batchIdentifier",
                                        "packageItem",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MedicinalProductPackaged {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#subject: r#subject.unwrap_or(vec![]),
                        r#description,
                        r#legal_status_of_supply,
                        r#marketing_status: r#marketing_status.unwrap_or(vec![]),
                        r#marketing_authorization,
                        r#manufacturer: r#manufacturer.unwrap_or(vec![]),
                        r#batch_identifier: r#batch_identifier.unwrap_or(vec![]),
                        r#package_item: r#package_item.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
