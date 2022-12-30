// Generated on 2022-12-29 by fhirbolt-codegen v0.1.0
#[doc = "The linkages between sugar residues will also be captured."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceNucleicAcidSubunitLinkage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The entity that links the sugar residues together should also be captured for nearly all naturally occurring nucleic acid the linkage is a phosphate group. For many synthetic oligonucleotides phosphorothioate linkages are often seen. Linkage connectivity is assumed to be 3’-5’. If the linkage is either 3’-3’ or 5’-5’ this should be specified."]
    pub r#connectivity: Option<super::super::types::String>,
    #[doc = "Each linkage will be registered as a fragment and have an ID."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Each linkage will be registered as a fragment and have at least one name. A single name shall be assigned to each linkage."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Residues shall be captured as described in 5.3.6.8.3."]
    pub r#residue_site: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SubstanceNucleicAcidSubunitLinkage {
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
                if let Some(some) = self.r#connectivity.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("connectivity", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_connectivity", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#connectivity.as_ref() {
                    state.serialize_entry("connectivity", some)?;
                }
            }
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("name", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_name", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#name.as_ref() {
                    state.serialize_entry("name", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#residue_site.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("residueSite", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_residueSite", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#residue_site.as_ref() {
                    state.serialize_entry("residueSite", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceNucleicAcidSubunitLinkage {
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
            #[serde(rename = "connectivity")]
            Connectivity,
            #[serde(rename = "_connectivity")]
            ConnectivityPrimitiveElement,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "residueSite")]
            ResidueSite,
            #[serde(rename = "_residueSite")]
            ResidueSitePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceNucleicAcidSubunitLinkage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceNucleicAcidSubunitLinkage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceNucleicAcidSubunitLinkage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#connectivity: Option<super::super::types::String> = None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#residue_site: Option<super::super::types::String> = None;
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
                            Field::Connectivity => {
                                if _ctx.from_json {
                                    let some = r#connectivity.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "connectivity",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#connectivity.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "connectivity",
                                        ));
                                    }
                                    r#connectivity = Some(map_access.next_value()?);
                                }
                            }
                            Field::ConnectivityPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#connectivity.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_connectivity",
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
                                        "connectivity",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "connectivity",
                                            "identifier",
                                            "name",
                                            "residueSite",
                                        ],
                                    ));
                                }
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Name => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                }
                            }
                            Field::NamePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_name"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "name",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "connectivity",
                                            "identifier",
                                            "name",
                                            "residueSite",
                                        ],
                                    ));
                                }
                            }
                            Field::ResidueSite => {
                                if _ctx.from_json {
                                    let some = r#residue_site.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "residueSite",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#residue_site.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "residueSite",
                                        ));
                                    }
                                    r#residue_site = Some(map_access.next_value()?);
                                }
                            }
                            Field::ResidueSitePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#residue_site.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_residueSite",
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
                                        "residueSite",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "connectivity",
                                            "identifier",
                                            "name",
                                            "residueSite",
                                        ],
                                    ));
                                }
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
                                        "connectivity",
                                        "identifier",
                                        "name",
                                        "residueSite",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceNucleicAcidSubunitLinkage {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#connectivity,
                        r#identifier,
                        r#name,
                        r#residue_site,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "5.3.6.8.1 Sugar ID (Mandatory)."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceNucleicAcidSubunitSugar {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The Substance ID of the sugar or sugar-like component that make up the nucleotide."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The name of the sugar or sugar-like component that make up the nucleotide."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The residues that contain a given sugar will be captured. The order of given residues will be captured in the 5‘-3‘direction consistent with the base sequences listed above."]
    pub r#residue_site: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SubstanceNucleicAcidSubunitSugar {
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
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("name", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_name", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#name.as_ref() {
                    state.serialize_entry("name", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#residue_site.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("residueSite", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_residueSite", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#residue_site.as_ref() {
                    state.serialize_entry("residueSite", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceNucleicAcidSubunitSugar {
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
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "residueSite")]
            ResidueSite,
            #[serde(rename = "_residueSite")]
            ResidueSitePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceNucleicAcidSubunitSugar;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceNucleicAcidSubunitSugar")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceNucleicAcidSubunitSugar, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#residue_site: Option<super::super::types::String> = None;
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
                            Field::Name => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                }
                            }
                            Field::NamePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_name"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "name",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "name",
                                            "residueSite",
                                        ],
                                    ));
                                }
                            }
                            Field::ResidueSite => {
                                if _ctx.from_json {
                                    let some = r#residue_site.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "residueSite",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#residue_site.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "residueSite",
                                        ));
                                    }
                                    r#residue_site = Some(map_access.next_value()?);
                                }
                            }
                            Field::ResidueSitePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#residue_site.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_residueSite",
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
                                        "residueSite",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "name",
                                            "residueSite",
                                        ],
                                    ));
                                }
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
                                        "name",
                                        "residueSite",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceNucleicAcidSubunitSugar {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier,
                        r#name,
                        r#residue_site,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Subunits are listed in order of decreasing length; sequences of the same length will be ordered by molecular weight; subunits that have identical sequences will be repeated multiple times."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceNucleicAcidSubunit {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Index of linear sequences of nucleic acids in order of decreasing length. Sequences of the same length will be ordered by molecular weight. Subunits that have identical sequences will be repeated and have sequential subscripts."]
    pub r#subunit: Option<super::super::types::Integer>,
    #[doc = "Actual nucleotide sequence notation from 5' to 3' end using standard single letter codes. In addition to the base sequence, sugar and type of phosphate or non-phosphate linkage should also be captured."]
    pub r#sequence: Option<super::super::types::String>,
    #[doc = "The length of the sequence shall be captured."]
    pub r#length: Option<super::super::types::Integer>,
    #[doc = "(TBC)."]
    pub r#sequence_attachment: Option<Box<super::super::types::Attachment>>,
    #[doc = "The nucleotide present at the 5’ terminal shall be specified based on a controlled vocabulary. Since the sequence is represented from the 5' to the 3' end, the 5’ prime nucleotide is the letter at the first position in the sequence. A separate representation would be redundant."]
    pub r#five_prime: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The nucleotide present at the 3’ terminal shall be specified based on a controlled vocabulary. Since the sequence is represented from the 5' to the 3' end, the 5’ prime nucleotide is the letter at the last position in the sequence. A separate representation would be redundant."]
    pub r#three_prime: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The linkages between sugar residues will also be captured."]
    pub r#linkage: Vec<SubstanceNucleicAcidSubunitLinkage>,
    #[doc = "5.3.6.8.1 Sugar ID (Mandatory)."]
    pub r#sugar: Vec<SubstanceNucleicAcidSubunitSugar>,
}
impl serde::ser::Serialize for SubstanceNucleicAcidSubunit {
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
                if let Some(some) = self.r#subunit.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("subunit", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_subunit", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#subunit.as_ref() {
                    state.serialize_entry("subunit", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#sequence.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sequence", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sequence", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#sequence.as_ref() {
                    state.serialize_entry("sequence", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#length.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("length", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_length", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#length.as_ref() {
                    state.serialize_entry("length", some)?;
                }
            }
            if let Some(some) = self.r#sequence_attachment.as_ref() {
                state.serialize_entry("sequenceAttachment", some)?;
            }
            if let Some(some) = self.r#five_prime.as_ref() {
                state.serialize_entry("fivePrime", some)?;
            }
            if let Some(some) = self.r#three_prime.as_ref() {
                state.serialize_entry("threePrime", some)?;
            }
            if !self.r#linkage.is_empty() {
                state.serialize_entry("linkage", &self.r#linkage)?;
            }
            if !self.r#sugar.is_empty() {
                state.serialize_entry("sugar", &self.r#sugar)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceNucleicAcidSubunit {
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
            #[serde(rename = "subunit")]
            Subunit,
            #[serde(rename = "_subunit")]
            SubunitPrimitiveElement,
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "length")]
            Length,
            #[serde(rename = "_length")]
            LengthPrimitiveElement,
            #[serde(rename = "sequenceAttachment")]
            SequenceAttachment,
            #[serde(rename = "fivePrime")]
            FivePrime,
            #[serde(rename = "threePrime")]
            ThreePrime,
            #[serde(rename = "linkage")]
            Linkage,
            #[serde(rename = "sugar")]
            Sugar,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceNucleicAcidSubunit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceNucleicAcidSubunit")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceNucleicAcidSubunit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#subunit: Option<super::super::types::Integer> = None;
                let mut r#sequence: Option<super::super::types::String> = None;
                let mut r#length: Option<super::super::types::Integer> = None;
                let mut r#sequence_attachment: Option<Box<super::super::types::Attachment>> = None;
                let mut r#five_prime: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#three_prime: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#linkage: Option<Vec<SubstanceNucleicAcidSubunitLinkage>> = None;
                let mut r#sugar: Option<Vec<SubstanceNucleicAcidSubunitSugar>> = None;
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
                            Field::Subunit => {
                                if _ctx.from_json {
                                    let some = r#subunit.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("subunit"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#subunit.is_some() {
                                        return Err(serde::de::Error::duplicate_field("subunit"));
                                    }
                                    r#subunit = Some(map_access.next_value()?);
                                }
                            }
                            Field::SubunitPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#subunit.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_subunit"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "subunit",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "subunit",
                                            "sequence",
                                            "length",
                                            "sequenceAttachment",
                                            "fivePrime",
                                            "threePrime",
                                            "linkage",
                                            "sugar",
                                        ],
                                    ));
                                }
                            }
                            Field::Sequence => {
                                if _ctx.from_json {
                                    let some = r#sequence.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    r#sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::SequencePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#sequence.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_sequence"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "subunit",
                                            "sequence",
                                            "length",
                                            "sequenceAttachment",
                                            "fivePrime",
                                            "threePrime",
                                            "linkage",
                                            "sugar",
                                        ],
                                    ));
                                }
                            }
                            Field::Length => {
                                if _ctx.from_json {
                                    let some = r#length.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("length"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#length.is_some() {
                                        return Err(serde::de::Error::duplicate_field("length"));
                                    }
                                    r#length = Some(map_access.next_value()?);
                                }
                            }
                            Field::LengthPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#length.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_length"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "length",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "subunit",
                                            "sequence",
                                            "length",
                                            "sequenceAttachment",
                                            "fivePrime",
                                            "threePrime",
                                            "linkage",
                                            "sugar",
                                        ],
                                    ));
                                }
                            }
                            Field::SequenceAttachment => {
                                if r#sequence_attachment.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "sequenceAttachment",
                                    ));
                                }
                                r#sequence_attachment = Some(map_access.next_value()?);
                            }
                            Field::FivePrime => {
                                if r#five_prime.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fivePrime"));
                                }
                                r#five_prime = Some(map_access.next_value()?);
                            }
                            Field::ThreePrime => {
                                if r#three_prime.is_some() {
                                    return Err(serde::de::Error::duplicate_field("threePrime"));
                                }
                                r#three_prime = Some(map_access.next_value()?);
                            }
                            Field::Linkage => {
                                if r#linkage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("linkage"));
                                }
                                r#linkage = Some(map_access.next_value()?);
                            }
                            Field::Sugar => {
                                if r#sugar.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sugar"));
                                }
                                r#sugar = Some(map_access.next_value()?);
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
                                        "subunit",
                                        "sequence",
                                        "length",
                                        "sequenceAttachment",
                                        "fivePrime",
                                        "threePrime",
                                        "linkage",
                                        "sugar",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceNucleicAcidSubunit {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#subunit,
                        r#sequence,
                        r#length,
                        r#sequence_attachment,
                        r#five_prime,
                        r#three_prime,
                        r#linkage: r#linkage.unwrap_or(vec![]),
                        r#sugar: r#sugar.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Nucleic acids are defined by three distinct elements: the base, sugar and linkage. Individual substance/moiety IDs will be created for each of these elements. The nucleotide sequence will be always entered in the 5’-3’ direction."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceNucleicAcid {
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
    #[doc = "The type of the sequence shall be specified based on a controlled vocabulary."]
    pub r#sequence_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of linear sequences of nucleotides linked through phosphodiester bonds shall be described. Subunits would be strands of nucleic acids that are tightly associated typically through Watson-Crick base pairing. NOTE: If not specified in the reference source, the assumption is that there is 1 subunit."]
    pub r#number_of_subunits: Option<super::super::types::Integer>,
    #[doc = "The area of hybridisation shall be described if applicable for double stranded RNA or DNA. The number associated with the subunit followed by the number associated to the residue shall be specified in increasing order. The underscore “” shall be used as separator as follows: “Subunitnumber Residue”."]
    pub r#area_of_hybridisation: Option<super::super::types::String>,
    #[doc = "(TBC)."]
    pub r#oligo_nucleotide_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Subunits are listed in order of decreasing length; sequences of the same length will be ordered by molecular weight; subunits that have identical sequences will be repeated multiple times."]
    pub r#subunit: Vec<SubstanceNucleicAcidSubunit>,
}
impl crate::AnyResource for SubstanceNucleicAcid {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for SubstanceNucleicAcid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "SubstanceNucleicAcid")?;
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
            if let Some(some) = self.r#sequence_type.as_ref() {
                state.serialize_entry("sequenceType", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number_of_subunits.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("numberOfSubunits", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_numberOfSubunits", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number_of_subunits.as_ref() {
                    state.serialize_entry("numberOfSubunits", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#area_of_hybridisation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("areaOfHybridisation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_areaOfHybridisation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#area_of_hybridisation.as_ref() {
                    state.serialize_entry("areaOfHybridisation", some)?;
                }
            }
            if let Some(some) = self.r#oligo_nucleotide_type.as_ref() {
                state.serialize_entry("oligoNucleotideType", some)?;
            }
            if !self.r#subunit.is_empty() {
                state.serialize_entry("subunit", &self.r#subunit)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceNucleicAcid {
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
            #[serde(rename = "sequenceType")]
            SequenceType,
            #[serde(rename = "numberOfSubunits")]
            NumberOfSubunits,
            #[serde(rename = "_numberOfSubunits")]
            NumberOfSubunitsPrimitiveElement,
            #[serde(rename = "areaOfHybridisation")]
            AreaOfHybridisation,
            #[serde(rename = "_areaOfHybridisation")]
            AreaOfHybridisationPrimitiveElement,
            #[serde(rename = "oligoNucleotideType")]
            OligoNucleotideType,
            #[serde(rename = "subunit")]
            Subunit,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceNucleicAcid;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceNucleicAcid")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceNucleicAcid, V::Error>
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
                let mut r#sequence_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#number_of_subunits: Option<super::super::types::Integer> = None;
                let mut r#area_of_hybridisation: Option<super::super::types::String> = None;
                let mut r#oligo_nucleotide_type: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#subunit: Option<Vec<SubstanceNucleicAcidSubunit>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "SubstanceNucleicAcid" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"SubstanceNucleicAcid",
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
                                            "sequenceType",
                                            "numberOfSubunits",
                                            "areaOfHybridisation",
                                            "oligoNucleotideType",
                                            "subunit",
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
                                            "sequenceType",
                                            "numberOfSubunits",
                                            "areaOfHybridisation",
                                            "oligoNucleotideType",
                                            "subunit",
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
                            Field::SequenceType => {
                                if r#sequence_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequenceType"));
                                }
                                r#sequence_type = Some(map_access.next_value()?);
                            }
                            Field::NumberOfSubunits => {
                                if _ctx.from_json {
                                    let some =
                                        r#number_of_subunits.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfSubunits",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#number_of_subunits.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfSubunits",
                                        ));
                                    }
                                    r#number_of_subunits = Some(map_access.next_value()?);
                                }
                            }
                            Field::NumberOfSubunitsPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#number_of_subunits.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_numberOfSubunits",
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
                                        "numberOfSubunits",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "sequenceType",
                                            "numberOfSubunits",
                                            "areaOfHybridisation",
                                            "oligoNucleotideType",
                                            "subunit",
                                        ],
                                    ));
                                }
                            }
                            Field::AreaOfHybridisation => {
                                if _ctx.from_json {
                                    let some =
                                        r#area_of_hybridisation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "areaOfHybridisation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#area_of_hybridisation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "areaOfHybridisation",
                                        ));
                                    }
                                    r#area_of_hybridisation = Some(map_access.next_value()?);
                                }
                            }
                            Field::AreaOfHybridisationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#area_of_hybridisation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_areaOfHybridisation",
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
                                        "areaOfHybridisation",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "sequenceType",
                                            "numberOfSubunits",
                                            "areaOfHybridisation",
                                            "oligoNucleotideType",
                                            "subunit",
                                        ],
                                    ));
                                }
                            }
                            Field::OligoNucleotideType => {
                                if r#oligo_nucleotide_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "oligoNucleotideType",
                                    ));
                                }
                                r#oligo_nucleotide_type = Some(map_access.next_value()?);
                            }
                            Field::Subunit => {
                                if r#subunit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subunit"));
                                }
                                r#subunit = Some(map_access.next_value()?);
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
                                        "sequenceType",
                                        "numberOfSubunits",
                                        "areaOfHybridisation",
                                        "oligoNucleotideType",
                                        "subunit",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceNucleicAcid {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence_type,
                        r#number_of_subunits,
                        r#area_of_hybridisation,
                        r#oligo_nucleotide_type,
                        r#subunit: r#subunit.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
