// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4b::resources::NutritionOrderOralDietNutrient;
impl serde::ser::Serialize for SerializationContext<&NutritionOrderOralDietNutrient> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "NutritionOrder.oralDiet.nutrient", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if let Some(some) = self.value.r#modifier.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("modifier", ctx)));
        }
        if let Some(some) = self.value.r#amount.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("amount", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<NutritionOrderOralDietNutrient>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<NutritionOrderOralDietNutrient>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<NutritionOrderOralDietNutrient>
{
    type Value = NutritionOrderOralDietNutrient;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<NutritionOrderOralDietNutrient>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
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
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "modifier", "amount"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#modifier: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#amount: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Modifier => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#modifier = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#amount = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
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
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<NutritionOrderOralDietNutrient>>
{
    type Value = Box<NutritionOrderOralDietNutrient>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<NutritionOrderOralDietNutrient>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<NutritionOrderOralDietNutrient>>
{
    type Value = Vec<NutritionOrderOralDietNutrient>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<NutritionOrderOralDietNutrient>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<NutritionOrderOralDietNutrient>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<NutritionOrderOralDietNutrient> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::resources::NutritionOrderOralDietTexture;
impl serde::ser::Serialize for SerializationContext<&NutritionOrderOralDietTexture> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "NutritionOrder.oralDiet.texture", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if let Some(some) = self.value.r#modifier.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("modifier", ctx)));
        }
        if let Some(some) = self.value.r#food_type.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("foodType", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<NutritionOrderOralDietTexture>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<NutritionOrderOralDietTexture>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<NutritionOrderOralDietTexture>
{
    type Value = NutritionOrderOralDietTexture;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<NutritionOrderOralDietTexture>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
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
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "modifier",
                            "foodType",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#modifier: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#food_type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Modifier => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#modifier = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::FoodType => {
                            if r#food_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("foodType"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#food_type = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
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
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<NutritionOrderOralDietTexture>>
{
    type Value = Box<NutritionOrderOralDietTexture>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<NutritionOrderOralDietTexture>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<NutritionOrderOralDietTexture>>
{
    type Value = Vec<NutritionOrderOralDietTexture>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<NutritionOrderOralDietTexture>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<NutritionOrderOralDietTexture>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<NutritionOrderOralDietTexture> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::resources::NutritionOrderOralDiet;
impl serde::ser::Serialize for SerializationContext<&NutritionOrderOralDiet> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "NutritionOrder.oralDiet", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if !self.value.r#type.is_empty() {
            tri!(self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx)));
        }
        if !self.value.r#schedule.is_empty() {
            tri!(self.with_context(&self.value.r#schedule, |ctx| state
                .serialize_entry("schedule", ctx)));
        }
        if !self.value.r#nutrient.is_empty() {
            tri!(self.with_context(&self.value.r#nutrient, |ctx| state
                .serialize_entry("nutrient", ctx)));
        }
        if !self.value.r#texture.is_empty() {
            tri!(self.with_context(&self.value.r#texture, |ctx| state
                .serialize_entry("texture", ctx)));
        }
        if !self.value.r#fluid_consistency_type.is_empty() {
            tri!(
                self.with_context(&self.value.r#fluid_consistency_type, |ctx| state
                    .serialize_entry("fluidConsistencyType", ctx))
            );
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#instruction.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("instruction", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_instruction", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#instruction.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("instruction", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<NutritionOrderOralDiet>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<NutritionOrderOralDiet>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<NutritionOrderOralDiet> {
    type Value = NutritionOrderOralDiet;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<NutritionOrderOralDiet>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = NutritionOrderOralDiet;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionOrderOralDiet")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<NutritionOrderOralDiet, V::Error>
            where
                V: serde::de::MapAccess<'de>,
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
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#type: Option<Vec<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#schedule: Option<Vec<fhirbolt_model::r4b::types::Timing>> = None;
                let mut r#nutrient: Option<
                    Vec<fhirbolt_model::r4b::resources::NutritionOrderOralDietNutrient>,
                > = None;
                let mut r#texture: Option<
                    Vec<fhirbolt_model::r4b::resources::NutritionOrderOralDietTexture>,
                > = None;
                let mut r#fluid_consistency_type: Option<
                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#instruction: Option<fhirbolt_model::r4b::types::String> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Schedule => {
                            if self.0.from == crate::context::Format::Json {
                                if r#schedule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("schedule"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Timing>,
                                > = self.0.transmute();
                                r#schedule = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#schedule.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Timing,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Nutrient => {
                            if self.0.from == crate::context::Format::Json {
                                if r#nutrient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("nutrient"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: NutritionOrderOralDietNutrient >> = self . 0 . transmute () ;
                                r#nutrient = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#nutrient.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::resources::NutritionOrderOralDietNutrient,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Texture => {
                            if self.0.from == crate::context::Format::Json {
                                if r#texture.is_some() {
                                    return Err(serde::de::Error::duplicate_field("texture"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: NutritionOrderOralDietTexture >> = self . 0 . transmute () ;
                                r#texture = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#texture.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::resources::NutritionOrderOralDietTexture,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::FluidConsistencyType => {
                            if self.0.from == crate::context::Format::Json {
                                if r#fluid_consistency_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fluidConsistencyType",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#fluid_consistency_type =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec =
                                    r#fluid_consistency_type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Instruction => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("instruction"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("instruction"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#instruction =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::InstructionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#instruction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_instruction"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("instruction");
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
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
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<NutritionOrderOralDiet>>
{
    type Value = Box<NutritionOrderOralDiet>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<NutritionOrderOralDiet>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<NutritionOrderOralDiet>>
{
    type Value = Vec<NutritionOrderOralDiet>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<NutritionOrderOralDiet>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<NutritionOrderOralDiet>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<NutritionOrderOralDiet> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::resources::NutritionOrderSupplement;
impl serde::ser::Serialize for SerializationContext<&NutritionOrderSupplement> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "NutritionOrder.supplement", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if let Some(some) = self.value.r#type.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("type", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#product_name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("productName", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_productName", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#product_name.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("productName", ctx)));
        }
        if !self.value.r#schedule.is_empty() {
            tri!(self.with_context(&self.value.r#schedule, |ctx| state
                .serialize_entry("schedule", ctx)));
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("quantity", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#instruction.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("instruction", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_instruction", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#instruction.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("instruction", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<NutritionOrderSupplement>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<NutritionOrderSupplement>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<NutritionOrderSupplement>
{
    type Value = NutritionOrderSupplement;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<NutritionOrderSupplement>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = NutritionOrderSupplement;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionOrderSupplement")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<NutritionOrderSupplement, V::Error>
            where
                V: serde::de::MapAccess<'de>,
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
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#product_name: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#schedule: Option<Vec<fhirbolt_model::r4b::types::Timing>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                let mut r#instruction: Option<fhirbolt_model::r4b::types::String> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::ProductName => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#product_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("productName"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#product_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("productName"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#product_name =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ProductNamePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#product_name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_productName"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("productName");
                            }
                        }
                        Field::Schedule => {
                            if self.0.from == crate::context::Format::Json {
                                if r#schedule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("schedule"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Timing>,
                                > = self.0.transmute();
                                r#schedule = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#schedule.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Timing,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Instruction => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("instruction"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("instruction"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#instruction =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::InstructionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#instruction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_instruction"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("instruction");
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
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
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<NutritionOrderSupplement>>
{
    type Value = Box<NutritionOrderSupplement>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<NutritionOrderSupplement>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<NutritionOrderSupplement>>
{
    type Value = Vec<NutritionOrderSupplement>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<NutritionOrderSupplement>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<NutritionOrderSupplement>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<NutritionOrderSupplement> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::resources::NutritionOrderEnteralFormulaAdministration;
impl serde::ser::Serialize for SerializationContext<&NutritionOrderEnteralFormulaAdministration> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "NutritionOrder.enteralFormula.administration", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if let Some(some) = self.value.r#schedule.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("schedule", ctx)));
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("quantity", ctx)));
        }
        {
            use fhirbolt_model::r4b::resources::NutritionOrderEnteralFormulaAdministrationRate as _Enum;
            if let Some(some) = self.value.r#rate.as_ref() {
                match some {
                    _Enum::Quantity(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("rateQuantity", ctx)));
                    }
                    _Enum::Ratio(ref value) => {
                        tri!(
                            self.with_context(value, |ctx| state.serialize_entry("rateRatio", ctx))
                        );
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("rate is invalid")),
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<NutritionOrderEnteralFormulaAdministration>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<NutritionOrderEnteralFormulaAdministration>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<NutritionOrderEnteralFormulaAdministration>
{
    type Value = NutritionOrderEnteralFormulaAdministration;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<NutritionOrderEnteralFormulaAdministration>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
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
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#schedule: Option<Box<fhirbolt_model::r4b::types::Timing>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                let mut r#rate: Option<
                    fhirbolt_model::r4b::resources::NutritionOrderEnteralFormulaAdministrationRate,
                > = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Schedule => {
                            if r#schedule.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Timing>,
                            > = self.0.transmute();
                            r#schedule = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::RateQuantity => {
                            use fhirbolt_model::r4b::resources::NutritionOrderEnteralFormulaAdministrationRate as _Enum;
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#rate = Some(_Enum::Quantity(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::RateRatio => {
                            use fhirbolt_model::r4b::resources::NutritionOrderEnteralFormulaAdministrationRate as _Enum;
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateRatio"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Ratio>,
                            > = self.0.transmute();
                            r#rate = Some(_Enum::Ratio(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
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
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<NutritionOrderEnteralFormulaAdministration>>
{
    type Value = Box<NutritionOrderEnteralFormulaAdministration>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<NutritionOrderEnteralFormulaAdministration>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<NutritionOrderEnteralFormulaAdministration>>
{
    type Value = Vec<NutritionOrderEnteralFormulaAdministration>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<NutritionOrderEnteralFormulaAdministration>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<NutritionOrderEnteralFormulaAdministration>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    NutritionOrderEnteralFormulaAdministration,
                > = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::resources::NutritionOrderEnteralFormula;
impl serde::ser::Serialize for SerializationContext<&NutritionOrderEnteralFormula> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "NutritionOrder.enteralFormula", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if let Some(some) = self.value.r#base_formula_type.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("baseFormulaType", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#base_formula_product_name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("baseFormulaProductName", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_baseFormulaProductName", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#base_formula_product_name.as_ref() {
            tri!(self.with_context(some, |ctx| state
                .serialize_entry("baseFormulaProductName", ctx)));
        }
        if let Some(some) = self.value.r#additive_type.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("additiveType", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#additive_product_name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("additiveProductName", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_additiveProductName", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#additive_product_name.as_ref() {
            tri!(self.with_context(some, |ctx| state
                .serialize_entry("additiveProductName", ctx)));
        }
        if let Some(some) = self.value.r#caloric_density.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("caloricDensity", ctx)));
        }
        if let Some(some) = self.value.r#routeof_administration.as_ref() {
            tri!(self.with_context(some, |ctx| state
                .serialize_entry("routeofAdministration", ctx)));
        }
        if !self.value.r#administration.is_empty() {
            tri!(self.with_context(&self.value.r#administration, |ctx| state
                .serialize_entry("administration", ctx)));
        }
        if let Some(some) = self.value.r#max_volume_to_deliver.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("maxVolumeToDeliver", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#administration_instruction.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("administrationInstruction", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_administrationInstruction", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#administration_instruction.as_ref() {
            tri!(self.with_context(some, |ctx| state
                .serialize_entry("administrationInstruction", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<NutritionOrderEnteralFormula>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<NutritionOrderEnteralFormula>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<NutritionOrderEnteralFormula>
{
    type Value = NutritionOrderEnteralFormula;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<NutritionOrderEnteralFormula>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
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
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#base_formula_type: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#base_formula_product_name: Option<fhirbolt_model::r4b::types::String> =
                    None;
                let mut r#additive_type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#additive_product_name: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#caloric_density: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                let mut r#routeof_administration: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#administration: Option<
                    Vec<fhirbolt_model::r4b::resources::NutritionOrderEnteralFormulaAdministration>,
                > = None;
                let mut r#max_volume_to_deliver: Option<Box<fhirbolt_model::r4b::types::Quantity>> =
                    None;
                let mut r#administration_instruction: Option<fhirbolt_model::r4b::types::String> =
                    None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::BaseFormulaType => {
                            if r#base_formula_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseFormulaType"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#base_formula_type =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::BaseFormulaProductName => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#base_formula_product_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "baseFormulaProductName",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#base_formula_product_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "baseFormulaProductName",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#base_formula_product_name =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::BaseFormulaProductNamePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#base_formula_product_name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_baseFormulaProductName",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("baseFormulaProductName");
                            }
                        }
                        Field::AdditiveType => {
                            if r#additive_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("additiveType"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#additive_type =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::AdditiveProductName => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#additive_product_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "additiveProductName",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#additive_product_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "additiveProductName",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#additive_product_name =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::AdditiveProductNamePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#additive_product_name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_additiveProductName",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("additiveProductName");
                            }
                        }
                        Field::CaloricDensity => {
                            if r#caloric_density.is_some() {
                                return Err(serde::de::Error::duplicate_field("caloricDensity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#caloric_density =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::RouteofAdministration => {
                            if r#routeof_administration.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "routeofAdministration",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#routeof_administration =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Administration => {
                            if self.0.from == crate::context::Format::Json {
                                if r#administration.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "administration",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: NutritionOrderEnteralFormulaAdministration >> = self . 0 . transmute () ;
                                r#administration =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#administration.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4b :: resources :: NutritionOrderEnteralFormulaAdministration > = self . 0 . transmute () ;
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::MaxVolumeToDeliver => {
                            if r#max_volume_to_deliver.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxVolumeToDeliver",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#max_volume_to_deliver =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::AdministrationInstruction => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#administration_instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "administrationInstruction",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#administration_instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "administrationInstruction",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#administration_instruction =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::AdministrationInstructionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#administration_instruction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_administrationInstruction",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("administrationInstruction");
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
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
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<NutritionOrderEnteralFormula>>
{
    type Value = Box<NutritionOrderEnteralFormula>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<NutritionOrderEnteralFormula>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<NutritionOrderEnteralFormula>>
{
    type Value = Vec<NutritionOrderEnteralFormula>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<NutritionOrderEnteralFormula>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<NutritionOrderEnteralFormula>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<NutritionOrderEnteralFormula> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::resources::NutritionOrder;
impl crate::Resource for NutritionOrder {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4B;
}
impl serde::ser::Serialize for SerializationContext<&NutritionOrder> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "NutritionOrder", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        tri!(state.serialize_entry("resourceType", "NutritionOrder"));
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("id", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self
                        .with_context(&primitive_element, |ctx| state.serialize_entry("_id", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#id.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("id", ctx)));
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("meta", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("implicitRules", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_implicitRules", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#implicit_rules.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("language", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_language", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#language.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("language", ctx)));
        }
        if let Some(some) = self.value.r#text.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("text", ctx)));
        }
        if !self.value.r#contained.is_empty() {
            tri!(self.with_context(&self.value.r#contained, |ctx| state
                .serialize_entry("contained", ctx)));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if !self.value.r#identifier.is_empty() {
            tri!(self.with_context(&self.value.r#identifier, |ctx| state
                .serialize_entry("identifier", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#instantiates_canonical.is_empty() {
                let values = tri!(self
                    .value
                    .r#instantiates_canonical
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("instantiatesCanonical", &values));
                }
                let requires_elements = self
                    .value
                    .r#instantiates_canonical
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#instantiates_canonical
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    tri!(self.with_context(&primitive_elements, |ctx| state
                        .serialize_entry("_instantiatesCanonical", ctx)));
                }
            }
        } else if !self.value.r#instantiates_canonical.is_empty() {
            tri!(
                self.with_context(&self.value.r#instantiates_canonical, |ctx| state
                    .serialize_entry("instantiatesCanonical", ctx))
            );
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#instantiates_uri.is_empty() {
                let values = tri!(self
                    .value
                    .r#instantiates_uri
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("instantiatesUri", &values));
                }
                let requires_elements = self
                    .value
                    .r#instantiates_uri
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#instantiates_uri
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    tri!(self.with_context(&primitive_elements, |ctx| state
                        .serialize_entry("_instantiatesUri", ctx)));
                }
            }
        } else if !self.value.r#instantiates_uri.is_empty() {
            tri!(
                self.with_context(&self.value.r#instantiates_uri, |ctx| state
                    .serialize_entry("instantiatesUri", ctx))
            );
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#instantiates.is_empty() {
                let values = tri!(self
                    .value
                    .r#instantiates
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("instantiates", &values));
                }
                let requires_elements = self
                    .value
                    .r#instantiates
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#instantiates
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    tri!(self.with_context(&primitive_elements, |ctx| state
                        .serialize_entry("_instantiates", ctx)));
                }
            }
        } else if !self.value.r#instantiates.is_empty() {
            tri!(self.with_context(&self.value.r#instantiates, |ctx| state
                .serialize_entry("instantiates", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#status.id.as_deref() == Some("$invalid") {
                return missing_field_error("status");
            }
            if let Some(some) = self.value.r#status.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("status", &some?));
            }
            if self.value.r#status.id.is_some() || !self.value.r#status.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#status.id.as_ref(),
                    extension: &self.value.r#status.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_status", ctx)));
            }
        } else if self.value.r#status.id.as_deref() == Some("$invalid") {
            return missing_field_error("status");
        } else {
            tri!(self.with_context(&self.value.r#status, |ctx| state
                .serialize_entry("status", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#intent.id.as_deref() == Some("$invalid") {
                return missing_field_error("intent");
            }
            if let Some(some) = self.value.r#intent.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("intent", &some?));
            }
            if self.value.r#intent.id.is_some() || !self.value.r#intent.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#intent.id.as_ref(),
                    extension: &self.value.r#intent.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_intent", ctx)));
            }
        } else if self.value.r#intent.id.as_deref() == Some("$invalid") {
            return missing_field_error("intent");
        } else {
            tri!(self.with_context(&self.value.r#intent, |ctx| state
                .serialize_entry("intent", ctx)));
        }
        if self.value.r#patient.id.as_deref() == Some("$invalid") {
            return missing_field_error("patient");
        } else {
            tri!(self.with_context(&self.value.r#patient, |ctx| state
                .serialize_entry("patient", ctx)));
        }
        if let Some(some) = self.value.r#encounter.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("encounter", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#date_time.id.as_deref() == Some("$invalid") {
                return missing_field_error("dateTime");
            }
            if let Some(some) = self.value.r#date_time.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("dateTime", &some?));
            }
            if self.value.r#date_time.id.is_some() || !self.value.r#date_time.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#date_time.id.as_ref(),
                    extension: &self.value.r#date_time.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_dateTime", ctx)));
            }
        } else if self.value.r#date_time.id.as_deref() == Some("$invalid") {
            return missing_field_error("dateTime");
        } else {
            tri!(self.with_context(&self.value.r#date_time, |ctx| state
                .serialize_entry("dateTime", ctx)));
        }
        if let Some(some) = self.value.r#orderer.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("orderer", ctx)));
        }
        if !self.value.r#allergy_intolerance.is_empty() {
            tri!(
                self.with_context(&self.value.r#allergy_intolerance, |ctx| state
                    .serialize_entry("allergyIntolerance", ctx))
            );
        }
        if !self.value.r#food_preference_modifier.is_empty() {
            tri!(
                self.with_context(&self.value.r#food_preference_modifier, |ctx| state
                    .serialize_entry("foodPreferenceModifier", ctx))
            );
        }
        if !self.value.r#exclude_food_modifier.is_empty() {
            tri!(
                self.with_context(&self.value.r#exclude_food_modifier, |ctx| state
                    .serialize_entry("excludeFoodModifier", ctx))
            );
        }
        if let Some(some) = self.value.r#oral_diet.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("oralDiet", ctx)));
        }
        if !self.value.r#supplement.is_empty() {
            tri!(self.with_context(&self.value.r#supplement, |ctx| state
                .serialize_entry("supplement", ctx)));
        }
        if let Some(some) = self.value.r#enteral_formula.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("enteralFormula", ctx)));
        }
        if !self.value.r#note.is_empty() {
            tri!(self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<NutritionOrder>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<NutritionOrder>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<NutritionOrder> {
    type Value = NutritionOrder;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<NutritionOrder> {
    type Value = NutritionOrder;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<NutritionOrder>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = NutritionOrder;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("NutritionOrder")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<NutritionOrder, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "resourceType")]
                    ResourceType,
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "_id")]
                    IdPrimitiveElement,
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
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
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
                let mut r#id: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4b::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4b::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r4b::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r4b::types::Identifier>> = None;
                let mut r#instantiates_canonical: Option<
                    Vec<fhirbolt_model::r4b::types::Canonical>,
                > = None;
                let mut r#instantiates_uri: Option<Vec<fhirbolt_model::r4b::types::Uri>> = None;
                let mut r#instantiates: Option<Vec<fhirbolt_model::r4b::types::Uri>> = None;
                let mut r#status: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#intent: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#patient: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#encounter: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#date_time: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#orderer: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#allergy_intolerance: Option<Vec<fhirbolt_model::r4b::types::Reference>> =
                    None;
                let mut r#food_preference_modifier: Option<
                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#exclude_food_modifier: Option<
                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#oral_diet: Option<
                    fhirbolt_model::r4b::resources::NutritionOrderOralDiet,
                > = None;
                let mut r#supplement: Option<
                    Vec<fhirbolt_model::r4b::resources::NutritionOrderSupplement>,
                > = None;
                let mut r#enteral_formula: Option<
                    fhirbolt_model::r4b::resources::NutritionOrderEnteralFormula,
                > = None;
                let mut r#note: Option<Vec<fhirbolt_model::r4b::types::Annotation>> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = tri!(map_access.next_value());
                            if value != "NutritionOrder" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"NutritionOrder",
                                ));
                            }
                        }
                        Field::Id => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Id,
                                > = self.0.transmute();
                                r#id = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::IdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_id"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("id");
                            }
                        }
                        Field::Meta => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::ImplicitRules => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("implicitRules");
                            }
                        }
                        Field::Language => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#language = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("language");
                            }
                        }
                        Field::Text => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::Resource>,
                                > = self.0.transmute();
                                r#contained =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::Resource,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Identifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Identifier,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::InstantiatesCanonical => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
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
                            } else {
                                let vec =
                                    r#instantiates_canonical.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Canonical,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::InstantiatesCanonicalPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
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
                            } else {
                                return unknown_field_error("instantiatesCanonical");
                            }
                        }
                        Field::InstantiatesUri => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
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
                            } else {
                                let vec = r#instantiates_uri.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Uri,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::InstantiatesUriPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
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
                            } else {
                                return unknown_field_error("instantiatesUri");
                            }
                        }
                        Field::Instantiates => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
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
                            } else {
                                let vec = r#instantiates.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Uri,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::InstantiatesPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
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
                            } else {
                                return unknown_field_error("instantiates");
                            }
                        }
                        Field::Status => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#status = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::StatusPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::Intent => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#intent.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intent"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#intent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intent"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#intent = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::IntentPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#intent.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_intent"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("intent");
                            }
                        }
                        Field::Patient => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#patient = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Encounter => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#encounter = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::DateTime => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#date_time.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dateTime"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#date_time.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dateTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::DateTime,
                                > = self.0.transmute();
                                r#date_time =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::DateTimePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#date_time.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_dateTime"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("dateTime");
                            }
                        }
                        Field::Orderer => {
                            if r#orderer.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderer"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#orderer = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::AllergyIntolerance => {
                            if self.0.from == crate::context::Format::Json {
                                if r#allergy_intolerance.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "allergyIntolerance",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Reference>,
                                > = self.0.transmute();
                                r#allergy_intolerance =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#allergy_intolerance.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Reference,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::FoodPreferenceModifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#food_preference_modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "foodPreferenceModifier",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#food_preference_modifier =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec =
                                    r#food_preference_modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ExcludeFoodModifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#exclude_food_modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "excludeFoodModifier",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#exclude_food_modifier =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#exclude_food_modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::OralDiet => {
                            if r#oral_diet.is_some() {
                                return Err(serde::de::Error::duplicate_field("oralDiet"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4b::resources::NutritionOrderOralDiet,
                            > = self.0.transmute();
                            r#oral_diet = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Supplement => {
                            if self.0.from == crate::context::Format::Json {
                                if r#supplement.is_some() {
                                    return Err(serde::de::Error::duplicate_field("supplement"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::resources::NutritionOrderSupplement>,
                                > = self.0.transmute();
                                r#supplement =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#supplement.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::resources::NutritionOrderSupplement,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::EnteralFormula => {
                            if r#enteral_formula.is_some() {
                                return Err(serde::de::Error::duplicate_field("enteralFormula"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4b::resources::NutritionOrderEnteralFormula,
                            > = self.0.transmute();
                            r#enteral_formula =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Note => {
                            if self.0.from == crate::context::Format::Json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Annotation>,
                                > = self.0.transmute();
                                r#note = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Annotation,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
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
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        tri!(r#status.ok_or(serde::de::Error::missing_field("status")))
                    },
                    r#intent: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#intent.unwrap_or(Default::default())
                    } else {
                        tri!(r#intent.ok_or(serde::de::Error::missing_field("intent")))
                    },
                    r#patient: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#patient.unwrap_or(Default::default())
                    } else {
                        tri!(r#patient.ok_or(serde::de::Error::missing_field("patient")))
                    },
                    r#encounter,
                    r#date_time: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#date_time.unwrap_or(Default::default())
                    } else {
                        tri!(r#date_time.ok_or(serde::de::Error::missing_field("dateTime")))
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
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<NutritionOrder>> {
    type Value = Box<NutritionOrder>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<NutritionOrder>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<NutritionOrder>> {
    type Value = Vec<NutritionOrder>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<NutritionOrder>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<NutritionOrder>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<NutritionOrder> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
