// Generated on 2022-10-14 by fhirbolt-codegen v0.1.0
#[doc = "The rate of administration of formula via a feeding pump, e.g. 60 mL per hour, according to the specified schedule."]
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
#[doc = "Class that defines the quantity and type of nutrient modifications (for example carbohydrate, fiber or sodium) required for the oral diet."]
#[derive(Default, Debug, Clone)]
pub struct NutritionOrderOralDietNutrient {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The nutrient that is being modified such as carbohydrate or sodium."]
    pub r#modifier: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The quantity of the specified nutrient to include in diet."]
    pub r#amount: Option<Box<super::super::types::Quantity>>,
}
impl crate::AnyResource for NutritionOrderOralDietNutrient {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "amount")]
            Amount,
            Unknown(String),
        }
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Modifier => {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier = Some(map_access.next_value()?);
                            }
                            Field::Amount => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amount"));
                                }
                                r#amount = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "modifier", "amount"],
                                ));
                            },
                        }
                    }
                    Ok(NutritionOrderOralDietNutrient {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#modifier,
                        r#amount,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Class that describes any texture modifications required for the patient to safely consume various types of solid foods."]
#[derive(Default, Debug, Clone)]
pub struct NutritionOrderOralDietTexture {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Any texture modifications (for solid foods) that should be made, e.g. easy to chew, chopped, ground, and pureed."]
    pub r#modifier: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The food type(s) (e.g. meats, all foods)  that the texture modification applies to.  This could be all foods types."]
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "foodType")]
            FoodType,
            Unknown(String),
        }
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Modifier => {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier = Some(map_access.next_value()?);
                            }
                            Field::FoodType => {
                                if r#food_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("foodType"));
                                }
                                r#food_type = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "modifier",
                                        "foodType",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(NutritionOrderOralDietTexture {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#modifier,
                        r#food_type,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Diet given orally in contrast to enteral (tube) feeding."]
#[derive(Default, Debug, Clone)]
pub struct NutritionOrderOralDiet {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of diet or dietary restriction such as fiber restricted diet or diabetic diet."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The time period and frequency at which the diet should be given.  The diet should be given for the combination of all schedules if more than one schedule is present."]
    pub r#schedule: Vec<Box<super::super::types::Timing>>,
    #[doc = "Class that defines the quantity and type of nutrient modifications (for example carbohydrate, fiber or sodium) required for the oral diet."]
    pub r#nutrient: Vec<NutritionOrderOralDietNutrient>,
    #[doc = "Class that describes any texture modifications required for the patient to safely consume various types of solid foods."]
    pub r#texture: Vec<NutritionOrderOralDietTexture>,
    #[doc = "The required consistency (e.g. honey-thick, nectar-thick, thin, thickened.) of liquids or fluids served to the patient."]
    pub r#fluid_consistency_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Free text or additional instructions or information pertaining to the oral diet."]
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
                let some = Ok(some)?;
                state.serialize_entry("instruction", &some)?;
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "schedule")]
            Schedule,
            #[serde(rename = "nutrient")]
            Nutrient,
            #[serde(rename = "texture")]
            Texture,
            #[serde(rename = "fluidConsistencyType")]
            FluidConsistencyType,
            #[serde(rename = "instruction")]
            Instruction,
            #[serde(rename = "_instruction")]
            InstructionPrimitiveElement,
            Unknown(String),
        }
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Schedule => {
                                if r#schedule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("schedule"));
                                }
                                r#schedule = Some(map_access.next_value()?);
                            }
                            Field::Nutrient => {
                                if r#nutrient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("nutrient"));
                                }
                                r#nutrient = Some(map_access.next_value()?);
                            }
                            Field::Texture => {
                                if r#texture.is_some() {
                                    return Err(serde::de::Error::duplicate_field("texture"));
                                }
                                r#texture = Some(map_access.next_value()?);
                            }
                            Field::FluidConsistencyType => {
                                if r#fluid_consistency_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fluidConsistencyType",
                                    ));
                                }
                                r#fluid_consistency_type = Some(map_access.next_value()?);
                            }
                            Field::Instruction => {
                                let some = r#instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("instruction"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
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
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Oral nutritional products given in order to add further nutritional value to the patient's diet."]
#[derive(Default, Debug, Clone)]
pub struct NutritionOrderSupplement {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of nutritional supplement product required such as a high protein or pediatric clear liquid supplement."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The product or brand name of the nutritional supplement such as \"Acme Protein Shake\"."]
    pub r#product_name: Option<super::super::types::String>,
    #[doc = "The time period and frequency at which the supplement(s) should be given.  The supplement should be given for the combination of all schedules if more than one schedule is present."]
    pub r#schedule: Vec<Box<super::super::types::Timing>>,
    #[doc = "The amount of the nutritional supplement to be given."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Free text or additional instructions or information pertaining to the oral supplement."]
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
                let some = Ok(some)?;
                state.serialize_entry("productName", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("instruction", &some)?;
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "productName")]
            ProductName,
            #[serde(rename = "_productName")]
            ProductNamePrimitiveElement,
            #[serde(rename = "schedule")]
            Schedule,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "instruction")]
            Instruction,
            #[serde(rename = "_instruction")]
            InstructionPrimitiveElement,
            Unknown(String),
        }
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::ProductName => {
                                let some = r#product_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("productName"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Schedule => {
                                if r#schedule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("schedule"));
                                }
                                r#schedule = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::Instruction => {
                                let some = r#instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("instruction"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
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
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Formula administration instructions as structured data.  This repeating structure allows for changing the administration rate or volume over time for both bolus and continuous feeding.  An example of this would be an instruction to increase the rate of continuous feeding every 2 hours."]
#[derive(Default, Debug, Clone)]
pub struct NutritionOrderEnteralFormulaAdministration {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The time period and frequency at which the enteral formula should be delivered to the patient."]
    pub r#schedule: Option<Box<super::super::types::Timing>>,
    #[doc = "The volume of formula to provide to the patient per the specified administration schedule."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The rate of administration of formula via a feeding pump, e.g. 60 mL per hour, according to the specified schedule."]
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "schedule")]
            Schedule,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "rateQuantity")]
            RateQuantity,
            #[serde(rename = "rateRatio")]
            RateRatio,
            Unknown(String),
        }
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Schedule => {
                                if r#schedule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("schedule"));
                                }
                                r#schedule = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::RateQuantity => {
                                if r#rate.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rateQuantity"));
                                }
                                r#rate =
                                    Some(NutritionOrderEnteralFormulaAdministrationRate::Quantity(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::RateRatio => {
                                if r#rate.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rateRatio"));
                                }
                                r#rate =
                                    Some(NutritionOrderEnteralFormulaAdministrationRate::Ratio(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "schedule",
                                        "quantity",
                                        "rateQuantity",
                                        "rateRatio",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Feeding provided through the gastrointestinal tract via a tube, catheter, or stoma that delivers nutrition distal to the oral cavity."]
#[derive(Default, Debug, Clone)]
pub struct NutritionOrderEnteralFormula {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of enteral or infant formula such as an adult standard formula with fiber or a soy-based infant formula."]
    pub r#base_formula_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The product or brand name of the enteral or infant formula product such as \"ACME Adult Standard Formula\"."]
    pub r#base_formula_product_name: Option<super::super::types::String>,
    #[doc = "Indicates the type of modular component such as protein, carbohydrate, fat or fiber to be provided in addition to or mixed with the base formula."]
    pub r#additive_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The product or brand name of the type of modular component to be added to the formula."]
    pub r#additive_product_name: Option<super::super::types::String>,
    #[doc = "The amount of energy (calories) that the formula should provide per specified volume, typically per mL or fluid oz.  For example, an infant may require a formula that provides 24 calories per fluid ounce or an adult may require an enteral formula that provides 1.5 calorie/mL."]
    pub r#caloric_density: Option<Box<super::super::types::Quantity>>,
    #[doc = "The route or physiological path of administration into the patient's gastrointestinal  tract for purposes of providing the formula feeding, e.g. nasogastric tube."]
    pub r#routeof_administration: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Formula administration instructions as structured data.  This repeating structure allows for changing the administration rate or volume over time for both bolus and continuous feeding.  An example of this would be an instruction to increase the rate of continuous feeding every 2 hours."]
    pub r#administration: Vec<NutritionOrderEnteralFormulaAdministration>,
    #[doc = "The maximum total quantity of formula that may be administered to a subject over the period of time, e.g. 1440 mL over 24 hours."]
    pub r#max_volume_to_deliver: Option<Box<super::super::types::Quantity>>,
    #[doc = "Free text formula administration, feeding instructions or additional instructions or information."]
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
                let some = Ok(some)?;
                state.serialize_entry("baseFormulaProductName", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("additiveProductName", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("administrationInstruction", &some)?;
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "baseFormulaType")]
            BaseFormulaType,
            #[serde(rename = "baseFormulaProductName")]
            BaseFormulaProductName,
            #[serde(rename = "_baseFormulaProductName")]
            BaseFormulaProductNamePrimitiveElement,
            #[serde(rename = "additiveType")]
            AdditiveType,
            #[serde(rename = "additiveProductName")]
            AdditiveProductName,
            #[serde(rename = "_additiveProductName")]
            AdditiveProductNamePrimitiveElement,
            #[serde(rename = "caloricDensity")]
            CaloricDensity,
            #[serde(rename = "routeofAdministration")]
            RouteofAdministration,
            #[serde(rename = "administration")]
            Administration,
            #[serde(rename = "maxVolumeToDeliver")]
            MaxVolumeToDeliver,
            #[serde(rename = "administrationInstruction")]
            AdministrationInstruction,
            #[serde(rename = "_administrationInstruction")]
            AdministrationInstructionPrimitiveElement,
            Unknown(String),
        }
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::BaseFormulaType => {
                                if r#base_formula_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "baseFormulaType",
                                    ));
                                }
                                r#base_formula_type = Some(map_access.next_value()?);
                            }
                            Field::BaseFormulaProductName => {
                                let some =
                                    r#base_formula_product_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "baseFormulaProductName",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::BaseFormulaProductNamePrimitiveElement => {
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
                            Field::AdditiveType => {
                                if r#additive_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("additiveType"));
                                }
                                r#additive_type = Some(map_access.next_value()?);
                            }
                            Field::AdditiveProductName => {
                                let some =
                                    r#additive_product_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "additiveProductName",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::AdditiveProductNamePrimitiveElement => {
                                let some =
                                    r#additive_product_name.get_or_insert(Default::default());
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
                            Field::CaloricDensity => {
                                if r#caloric_density.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "caloricDensity",
                                    ));
                                }
                                r#caloric_density = Some(map_access.next_value()?);
                            }
                            Field::RouteofAdministration => {
                                if r#routeof_administration.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "routeofAdministration",
                                    ));
                                }
                                r#routeof_administration = Some(map_access.next_value()?);
                            }
                            Field::Administration => {
                                if r#administration.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "administration",
                                    ));
                                }
                                r#administration = Some(map_access.next_value()?);
                            }
                            Field::MaxVolumeToDeliver => {
                                if r#max_volume_to_deliver.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxVolumeToDeliver",
                                    ));
                                }
                                r#max_volume_to_deliver = Some(map_access.next_value()?);
                            }
                            Field::AdministrationInstruction => {
                                let some =
                                    r#administration_instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "administrationInstruction",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::AdministrationInstructionPrimitiveElement => {
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
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
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
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A request to supply a diet, formula feeding (enteral) or oral nutritional supplement to a patient/resident."]
#[derive(Default, Debug, Clone)]
pub struct NutritionOrder {
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
    #[doc = "Identifiers assigned to this order by the order sender or by the order receiver."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this NutritionOrder."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, orderset or other definition that is adhered to in whole or in part by this NutritionOrder."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "The URL pointing to a protocol, guideline, orderset or other definition that is adhered to in whole or in part by this NutritionOrder."]
    pub r#instantiates: Vec<super::super::types::Uri>,
    #[doc = "The workflow status of the nutrition order/request."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the level of authority/intentionality associated with the NutrionOrder and where the request fits into the workflow chain."]
    pub r#intent: super::super::types::Code,
    #[doc = "The person (patient) who needs the nutrition order for an oral diet, nutritional supplement and/or enteral or formula feeding."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "An encounter that provides additional information about the healthcare context in which this request is made."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date and time that this nutrition order was requested."]
    pub r#date_time: super::super::types::DateTime,
    #[doc = "The practitioner that holds legal responsibility for ordering the diet, nutritional supplement, or formula feedings."]
    pub r#orderer: Option<Box<super::super::types::Reference>>,
    #[doc = "A link to a record of allergies or intolerances  which should be included in the nutrition order."]
    pub r#allergy_intolerance: Vec<Box<super::super::types::Reference>>,
    #[doc = "This modifier is used to convey order-specific modifiers about the type of food that should be given. These can be derived from patient allergies, intolerances, or preferences such as Halal, Vegan or Kosher. This modifier applies to the entire nutrition order inclusive of the oral diet, nutritional supplements and enteral formula feedings."]
    pub r#food_preference_modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "This modifier is used to convey Order-specific modifier about the type of oral food or oral fluids that should not be given. These can be derived from patient allergies, intolerances, or preferences such as No Red Meat, No Soy or No Wheat or  Gluten-Free.  While it should not be necessary to repeat allergy or intolerance information captured in the referenced AllergyIntolerance resource in the excludeFoodModifier, this element may be used to convey additional specificity related to foods that should be eliminated from the patient’s diet for any reason.  This modifier applies to the entire nutrition order inclusive of the oral diet, nutritional supplements and enteral formula feedings."]
    pub r#exclude_food_modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Diet given orally in contrast to enteral (tube) feeding."]
    pub r#oral_diet: Option<NutritionOrderOralDiet>,
    #[doc = "Oral nutritional products given in order to add further nutritional value to the patient's diet."]
    pub r#supplement: Vec<NutritionOrderSupplement>,
    #[doc = "Feeding provided through the gastrointestinal tract via a tube, catheter, or stoma that delivers nutrition distal to the oral cavity."]
    pub r#enteral_formula: Option<NutritionOrderEnteralFormula>,
    #[doc = "Comments made about the {{title}} by the requester, performer, subject or other participants."]
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
                let some = Ok(some)?;
                state.serialize_entry("implicitRules", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("language", &some)?;
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
            let values = self
                .r#instantiates_canonical
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
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
            let values = self
                .r#instantiates_uri
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
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
            let values = self
                .r#instantiates
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
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
            let some = Ok(some)?;
            state.serialize_entry("status", &some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if let Some(some) = self.r#intent.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("intent", &some)?;
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
            let some = Ok(some)?;
            state.serialize_entry("dateTime", &some)?;
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
            #[serde(rename = "instantiatesCanonical")]
            InstantiatesCanonical,
            #[serde(rename = "_instantiatesCanonical")]
            InstantiatesCanonicalPrimitiveElement,
            #[serde(rename = "instantiatesUri")]
            InstantiatesUri,
            #[serde(rename = "_instantiatesUri")]
            InstantiatesUriPrimitiveElement,
            #[serde(rename = "instantiates")]
            Instantiates,
            #[serde(rename = "_instantiates")]
            InstantiatesPrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "intent")]
            Intent,
            #[serde(rename = "_intent")]
            IntentPrimitiveElement,
            #[serde(rename = "patient")]
            Patient,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "dateTime")]
            DateTime,
            #[serde(rename = "_dateTime")]
            DateTimePrimitiveElement,
            #[serde(rename = "orderer")]
            Orderer,
            #[serde(rename = "allergyIntolerance")]
            AllergyIntolerance,
            #[serde(rename = "foodPreferenceModifier")]
            FoodPreferenceModifier,
            #[serde(rename = "excludeFoodModifier")]
            ExcludeFoodModifier,
            #[serde(rename = "oralDiet")]
            OralDiet,
            #[serde(rename = "supplement")]
            Supplement,
            #[serde(rename = "enteralFormula")]
            EnteralFormula,
            #[serde(rename = "note")]
            Note,
            Unknown(String),
        }
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
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "NutritionOrder" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"NutritionOrder",
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
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::InstantiatesCanonical => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#instantiates_canonical.get_or_insert(
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
                                        "instantiatesCanonical",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::InstantiatesCanonicalPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#instantiates_canonical.get_or_insert(
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
                                        "_instantiatesCanonical",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::InstantiatesUri => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#instantiates_uri.get_or_insert(
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
                                        "instantiatesUri",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::InstantiatesUriPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#instantiates_uri.get_or_insert(
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
                                        "_instantiatesUri",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::Instantiates => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#instantiates.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("instantiates"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::InstantiatesPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#instantiates.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_instantiates"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::Status => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::StatusPrimitiveElement => {
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
                            Field::Intent => {
                                let some = r#intent.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intent"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::IntentPrimitiveElement => {
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
                            Field::Patient => {
                                if r#patient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patient"));
                                }
                                r#patient = Some(map_access.next_value()?);
                            }
                            Field::Encounter => {
                                if r#encounter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("encounter"));
                                }
                                r#encounter = Some(map_access.next_value()?);
                            }
                            Field::DateTime => {
                                let some = r#date_time.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dateTime"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DateTimePrimitiveElement => {
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
                            Field::Orderer => {
                                if r#orderer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("orderer"));
                                }
                                r#orderer = Some(map_access.next_value()?);
                            }
                            Field::AllergyIntolerance => {
                                if r#allergy_intolerance.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "allergyIntolerance",
                                    ));
                                }
                                r#allergy_intolerance = Some(map_access.next_value()?);
                            }
                            Field::FoodPreferenceModifier => {
                                if r#food_preference_modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "foodPreferenceModifier",
                                    ));
                                }
                                r#food_preference_modifier = Some(map_access.next_value()?);
                            }
                            Field::ExcludeFoodModifier => {
                                if r#exclude_food_modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "excludeFoodModifier",
                                    ));
                                }
                                r#exclude_food_modifier = Some(map_access.next_value()?);
                            }
                            Field::OralDiet => {
                                if r#oral_diet.is_some() {
                                    return Err(serde::de::Error::duplicate_field("oralDiet"));
                                }
                                r#oral_diet = Some(map_access.next_value()?);
                            }
                            Field::Supplement => {
                                if r#supplement.is_some() {
                                    return Err(serde::de::Error::duplicate_field("supplement"));
                                }
                                r#supplement = Some(map_access.next_value()?);
                            }
                            Field::EnteralFormula => {
                                if r#enteral_formula.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "enteralFormula",
                                    ));
                                }
                                r#enteral_formula = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
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
                                ));
                            },
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
                        r#status: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#intent: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#intent.unwrap_or(Default::default())
                        } else {
                            r#intent.ok_or(serde::de::Error::missing_field("intent"))?
                        },
                        r#patient: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#patient.unwrap_or(Default::default())
                        } else {
                            r#patient.ok_or(serde::de::Error::missing_field("patient"))?
                        },
                        r#encounter,
                        r#date_time: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#date_time.unwrap_or(Default::default())
                        } else {
                            r#date_time.ok_or(serde::de::Error::missing_field("dateTime"))?
                        },
                        r#orderer,
                        r#allergy_intolerance: r#allergy_intolerance.unwrap_or(vec![]),
                        r#food_preference_modifier: r#food_preference_modifier.unwrap_or(vec![]),
                        r#exclude_food_modifier: r#exclude_food_modifier.unwrap_or(vec![]),
                        r#oral_diet,
                        r#supplement: r#supplement.unwrap_or(vec![]),
                        r#enteral_formula,
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
