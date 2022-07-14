// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum NutritionOrderEnteralFormulaAdministrationRate {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
    Invalid,
}
impl Default for NutritionOrderEnteralFormulaAdministrationRate {
    fn default() -> NutritionOrderEnteralFormulaAdministrationRate {
        NutritionOrderEnteralFormulaAdministrationRate::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct NutritionOrderOralDietNutrient {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for NutritionOrderOralDietNutrient {
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
        if let Some(some) = self.r#modifier.as_ref() {
            state.serialize_entry("modifier", some)?;
        }
        if let Some(some) = self.r#amount.as_ref() {
            state.serialize_entry("amount", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for NutritionOrderOralDietNutrient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NutritionOrderOralDietNutrient;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionOrderOralDietNutrient")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<NutritionOrderOralDietNutrient, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#modifier: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#amount: Option<Box<super::super::types::Quantity>> = None;
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
                        "modifier" => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            r#modifier = Some(map_access.next_value()?);
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
                                &["id", "extension", "modifierExtension", "modifier", "amount"],
                            ))
                        }
                    }
                }
                Ok(NutritionOrderOralDietNutrient {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#modifier,
                    r#amount,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct NutritionOrderOralDietTexture {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier: Option<Box<super::super::types::CodeableConcept>>,
    pub r#food_type: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for NutritionOrderOralDietTexture {
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
        if let Some(some) = self.r#modifier.as_ref() {
            state.serialize_entry("modifier", some)?;
        }
        if let Some(some) = self.r#food_type.as_ref() {
            state.serialize_entry("foodType", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for NutritionOrderOralDietTexture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NutritionOrderOralDietTexture;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionOrderOralDietTexture")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<NutritionOrderOralDietTexture, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#modifier: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#food_type: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        "modifier" => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            r#modifier = Some(map_access.next_value()?);
                        }
                        "foodType" => {
                            if r#food_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("foodType"));
                            }
                            r#food_type = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifierExtension",
                                    "modifier",
                                    "foodType",
                                ],
                            ))
                        }
                    }
                }
                Ok(NutritionOrderOralDietTexture {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#modifier,
                    r#food_type,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct NutritionOrderOralDiet {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#schedule: Vec<Box<super::super::types::Timing>>,
    pub r#nutrient: Vec<NutritionOrderOralDietNutrient>,
    pub r#texture: Vec<NutritionOrderOralDietTexture>,
    pub r#fluid_consistency_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#instruction: Option<super::super::types::String>,
}
impl serde::ser::Serialize for NutritionOrderOralDiet {
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
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
        }
        if !self.r#schedule.is_empty() {
            state.serialize_entry("schedule", &self.r#schedule)?;
        }
        if !self.r#nutrient.is_empty() {
            state.serialize_entry("nutrient", &self.r#nutrient)?;
        }
        if !self.r#texture.is_empty() {
            state.serialize_entry("texture", &self.r#texture)?;
        }
        if !self.r#fluid_consistency_type.is_empty() {
            state.serialize_entry("fluidConsistencyType", &self.r#fluid_consistency_type)?;
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
impl<'de> serde::de::Deserialize<'de> for NutritionOrderOralDiet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NutritionOrderOralDiet;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionOrderOralDiet")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<NutritionOrderOralDiet, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#schedule: Option<Vec<Box<super::super::types::Timing>>> = None;
                let mut r#nutrient: Option<Vec<NutritionOrderOralDietNutrient>> = None;
                let mut r#texture: Option<Vec<NutritionOrderOralDietTexture>> = None;
                let mut r#fluid_consistency_type: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#instruction: Option<super::super::types::String> = None;
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
                        "schedule" => {
                            if r#schedule.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            r#schedule = Some(map_access.next_value()?);
                        }
                        "nutrient" => {
                            if r#nutrient.is_some() {
                                return Err(serde::de::Error::duplicate_field("nutrient"));
                            }
                            r#nutrient = Some(map_access.next_value()?);
                        }
                        "texture" => {
                            if r#texture.is_some() {
                                return Err(serde::de::Error::duplicate_field("texture"));
                            }
                            r#texture = Some(map_access.next_value()?);
                        }
                        "fluidConsistencyType" => {
                            if r#fluid_consistency_type.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fluidConsistencyType",
                                ));
                            }
                            r#fluid_consistency_type = Some(map_access.next_value()?);
                        }
                        "instruction" => {
                            let some = r#instruction.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("instruction"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_instruction" => {
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
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifierExtension",
                                    "type",
                                    "schedule",
                                    "nutrient",
                                    "texture",
                                    "fluidConsistencyType",
                                    "instruction",
                                ],
                            ))
                        }
                    }
                }
                Ok(NutritionOrderOralDiet {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.unwrap_or(vec![]),
                    r#schedule: r#schedule.unwrap_or(vec![]),
                    r#nutrient: r#nutrient.unwrap_or(vec![]),
                    r#texture: r#texture.unwrap_or(vec![]),
                    r#fluid_consistency_type: r#fluid_consistency_type.unwrap_or(vec![]),
                    r#instruction,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct NutritionOrderSupplement {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_name: Option<super::super::types::String>,
    pub r#schedule: Vec<Box<super::super::types::Timing>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#instruction: Option<super::super::types::String>,
}
impl serde::ser::Serialize for NutritionOrderSupplement {
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
        if let Some(some) = self.r#product_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("productName", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_productName", &primitive_element)?;
            }
        }
        if !self.r#schedule.is_empty() {
            state.serialize_entry("schedule", &self.r#schedule)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
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
impl<'de> serde::de::Deserialize<'de> for NutritionOrderSupplement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NutritionOrderSupplement;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionOrderSupplement")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<NutritionOrderSupplement, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#product_name: Option<super::super::types::String> = None;
                let mut r#schedule: Option<Vec<Box<super::super::types::Timing>>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#instruction: Option<super::super::types::String> = None;
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
                        "productName" => {
                            let some = r#product_name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("productName"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_productName" => {
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
                        "schedule" => {
                            if r#schedule.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            r#schedule = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "instruction" => {
                            let some = r#instruction.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("instruction"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_instruction" => {
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
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifierExtension",
                                    "type",
                                    "productName",
                                    "schedule",
                                    "quantity",
                                    "instruction",
                                ],
                            ))
                        }
                    }
                }
                Ok(NutritionOrderSupplement {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#product_name,
                    r#schedule: r#schedule.unwrap_or(vec![]),
                    r#quantity,
                    r#instruction,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct NutritionOrderEnteralFormulaAdministration {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#schedule: Option<Box<super::super::types::Timing>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#rate: Option<NutritionOrderEnteralFormulaAdministrationRate>,
}
impl serde::ser::Serialize for NutritionOrderEnteralFormulaAdministration {
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
        if let Some(some) = self.r#schedule.as_ref() {
            state.serialize_entry("schedule", some)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#rate.as_ref() {
            match some {
                NutritionOrderEnteralFormulaAdministrationRate::Quantity(ref value) => {
                    state.serialize_entry("rateQuantity", value)?;
                }
                NutritionOrderEnteralFormulaAdministrationRate::Ratio(ref value) => {
                    state.serialize_entry("rateRatio", value)?;
                }
                NutritionOrderEnteralFormulaAdministrationRate::Invalid => {
                    return Err(serde::ser::Error::custom("rate is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for NutritionOrderEnteralFormulaAdministration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NutritionOrderEnteralFormulaAdministration;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionOrderEnteralFormulaAdministration")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<NutritionOrderEnteralFormulaAdministration, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#schedule: Option<Box<super::super::types::Timing>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#rate: Option<NutritionOrderEnteralFormulaAdministrationRate> = None;
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
                        "schedule" => {
                            if r#schedule.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            r#schedule = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "rateQuantity" => {
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateQuantity"));
                            }
                            r#rate =
                                Some(NutritionOrderEnteralFormulaAdministrationRate::Quantity(
                                    map_access.next_value()?,
                                ));
                        }
                        "rateRatio" => {
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateRatio"));
                            }
                            r#rate = Some(NutritionOrderEnteralFormulaAdministrationRate::Ratio(
                                map_access.next_value()?,
                            ));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifierExtension",
                                    "schedule",
                                    "quantity",
                                    "rateQuantity",
                                    "rateRatio",
                                ],
                            ))
                        }
                    }
                }
                Ok(NutritionOrderEnteralFormulaAdministration {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#schedule,
                    r#quantity,
                    r#rate,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct NutritionOrderEnteralFormula {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#base_formula_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#base_formula_product_name: Option<super::super::types::String>,
    pub r#additive_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#additive_product_name: Option<super::super::types::String>,
    pub r#caloric_density: Option<Box<super::super::types::Quantity>>,
    pub r#routeof_administration: Option<Box<super::super::types::CodeableConcept>>,
    pub r#administration: Vec<NutritionOrderEnteralFormulaAdministration>,
    pub r#max_volume_to_deliver: Option<Box<super::super::types::Quantity>>,
    pub r#administration_instruction: Option<super::super::types::String>,
}
impl serde::ser::Serialize for NutritionOrderEnteralFormula {
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
        if let Some(some) = self.r#base_formula_type.as_ref() {
            state.serialize_entry("baseFormulaType", some)?;
        }
        if let Some(some) = self.r#base_formula_product_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("baseFormulaProductName", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_baseFormulaProductName", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#additive_type.as_ref() {
            state.serialize_entry("additiveType", some)?;
        }
        if let Some(some) = self.r#additive_product_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("additiveProductName", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_additiveProductName", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#caloric_density.as_ref() {
            state.serialize_entry("caloricDensity", some)?;
        }
        if let Some(some) = self.r#routeof_administration.as_ref() {
            state.serialize_entry("routeofAdministration", some)?;
        }
        if !self.r#administration.is_empty() {
            state.serialize_entry("administration", &self.r#administration)?;
        }
        if let Some(some) = self.r#max_volume_to_deliver.as_ref() {
            state.serialize_entry("maxVolumeToDeliver", some)?;
        }
        if let Some(some) = self.r#administration_instruction.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("administrationInstruction", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_administrationInstruction", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for NutritionOrderEnteralFormula {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NutritionOrderEnteralFormula;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionOrderEnteralFormula")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<NutritionOrderEnteralFormula, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#base_formula_type: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#base_formula_product_name: Option<super::super::types::String> = None;
                let mut r#additive_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#additive_product_name: Option<super::super::types::String> = None;
                let mut r#caloric_density: Option<Box<super::super::types::Quantity>> = None;
                let mut r#routeof_administration: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#administration: Option<Vec<NutritionOrderEnteralFormulaAdministration>> =
                    None;
                let mut r#max_volume_to_deliver: Option<Box<super::super::types::Quantity>> = None;
                let mut r#administration_instruction: Option<super::super::types::String> = None;
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
                        "baseFormulaType" => {
                            if r#base_formula_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseFormulaType"));
                            }
                            r#base_formula_type = Some(map_access.next_value()?);
                        }
                        "baseFormulaProductName" => {
                            let some =
                                r#base_formula_product_name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "baseFormulaProductName",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_baseFormulaProductName" => {
                            let some =
                                r#base_formula_product_name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_baseFormulaProductName",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "additiveType" => {
                            if r#additive_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("additiveType"));
                            }
                            r#additive_type = Some(map_access.next_value()?);
                        }
                        "additiveProductName" => {
                            let some = r#additive_product_name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "additiveProductName",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_additiveProductName" => {
                            let some = r#additive_product_name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_additiveProductName",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "caloricDensity" => {
                            if r#caloric_density.is_some() {
                                return Err(serde::de::Error::duplicate_field("caloricDensity"));
                            }
                            r#caloric_density = Some(map_access.next_value()?);
                        }
                        "routeofAdministration" => {
                            if r#routeof_administration.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "routeofAdministration",
                                ));
                            }
                            r#routeof_administration = Some(map_access.next_value()?);
                        }
                        "administration" => {
                            if r#administration.is_some() {
                                return Err(serde::de::Error::duplicate_field("administration"));
                            }
                            r#administration = Some(map_access.next_value()?);
                        }
                        "maxVolumeToDeliver" => {
                            if r#max_volume_to_deliver.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxVolumeToDeliver",
                                ));
                            }
                            r#max_volume_to_deliver = Some(map_access.next_value()?);
                        }
                        "administrationInstruction" => {
                            let some =
                                r#administration_instruction.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "administrationInstruction",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_administrationInstruction" => {
                            let some =
                                r#administration_instruction.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_administrationInstruction",
                                ));
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
                                    "modifierExtension",
                                    "baseFormulaType",
                                    "baseFormulaProductName",
                                    "additiveType",
                                    "additiveProductName",
                                    "caloricDensity",
                                    "routeofAdministration",
                                    "administration",
                                    "maxVolumeToDeliver",
                                    "administrationInstruction",
                                ],
                            ))
                        }
                    }
                }
                Ok(NutritionOrderEnteralFormula {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#base_formula_type,
                    r#base_formula_product_name,
                    r#additive_type,
                    r#additive_product_name,
                    r#caloric_density,
                    r#routeof_administration,
                    r#administration: r#administration.unwrap_or(vec![]),
                    r#max_volume_to_deliver,
                    r#administration_instruction,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct NutritionOrder {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#instantiates: Vec<super::super::types::Uri>,
    pub r#status: super::super::types::Code,
    pub r#intent: super::super::types::Code,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#date_time: super::super::types::DateTime,
    pub r#orderer: Option<Box<super::super::types::Reference>>,
    pub r#allergy_intolerance: Vec<Box<super::super::types::Reference>>,
    pub r#food_preference_modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#exclude_food_modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#oral_diet: Option<NutritionOrderOralDiet>,
    pub r#supplement: Vec<NutritionOrderSupplement>,
    pub r#enteral_formula: Option<NutritionOrderEnteralFormula>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for NutritionOrder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "NutritionOrder")?;
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
        if !self.r#instantiates_canonical.is_empty() {
            let values: Vec<_> = self
                .r#instantiates_canonical
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiatesCanonical", &values)?;
            }
            let requires_elements = self
                .r#instantiates_canonical
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#instantiates_canonical
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
                state.serialize_entry("_instantiatesCanonical", &primitive_elements)?;
            }
        }
        if !self.r#instantiates_uri.is_empty() {
            let values: Vec<_> = self.r#instantiates_uri.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiatesUri", &values)?;
            }
            let requires_elements = self
                .r#instantiates_uri
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#instantiates_uri
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
                state.serialize_entry("_instantiatesUri", &primitive_elements)?;
            }
        }
        if !self.r#instantiates.is_empty() {
            let values: Vec<_> = self.r#instantiates.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiates", &values)?;
            }
            let requires_elements = self
                .r#instantiates
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#instantiates
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
                state.serialize_entry("_instantiates", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#status.value.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if let Some(some) = self.r#intent.value.as_ref() {
            state.serialize_entry("intent", some)?;
        }
        if self.r#intent.id.is_some() || !self.r#intent.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#intent.id,
                extension: &self.r#intent.extension,
            };
            state.serialize_entry("_intent", &primitive_element)?;
        }
        state.serialize_entry("patient", &self.r#patient)?;
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#date_time.value.as_ref() {
            state.serialize_entry("dateTime", some)?;
        }
        if self.r#date_time.id.is_some() || !self.r#date_time.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#date_time.id,
                extension: &self.r#date_time.extension,
            };
            state.serialize_entry("_dateTime", &primitive_element)?;
        }
        if let Some(some) = self.r#orderer.as_ref() {
            state.serialize_entry("orderer", some)?;
        }
        if !self.r#allergy_intolerance.is_empty() {
            state.serialize_entry("allergyIntolerance", &self.r#allergy_intolerance)?;
        }
        if !self.r#food_preference_modifier.is_empty() {
            state.serialize_entry("foodPreferenceModifier", &self.r#food_preference_modifier)?;
        }
        if !self.r#exclude_food_modifier.is_empty() {
            state.serialize_entry("excludeFoodModifier", &self.r#exclude_food_modifier)?;
        }
        if let Some(some) = self.r#oral_diet.as_ref() {
            state.serialize_entry("oralDiet", some)?;
        }
        if !self.r#supplement.is_empty() {
            state.serialize_entry("supplement", &self.r#supplement)?;
        }
        if let Some(some) = self.r#enteral_formula.as_ref() {
            state.serialize_entry("enteralFormula", some)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for NutritionOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NutritionOrder;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionOrder")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<NutritionOrder, V::Error>
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
                let mut r#instantiates_canonical: Option<Vec<super::super::types::Canonical>> =
                    None;
                let mut r#instantiates_uri: Option<Vec<super::super::types::Uri>> = None;
                let mut r#instantiates: Option<Vec<super::super::types::Uri>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#intent: Option<super::super::types::Code> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#date_time: Option<super::super::types::DateTime> = None;
                let mut r#orderer: Option<Box<super::super::types::Reference>> = None;
                let mut r#allergy_intolerance: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#food_preference_modifier: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#exclude_food_modifier: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#oral_diet: Option<NutritionOrderOralDiet> = None;
                let mut r#supplement: Option<Vec<NutritionOrderSupplement>> = None;
                let mut r#enteral_formula: Option<NutritionOrderEnteralFormula> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
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
                        "instantiatesCanonical" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#instantiates_canonical
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatesCanonical",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_instantiatesCanonical" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#instantiates_canonical
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                    "_instantiatesCanonical",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "instantiatesUri" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#instantiates_uri.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("instantiatesUri"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_instantiatesUri" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#instantiates_uri
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_instantiatesUri"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "instantiates" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#instantiates.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("instantiates"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_instantiates" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#instantiates.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_instantiates"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_status"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "intent" => {
                            let some = r#intent.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("intent"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_intent" => {
                            let some = r#intent.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_intent"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "patient" => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            r#patient = Some(map_access.next_value()?);
                        }
                        "encounter" => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(map_access.next_value()?);
                        }
                        "dateTime" => {
                            let some = r#date_time.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateTime"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_dateTime" => {
                            let some = r#date_time.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_dateTime"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "orderer" => {
                            if r#orderer.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderer"));
                            }
                            r#orderer = Some(map_access.next_value()?);
                        }
                        "allergyIntolerance" => {
                            if r#allergy_intolerance.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "allergyIntolerance",
                                ));
                            }
                            r#allergy_intolerance = Some(map_access.next_value()?);
                        }
                        "foodPreferenceModifier" => {
                            if r#food_preference_modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "foodPreferenceModifier",
                                ));
                            }
                            r#food_preference_modifier = Some(map_access.next_value()?);
                        }
                        "excludeFoodModifier" => {
                            if r#exclude_food_modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "excludeFoodModifier",
                                ));
                            }
                            r#exclude_food_modifier = Some(map_access.next_value()?);
                        }
                        "oralDiet" => {
                            if r#oral_diet.is_some() {
                                return Err(serde::de::Error::duplicate_field("oralDiet"));
                            }
                            r#oral_diet = Some(map_access.next_value()?);
                        }
                        "supplement" => {
                            if r#supplement.is_some() {
                                return Err(serde::de::Error::duplicate_field("supplement"));
                            }
                            r#supplement = Some(map_access.next_value()?);
                        }
                        "enteralFormula" => {
                            if r#enteral_formula.is_some() {
                                return Err(serde::de::Error::duplicate_field("enteralFormula"));
                            }
                            r#enteral_formula = Some(map_access.next_value()?);
                        }
                        "note" => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
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
                                    "instantiatesCanonical",
                                    "instantiatesUri",
                                    "instantiates",
                                    "status",
                                    "intent",
                                    "patient",
                                    "encounter",
                                    "dateTime",
                                    "orderer",
                                    "allergyIntolerance",
                                    "foodPreferenceModifier",
                                    "excludeFoodModifier",
                                    "oralDiet",
                                    "supplement",
                                    "enteralFormula",
                                    "note",
                                ],
                            ))
                        }
                    }
                }
                Ok(NutritionOrder {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#instantiates_canonical: r#instantiates_canonical.unwrap_or(vec![]),
                    r#instantiates_uri: r#instantiates_uri.unwrap_or(vec![]),
                    r#instantiates: r#instantiates.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#intent: r#intent.ok_or(serde::de::Error::missing_field("intent"))?,
                    r#patient: r#patient.ok_or(serde::de::Error::missing_field("patient"))?,
                    r#encounter,
                    r#date_time: r#date_time.ok_or(serde::de::Error::missing_field("date_time"))?,
                    r#orderer,
                    r#allergy_intolerance: r#allergy_intolerance.unwrap_or(vec![]),
                    r#food_preference_modifier: r#food_preference_modifier.unwrap_or(vec![]),
                    r#exclude_food_modifier: r#exclude_food_modifier.unwrap_or(vec![]),
                    r#oral_diet,
                    r#supplement: r#supplement.unwrap_or(vec![]),
                    r#enteral_formula,
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
