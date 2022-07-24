// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum SpecimenDefinitionTypeTestedContainerMinimumVolume {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SpecimenDefinitionTypeTestedContainerMinimumVolume {
    fn default() -> SpecimenDefinitionTypeTestedContainerMinimumVolume {
        SpecimenDefinitionTypeTestedContainerMinimumVolume::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
    fn default() -> SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
        SpecimenDefinitionTypeTestedContainerAdditiveAdditive::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct SpecimenDefinitionTypeTestedContainerAdditive {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#additive: SpecimenDefinitionTypeTestedContainerAdditiveAdditive,
}
impl serde::ser::Serialize for SpecimenDefinitionTypeTestedContainerAdditive {
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
        match self.r#additive {
            SpecimenDefinitionTypeTestedContainerAdditiveAdditive::CodeableConcept(ref value) => {
                state.serialize_entry("additiveCodeableConcept", value)?;
            }
            SpecimenDefinitionTypeTestedContainerAdditiveAdditive::Reference(ref value) => {
                state.serialize_entry("additiveReference", value)?;
            }
            SpecimenDefinitionTypeTestedContainerAdditiveAdditive::Invalid => {
                return Err(serde::ser::Error::custom("additive is a required field"))
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenDefinitionTypeTestedContainerAdditive {
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
            #[serde(rename = "additiveCodeableConcept")]
            AdditiveCodeableConcept,
            #[serde(rename = "additiveReference")]
            AdditiveReference,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenDefinitionTypeTestedContainerAdditive;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinitionTypeTestedContainerAdditive")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SpecimenDefinitionTypeTestedContainerAdditive, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#additive: Option<SpecimenDefinitionTypeTestedContainerAdditiveAdditive> =
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
                        Field::AdditiveCodeableConcept => {
                            if r#additive.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "additiveCodeableConcept",
                                ));
                            }
                            r#additive = Some (SpecimenDefinitionTypeTestedContainerAdditiveAdditive :: CodeableConcept (map_access . next_value () ?)) ;
                        }
                        Field::AdditiveReference => {
                            if r#additive.is_some() {
                                return Err(serde::de::Error::duplicate_field("additiveReference"));
                            }
                            r#additive = Some(
                                SpecimenDefinitionTypeTestedContainerAdditiveAdditive::Reference(
                                    map_access.next_value()?,
                                ),
                            );
                        }
                    }
                }
                Ok(SpecimenDefinitionTypeTestedContainerAdditive {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#additive: r#additive.ok_or(serde::de::Error::missing_field("additive"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SpecimenDefinitionTypeTestedContainer {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#material: Option<Box<super::super::types::CodeableConcept>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#cap: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#capacity: Option<Box<super::super::types::Quantity>>,
    pub r#minimum_volume: Option<SpecimenDefinitionTypeTestedContainerMinimumVolume>,
    pub r#additive: Vec<SpecimenDefinitionTypeTestedContainerAdditive>,
    pub r#preparation: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SpecimenDefinitionTypeTestedContainer {
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
        if let Some(some) = self.r#material.as_ref() {
            state.serialize_entry("material", some)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#cap.as_ref() {
            state.serialize_entry("cap", some)?;
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
        if let Some(some) = self.r#capacity.as_ref() {
            state.serialize_entry("capacity", some)?;
        }
        if let Some(some) = self.r#minimum_volume.as_ref() {
            match some {
                SpecimenDefinitionTypeTestedContainerMinimumVolume::Quantity(ref value) => {
                    state.serialize_entry("minimumVolumeQuantity", value)?;
                }
                SpecimenDefinitionTypeTestedContainerMinimumVolume::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("minimumVolumeString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_minimumVolumeString", &primitive_element)?;
                    }
                }
                SpecimenDefinitionTypeTestedContainerMinimumVolume::Invalid => {
                    return Err(serde::ser::Error::custom("minimum_volume is invalid"))
                }
            }
        }
        if !self.r#additive.is_empty() {
            state.serialize_entry("additive", &self.r#additive)?;
        }
        if let Some(some) = self.r#preparation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("preparation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_preparation", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenDefinitionTypeTestedContainer {
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
            #[serde(rename = "material")]
            Material,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "cap")]
            Cap,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "capacity")]
            Capacity,
            #[serde(rename = "minimumVolumeQuantity")]
            MinimumVolumeQuantity,
            #[serde(rename = "minimumVolumeString")]
            MinimumVolumeString,
            #[serde(rename = "_minimumVolumeString")]
            MinimumVolumeStringPrimitiveElement,
            #[serde(rename = "additive")]
            Additive,
            #[serde(rename = "preparation")]
            Preparation,
            #[serde(rename = "_preparation")]
            PreparationPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenDefinitionTypeTestedContainer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinitionTypeTestedContainer")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SpecimenDefinitionTypeTestedContainer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#material: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#cap: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#capacity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#minimum_volume: Option<
                    SpecimenDefinitionTypeTestedContainerMinimumVolume,
                > = None;
                let mut r#additive: Option<Vec<SpecimenDefinitionTypeTestedContainerAdditive>> =
                    None;
                let mut r#preparation: Option<super::super::types::String> = None;
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
                        Field::Material => {
                            if r#material.is_some() {
                                return Err(serde::de::Error::duplicate_field("material"));
                            }
                            r#material = Some(map_access.next_value()?);
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Cap => {
                            if r#cap.is_some() {
                                return Err(serde::de::Error::duplicate_field("cap"));
                            }
                            r#cap = Some(map_access.next_value()?);
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DescriptionPrimitiveElement => {
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
                        Field::Capacity => {
                            if r#capacity.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacity"));
                            }
                            r#capacity = Some(map_access.next_value()?);
                        }
                        Field::MinimumVolumeQuantity => {
                            if r#minimum_volume.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimumVolumeQuantity",
                                ));
                            }
                            r#minimum_volume = Some(
                                SpecimenDefinitionTypeTestedContainerMinimumVolume::Quantity(
                                    map_access.next_value()?,
                                ),
                            );
                        }
                        Field::MinimumVolumeString => {
                            let r#enum = r#minimum_volume.get_or_insert(
                                SpecimenDefinitionTypeTestedContainerMinimumVolume::String(
                                    Default::default(),
                                ),
                            );
                            if let SpecimenDefinitionTypeTestedContainerMinimumVolume::String(
                                variant,
                            ) = r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minimumVolumeString",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("minimumVolume[x]"));
                            }
                        }
                        Field::MinimumVolumeStringPrimitiveElement => {
                            let r#enum = r#minimum_volume.get_or_insert(
                                SpecimenDefinitionTypeTestedContainerMinimumVolume::String(
                                    Default::default(),
                                ),
                            );
                            if let SpecimenDefinitionTypeTestedContainerMinimumVolume::String(
                                variant,
                            ) = r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_minimumVolumeString",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_minimumVolume[x]"));
                            }
                        }
                        Field::Additive => {
                            if r#additive.is_some() {
                                return Err(serde::de::Error::duplicate_field("additive"));
                            }
                            r#additive = Some(map_access.next_value()?);
                        }
                        Field::Preparation => {
                            let some = r#preparation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("preparation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PreparationPrimitiveElement => {
                            let some = r#preparation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_preparation"));
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
                Ok(SpecimenDefinitionTypeTestedContainer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#material,
                    r#type,
                    r#cap,
                    r#description,
                    r#capacity,
                    r#minimum_volume,
                    r#additive: r#additive.unwrap_or(vec![]),
                    r#preparation,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SpecimenDefinitionTypeTestedHandling {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#temperature_qualifier: Option<Box<super::super::types::CodeableConcept>>,
    pub r#temperature_range: Option<Box<super::super::types::Range>>,
    pub r#max_duration: Option<Box<super::super::types::Duration>>,
    pub r#instruction: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SpecimenDefinitionTypeTestedHandling {
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
        if let Some(some) = self.r#temperature_qualifier.as_ref() {
            state.serialize_entry("temperatureQualifier", some)?;
        }
        if let Some(some) = self.r#temperature_range.as_ref() {
            state.serialize_entry("temperatureRange", some)?;
        }
        if let Some(some) = self.r#max_duration.as_ref() {
            state.serialize_entry("maxDuration", some)?;
        }
        if let Some(some) = self.r#instruction.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("instruction", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_instruction", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenDefinitionTypeTestedHandling {
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
            #[serde(rename = "temperatureQualifier")]
            TemperatureQualifier,
            #[serde(rename = "temperatureRange")]
            TemperatureRange,
            #[serde(rename = "maxDuration")]
            MaxDuration,
            #[serde(rename = "instruction")]
            Instruction,
            #[serde(rename = "_instruction")]
            InstructionPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenDefinitionTypeTestedHandling;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinitionTypeTestedHandling")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SpecimenDefinitionTypeTestedHandling, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#temperature_qualifier: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#temperature_range: Option<Box<super::super::types::Range>> = None;
                let mut r#max_duration: Option<Box<super::super::types::Duration>> = None;
                let mut r#instruction: Option<super::super::types::String> = None;
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
                        Field::TemperatureQualifier => {
                            if r#temperature_qualifier.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "temperatureQualifier",
                                ));
                            }
                            r#temperature_qualifier = Some(map_access.next_value()?);
                        }
                        Field::TemperatureRange => {
                            if r#temperature_range.is_some() {
                                return Err(serde::de::Error::duplicate_field("temperatureRange"));
                            }
                            r#temperature_range = Some(map_access.next_value()?);
                        }
                        Field::MaxDuration => {
                            if r#max_duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDuration"));
                            }
                            r#max_duration = Some(map_access.next_value()?);
                        }
                        Field::Instruction => {
                            let some = r#instruction.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("instruction"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::InstructionPrimitiveElement => {
                            let some = r#instruction.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_instruction"));
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
                Ok(SpecimenDefinitionTypeTestedHandling {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#temperature_qualifier,
                    r#temperature_range,
                    r#max_duration,
                    r#instruction,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SpecimenDefinitionTypeTested {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#is_derived: Option<super::super::types::Boolean>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#preference: super::super::types::Code,
    pub r#container: Option<SpecimenDefinitionTypeTestedContainer>,
    pub r#requirement: Option<super::super::types::String>,
    pub r#retention_time: Option<Box<super::super::types::Duration>>,
    pub r#rejection_criterion: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#handling: Vec<SpecimenDefinitionTypeTestedHandling>,
}
impl serde::ser::Serialize for SpecimenDefinitionTypeTested {
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
        if let Some(some) = self.r#is_derived.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isDerived", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isDerived", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#preference.value.as_ref() {
            state.serialize_entry("preference", some)?;
        }
        if self.r#preference.id.is_some() || !self.r#preference.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#preference.id,
                extension: &self.r#preference.extension,
            };
            state.serialize_entry("_preference", &primitive_element)?;
        }
        if let Some(some) = self.r#container.as_ref() {
            state.serialize_entry("container", some)?;
        }
        if let Some(some) = self.r#requirement.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("requirement", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_requirement", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#retention_time.as_ref() {
            state.serialize_entry("retentionTime", some)?;
        }
        if !self.r#rejection_criterion.is_empty() {
            state.serialize_entry("rejectionCriterion", &self.r#rejection_criterion)?;
        }
        if !self.r#handling.is_empty() {
            state.serialize_entry("handling", &self.r#handling)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenDefinitionTypeTested {
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
            #[serde(rename = "isDerived")]
            IsDerived,
            #[serde(rename = "_isDerived")]
            IsDerivedPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "preference")]
            Preference,
            #[serde(rename = "_preference")]
            PreferencePrimitiveElement,
            #[serde(rename = "container")]
            Container,
            #[serde(rename = "requirement")]
            Requirement,
            #[serde(rename = "_requirement")]
            RequirementPrimitiveElement,
            #[serde(rename = "retentionTime")]
            RetentionTime,
            #[serde(rename = "rejectionCriterion")]
            RejectionCriterion,
            #[serde(rename = "handling")]
            Handling,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenDefinitionTypeTested;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinitionTypeTested")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SpecimenDefinitionTypeTested, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#is_derived: Option<super::super::types::Boolean> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#preference: Option<super::super::types::Code> = None;
                let mut r#container: Option<SpecimenDefinitionTypeTestedContainer> = None;
                let mut r#requirement: Option<super::super::types::String> = None;
                let mut r#retention_time: Option<Box<super::super::types::Duration>> = None;
                let mut r#rejection_criterion: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#handling: Option<Vec<SpecimenDefinitionTypeTestedHandling>> = None;
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
                        Field::IsDerived => {
                            let some = r#is_derived.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDerived"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IsDerivedPrimitiveElement => {
                            let some = r#is_derived.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_isDerived"));
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
                        Field::Preference => {
                            let some = r#preference.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("preference"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PreferencePrimitiveElement => {
                            let some = r#preference.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_preference"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Container => {
                            if r#container.is_some() {
                                return Err(serde::de::Error::duplicate_field("container"));
                            }
                            r#container = Some(map_access.next_value()?);
                        }
                        Field::Requirement => {
                            let some = r#requirement.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirement"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RequirementPrimitiveElement => {
                            let some = r#requirement.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_requirement"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::RetentionTime => {
                            if r#retention_time.is_some() {
                                return Err(serde::de::Error::duplicate_field("retentionTime"));
                            }
                            r#retention_time = Some(map_access.next_value()?);
                        }
                        Field::RejectionCriterion => {
                            if r#rejection_criterion.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "rejectionCriterion",
                                ));
                            }
                            r#rejection_criterion = Some(map_access.next_value()?);
                        }
                        Field::Handling => {
                            if r#handling.is_some() {
                                return Err(serde::de::Error::duplicate_field("handling"));
                            }
                            r#handling = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(SpecimenDefinitionTypeTested {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#is_derived,
                    r#type,
                    r#preference: r#preference
                        .ok_or(serde::de::Error::missing_field("preference"))?,
                    r#container,
                    r#requirement,
                    r#retention_time,
                    r#rejection_criterion: r#rejection_criterion.unwrap_or(vec![]),
                    r#handling: r#handling.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SpecimenDefinition {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#type_collected: Option<Box<super::super::types::CodeableConcept>>,
    pub r#patient_preparation: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#time_aspect: Option<super::super::types::String>,
    pub r#collection: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#type_tested: Vec<SpecimenDefinitionTypeTested>,
}
impl serde::ser::Serialize for SpecimenDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SpecimenDefinition")?;
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
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        if let Some(some) = self.r#type_collected.as_ref() {
            state.serialize_entry("typeCollected", some)?;
        }
        if !self.r#patient_preparation.is_empty() {
            state.serialize_entry("patientPreparation", &self.r#patient_preparation)?;
        }
        if let Some(some) = self.r#time_aspect.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("timeAspect", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_timeAspect", &primitive_element)?;
            }
        }
        if !self.r#collection.is_empty() {
            state.serialize_entry("collection", &self.r#collection)?;
        }
        if !self.r#type_tested.is_empty() {
            state.serialize_entry("typeTested", &self.r#type_tested)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenDefinition {
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
            #[serde(rename = "typeCollected")]
            TypeCollected,
            #[serde(rename = "patientPreparation")]
            PatientPreparation,
            #[serde(rename = "timeAspect")]
            TimeAspect,
            #[serde(rename = "_timeAspect")]
            TimeAspectPrimitiveElement,
            #[serde(rename = "collection")]
            Collection,
            #[serde(rename = "typeTested")]
            TypeTested,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SpecimenDefinition, V::Error>
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
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#type_collected: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#patient_preparation: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#time_aspect: Option<super::super::types::String> = None;
                let mut r#collection: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#type_tested: Option<Vec<SpecimenDefinitionTypeTested>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "SpecimenDefinition" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"SpecimenDefinition",
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
                        Field::TypeCollected => {
                            if r#type_collected.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeCollected"));
                            }
                            r#type_collected = Some(map_access.next_value()?);
                        }
                        Field::PatientPreparation => {
                            if r#patient_preparation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patientPreparation",
                                ));
                            }
                            r#patient_preparation = Some(map_access.next_value()?);
                        }
                        Field::TimeAspect => {
                            let some = r#time_aspect.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeAspect"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TimeAspectPrimitiveElement => {
                            let some = r#time_aspect.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_timeAspect"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Collection => {
                            if r#collection.is_some() {
                                return Err(serde::de::Error::duplicate_field("collection"));
                            }
                            r#collection = Some(map_access.next_value()?);
                        }
                        Field::TypeTested => {
                            if r#type_tested.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeTested"));
                            }
                            r#type_tested = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(SpecimenDefinition {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier,
                    r#type_collected,
                    r#patient_preparation: r#patient_preparation.unwrap_or(vec![]),
                    r#time_aspect,
                    r#collection: r#collection.unwrap_or(vec![]),
                    r#type_tested: r#type_tested.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
