// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterialFractionDescription {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#fraction: Option<super::super::types::String>,
    pub r#material_type: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubstanceSourceMaterialFractionDescription {
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
        if let Some(some) = self.r#fraction.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("fraction", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_fraction", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#material_type.as_ref() {
            state.serialize_entry("materialType", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSourceMaterialFractionDescription {
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
            #[serde(rename = "fraction")]
            Fraction,
            #[serde(rename = "_fraction")]
            FractionPrimitiveElement,
            #[serde(rename = "materialType")]
            MaterialType,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSourceMaterialFractionDescription;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSourceMaterialFractionDescription")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSourceMaterialFractionDescription, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#fraction: Option<super::super::types::String> = None;
                let mut r#material_type: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        Field::Fraction => {
                            let some = r#fraction.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("fraction"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::FractionPrimitiveElement => {
                            let some = r#fraction.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_fraction"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::MaterialType => {
                            if r#material_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("materialType"));
                            }
                            r#material_type = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(SubstanceSourceMaterialFractionDescription {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#fraction,
                    r#material_type,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterialOrganismAuthor {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#author_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#author_description: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SubstanceSourceMaterialOrganismAuthor {
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
        if let Some(some) = self.r#author_type.as_ref() {
            state.serialize_entry("authorType", some)?;
        }
        if let Some(some) = self.r#author_description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("authorDescription", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_authorDescription", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSourceMaterialOrganismAuthor {
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
            #[serde(rename = "authorType")]
            AuthorType,
            #[serde(rename = "authorDescription")]
            AuthorDescription,
            #[serde(rename = "_authorDescription")]
            AuthorDescriptionPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSourceMaterialOrganismAuthor;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSourceMaterialOrganismAuthor")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSourceMaterialOrganismAuthor, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#author_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#author_description: Option<super::super::types::String> = None;
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
                        Field::AuthorType => {
                            if r#author_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorType"));
                            }
                            r#author_type = Some(map_access.next_value()?);
                        }
                        Field::AuthorDescription => {
                            let some = r#author_description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorDescription"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::AuthorDescriptionPrimitiveElement => {
                            let some = r#author_description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_authorDescription",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(SubstanceSourceMaterialOrganismAuthor {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#author_type,
                    r#author_description,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterialOrganismHybrid {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#maternal_organism_id: Option<super::super::types::String>,
    pub r#maternal_organism_name: Option<super::super::types::String>,
    pub r#paternal_organism_id: Option<super::super::types::String>,
    pub r#paternal_organism_name: Option<super::super::types::String>,
    pub r#hybrid_type: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubstanceSourceMaterialOrganismHybrid {
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
        if let Some(some) = self.r#maternal_organism_id.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("maternalOrganismId", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_maternalOrganismId", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#maternal_organism_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("maternalOrganismName", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_maternalOrganismName", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#paternal_organism_id.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("paternalOrganismId", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_paternalOrganismId", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#paternal_organism_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("paternalOrganismName", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_paternalOrganismName", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#hybrid_type.as_ref() {
            state.serialize_entry("hybridType", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSourceMaterialOrganismHybrid {
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
            #[serde(rename = "maternalOrganismId")]
            MaternalOrganismId,
            #[serde(rename = "_maternalOrganismId")]
            MaternalOrganismIdPrimitiveElement,
            #[serde(rename = "maternalOrganismName")]
            MaternalOrganismName,
            #[serde(rename = "_maternalOrganismName")]
            MaternalOrganismNamePrimitiveElement,
            #[serde(rename = "paternalOrganismId")]
            PaternalOrganismId,
            #[serde(rename = "_paternalOrganismId")]
            PaternalOrganismIdPrimitiveElement,
            #[serde(rename = "paternalOrganismName")]
            PaternalOrganismName,
            #[serde(rename = "_paternalOrganismName")]
            PaternalOrganismNamePrimitiveElement,
            #[serde(rename = "hybridType")]
            HybridType,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSourceMaterialOrganismHybrid;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSourceMaterialOrganismHybrid")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSourceMaterialOrganismHybrid, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#maternal_organism_id: Option<super::super::types::String> = None;
                let mut r#maternal_organism_name: Option<super::super::types::String> = None;
                let mut r#paternal_organism_id: Option<super::super::types::String> = None;
                let mut r#paternal_organism_name: Option<super::super::types::String> = None;
                let mut r#hybrid_type: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        Field::MaternalOrganismId => {
                            let some = r#maternal_organism_id.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maternalOrganismId",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::MaternalOrganismIdPrimitiveElement => {
                            let some = r#maternal_organism_id.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_maternalOrganismId",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::MaternalOrganismName => {
                            let some = r#maternal_organism_name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maternalOrganismName",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::MaternalOrganismNamePrimitiveElement => {
                            let some = r#maternal_organism_name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_maternalOrganismName",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::PaternalOrganismId => {
                            let some = r#paternal_organism_id.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "paternalOrganismId",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PaternalOrganismIdPrimitiveElement => {
                            let some = r#paternal_organism_id.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_paternalOrganismId",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::PaternalOrganismName => {
                            let some = r#paternal_organism_name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "paternalOrganismName",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PaternalOrganismNamePrimitiveElement => {
                            let some = r#paternal_organism_name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_paternalOrganismName",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::HybridType => {
                            if r#hybrid_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("hybridType"));
                            }
                            r#hybrid_type = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(SubstanceSourceMaterialOrganismHybrid {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#maternal_organism_id,
                    r#maternal_organism_name,
                    r#paternal_organism_id,
                    r#paternal_organism_name,
                    r#hybrid_type,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterialOrganismOrganismGeneral {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#kingdom: Option<Box<super::super::types::CodeableConcept>>,
    pub r#phylum: Option<Box<super::super::types::CodeableConcept>>,
    pub r#class: Option<Box<super::super::types::CodeableConcept>>,
    pub r#order: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubstanceSourceMaterialOrganismOrganismGeneral {
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
        if let Some(some) = self.r#kingdom.as_ref() {
            state.serialize_entry("kingdom", some)?;
        }
        if let Some(some) = self.r#phylum.as_ref() {
            state.serialize_entry("phylum", some)?;
        }
        if let Some(some) = self.r#class.as_ref() {
            state.serialize_entry("class", some)?;
        }
        if let Some(some) = self.r#order.as_ref() {
            state.serialize_entry("order", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSourceMaterialOrganismOrganismGeneral {
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
            #[serde(rename = "kingdom")]
            Kingdom,
            #[serde(rename = "phylum")]
            Phylum,
            #[serde(rename = "class")]
            Class,
            #[serde(rename = "order")]
            Order,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSourceMaterialOrganismOrganismGeneral;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSourceMaterialOrganismOrganismGeneral")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSourceMaterialOrganismOrganismGeneral, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#kingdom: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#phylum: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#class: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#order: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        Field::Kingdom => {
                            if r#kingdom.is_some() {
                                return Err(serde::de::Error::duplicate_field("kingdom"));
                            }
                            r#kingdom = Some(map_access.next_value()?);
                        }
                        Field::Phylum => {
                            if r#phylum.is_some() {
                                return Err(serde::de::Error::duplicate_field("phylum"));
                            }
                            r#phylum = Some(map_access.next_value()?);
                        }
                        Field::Class => {
                            if r#class.is_some() {
                                return Err(serde::de::Error::duplicate_field("class"));
                            }
                            r#class = Some(map_access.next_value()?);
                        }
                        Field::Order => {
                            if r#order.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            r#order = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(SubstanceSourceMaterialOrganismOrganismGeneral {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#kingdom,
                    r#phylum,
                    r#class,
                    r#order,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterialOrganism {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#family: Option<Box<super::super::types::CodeableConcept>>,
    pub r#genus: Option<Box<super::super::types::CodeableConcept>>,
    pub r#species: Option<Box<super::super::types::CodeableConcept>>,
    pub r#intraspecific_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#intraspecific_description: Option<super::super::types::String>,
    pub r#author: Vec<SubstanceSourceMaterialOrganismAuthor>,
    pub r#hybrid: Option<SubstanceSourceMaterialOrganismHybrid>,
    pub r#organism_general: Option<SubstanceSourceMaterialOrganismOrganismGeneral>,
}
impl serde::ser::Serialize for SubstanceSourceMaterialOrganism {
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
        if let Some(some) = self.r#family.as_ref() {
            state.serialize_entry("family", some)?;
        }
        if let Some(some) = self.r#genus.as_ref() {
            state.serialize_entry("genus", some)?;
        }
        if let Some(some) = self.r#species.as_ref() {
            state.serialize_entry("species", some)?;
        }
        if let Some(some) = self.r#intraspecific_type.as_ref() {
            state.serialize_entry("intraspecificType", some)?;
        }
        if let Some(some) = self.r#intraspecific_description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("intraspecificDescription", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_intraspecificDescription", &primitive_element)?;
            }
        }
        if !self.r#author.is_empty() {
            state.serialize_entry("author", &self.r#author)?;
        }
        if let Some(some) = self.r#hybrid.as_ref() {
            state.serialize_entry("hybrid", some)?;
        }
        if let Some(some) = self.r#organism_general.as_ref() {
            state.serialize_entry("organismGeneral", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSourceMaterialOrganism {
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
            #[serde(rename = "family")]
            Family,
            #[serde(rename = "genus")]
            Genus,
            #[serde(rename = "species")]
            Species,
            #[serde(rename = "intraspecificType")]
            IntraspecificType,
            #[serde(rename = "intraspecificDescription")]
            IntraspecificDescription,
            #[serde(rename = "_intraspecificDescription")]
            IntraspecificDescriptionPrimitiveElement,
            #[serde(rename = "author")]
            Author,
            #[serde(rename = "hybrid")]
            Hybrid,
            #[serde(rename = "organismGeneral")]
            OrganismGeneral,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSourceMaterialOrganism;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSourceMaterialOrganism")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSourceMaterialOrganism, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#family: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#genus: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#species: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#intraspecific_type: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#intraspecific_description: Option<super::super::types::String> = None;
                let mut r#author: Option<Vec<SubstanceSourceMaterialOrganismAuthor>> = None;
                let mut r#hybrid: Option<SubstanceSourceMaterialOrganismHybrid> = None;
                let mut r#organism_general: Option<SubstanceSourceMaterialOrganismOrganismGeneral> =
                    None;
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
                        Field::Family => {
                            if r#family.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            r#family = Some(map_access.next_value()?);
                        }
                        Field::Genus => {
                            if r#genus.is_some() {
                                return Err(serde::de::Error::duplicate_field("genus"));
                            }
                            r#genus = Some(map_access.next_value()?);
                        }
                        Field::Species => {
                            if r#species.is_some() {
                                return Err(serde::de::Error::duplicate_field("species"));
                            }
                            r#species = Some(map_access.next_value()?);
                        }
                        Field::IntraspecificType => {
                            if r#intraspecific_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("intraspecificType"));
                            }
                            r#intraspecific_type = Some(map_access.next_value()?);
                        }
                        Field::IntraspecificDescription => {
                            let some =
                                r#intraspecific_description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "intraspecificDescription",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IntraspecificDescriptionPrimitiveElement => {
                            let some =
                                r#intraspecific_description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_intraspecificDescription",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Author => {
                            if r#author.is_some() {
                                return Err(serde::de::Error::duplicate_field("author"));
                            }
                            r#author = Some(map_access.next_value()?);
                        }
                        Field::Hybrid => {
                            if r#hybrid.is_some() {
                                return Err(serde::de::Error::duplicate_field("hybrid"));
                            }
                            r#hybrid = Some(map_access.next_value()?);
                        }
                        Field::OrganismGeneral => {
                            if r#organism_general.is_some() {
                                return Err(serde::de::Error::duplicate_field("organismGeneral"));
                            }
                            r#organism_general = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(SubstanceSourceMaterialOrganism {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#family,
                    r#genus,
                    r#species,
                    r#intraspecific_type,
                    r#intraspecific_description,
                    r#author: r#author.unwrap_or(vec![]),
                    r#hybrid,
                    r#organism_general,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterialPartDescription {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#part: Option<Box<super::super::types::CodeableConcept>>,
    pub r#part_location: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubstanceSourceMaterialPartDescription {
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
        if let Some(some) = self.r#part.as_ref() {
            state.serialize_entry("part", some)?;
        }
        if let Some(some) = self.r#part_location.as_ref() {
            state.serialize_entry("partLocation", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSourceMaterialPartDescription {
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
            #[serde(rename = "partLocation")]
            PartLocation,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSourceMaterialPartDescription;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSourceMaterialPartDescription")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSourceMaterialPartDescription, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#part: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#part_location: Option<Box<super::super::types::CodeableConcept>> = None;
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
                            if r#part.is_some() {
                                return Err(serde::de::Error::duplicate_field("part"));
                            }
                            r#part = Some(map_access.next_value()?);
                        }
                        Field::PartLocation => {
                            if r#part_location.is_some() {
                                return Err(serde::de::Error::duplicate_field("partLocation"));
                            }
                            r#part_location = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(SubstanceSourceMaterialPartDescription {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#part,
                    r#part_location,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterial {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#source_material_class: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source_material_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source_material_state: Option<Box<super::super::types::CodeableConcept>>,
    pub r#organism_id: Option<Box<super::super::types::Identifier>>,
    pub r#organism_name: Option<super::super::types::String>,
    pub r#parent_substance_id: Vec<Box<super::super::types::Identifier>>,
    pub r#parent_substance_name: Vec<super::super::types::String>,
    pub r#country_of_origin: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#geographical_location: Vec<super::super::types::String>,
    pub r#development_stage: Option<Box<super::super::types::CodeableConcept>>,
    pub r#fraction_description: Vec<SubstanceSourceMaterialFractionDescription>,
    pub r#organism: Option<SubstanceSourceMaterialOrganism>,
    pub r#part_description: Vec<SubstanceSourceMaterialPartDescription>,
}
impl serde::ser::Serialize for SubstanceSourceMaterial {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SubstanceSourceMaterial")?;
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
        if let Some(some) = self.r#source_material_class.as_ref() {
            state.serialize_entry("sourceMaterialClass", some)?;
        }
        if let Some(some) = self.r#source_material_type.as_ref() {
            state.serialize_entry("sourceMaterialType", some)?;
        }
        if let Some(some) = self.r#source_material_state.as_ref() {
            state.serialize_entry("sourceMaterialState", some)?;
        }
        if let Some(some) = self.r#organism_id.as_ref() {
            state.serialize_entry("organismId", some)?;
        }
        if let Some(some) = self.r#organism_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("organismName", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_organismName", &primitive_element)?;
            }
        }
        if !self.r#parent_substance_id.is_empty() {
            state.serialize_entry("parentSubstanceId", &self.r#parent_substance_id)?;
        }
        if !self.r#parent_substance_name.is_empty() {
            let values: Vec<_> = self
                .r#parent_substance_name
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("parentSubstanceName", &values)?;
            }
            let requires_elements = self
                .r#parent_substance_name
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#parent_substance_name
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
                state.serialize_entry("_parentSubstanceName", &primitive_elements)?;
            }
        }
        if !self.r#country_of_origin.is_empty() {
            state.serialize_entry("countryOfOrigin", &self.r#country_of_origin)?;
        }
        if !self.r#geographical_location.is_empty() {
            let values: Vec<_> = self
                .r#geographical_location
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("geographicalLocation", &values)?;
            }
            let requires_elements = self
                .r#geographical_location
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#geographical_location
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
                state.serialize_entry("_geographicalLocation", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#development_stage.as_ref() {
            state.serialize_entry("developmentStage", some)?;
        }
        if !self.r#fraction_description.is_empty() {
            state.serialize_entry("fractionDescription", &self.r#fraction_description)?;
        }
        if let Some(some) = self.r#organism.as_ref() {
            state.serialize_entry("organism", some)?;
        }
        if !self.r#part_description.is_empty() {
            state.serialize_entry("partDescription", &self.r#part_description)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSourceMaterial {
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
            #[serde(rename = "sourceMaterialClass")]
            SourceMaterialClass,
            #[serde(rename = "sourceMaterialType")]
            SourceMaterialType,
            #[serde(rename = "sourceMaterialState")]
            SourceMaterialState,
            #[serde(rename = "organismId")]
            OrganismId,
            #[serde(rename = "organismName")]
            OrganismName,
            #[serde(rename = "_organismName")]
            OrganismNamePrimitiveElement,
            #[serde(rename = "parentSubstanceId")]
            ParentSubstanceId,
            #[serde(rename = "parentSubstanceName")]
            ParentSubstanceName,
            #[serde(rename = "_parentSubstanceName")]
            ParentSubstanceNamePrimitiveElement,
            #[serde(rename = "countryOfOrigin")]
            CountryOfOrigin,
            #[serde(rename = "geographicalLocation")]
            GeographicalLocation,
            #[serde(rename = "_geographicalLocation")]
            GeographicalLocationPrimitiveElement,
            #[serde(rename = "developmentStage")]
            DevelopmentStage,
            #[serde(rename = "fractionDescription")]
            FractionDescription,
            #[serde(rename = "organism")]
            Organism,
            #[serde(rename = "partDescription")]
            PartDescription,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSourceMaterial;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSourceMaterial")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceSourceMaterial, V::Error>
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
                let mut r#source_material_class: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#source_material_type: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#source_material_state: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#organism_id: Option<Box<super::super::types::Identifier>> = None;
                let mut r#organism_name: Option<super::super::types::String> = None;
                let mut r#parent_substance_id: Option<Vec<Box<super::super::types::Identifier>>> =
                    None;
                let mut r#parent_substance_name: Option<Vec<super::super::types::String>> = None;
                let mut r#country_of_origin: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#geographical_location: Option<Vec<super::super::types::String>> = None;
                let mut r#development_stage: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#fraction_description: Option<
                    Vec<SubstanceSourceMaterialFractionDescription>,
                > = None;
                let mut r#organism: Option<SubstanceSourceMaterialOrganism> = None;
                let mut r#part_description: Option<Vec<SubstanceSourceMaterialPartDescription>> =
                    None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "SubstanceSourceMaterial" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"SubstanceSourceMaterial",
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
                        Field::SourceMaterialClass => {
                            if r#source_material_class.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sourceMaterialClass",
                                ));
                            }
                            r#source_material_class = Some(map_access.next_value()?);
                        }
                        Field::SourceMaterialType => {
                            if r#source_material_type.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sourceMaterialType",
                                ));
                            }
                            r#source_material_type = Some(map_access.next_value()?);
                        }
                        Field::SourceMaterialState => {
                            if r#source_material_state.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sourceMaterialState",
                                ));
                            }
                            r#source_material_state = Some(map_access.next_value()?);
                        }
                        Field::OrganismId => {
                            if r#organism_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("organismId"));
                            }
                            r#organism_id = Some(map_access.next_value()?);
                        }
                        Field::OrganismName => {
                            let some = r#organism_name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("organismName"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::OrganismNamePrimitiveElement => {
                            let some = r#organism_name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_organismName"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ParentSubstanceId => {
                            if r#parent_substance_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentSubstanceId"));
                            }
                            r#parent_substance_id = Some(map_access.next_value()?);
                        }
                        Field::ParentSubstanceName => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#parent_substance_name.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field(
                                    "parentSubstanceName",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::ParentSubstanceNamePrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
                            let vec = r#parent_substance_name.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field(
                                    "_parentSubstanceName",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                if let Some(element) = element {
                                    vec[i].id = element.id;
                                    vec[i].extension = element.extension;
                                }
                            }
                        }
                        Field::CountryOfOrigin => {
                            if r#country_of_origin.is_some() {
                                return Err(serde::de::Error::duplicate_field("countryOfOrigin"));
                            }
                            r#country_of_origin = Some(map_access.next_value()?);
                        }
                        Field::GeographicalLocation => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#geographical_location.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field(
                                    "geographicalLocation",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::GeographicalLocationPrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
                            let vec = r#geographical_location.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field(
                                    "_geographicalLocation",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                if let Some(element) = element {
                                    vec[i].id = element.id;
                                    vec[i].extension = element.extension;
                                }
                            }
                        }
                        Field::DevelopmentStage => {
                            if r#development_stage.is_some() {
                                return Err(serde::de::Error::duplicate_field("developmentStage"));
                            }
                            r#development_stage = Some(map_access.next_value()?);
                        }
                        Field::FractionDescription => {
                            if r#fraction_description.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fractionDescription",
                                ));
                            }
                            r#fraction_description = Some(map_access.next_value()?);
                        }
                        Field::Organism => {
                            if r#organism.is_some() {
                                return Err(serde::de::Error::duplicate_field("organism"));
                            }
                            r#organism = Some(map_access.next_value()?);
                        }
                        Field::PartDescription => {
                            if r#part_description.is_some() {
                                return Err(serde::de::Error::duplicate_field("partDescription"));
                            }
                            r#part_description = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(SubstanceSourceMaterial {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#source_material_class,
                    r#source_material_type,
                    r#source_material_state,
                    r#organism_id,
                    r#organism_name,
                    r#parent_substance_id: r#parent_substance_id.unwrap_or(vec![]),
                    r#parent_substance_name: r#parent_substance_name.unwrap_or(vec![]),
                    r#country_of_origin: r#country_of_origin.unwrap_or(vec![]),
                    r#geographical_location: r#geographical_location.unwrap_or(vec![]),
                    r#development_stage,
                    r#fraction_description: r#fraction_description.unwrap_or(vec![]),
                    r#organism,
                    r#part_description: r#part_description.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
