// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductPackagedBatchIdentifier {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#outer_packaging: Box<super::super::types::Identifier>,
    pub r#immediate_packaging: Option<Box<super::super::types::Identifier>>,
}
impl serde::ser::Serialize for MedicinalProductPackagedBatchIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductPackagedBatchIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "outerPackaging" => {
                            if r#outer_packaging.is_some() {
                                return Err(serde::de::Error::duplicate_field("outerPackaging"));
                            }
                            r#outer_packaging = Some(map_access.next_value()?);
                        }
                        "immediatePackaging" => {
                            if r#immediate_packaging.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "immediatePackaging",
                                ));
                            }
                            r#immediate_packaging = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifierExtension",
                                    "outerPackaging",
                                    "immediatePackaging",
                                ],
                            ))
                        }
                    }
                }
                Ok(MedicinalProductPackagedBatchIdentifier {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#outer_packaging: r#outer_packaging
                        .ok_or(serde::de::Error::missing_field("outer_packaging"))?,
                    r#immediate_packaging,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductPackagedPackageItem {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#quantity: Box<super::super::types::Quantity>,
    pub r#material: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#alternate_material: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#device: Vec<Box<super::super::types::Reference>>,
    pub r#manufactured_item: Vec<Box<super::super::types::Reference>>,
    pub r#package_item: Vec<MedicinalProductPackagedPackageItem>,
    pub r#physical_characteristics: Option<Box<super::super::types::ProdCharacteristic>>,
    pub r#other_characteristics: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#shelf_life_storage: Vec<Box<super::super::types::ProductShelfLife>>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MedicinalProductPackagedPackageItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductPackagedPackageItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "material" => {
                            if r#material.is_some() {
                                return Err(serde::de::Error::duplicate_field("material"));
                            }
                            r#material = Some(map_access.next_value()?);
                        }
                        "alternateMaterial" => {
                            if r#alternate_material.is_some() {
                                return Err(serde::de::Error::duplicate_field("alternateMaterial"));
                            }
                            r#alternate_material = Some(map_access.next_value()?);
                        }
                        "device" => {
                            if r#device.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            r#device = Some(map_access.next_value()?);
                        }
                        "manufacturedItem" => {
                            if r#manufactured_item.is_some() {
                                return Err(serde::de::Error::duplicate_field("manufacturedItem"));
                            }
                            r#manufactured_item = Some(map_access.next_value()?);
                        }
                        "packageItem" => {
                            if r#package_item.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageItem"));
                            }
                            r#package_item = Some(map_access.next_value()?);
                        }
                        "physicalCharacteristics" => {
                            if r#physical_characteristics.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "physicalCharacteristics",
                                ));
                            }
                            r#physical_characteristics = Some(map_access.next_value()?);
                        }
                        "otherCharacteristics" => {
                            if r#other_characteristics.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "otherCharacteristics",
                                ));
                            }
                            r#other_characteristics = Some(map_access.next_value()?);
                        }
                        "shelfLifeStorage" => {
                            if r#shelf_life_storage.is_some() {
                                return Err(serde::de::Error::duplicate_field("shelfLifeStorage"));
                            }
                            r#shelf_life_storage = Some(map_access.next_value()?);
                        }
                        "manufacturer" => {
                            if r#manufacturer.is_some() {
                                return Err(serde::de::Error::duplicate_field("manufacturer"));
                            }
                            r#manufacturer = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
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
                            ))
                        }
                    }
                }
                Ok(MedicinalProductPackagedPackageItem {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#quantity: r#quantity.ok_or(serde::de::Error::missing_field("quantity"))?,
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductPackaged {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    pub r#description: Option<super::super::types::String>,
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
    pub r#marketing_status: Vec<Box<super::super::types::MarketingStatus>>,
    pub r#marketing_authorization: Option<Box<super::super::types::Reference>>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    pub r#batch_identifier: Vec<MedicinalProductPackagedBatchIdentifier>,
    pub r#package_item: Vec<MedicinalProductPackagedPackageItem>,
}
impl serde::ser::Serialize for MedicinalProductPackaged {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProductPackaged")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
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
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
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
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
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
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductPackaged {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
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
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "meta" => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        "implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
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
                        }
                        "text" => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        "contained" => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "subject" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        "description" => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_description" => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "legalStatusOfSupply" => {
                            if r#legal_status_of_supply.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "legalStatusOfSupply",
                                ));
                            }
                            r#legal_status_of_supply = Some(map_access.next_value()?);
                        }
                        "marketingStatus" => {
                            if r#marketing_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketingStatus"));
                            }
                            r#marketing_status = Some(map_access.next_value()?);
                        }
                        "marketingAuthorization" => {
                            if r#marketing_authorization.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "marketingAuthorization",
                                ));
                            }
                            r#marketing_authorization = Some(map_access.next_value()?);
                        }
                        "manufacturer" => {
                            if r#manufacturer.is_some() {
                                return Err(serde::de::Error::duplicate_field("manufacturer"));
                            }
                            r#manufacturer = Some(map_access.next_value()?);
                        }
                        "batchIdentifier" => {
                            if r#batch_identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchIdentifier"));
                            }
                            r#batch_identifier = Some(map_access.next_value()?);
                        }
                        "packageItem" => {
                            if r#package_item.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageItem"));
                            }
                            r#package_item = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
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
                            ))
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
