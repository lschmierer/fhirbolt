// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct SubstanceNucleicAcidSubunitLinkage {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#connectivity: Option<super::super::types::String>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#name: Option<super::super::types::String>,
    pub r#residue_site: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SubstanceNucleicAcidSubunitLinkage {
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
        if let Some(some) = self.r#connectivity.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("connectivity", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_connectivity", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#residue_site.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("residueSite", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_residueSite", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceNucleicAcidSubunitLinkage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                        "connectivity" => {
                            let some = r#connectivity.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectivity"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_connectivity" => {
                            let some = r#connectivity.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_connectivity"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_name" => {
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
                        }
                        "residueSite" => {
                            let some = r#residue_site.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("residueSite"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_residueSite" => {
                            let some = r#residue_site.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_residueSite"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "connectivity",
                                    "identifier",
                                    "name",
                                    "residue_site",
                                ],
                            ))
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceNucleicAcidSubunitSugar {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#name: Option<super::super::types::String>,
    pub r#residue_site: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SubstanceNucleicAcidSubunitSugar {
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
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#residue_site.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("residueSite", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_residueSite", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceNucleicAcidSubunitSugar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                        "name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_name" => {
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
                        }
                        "residueSite" => {
                            let some = r#residue_site.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("residueSite"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_residueSite" => {
                            let some = r#residue_site.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_residueSite"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "identifier",
                                    "name",
                                    "residue_site",
                                ],
                            ))
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceNucleicAcidSubunit {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#subunit: Option<super::super::types::Integer>,
    pub r#sequence: Option<super::super::types::String>,
    pub r#length: Option<super::super::types::Integer>,
    pub r#sequence_attachment: Option<Box<super::super::types::Attachment>>,
    pub r#five_prime: Option<Box<super::super::types::CodeableConcept>>,
    pub r#three_prime: Option<Box<super::super::types::CodeableConcept>>,
    pub r#linkage: Vec<SubstanceNucleicAcidSubunitLinkage>,
    pub r#sugar: Vec<SubstanceNucleicAcidSubunitSugar>,
}
impl serde::ser::Serialize for SubstanceNucleicAcidSubunit {
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
        if let Some(some) = self.r#subunit.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("subunit", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_subunit", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#sequence.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("sequence", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_sequence", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#length.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("length", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_length", &primitive_element)?;
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
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceNucleicAcidSubunit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                        "subunit" => {
                            let some = r#subunit.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("subunit"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_subunit" => {
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
                        }
                        "sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_sequence" => {
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
                        }
                        "length" => {
                            let some = r#length.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_length" => {
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
                        }
                        "sequenceAttachment" => {
                            if r#sequence_attachment.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sequenceAttachment",
                                ));
                            }
                            r#sequence_attachment = Some(map_access.next_value()?);
                        }
                        "fivePrime" => {
                            if r#five_prime.is_some() {
                                return Err(serde::de::Error::duplicate_field("fivePrime"));
                            }
                            r#five_prime = Some(map_access.next_value()?);
                        }
                        "threePrime" => {
                            if r#three_prime.is_some() {
                                return Err(serde::de::Error::duplicate_field("threePrime"));
                            }
                            r#three_prime = Some(map_access.next_value()?);
                        }
                        "linkage" => {
                            if r#linkage.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkage"));
                            }
                            r#linkage = Some(map_access.next_value()?);
                        }
                        "sugar" => {
                            if r#sugar.is_some() {
                                return Err(serde::de::Error::duplicate_field("sugar"));
                            }
                            r#sugar = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "subunit",
                                    "sequence",
                                    "length",
                                    "sequence_attachment",
                                    "five_prime",
                                    "three_prime",
                                    "linkage",
                                    "sugar",
                                ],
                            ))
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceNucleicAcid {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#number_of_subunits: Option<super::super::types::Integer>,
    pub r#area_of_hybridisation: Option<super::super::types::String>,
    pub r#oligo_nucleotide_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subunit: Vec<SubstanceNucleicAcidSubunit>,
}
impl serde::ser::Serialize for SubstanceNucleicAcid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SubstanceNucleicAcid")?;
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
        if let Some(some) = self.r#sequence_type.as_ref() {
            state.serialize_entry("sequenceType", some)?;
        }
        if let Some(some) = self.r#number_of_subunits.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("numberOfSubunits", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_numberOfSubunits", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#area_of_hybridisation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("areaOfHybridisation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_areaOfHybridisation", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#oligo_nucleotide_type.as_ref() {
            state.serialize_entry("oligoNucleotideType", some)?;
        }
        if !self.r#subunit.is_empty() {
            state.serialize_entry("subunit", &self.r#subunit)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceNucleicAcid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#number_of_subunits: Option<super::super::types::Integer> = None;
                let mut r#area_of_hybridisation: Option<super::super::types::String> = None;
                let mut r#oligo_nucleotide_type: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#subunit: Option<Vec<SubstanceNucleicAcidSubunit>> = None;
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
                        "sequenceType" => {
                            if r#sequence_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequenceType"));
                            }
                            r#sequence_type = Some(map_access.next_value()?);
                        }
                        "numberOfSubunits" => {
                            let some = r#number_of_subunits.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberOfSubunits"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_numberOfSubunits" => {
                            let some = r#number_of_subunits.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_numberOfSubunits"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "areaOfHybridisation" => {
                            let some = r#area_of_hybridisation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "areaOfHybridisation",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_areaOfHybridisation" => {
                            let some = r#area_of_hybridisation.get_or_insert(Default::default());
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
                        }
                        "oligoNucleotideType" => {
                            if r#oligo_nucleotide_type.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "oligoNucleotideType",
                                ));
                            }
                            r#oligo_nucleotide_type = Some(map_access.next_value()?);
                        }
                        "subunit" => {
                            if r#subunit.is_some() {
                                return Err(serde::de::Error::duplicate_field("subunit"));
                            }
                            r#subunit = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "meta",
                                    "implicit_rules",
                                    "language",
                                    "text",
                                    "contained",
                                    "extension",
                                    "modifier_extension",
                                    "sequence_type",
                                    "number_of_subunits",
                                    "area_of_hybridisation",
                                    "oligo_nucleotide_type",
                                    "subunit",
                                ],
                            ))
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
