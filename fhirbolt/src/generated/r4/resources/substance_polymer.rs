// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct SubstancePolymerMonomerSetStartingMaterial {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#material: Option<Box<super::super::types::CodeableConcept>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#is_defining: Option<super::super::types::Boolean>,
    pub r#amount: Option<Box<super::super::types::SubstanceAmount>>,
}
impl serde::ser::Serialize for SubstancePolymerMonomerSetStartingMaterial {
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
        if let Some(some) = self.r#is_defining.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isDefining", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isDefining", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#amount.as_ref() {
            state.serialize_entry("amount", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstancePolymerMonomerSetStartingMaterial {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstancePolymerMonomerSetStartingMaterial;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstancePolymerMonomerSetStartingMaterial")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstancePolymerMonomerSetStartingMaterial, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#material: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#is_defining: Option<super::super::types::Boolean> = None;
                let mut r#amount: Option<Box<super::super::types::SubstanceAmount>> = None;
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
                        "material" => {
                            if r#material.is_some() {
                                return Err(serde::de::Error::duplicate_field("material"));
                            }
                            r#material = Some(map_access.next_value()?);
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "isDefining" => {
                            let some = r#is_defining.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefining"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_isDefining" => {
                            let some = r#is_defining.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_isDefining"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "amount" => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "material",
                                    "type",
                                    "is_defining",
                                    "amount",
                                ],
                            ))
                        }
                    }
                }
                Ok(SubstancePolymerMonomerSetStartingMaterial {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#material,
                    r#type,
                    r#is_defining,
                    r#amount,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstancePolymerMonomerSet {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#ratio_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#starting_material: Vec<SubstancePolymerMonomerSetStartingMaterial>,
}
impl serde::ser::Serialize for SubstancePolymerMonomerSet {
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
        if let Some(some) = self.r#ratio_type.as_ref() {
            state.serialize_entry("ratioType", some)?;
        }
        if !self.r#starting_material.is_empty() {
            state.serialize_entry("startingMaterial", &self.r#starting_material)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstancePolymerMonomerSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstancePolymerMonomerSet;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstancePolymerMonomerSet")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstancePolymerMonomerSet, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#ratio_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#starting_material: Option<
                    Vec<SubstancePolymerMonomerSetStartingMaterial>,
                > = None;
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
                        "ratioType" => {
                            if r#ratio_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("ratioType"));
                            }
                            r#ratio_type = Some(map_access.next_value()?);
                        }
                        "startingMaterial" => {
                            if r#starting_material.is_some() {
                                return Err(serde::de::Error::duplicate_field("startingMaterial"));
                            }
                            r#starting_material = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "ratio_type",
                                    "starting_material",
                                ],
                            ))
                        }
                    }
                }
                Ok(SubstancePolymerMonomerSet {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#ratio_type,
                    r#starting_material: r#starting_material.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#degree: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Option<Box<super::super::types::SubstanceAmount>>,
}
impl serde::ser::Serialize for SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
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
        if let Some(some) = self.r#degree.as_ref() {
            state.serialize_entry("degree", some)?;
        }
        if let Some(some) = self.r#amount.as_ref() {
            state.serialize_entry("amount", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#degree: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#amount: Option<Box<super::super::types::SubstanceAmount>> = None;
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
                        "degree" => {
                            if r#degree.is_some() {
                                return Err(serde::de::Error::duplicate_field("degree"));
                            }
                            r#degree = Some(map_access.next_value()?);
                        }
                        "amount" => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "degree", "amount"],
                            ))
                        }
                    }
                }
                Ok(SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#degree,
                    r#amount,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#representation: Option<super::super::types::String>,
    pub r#attachment: Option<Box<super::super::types::Attachment>>,
}
impl serde::ser::Serialize for SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
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
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#representation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("representation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_representation", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#attachment.as_ref() {
            state.serialize_entry("attachment", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstancePolymerRepeatRepeatUnitStructuralRepresentation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstancePolymerRepeatRepeatUnitStructuralRepresentation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstancePolymerRepeatRepeatUnitStructuralRepresentation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#representation: Option<super::super::types::String> = None;
                let mut r#attachment: Option<Box<super::super::types::Attachment>> = None;
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
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "representation" => {
                            let some = r#representation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("representation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_representation" => {
                            let some = r#representation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_representation"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "attachment" => {
                            if r#attachment.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachment"));
                            }
                            r#attachment = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "type",
                                    "representation",
                                    "attachment",
                                ],
                            ))
                        }
                    }
                }
                Ok(SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#representation,
                    r#attachment,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstancePolymerRepeatRepeatUnit {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#orientation_of_polymerisation: Option<Box<super::super::types::CodeableConcept>>,
    pub r#repeat_unit: Option<super::super::types::String>,
    pub r#amount: Option<Box<super::super::types::SubstanceAmount>>,
    pub r#degree_of_polymerisation: Vec<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation>,
    pub r#structural_representation: Vec<SubstancePolymerRepeatRepeatUnitStructuralRepresentation>,
}
impl serde::ser::Serialize for SubstancePolymerRepeatRepeatUnit {
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
        if let Some(some) = self.r#orientation_of_polymerisation.as_ref() {
            state.serialize_entry("orientationOfPolymerisation", some)?;
        }
        if let Some(some) = self.r#repeat_unit.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("repeatUnit", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_repeatUnit", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#amount.as_ref() {
            state.serialize_entry("amount", some)?;
        }
        if !self.r#degree_of_polymerisation.is_empty() {
            state.serialize_entry("degreeOfPolymerisation", &self.r#degree_of_polymerisation)?;
        }
        if !self.r#structural_representation.is_empty() {
            state.serialize_entry(
                "structuralRepresentation",
                &self.r#structural_representation,
            )?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstancePolymerRepeatRepeatUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstancePolymerRepeatRepeatUnit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstancePolymerRepeatRepeatUnit")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstancePolymerRepeatRepeatUnit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#orientation_of_polymerisation: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#repeat_unit: Option<super::super::types::String> = None;
                let mut r#amount: Option<Box<super::super::types::SubstanceAmount>> = None;
                let mut r#degree_of_polymerisation: Option<
                    Vec<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation>,
                > = None;
                let mut r#structural_representation: Option<
                    Vec<SubstancePolymerRepeatRepeatUnitStructuralRepresentation>,
                > = None;
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
                        "orientationOfPolymerisation" => {
                            if r#orientation_of_polymerisation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orientationOfPolymerisation",
                                ));
                            }
                            r#orientation_of_polymerisation = Some(map_access.next_value()?);
                        }
                        "repeatUnit" => {
                            let some = r#repeat_unit.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("repeatUnit"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_repeatUnit" => {
                            let some = r#repeat_unit.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_repeatUnit"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "amount" => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(map_access.next_value()?);
                        }
                        "degreeOfPolymerisation" => {
                            if r#degree_of_polymerisation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "degreeOfPolymerisation",
                                ));
                            }
                            r#degree_of_polymerisation = Some(map_access.next_value()?);
                        }
                        "structuralRepresentation" => {
                            if r#structural_representation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "structuralRepresentation",
                                ));
                            }
                            r#structural_representation = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "orientation_of_polymerisation",
                                    "repeat_unit",
                                    "amount",
                                    "degree_of_polymerisation",
                                    "structural_representation",
                                ],
                            ))
                        }
                    }
                }
                Ok(SubstancePolymerRepeatRepeatUnit {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#orientation_of_polymerisation,
                    r#repeat_unit,
                    r#amount,
                    r#degree_of_polymerisation: r#degree_of_polymerisation.unwrap_or(vec![]),
                    r#structural_representation: r#structural_representation.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstancePolymerRepeat {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#number_of_units: Option<super::super::types::Integer>,
    pub r#average_molecular_formula: Option<super::super::types::String>,
    pub r#repeat_unit_amount_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#repeat_unit: Vec<SubstancePolymerRepeatRepeatUnit>,
}
impl serde::ser::Serialize for SubstancePolymerRepeat {
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
        if let Some(some) = self.r#number_of_units.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("numberOfUnits", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_numberOfUnits", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#average_molecular_formula.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("averageMolecularFormula", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_averageMolecularFormula", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#repeat_unit_amount_type.as_ref() {
            state.serialize_entry("repeatUnitAmountType", some)?;
        }
        if !self.r#repeat_unit.is_empty() {
            state.serialize_entry("repeatUnit", &self.r#repeat_unit)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstancePolymerRepeat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstancePolymerRepeat;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstancePolymerRepeat")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstancePolymerRepeat, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#number_of_units: Option<super::super::types::Integer> = None;
                let mut r#average_molecular_formula: Option<super::super::types::String> = None;
                let mut r#repeat_unit_amount_type: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#repeat_unit: Option<Vec<SubstancePolymerRepeatRepeatUnit>> = None;
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
                        "numberOfUnits" => {
                            let some = r#number_of_units.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberOfUnits"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_numberOfUnits" => {
                            let some = r#number_of_units.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_numberOfUnits"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "averageMolecularFormula" => {
                            let some =
                                r#average_molecular_formula.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "averageMolecularFormula",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_averageMolecularFormula" => {
                            let some =
                                r#average_molecular_formula.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_averageMolecularFormula",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "repeatUnitAmountType" => {
                            if r#repeat_unit_amount_type.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "repeatUnitAmountType",
                                ));
                            }
                            r#repeat_unit_amount_type = Some(map_access.next_value()?);
                        }
                        "repeatUnit" => {
                            if r#repeat_unit.is_some() {
                                return Err(serde::de::Error::duplicate_field("repeatUnit"));
                            }
                            r#repeat_unit = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "number_of_units",
                                    "average_molecular_formula",
                                    "repeat_unit_amount_type",
                                    "repeat_unit",
                                ],
                            ))
                        }
                    }
                }
                Ok(SubstancePolymerRepeat {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#number_of_units,
                    r#average_molecular_formula,
                    r#repeat_unit_amount_type,
                    r#repeat_unit: r#repeat_unit.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstancePolymer {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#class: Option<Box<super::super::types::CodeableConcept>>,
    pub r#geometry: Option<Box<super::super::types::CodeableConcept>>,
    pub r#copolymer_connectivity: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modification: Vec<super::super::types::String>,
    pub r#monomer_set: Vec<SubstancePolymerMonomerSet>,
    pub r#repeat: Vec<SubstancePolymerRepeat>,
}
impl serde::ser::Serialize for SubstancePolymer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SubstancePolymer")?;
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
        if let Some(some) = self.r#class.as_ref() {
            state.serialize_entry("class", some)?;
        }
        if let Some(some) = self.r#geometry.as_ref() {
            state.serialize_entry("geometry", some)?;
        }
        if !self.r#copolymer_connectivity.is_empty() {
            state.serialize_entry("copolymerConnectivity", &self.r#copolymer_connectivity)?;
        }
        if !self.r#modification.is_empty() {
            let values: Vec<_> = self.r#modification.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("modification", &values)?;
            }
            let requires_elements = self
                .r#modification
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#modification
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
                state.serialize_entry("_modification", &primitive_elements)?;
            }
        }
        if !self.r#monomer_set.is_empty() {
            state.serialize_entry("monomerSet", &self.r#monomer_set)?;
        }
        if !self.r#repeat.is_empty() {
            state.serialize_entry("repeat", &self.r#repeat)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstancePolymer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstancePolymer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstancePolymer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstancePolymer, V::Error>
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
                let mut r#class: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#geometry: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#copolymer_connectivity: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#modification: Option<Vec<super::super::types::String>> = None;
                let mut r#monomer_set: Option<Vec<SubstancePolymerMonomerSet>> = None;
                let mut r#repeat: Option<Vec<SubstancePolymerRepeat>> = None;
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
                        "class" => {
                            if r#class.is_some() {
                                return Err(serde::de::Error::duplicate_field("class"));
                            }
                            r#class = Some(map_access.next_value()?);
                        }
                        "geometry" => {
                            if r#geometry.is_some() {
                                return Err(serde::de::Error::duplicate_field("geometry"));
                            }
                            r#geometry = Some(map_access.next_value()?);
                        }
                        "copolymerConnectivity" => {
                            if r#copolymer_connectivity.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "copolymerConnectivity",
                                ));
                            }
                            r#copolymer_connectivity = Some(map_access.next_value()?);
                        }
                        "modification" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#modification.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("modification"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_modification" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#modification.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_modification"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "monomerSet" => {
                            if r#monomer_set.is_some() {
                                return Err(serde::de::Error::duplicate_field("monomerSet"));
                            }
                            r#monomer_set = Some(map_access.next_value()?);
                        }
                        "repeat" => {
                            if r#repeat.is_some() {
                                return Err(serde::de::Error::duplicate_field("repeat"));
                            }
                            r#repeat = Some(map_access.next_value()?);
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
                                    "class",
                                    "geometry",
                                    "copolymer_connectivity",
                                    "modification",
                                    "monomer_set",
                                    "repeat",
                                ],
                            ))
                        }
                    }
                }
                Ok(SubstancePolymer {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#class,
                    r#geometry,
                    r#copolymer_connectivity: r#copolymer_connectivity.unwrap_or(vec![]),
                    r#modification: r#modification.unwrap_or(vec![]),
                    r#monomer_set: r#monomer_set.unwrap_or(vec![]),
                    r#repeat: r#repeat.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
