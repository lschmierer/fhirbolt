// Generated on 2022-12-16 by fhirbolt-codegen v0.1.0
#[doc = "Many complex materials are fractions of parts of plants, animals, or minerals. Fraction elements are often necessary to define both Substances and Specified Group 1 Substances. For substances derived from Plants, fraction information will be captured at the Substance information level ( . Oils, Juices and Exudates). Additional information for Extracts, such as extraction solvent composition, will be captured at the Specified Substance Group 1 information level. For plasma-derived products fraction information will be captured at the Substance and the Specified Substance Group 1 levels."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterialFractionDescription {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "This element is capturing information about the fraction of a plant part, or human plasma for fractionation."]
    pub r#fraction: Option<super::super::types::String>,
    #[doc = "The specific type of the material constituting the component. For Herbal preparations the particulars of the extracts (liquid/dry) is described in Specified Substance Group 1."]
    pub r#material_type: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubstanceSourceMaterialFractionDescription {
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
            if _ctx.output_json {
                if let Some(some) = self.r#fraction.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("fraction", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_fraction", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#fraction.as_ref() {
                    state.serialize_entry("fraction", some)?;
                }
            }
            if let Some(some) = self.r#material_type.as_ref() {
                state.serialize_entry("materialType", some)?;
            }
            state.end()
        })
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
            Unknown(String),
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
                            Field::Fraction => {
                                let some = r#fraction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fraction"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "fraction",
                                        "materialType",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceSourceMaterialFractionDescription {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#fraction,
                        r#material_type,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "4.9.13.6.1 Author type (Conditional)."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterialOrganismAuthor {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of author of an organism species shall be specified. The parenthetical author of an organism species refers to the first author who published the plant/animal name (of any rank). The primary author of an organism species refers to the first author(s), who validly published the plant/animal name."]
    pub r#author_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The author of an organism species shall be specified. The author year of an organism shall also be specified when applicable; refers to the year in which the first author(s) published the infraspecific plant/animal name (of any rank)."]
    pub r#author_description: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SubstanceSourceMaterialOrganismAuthor {
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
            if let Some(some) = self.r#author_type.as_ref() {
                state.serialize_entry("authorType", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#author_description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("authorDescription", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_authorDescription", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#author_description.as_ref() {
                    state.serialize_entry("authorDescription", some)?;
                }
            }
            state.end()
        })
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
            Unknown(String),
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
                            Field::AuthorType => {
                                if r#author_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authorType"));
                                }
                                r#author_type = Some(map_access.next_value()?);
                            }
                            Field::AuthorDescription => {
                                let some = r#author_description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorDescription",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "authorType",
                                        "authorDescription",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceSourceMaterialOrganismAuthor {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#author_type,
                        r#author_description,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "4.9.13.8.1 Hybrid species maternal organism ID (Optional)."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterialOrganismHybrid {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The identifier of the maternal species constituting the hybrid organism shall be specified based on a controlled vocabulary. For plants, the parents aren’t always known, and it is unlikely that it will be known which is maternal and which is paternal."]
    pub r#maternal_organism_id: Option<super::super::types::String>,
    #[doc = "The name of the maternal species constituting the hybrid organism shall be specified. For plants, the parents aren’t always known, and it is unlikely that it will be known which is maternal and which is paternal."]
    pub r#maternal_organism_name: Option<super::super::types::String>,
    #[doc = "The identifier of the paternal species constituting the hybrid organism shall be specified based on a controlled vocabulary."]
    pub r#paternal_organism_id: Option<super::super::types::String>,
    #[doc = "The name of the paternal species constituting the hybrid organism shall be specified."]
    pub r#paternal_organism_name: Option<super::super::types::String>,
    #[doc = "The hybrid type of an organism shall be specified."]
    pub r#hybrid_type: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubstanceSourceMaterialOrganismHybrid {
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
            if _ctx.output_json {
                if let Some(some) = self.r#maternal_organism_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("maternalOrganismId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_maternalOrganismId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#maternal_organism_id.as_ref() {
                    state.serialize_entry("maternalOrganismId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#maternal_organism_name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("maternalOrganismName", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_maternalOrganismName", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#maternal_organism_name.as_ref() {
                    state.serialize_entry("maternalOrganismName", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#paternal_organism_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("paternalOrganismId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_paternalOrganismId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#paternal_organism_id.as_ref() {
                    state.serialize_entry("paternalOrganismId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#paternal_organism_name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("paternalOrganismName", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_paternalOrganismName", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#paternal_organism_name.as_ref() {
                    state.serialize_entry("paternalOrganismName", some)?;
                }
            }
            if let Some(some) = self.r#hybrid_type.as_ref() {
                state.serialize_entry("hybridType", some)?;
            }
            state.end()
        })
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
            Unknown(String),
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
                            Field::MaternalOrganismId => {
                                let some = r#maternal_organism_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maternalOrganismId",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let some =
                                    r#maternal_organism_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maternalOrganismName",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::MaternalOrganismNamePrimitiveElement => {
                                let some =
                                    r#maternal_organism_name.get_or_insert(Default::default());
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let some =
                                    r#paternal_organism_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "paternalOrganismName",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PaternalOrganismNamePrimitiveElement => {
                                let some =
                                    r#paternal_organism_name.get_or_insert(Default::default());
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
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "maternalOrganismId",
                                        "maternalOrganismName",
                                        "paternalOrganismId",
                                        "paternalOrganismName",
                                        "hybridType",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "4.9.13.7.1 Kingdom (Conditional)."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterialOrganismOrganismGeneral {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kingdom of an organism shall be specified."]
    pub r#kingdom: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The phylum of an organism shall be specified."]
    pub r#phylum: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The class of an organism shall be specified."]
    pub r#class: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The order of an organism shall be specified,."]
    pub r#order: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubstanceSourceMaterialOrganismOrganismGeneral {
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
        })
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
            Unknown(String),
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
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "kingdom",
                                        "phylum",
                                        "class",
                                        "order",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "This subclause describes the organism which the substance is derived from. For vaccines, the parent organism shall be specified based on these subclause elements. As an example, full taxonomy will be described for the Substance Name: ., Leaf."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterialOrganism {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The family of an organism shall be specified."]
    pub r#family: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The genus of an organism shall be specified; refers to the Latin epithet of the genus element of the plant/animal scientific name; it is present in names for genera, species and infraspecies."]
    pub r#genus: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The species of an organism shall be specified; refers to the Latin epithet of the species of the plant/animal; it is present in names for species and infraspecies."]
    pub r#species: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The Intraspecific type of an organism shall be specified."]
    pub r#intraspecific_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The intraspecific description of an organism shall be specified based on a controlled vocabulary. For Influenza Vaccine, the intraspecific description shall contain the syntax of the antigen in line with the WHO convention."]
    pub r#intraspecific_description: Option<super::super::types::String>,
    #[doc = "4.9.13.6.1 Author type (Conditional)."]
    pub r#author: Vec<SubstanceSourceMaterialOrganismAuthor>,
    #[doc = "4.9.13.8.1 Hybrid species maternal organism ID (Optional)."]
    pub r#hybrid: Option<SubstanceSourceMaterialOrganismHybrid>,
    #[doc = "4.9.13.7.1 Kingdom (Conditional)."]
    pub r#organism_general: Option<SubstanceSourceMaterialOrganismOrganismGeneral>,
}
impl serde::ser::Serialize for SubstanceSourceMaterialOrganism {
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
            if _ctx.output_json {
                if let Some(some) = self.r#intraspecific_description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("intraspecificDescription", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_intraspecificDescription", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#intraspecific_description.as_ref() {
                    state.serialize_entry("intraspecificDescription", some)?;
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
        })
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
            Unknown(String),
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "intraspecificType",
                                    ));
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "organismGeneral",
                                    ));
                                }
                                r#organism_general = Some(map_access.next_value()?);
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
                                        "family",
                                        "genus",
                                        "species",
                                        "intraspecificType",
                                        "intraspecificDescription",
                                        "author",
                                        "hybrid",
                                        "organismGeneral",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "To do."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterialPartDescription {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Entity of anatomical origin of source material within an organism."]
    pub r#part: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The detailed anatomic location when the part can be extracted from different anatomical locations of the organism. Multiple alternative locations may apply."]
    pub r#part_location: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubstanceSourceMaterialPartDescription {
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
            if let Some(some) = self.r#part.as_ref() {
                state.serialize_entry("part", some)?;
            }
            if let Some(some) = self.r#part_location.as_ref() {
                state.serialize_entry("partLocation", some)?;
            }
            state.end()
        })
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
            Unknown(String),
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
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "part",
                                        "partLocation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceSourceMaterialPartDescription {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#part,
                        r#part_location,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Source material shall capture information on the taxonomic and anatomical origins as well as the fraction of a material that can result in or can be modified to form a substance. This set of data elements shall be used to define polymer substances isolated from biological matrices. Taxonomic and anatomical origins shall be described using a controlled vocabulary as required. This information is captured for naturally derived polymers ( . starch) and structurally diverse substances. For Organisms belonging to the Kingdom Plantae the Substance level defines the fresh material of a single species or infraspecies, the Herbal Drug and the Herbal preparation. For Herbal preparations, the fraction information will be captured at the Substance information level and additional information for herbal extracts will be captured at the Specified Substance Group 1 information level. See for further explanation the Substance Class: Structurally Diverse and the herbal annex."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSourceMaterial {
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
    #[doc = "General high level classification of the source material specific to the origin of the material."]
    pub r#source_material_class: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of the source material shall be specified based on a controlled vocabulary. For vaccines, this subclause refers to the class of infectious agent."]
    pub r#source_material_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The state of the source material when extracted."]
    pub r#source_material_state: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The unique identifier associated with the source material parent organism shall be specified."]
    pub r#organism_id: Option<Box<super::super::types::Identifier>>,
    #[doc = "The organism accepted Scientific name shall be provided based on the organism taxonomy."]
    pub r#organism_name: Option<super::super::types::String>,
    #[doc = "The parent of the herbal drug Ginkgo biloba, Leaf is the substance ID of the substance (fresh) of Ginkgo biloba L. or Ginkgo biloba L. (Whole plant)."]
    pub r#parent_substance_id: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The parent substance of the Herbal Drug, or Herbal preparation."]
    pub r#parent_substance_name: Vec<super::super::types::String>,
    #[doc = "The country where the plant material is harvested or the countries where the plasma is sourced from as laid down in accordance with the Plasma Master File. For “Plasma-derived substances” the attribute country of origin provides information about the countries used for the manufacturing of the Cryopoor plama or Crioprecipitate."]
    pub r#country_of_origin: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The place/region where the plant is harvested or the places/regions where the animal source material has its habitat."]
    pub r#geographical_location: Vec<super::super::types::String>,
    #[doc = "Stage of life for animals, plants, insects and microorganisms. This information shall be provided only when the substance is significantly different in these stages (e.g. foetal bovine serum)."]
    pub r#development_stage: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Many complex materials are fractions of parts of plants, animals, or minerals. Fraction elements are often necessary to define both Substances and Specified Group 1 Substances. For substances derived from Plants, fraction information will be captured at the Substance information level ( . Oils, Juices and Exudates). Additional information for Extracts, such as extraction solvent composition, will be captured at the Specified Substance Group 1 information level. For plasma-derived products fraction information will be captured at the Substance and the Specified Substance Group 1 levels."]
    pub r#fraction_description: Vec<SubstanceSourceMaterialFractionDescription>,
    #[doc = "This subclause describes the organism which the substance is derived from. For vaccines, the parent organism shall be specified based on these subclause elements. As an example, full taxonomy will be described for the Substance Name: ., Leaf."]
    pub r#organism: Option<SubstanceSourceMaterialOrganism>,
    #[doc = "To do."]
    pub r#part_description: Vec<SubstanceSourceMaterialPartDescription>,
}
impl crate::AnyResource for SubstanceSourceMaterial {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for SubstanceSourceMaterial {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "SubstanceSourceMaterial")?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#organism_name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("organismName", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_organismName", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#organism_name.as_ref() {
                    state.serialize_entry("organismName", some)?;
                }
            }
            if !self.r#parent_substance_id.is_empty() {
                state.serialize_entry("parentSubstanceId", &self.r#parent_substance_id)?;
            }
            if _ctx.output_json {
                if !self.r#parent_substance_name.is_empty() {
                    let values = self
                        .r#parent_substance_name
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#parent_substance_name.is_empty() {
                    state.serialize_entry("parentSubstanceName", &self.r#parent_substance_name)?;
                }
            }
            if !self.r#country_of_origin.is_empty() {
                state.serialize_entry("countryOfOrigin", &self.r#country_of_origin)?;
            }
            if _ctx.output_json {
                if !self.r#geographical_location.is_empty() {
                    let values = self
                        .r#geographical_location
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#geographical_location.is_empty() {
                    state.serialize_entry("geographicalLocation", &self.r#geographical_location)?;
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
        })
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
            Unknown(String),
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
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ImplicitRulesPrimitiveElement => {
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
                            }
                            Field::Language => {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "parentSubstanceId",
                                    ));
                                }
                                r#parent_substance_id = Some(map_access.next_value()?);
                            }
                            Field::ParentSubstanceName => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
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
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "countryOfOrigin",
                                    ));
                                }
                                r#country_of_origin = Some(map_access.next_value()?);
                            }
                            Field::GeographicalLocation => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
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
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "developmentStage",
                                    ));
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "partDescription",
                                    ));
                                }
                                r#part_description = Some(map_access.next_value()?);
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
                                        "sourceMaterialClass",
                                        "sourceMaterialType",
                                        "sourceMaterialState",
                                        "organismId",
                                        "organismName",
                                        "parentSubstanceId",
                                        "parentSubstanceName",
                                        "countryOfOrigin",
                                        "geographicalLocation",
                                        "developmentStage",
                                        "fractionDescription",
                                        "organism",
                                        "partDescription",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
