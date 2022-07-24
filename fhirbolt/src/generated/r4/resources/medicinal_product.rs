// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicinalProductSpecialDesignationIndication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicinalProductSpecialDesignationIndication {
    fn default() -> MedicinalProductSpecialDesignationIndication {
        MedicinalProductSpecialDesignationIndication::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductNameNamePart {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#part: super::super::types::String,
    pub r#type: Box<super::super::types::Coding>,
}
impl serde::ser::Serialize for MedicinalProductNameNamePart {
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
        if let Some(some) = self.r#part.value.as_ref() {
            state.serialize_entry("part", some)?;
        }
        if self.r#part.id.is_some() || !self.r#part.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#part.id,
                extension: &self.r#part.extension,
            };
            state.serialize_entry("_part", &primitive_element)?;
        }
        state.serialize_entry("type", &self.r#type)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductNameNamePart {
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
            #[serde(rename = "part")]
            Part,
            #[serde(rename = "_part")]
            PartPrimitiveElement,
            #[serde(rename = "type")]
            Type,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductNameNamePart;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductNameNamePart")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductNameNamePart, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#part: Option<super::super::types::String> = None;
                let mut r#type: Option<Box<super::super::types::Coding>> = None;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Part => {
                            let some = r#part.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("part"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PartPrimitiveElement => {
                            let some = r#part.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_part"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductNameNamePart {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#part: r#part.ok_or(serde::de::Error::missing_field("part"))?,
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductNameCountryLanguage {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#country: Box<super::super::types::CodeableConcept>,
    pub r#jurisdiction: Option<Box<super::super::types::CodeableConcept>>,
    pub r#language: Box<super::super::types::CodeableConcept>,
}
impl serde::ser::Serialize for MedicinalProductNameCountryLanguage {
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
        state.serialize_entry("country", &self.r#country)?;
        if let Some(some) = self.r#jurisdiction.as_ref() {
            state.serialize_entry("jurisdiction", some)?;
        }
        state.serialize_entry("language", &self.r#language)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductNameCountryLanguage {
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
            #[serde(rename = "country")]
            Country,
            #[serde(rename = "jurisdiction")]
            Jurisdiction,
            #[serde(rename = "language")]
            Language,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductNameCountryLanguage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductNameCountryLanguage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductNameCountryLanguage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#country: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#jurisdiction: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#language: Option<Box<super::super::types::CodeableConcept>> = None;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Country => {
                            if r#country.is_some() {
                                return Err(serde::de::Error::duplicate_field("country"));
                            }
                            r#country = Some(map_access.next_value()?);
                        }
                        Field::Jurisdiction => {
                            if r#jurisdiction.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            r#jurisdiction = Some(map_access.next_value()?);
                        }
                        Field::Language => {
                            if r#language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            r#language = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductNameCountryLanguage {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#country: r#country.ok_or(serde::de::Error::missing_field("country"))?,
                    r#jurisdiction,
                    r#language: r#language.ok_or(serde::de::Error::missing_field("language"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductName {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#product_name: super::super::types::String,
    pub r#name_part: Vec<MedicinalProductNameNamePart>,
    pub r#country_language: Vec<MedicinalProductNameCountryLanguage>,
}
impl serde::ser::Serialize for MedicinalProductName {
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
        if let Some(some) = self.r#product_name.value.as_ref() {
            state.serialize_entry("productName", some)?;
        }
        if self.r#product_name.id.is_some() || !self.r#product_name.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#product_name.id,
                extension: &self.r#product_name.extension,
            };
            state.serialize_entry("_productName", &primitive_element)?;
        }
        if !self.r#name_part.is_empty() {
            state.serialize_entry("namePart", &self.r#name_part)?;
        }
        if !self.r#country_language.is_empty() {
            state.serialize_entry("countryLanguage", &self.r#country_language)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductName {
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
            #[serde(rename = "productName")]
            ProductName,
            #[serde(rename = "_productName")]
            ProductNamePrimitiveElement,
            #[serde(rename = "namePart")]
            NamePart,
            #[serde(rename = "countryLanguage")]
            CountryLanguage,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductName")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicinalProductName, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#product_name: Option<super::super::types::String> = None;
                let mut r#name_part: Option<Vec<MedicinalProductNameNamePart>> = None;
                let mut r#country_language: Option<Vec<MedicinalProductNameCountryLanguage>> = None;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::ProductName => {
                            let some = r#product_name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("productName"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ProductNamePrimitiveElement => {
                            let some = r#product_name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_productName"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::NamePart => {
                            if r#name_part.is_some() {
                                return Err(serde::de::Error::duplicate_field("namePart"));
                            }
                            r#name_part = Some(map_access.next_value()?);
                        }
                        Field::CountryLanguage => {
                            if r#country_language.is_some() {
                                return Err(serde::de::Error::duplicate_field("countryLanguage"));
                            }
                            r#country_language = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductName {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#product_name: r#product_name
                        .ok_or(serde::de::Error::missing_field("productName"))?,
                    r#name_part: r#name_part.unwrap_or(vec![]),
                    r#country_language: r#country_language.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductManufacturingBusinessOperation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#operation_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#authorisation_reference_number: Option<Box<super::super::types::Identifier>>,
    pub r#effective_date: Option<super::super::types::DateTime>,
    pub r#confidentiality_indicator: Option<Box<super::super::types::CodeableConcept>>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    pub r#regulator: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MedicinalProductManufacturingBusinessOperation {
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
        if let Some(some) = self.r#operation_type.as_ref() {
            state.serialize_entry("operationType", some)?;
        }
        if let Some(some) = self.r#authorisation_reference_number.as_ref() {
            state.serialize_entry("authorisationReferenceNumber", some)?;
        }
        if let Some(some) = self.r#effective_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("effectiveDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_effectiveDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#confidentiality_indicator.as_ref() {
            state.serialize_entry("confidentialityIndicator", some)?;
        }
        if !self.r#manufacturer.is_empty() {
            state.serialize_entry("manufacturer", &self.r#manufacturer)?;
        }
        if let Some(some) = self.r#regulator.as_ref() {
            state.serialize_entry("regulator", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductManufacturingBusinessOperation {
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
            #[serde(rename = "operationType")]
            OperationType,
            #[serde(rename = "authorisationReferenceNumber")]
            AuthorisationReferenceNumber,
            #[serde(rename = "effectiveDate")]
            EffectiveDate,
            #[serde(rename = "_effectiveDate")]
            EffectiveDatePrimitiveElement,
            #[serde(rename = "confidentialityIndicator")]
            ConfidentialityIndicator,
            #[serde(rename = "manufacturer")]
            Manufacturer,
            #[serde(rename = "regulator")]
            Regulator,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductManufacturingBusinessOperation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductManufacturingBusinessOperation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductManufacturingBusinessOperation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#operation_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#authorisation_reference_number: Option<
                    Box<super::super::types::Identifier>,
                > = None;
                let mut r#effective_date: Option<super::super::types::DateTime> = None;
                let mut r#confidentiality_indicator: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#manufacturer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#regulator: Option<Box<super::super::types::Reference>> = None;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::OperationType => {
                            if r#operation_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationType"));
                            }
                            r#operation_type = Some(map_access.next_value()?);
                        }
                        Field::AuthorisationReferenceNumber => {
                            if r#authorisation_reference_number.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "authorisationReferenceNumber",
                                ));
                            }
                            r#authorisation_reference_number = Some(map_access.next_value()?);
                        }
                        Field::EffectiveDate => {
                            let some = r#effective_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::EffectiveDatePrimitiveElement => {
                            let some = r#effective_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_effectiveDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ConfidentialityIndicator => {
                            if r#confidentiality_indicator.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "confidentialityIndicator",
                                ));
                            }
                            r#confidentiality_indicator = Some(map_access.next_value()?);
                        }
                        Field::Manufacturer => {
                            if r#manufacturer.is_some() {
                                return Err(serde::de::Error::duplicate_field("manufacturer"));
                            }
                            r#manufacturer = Some(map_access.next_value()?);
                        }
                        Field::Regulator => {
                            if r#regulator.is_some() {
                                return Err(serde::de::Error::duplicate_field("regulator"));
                            }
                            r#regulator = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductManufacturingBusinessOperation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#operation_type,
                    r#authorisation_reference_number,
                    r#effective_date,
                    r#confidentiality_indicator,
                    r#manufacturer: r#manufacturer.unwrap_or(vec![]),
                    r#regulator,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductSpecialDesignation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#intended_use: Option<Box<super::super::types::CodeableConcept>>,
    pub r#indication: Option<MedicinalProductSpecialDesignationIndication>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#species: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for MedicinalProductSpecialDesignation {
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
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#intended_use.as_ref() {
            state.serialize_entry("intendedUse", some)?;
        }
        if let Some(some) = self.r#indication.as_ref() {
            match some {
                MedicinalProductSpecialDesignationIndication::CodeableConcept(ref value) => {
                    state.serialize_entry("indicationCodeableConcept", value)?;
                }
                MedicinalProductSpecialDesignationIndication::Reference(ref value) => {
                    state.serialize_entry("indicationReference", value)?;
                }
                MedicinalProductSpecialDesignationIndication::Invalid => {
                    return Err(serde::ser::Error::custom("indication is invalid"))
                }
            }
        }
        if let Some(some) = self.r#status.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("date", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_date", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#species.as_ref() {
            state.serialize_entry("species", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductSpecialDesignation {
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
            #[serde(rename = "intendedUse")]
            IntendedUse,
            #[serde(rename = "indicationCodeableConcept")]
            IndicationCodeableConcept,
            #[serde(rename = "indicationReference")]
            IndicationReference,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "species")]
            Species,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductSpecialDesignation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductSpecialDesignation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductSpecialDesignation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#intended_use: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#indication: Option<MedicinalProductSpecialDesignationIndication> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#species: Option<Box<super::super::types::CodeableConcept>> = None;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                        Field::IntendedUse => {
                            if r#intended_use.is_some() {
                                return Err(serde::de::Error::duplicate_field("intendedUse"));
                            }
                            r#intended_use = Some(map_access.next_value()?);
                        }
                        Field::IndicationCodeableConcept => {
                            if r#indication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "indicationCodeableConcept",
                                ));
                            }
                            r#indication = Some(
                                MedicinalProductSpecialDesignationIndication::CodeableConcept(
                                    map_access.next_value()?,
                                ),
                            );
                        }
                        Field::IndicationReference => {
                            if r#indication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "indicationReference",
                                ));
                            }
                            r#indication =
                                Some(MedicinalProductSpecialDesignationIndication::Reference(
                                    map_access.next_value()?,
                                ));
                        }
                        Field::Status => {
                            if r#status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            r#status = Some(map_access.next_value()?);
                        }
                        Field::Date => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DatePrimitiveElement => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_date"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Species => {
                            if r#species.is_some() {
                                return Err(serde::de::Error::duplicate_field("species"));
                            }
                            r#species = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductSpecialDesignation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#type,
                    r#intended_use,
                    r#indication,
                    r#status,
                    r#date,
                    r#species,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProduct {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#domain: Option<Box<super::super::types::Coding>>,
    pub r#combined_pharmaceutical_dose_form: Option<Box<super::super::types::CodeableConcept>>,
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
    pub r#additional_monitoring_indicator: Option<Box<super::super::types::CodeableConcept>>,
    pub r#special_measures: Vec<super::super::types::String>,
    pub r#paediatric_use_indicator: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_classification: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#marketing_status: Vec<Box<super::super::types::MarketingStatus>>,
    pub r#pharmaceutical_product: Vec<Box<super::super::types::Reference>>,
    pub r#packaged_medicinal_product: Vec<Box<super::super::types::Reference>>,
    pub r#attached_document: Vec<Box<super::super::types::Reference>>,
    pub r#master_file: Vec<Box<super::super::types::Reference>>,
    pub r#contact: Vec<Box<super::super::types::Reference>>,
    pub r#clinical_trial: Vec<Box<super::super::types::Reference>>,
    pub r#name: Vec<MedicinalProductName>,
    pub r#cross_reference: Vec<Box<super::super::types::Identifier>>,
    pub r#manufacturing_business_operation: Vec<MedicinalProductManufacturingBusinessOperation>,
    pub r#special_designation: Vec<MedicinalProductSpecialDesignation>,
}
impl serde::ser::Serialize for MedicinalProduct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProduct")?;
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
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#domain.as_ref() {
            state.serialize_entry("domain", some)?;
        }
        if let Some(some) = self.r#combined_pharmaceutical_dose_form.as_ref() {
            state.serialize_entry("combinedPharmaceuticalDoseForm", some)?;
        }
        if let Some(some) = self.r#legal_status_of_supply.as_ref() {
            state.serialize_entry("legalStatusOfSupply", some)?;
        }
        if let Some(some) = self.r#additional_monitoring_indicator.as_ref() {
            state.serialize_entry("additionalMonitoringIndicator", some)?;
        }
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
                let primitive_elements: Vec<_> = self
                    .r#special_measures
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
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
        if let Some(some) = self.r#paediatric_use_indicator.as_ref() {
            state.serialize_entry("paediatricUseIndicator", some)?;
        }
        if !self.r#product_classification.is_empty() {
            state.serialize_entry("productClassification", &self.r#product_classification)?;
        }
        if !self.r#marketing_status.is_empty() {
            state.serialize_entry("marketingStatus", &self.r#marketing_status)?;
        }
        if !self.r#pharmaceutical_product.is_empty() {
            state.serialize_entry("pharmaceuticalProduct", &self.r#pharmaceutical_product)?;
        }
        if !self.r#packaged_medicinal_product.is_empty() {
            state.serialize_entry(
                "packagedMedicinalProduct",
                &self.r#packaged_medicinal_product,
            )?;
        }
        if !self.r#attached_document.is_empty() {
            state.serialize_entry("attachedDocument", &self.r#attached_document)?;
        }
        if !self.r#master_file.is_empty() {
            state.serialize_entry("masterFile", &self.r#master_file)?;
        }
        if !self.r#contact.is_empty() {
            state.serialize_entry("contact", &self.r#contact)?;
        }
        if !self.r#clinical_trial.is_empty() {
            state.serialize_entry("clinicalTrial", &self.r#clinical_trial)?;
        }
        if !self.r#name.is_empty() {
            state.serialize_entry("name", &self.r#name)?;
        }
        if !self.r#cross_reference.is_empty() {
            state.serialize_entry("crossReference", &self.r#cross_reference)?;
        }
        if !self.r#manufacturing_business_operation.is_empty() {
            state.serialize_entry(
                "manufacturingBusinessOperation",
                &self.r#manufacturing_business_operation,
            )?;
        }
        if !self.r#special_designation.is_empty() {
            state.serialize_entry("specialDesignation", &self.r#special_designation)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProduct {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "domain")]
            Domain,
            #[serde(rename = "combinedPharmaceuticalDoseForm")]
            CombinedPharmaceuticalDoseForm,
            #[serde(rename = "legalStatusOfSupply")]
            LegalStatusOfSupply,
            #[serde(rename = "additionalMonitoringIndicator")]
            AdditionalMonitoringIndicator,
            #[serde(rename = "specialMeasures")]
            SpecialMeasures,
            #[serde(rename = "_specialMeasures")]
            SpecialMeasuresPrimitiveElement,
            #[serde(rename = "paediatricUseIndicator")]
            PaediatricUseIndicator,
            #[serde(rename = "productClassification")]
            ProductClassification,
            #[serde(rename = "marketingStatus")]
            MarketingStatus,
            #[serde(rename = "pharmaceuticalProduct")]
            PharmaceuticalProduct,
            #[serde(rename = "packagedMedicinalProduct")]
            PackagedMedicinalProduct,
            #[serde(rename = "attachedDocument")]
            AttachedDocument,
            #[serde(rename = "masterFile")]
            MasterFile,
            #[serde(rename = "contact")]
            Contact,
            #[serde(rename = "clinicalTrial")]
            ClinicalTrial,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "crossReference")]
            CrossReference,
            #[serde(rename = "manufacturingBusinessOperation")]
            ManufacturingBusinessOperation,
            #[serde(rename = "specialDesignation")]
            SpecialDesignation,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProduct;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProduct")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicinalProduct, V::Error>
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
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#domain: Option<Box<super::super::types::Coding>> = None;
                let mut r#combined_pharmaceutical_dose_form: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#legal_status_of_supply: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#additional_monitoring_indicator: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#special_measures: Option<Vec<super::super::types::String>> = None;
                let mut r#paediatric_use_indicator: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#product_classification: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#marketing_status: Option<Vec<Box<super::super::types::MarketingStatus>>> =
                    None;
                let mut r#pharmaceutical_product: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#packaged_medicinal_product: Option<
                    Vec<Box<super::super::types::Reference>>,
                > = None;
                let mut r#attached_document: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#master_file: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#clinical_trial: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#name: Option<Vec<MedicinalProductName>> = None;
                let mut r#cross_reference: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#manufacturing_business_operation: Option<
                    Vec<MedicinalProductManufacturingBusinessOperation>,
                > = None;
                let mut r#special_designation: Option<Vec<MedicinalProductSpecialDesignation>> =
                    None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicinalProduct" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicinalProduct",
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
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRulesPrimitiveElement => {
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
                        Field::Language => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LanguagePrimitiveElement => {
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                        Field::Domain => {
                            if r#domain.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            r#domain = Some(map_access.next_value()?);
                        }
                        Field::CombinedPharmaceuticalDoseForm => {
                            if r#combined_pharmaceutical_dose_form.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "combinedPharmaceuticalDoseForm",
                                ));
                            }
                            r#combined_pharmaceutical_dose_form = Some(map_access.next_value()?);
                        }
                        Field::LegalStatusOfSupply => {
                            if r#legal_status_of_supply.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "legalStatusOfSupply",
                                ));
                            }
                            r#legal_status_of_supply = Some(map_access.next_value()?);
                        }
                        Field::AdditionalMonitoringIndicator => {
                            if r#additional_monitoring_indicator.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "additionalMonitoringIndicator",
                                ));
                            }
                            r#additional_monitoring_indicator = Some(map_access.next_value()?);
                        }
                        Field::SpecialMeasures => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#special_measures.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(values.len())
                                    .collect::<Vec<_>>(),
                            );
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("specialMeasures"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::SpecialMeasuresPrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
                            let vec = r#special_measures.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(elements.len())
                                    .collect::<Vec<_>>(),
                            );
                            if vec.len() != elements.len() {
                                return Err(serde::de::Error::invalid_length(
                                    elements.len(),
                                    &"primitive values length",
                                ));
                            }
                            if vec
                                .iter()
                                .any(|e| e.id.is_some() || !e.extension.is_empty())
                            {
                                return Err(serde::de::Error::duplicate_field("_specialMeasures"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                if let Some(element) = element {
                                    vec[i].id = element.id;
                                    vec[i].extension = element.extension;
                                }
                            }
                        }
                        Field::PaediatricUseIndicator => {
                            if r#paediatric_use_indicator.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "paediatricUseIndicator",
                                ));
                            }
                            r#paediatric_use_indicator = Some(map_access.next_value()?);
                        }
                        Field::ProductClassification => {
                            if r#product_classification.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "productClassification",
                                ));
                            }
                            r#product_classification = Some(map_access.next_value()?);
                        }
                        Field::MarketingStatus => {
                            if r#marketing_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketingStatus"));
                            }
                            r#marketing_status = Some(map_access.next_value()?);
                        }
                        Field::PharmaceuticalProduct => {
                            if r#pharmaceutical_product.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "pharmaceuticalProduct",
                                ));
                            }
                            r#pharmaceutical_product = Some(map_access.next_value()?);
                        }
                        Field::PackagedMedicinalProduct => {
                            if r#packaged_medicinal_product.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "packagedMedicinalProduct",
                                ));
                            }
                            r#packaged_medicinal_product = Some(map_access.next_value()?);
                        }
                        Field::AttachedDocument => {
                            if r#attached_document.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachedDocument"));
                            }
                            r#attached_document = Some(map_access.next_value()?);
                        }
                        Field::MasterFile => {
                            if r#master_file.is_some() {
                                return Err(serde::de::Error::duplicate_field("masterFile"));
                            }
                            r#master_file = Some(map_access.next_value()?);
                        }
                        Field::Contact => {
                            if r#contact.is_some() {
                                return Err(serde::de::Error::duplicate_field("contact"));
                            }
                            r#contact = Some(map_access.next_value()?);
                        }
                        Field::ClinicalTrial => {
                            if r#clinical_trial.is_some() {
                                return Err(serde::de::Error::duplicate_field("clinicalTrial"));
                            }
                            r#clinical_trial = Some(map_access.next_value()?);
                        }
                        Field::Name => {
                            if r#name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            r#name = Some(map_access.next_value()?);
                        }
                        Field::CrossReference => {
                            if r#cross_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("crossReference"));
                            }
                            r#cross_reference = Some(map_access.next_value()?);
                        }
                        Field::ManufacturingBusinessOperation => {
                            if r#manufacturing_business_operation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "manufacturingBusinessOperation",
                                ));
                            }
                            r#manufacturing_business_operation = Some(map_access.next_value()?);
                        }
                        Field::SpecialDesignation => {
                            if r#special_designation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "specialDesignation",
                                ));
                            }
                            r#special_designation = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProduct {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#type,
                    r#domain,
                    r#combined_pharmaceutical_dose_form,
                    r#legal_status_of_supply,
                    r#additional_monitoring_indicator,
                    r#special_measures: r#special_measures.unwrap_or(vec![]),
                    r#paediatric_use_indicator,
                    r#product_classification: r#product_classification.unwrap_or(vec![]),
                    r#marketing_status: r#marketing_status.unwrap_or(vec![]),
                    r#pharmaceutical_product: r#pharmaceutical_product.unwrap_or(vec![]),
                    r#packaged_medicinal_product: r#packaged_medicinal_product.unwrap_or(vec![]),
                    r#attached_document: r#attached_document.unwrap_or(vec![]),
                    r#master_file: r#master_file.unwrap_or(vec![]),
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#clinical_trial: r#clinical_trial.unwrap_or(vec![]),
                    r#name: r#name.unwrap_or(vec![]),
                    r#cross_reference: r#cross_reference.unwrap_or(vec![]),
                    r#manufacturing_business_operation: r#manufacturing_business_operation
                        .unwrap_or(vec![]),
                    r#special_designation: r#special_designation.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
