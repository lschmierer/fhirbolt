// Generated on 2022-10-14 by fhirbolt-codegen v0.1.0
#[doc = "The actual characteristic value corresponding to the type."]
#[derive(Debug, Clone)]
pub enum NutritionProductProductCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Base64Binary(Box<super::super::types::Base64Binary>),
    Attachment(Box<super::super::types::Attachment>),
    Boolean(Box<super::super::types::Boolean>),
    Invalid,
}
impl Default for NutritionProductProductCharacteristicValue {
    fn default() -> NutritionProductProductCharacteristicValue {
        NutritionProductProductCharacteristicValue::Invalid
    }
}
#[doc = "The product's nutritional information expressed by the nutrients."]
#[derive(Default, Debug, Clone)]
pub struct NutritionProductNutrient {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The (relevant) nutrients in the product."]
    pub r#item: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The amount of nutrient expressed in one or more units: X per pack / per serving / per dose."]
    pub r#amount: Vec<Box<super::super::types::Ratio>>,
}
impl crate::AnyResource for NutritionProductNutrient {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for NutritionProductNutrient {
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
        if let Some(some) = self.r#item.as_ref() {
            state.serialize_entry("item", some)?;
        }
        if !self.r#amount.is_empty() {
            state.serialize_entry("amount", &self.r#amount)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for NutritionProductNutrient {
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
            #[serde(rename = "item")]
            Item,
            #[serde(rename = "amount")]
            Amount,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NutritionProductNutrient;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionProductNutrient")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<NutritionProductNutrient, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#item: Option<Box<super::super::types::CodeableReference>> = None;
                let mut r#amount: Option<Vec<Box<super::super::types::Ratio>>> = None;
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
                            Field::Item => {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("item"));
                                }
                                r#item = Some(map_access.next_value()?);
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
                                    &["id", "extension", "modifierExtension", "item", "amount"],
                                ));
                            },
                        }
                    }
                    Ok(NutritionProductNutrient {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#item,
                        r#amount: r#amount.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Ingredients contained in this product."]
#[derive(Default, Debug, Clone)]
pub struct NutritionProductIngredient {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The ingredient contained in the product."]
    pub r#item: Box<super::super::types::CodeableReference>,
    #[doc = "The amount of ingredient that is in the product."]
    pub r#amount: Vec<Box<super::super::types::Ratio>>,
}
impl serde::ser::Serialize for NutritionProductIngredient {
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
        state.serialize_entry("item", &self.r#item)?;
        if !self.r#amount.is_empty() {
            state.serialize_entry("amount", &self.r#amount)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for NutritionProductIngredient {
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
            #[serde(rename = "item")]
            Item,
            #[serde(rename = "amount")]
            Amount,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NutritionProductIngredient;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionProductIngredient")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<NutritionProductIngredient, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#item: Option<Box<super::super::types::CodeableReference>> = None;
                let mut r#amount: Option<Vec<Box<super::super::types::Ratio>>> = None;
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
                            Field::Item => {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("item"));
                                }
                                r#item = Some(map_access.next_value()?);
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
                                    &["id", "extension", "modifierExtension", "item", "amount"],
                                ));
                            },
                        }
                    }
                    Ok(NutritionProductIngredient {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#item: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#item.unwrap_or(Default::default())
                        } else {
                            r#item.ok_or(serde::de::Error::missing_field("item"))?
                        },
                        r#amount: r#amount.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Specifies descriptive properties of the nutrition product."]
#[derive(Default, Debug, Clone)]
pub struct NutritionProductProductCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code specifying which characteristic of the product is being described (for example, colour, shape)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The actual characteristic value corresponding to the type."]
    pub r#value: NutritionProductProductCharacteristicValue,
}
impl serde::ser::Serialize for NutritionProductProductCharacteristic {
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
        state.serialize_entry("type", &self.r#type)?;
        match self.r#value {
            NutritionProductProductCharacteristicValue::CodeableConcept(ref value) => {
                state.serialize_entry("valueCodeableConcept", value)?;
            }
            NutritionProductProductCharacteristicValue::String(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("valueString", &some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueString", &primitive_element)?;
                }
            }
            NutritionProductProductCharacteristicValue::Quantity(ref value) => {
                state.serialize_entry("valueQuantity", value)?;
            }
            NutritionProductProductCharacteristicValue::Base64Binary(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("valueBase64Binary", &some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueBase64Binary", &primitive_element)?;
                }
            }
            NutritionProductProductCharacteristicValue::Attachment(ref value) => {
                state.serialize_entry("valueAttachment", value)?;
            }
            NutritionProductProductCharacteristicValue::Boolean(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("valueBoolean", &some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueBoolean", &primitive_element)?;
                }
            }
            NutritionProductProductCharacteristicValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for NutritionProductProductCharacteristic {
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
            #[serde(rename = "valueCodeableConcept")]
            ValueCodeableConcept,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueBase64Binary")]
            ValueBase64Binary,
            #[serde(rename = "_valueBase64Binary")]
            ValueBase64BinaryPrimitiveElement,
            #[serde(rename = "valueAttachment")]
            ValueAttachment,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NutritionProductProductCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionProductProductCharacteristic")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<NutritionProductProductCharacteristic, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<NutritionProductProductCharacteristicValue> = None;
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
                            Field::ValueCodeableConcept => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCodeableConcept",
                                    ));
                                }
                                r#value = Some(
                                    NutritionProductProductCharacteristicValue::CodeableConcept(
                                        map_access.next_value()?,
                                    ),
                                );
                            }
                            Field::ValueString => {
                                let r#enum = r#value.get_or_insert(
                                    NutritionProductProductCharacteristicValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let NutritionProductProductCharacteristicValue::String(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueStringPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    NutritionProductProductCharacteristicValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let NutritionProductProductCharacteristicValue::String(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueString",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value =
                                    Some(NutritionProductProductCharacteristicValue::Quantity(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::ValueBase64Binary => {
                                let r#enum = r#value.get_or_insert(
                                    NutritionProductProductCharacteristicValue::Base64Binary(
                                        Default::default(),
                                    ),
                                );
                                if let NutritionProductProductCharacteristicValue::Base64Binary(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBase64Binary",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueBase64BinaryPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    NutritionProductProductCharacteristicValue::Base64Binary(
                                        Default::default(),
                                    ),
                                );
                                if let NutritionProductProductCharacteristicValue::Base64Binary(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBase64Binary",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            }
                            Field::ValueAttachment => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAttachment",
                                    ));
                                }
                                r#value =
                                    Some(NutritionProductProductCharacteristicValue::Attachment(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::ValueBoolean => {
                                let r#enum = r#value.get_or_insert(
                                    NutritionProductProductCharacteristicValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let NutritionProductProductCharacteristicValue::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueBooleanPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    NutritionProductProductCharacteristicValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let NutritionProductProductCharacteristicValue::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBoolean",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
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
                                        "valueCodeableConcept",
                                        "valueString",
                                        "valueQuantity",
                                        "valueBase64Binary",
                                        "valueAttachment",
                                        "valueBoolean",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(NutritionProductProductCharacteristic {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#value: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Conveys instance-level information about this product item. One or several physical, countable instances or occurrences of the product."]
#[derive(Default, Debug, Clone)]
pub struct NutritionProductInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The amount of items or instances that the resource considers, for instance when referring to 2 identical units together."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The identifier for the physical instance, typically a serial number."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identification of the batch or lot of the product."]
    pub r#lot_number: Option<super::super::types::String>,
    #[doc = "The time after which the product is no longer expected to be in proper condition, or its use is not advised or not allowed."]
    pub r#expiry: Option<super::super::types::DateTime>,
    #[doc = "The time after which the product is no longer expected to be in proper condition, or its use is not advised or not allowed."]
    pub r#use_by: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for NutritionProductInstance {
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
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if let Some(some) = self.r#lot_number.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("lotNumber", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_lotNumber", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#expiry.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("expiry", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_expiry", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#use_by.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("useBy", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_useBy", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for NutritionProductInstance {
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
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "lotNumber")]
            LotNumber,
            #[serde(rename = "_lotNumber")]
            LotNumberPrimitiveElement,
            #[serde(rename = "expiry")]
            Expiry,
            #[serde(rename = "_expiry")]
            ExpiryPrimitiveElement,
            #[serde(rename = "useBy")]
            UseBy,
            #[serde(rename = "_useBy")]
            UseByPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NutritionProductInstance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionProductInstance")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<NutritionProductInstance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#lot_number: Option<super::super::types::String> = None;
                let mut r#expiry: Option<super::super::types::DateTime> = None;
                let mut r#use_by: Option<super::super::types::DateTime> = None;
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
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::LotNumber => {
                                let some = r#lot_number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lotNumber"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::LotNumberPrimitiveElement => {
                                let some = r#lot_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_lotNumber"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Expiry => {
                                let some = r#expiry.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("expiry"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ExpiryPrimitiveElement => {
                                let some = r#expiry.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_expiry"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::UseBy => {
                                let some = r#use_by.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("useBy"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::UseByPrimitiveElement => {
                                let some = r#use_by.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_useBy"));
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
                                        "quantity",
                                        "identifier",
                                        "lotNumber",
                                        "expiry",
                                        "useBy",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(NutritionProductInstance {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#quantity,
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#lot_number,
                        r#expiry,
                        r#use_by,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A food or fluid product that is consumed by patients."]
#[derive(Default, Debug, Clone)]
pub struct NutritionProduct {
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
    #[doc = "The current state of the product."]
    pub r#status: super::super::types::Code,
    #[doc = "Nutrition products can have different classifications - according to its nutritional properties, preparation methods, etc."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The code assigned to the product, for example a manufacturer number or other terminology."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The organisation (manufacturer, representative or legal authorisation holder) that is responsible for the device."]
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    #[doc = "The product's nutritional information expressed by the nutrients."]
    pub r#nutrient: Vec<NutritionProductNutrient>,
    #[doc = "Ingredients contained in this product."]
    pub r#ingredient: Vec<NutritionProductIngredient>,
    #[doc = "Allergens that are known or suspected to be a part of this nutrition product."]
    pub r#known_allergen: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "Specifies descriptive properties of the nutrition product."]
    pub r#product_characteristic: Vec<NutritionProductProductCharacteristic>,
    #[doc = "Conveys instance-level information about this product item. One or several physical, countable instances or occurrences of the product."]
    pub r#instance: Option<NutritionProductInstance>,
    #[doc = "Comments made about the product."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for NutritionProduct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "NutritionProduct")?;
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
        if !self.r#category.is_empty() {
            state.serialize_entry("category", &self.r#category)?;
        }
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if !self.r#manufacturer.is_empty() {
            state.serialize_entry("manufacturer", &self.r#manufacturer)?;
        }
        if !self.r#nutrient.is_empty() {
            state.serialize_entry("nutrient", &self.r#nutrient)?;
        }
        if !self.r#ingredient.is_empty() {
            state.serialize_entry("ingredient", &self.r#ingredient)?;
        }
        if !self.r#known_allergen.is_empty() {
            state.serialize_entry("knownAllergen", &self.r#known_allergen)?;
        }
        if !self.r#product_characteristic.is_empty() {
            state.serialize_entry("productCharacteristic", &self.r#product_characteristic)?;
        }
        if let Some(some) = self.r#instance.as_ref() {
            state.serialize_entry("instance", some)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for NutritionProduct {
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
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "manufacturer")]
            Manufacturer,
            #[serde(rename = "nutrient")]
            Nutrient,
            #[serde(rename = "ingredient")]
            Ingredient,
            #[serde(rename = "knownAllergen")]
            KnownAllergen,
            #[serde(rename = "productCharacteristic")]
            ProductCharacteristic,
            #[serde(rename = "instance")]
            Instance,
            #[serde(rename = "note")]
            Note,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NutritionProduct;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionProduct")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<NutritionProduct, V::Error>
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
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#manufacturer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#nutrient: Option<Vec<NutritionProductNutrient>> = None;
                let mut r#ingredient: Option<Vec<NutritionProductIngredient>> = None;
                let mut r#known_allergen: Option<Vec<Box<super::super::types::CodeableReference>>> =
                    None;
                let mut r#product_characteristic: Option<
                    Vec<NutritionProductProductCharacteristic>,
                > = None;
                let mut r#instance: Option<NutritionProductInstance> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "NutritionProduct" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"NutritionProduct",
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
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Manufacturer => {
                                if r#manufacturer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manufacturer"));
                                }
                                r#manufacturer = Some(map_access.next_value()?);
                            }
                            Field::Nutrient => {
                                if r#nutrient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("nutrient"));
                                }
                                r#nutrient = Some(map_access.next_value()?);
                            }
                            Field::Ingredient => {
                                if r#ingredient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ingredient"));
                                }
                                r#ingredient = Some(map_access.next_value()?);
                            }
                            Field::KnownAllergen => {
                                if r#known_allergen.is_some() {
                                    return Err(serde::de::Error::duplicate_field("knownAllergen"));
                                }
                                r#known_allergen = Some(map_access.next_value()?);
                            }
                            Field::ProductCharacteristic => {
                                if r#product_characteristic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productCharacteristic",
                                    ));
                                }
                                r#product_characteristic = Some(map_access.next_value()?);
                            }
                            Field::Instance => {
                                if r#instance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("instance"));
                                }
                                r#instance = Some(map_access.next_value()?);
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
                                        "status",
                                        "category",
                                        "code",
                                        "manufacturer",
                                        "nutrient",
                                        "ingredient",
                                        "knownAllergen",
                                        "productCharacteristic",
                                        "instance",
                                        "note",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(NutritionProduct {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#status: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#category: r#category.unwrap_or(vec![]),
                        r#code,
                        r#manufacturer: r#manufacturer.unwrap_or(vec![]),
                        r#nutrient: r#nutrient.unwrap_or(vec![]),
                        r#ingredient: r#ingredient.unwrap_or(vec![]),
                        r#known_allergen: r#known_allergen.unwrap_or(vec![]),
                        r#product_characteristic: r#product_characteristic.unwrap_or(vec![]),
                        r#instance,
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
