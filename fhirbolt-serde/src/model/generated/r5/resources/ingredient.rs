// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::IngredientManufacturer;
impl serde::ser::Serialize for SerializationContext<&IngredientManufacturer> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Ingredient.manufacturer", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#role.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("role", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_role", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#role.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("role", ctx))?;
        }
        if self.value.r#manufacturer.id.as_deref() == Some("$invalid") {
            return missing_field_error("manufacturer");
        } else {
            self.with_context(&self.value.r#manufacturer, |ctx| {
                state.serialize_entry("manufacturer", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<IngredientManufacturer>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<IngredientManufacturer>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<IngredientManufacturer> {
    type Value = IngredientManufacturer;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<IngredientManufacturer>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = IngredientManufacturer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("IngredientManufacturer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<IngredientManufacturer, V::Error>
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
                    #[serde(rename = "role")]
                    Role,
                    #[serde(rename = "_role")]
                    RolePrimitiveElement,
                    #[serde(rename = "manufacturer")]
                    Manufacturer,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "role",
                            "manufacturer",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#role: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#manufacturer: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Role => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#role.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#role = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RolePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#role.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_role"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("role");
                            }
                        }
                        Field::Manufacturer => {
                            if r#manufacturer.is_some() {
                                return Err(serde::de::Error::duplicate_field("manufacturer"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#manufacturer = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(IngredientManufacturer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#role,
                    r#manufacturer: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#manufacturer.unwrap_or(Default::default())
                    } else {
                        r#manufacturer.ok_or(serde::de::Error::missing_field("manufacturer"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<IngredientManufacturer>>
{
    type Value = Box<IngredientManufacturer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<IngredientManufacturer>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<IngredientManufacturer>>
{
    type Value = Vec<IngredientManufacturer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<IngredientManufacturer>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<IngredientManufacturer>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<IngredientManufacturer> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::IngredientSubstanceStrengthReferenceStrength;
impl serde::ser::Serialize for SerializationContext<&IngredientSubstanceStrengthReferenceStrength> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Ingredient.substance.strength.referenceStrength", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.value.r#substance.id.as_deref() == Some("$invalid") {
            return missing_field_error("substance");
        } else {
            self.with_context(&self.value.r#substance, |ctx| {
                state.serialize_entry("substance", ctx)
            })?;
        }
        {
            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthReferenceStrengthStrength as _Enum;
            match self.value.r#strength {
                _Enum::Ratio(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("strengthRatio", ctx))?;
                }
                _Enum::RatioRange(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("strengthRatioRange", ctx)
                    })?;
                }
                _Enum::Quantity(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("strengthQuantity", ctx))?;
                }
                _Enum::Invalid => {
                    return Err(serde::ser::Error::custom("strength is a required field"))
                }
            }
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#measurement_point.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("measurementPoint", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_measurementPoint", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#measurement_point.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("measurementPoint", ctx))?;
        }
        if !self.value.r#country.is_empty() {
            self.with_context(&self.value.r#country, |ctx| {
                state.serialize_entry("country", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<IngredientSubstanceStrengthReferenceStrength>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<IngredientSubstanceStrengthReferenceStrength>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<IngredientSubstanceStrengthReferenceStrength>
{
    type Value = IngredientSubstanceStrengthReferenceStrength;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<IngredientSubstanceStrengthReferenceStrength>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = IngredientSubstanceStrengthReferenceStrength;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("IngredientSubstanceStrengthReferenceStrength")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<IngredientSubstanceStrengthReferenceStrength, V::Error>
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
                    #[serde(rename = "substance")]
                    Substance,
                    #[serde(rename = "strengthRatio")]
                    StrengthRatio,
                    #[serde(rename = "strengthRatioRange")]
                    StrengthRatioRange,
                    #[serde(rename = "strengthQuantity")]
                    StrengthQuantity,
                    #[serde(rename = "measurementPoint")]
                    MeasurementPoint,
                    #[serde(rename = "_measurementPoint")]
                    MeasurementPointPrimitiveElement,
                    #[serde(rename = "country")]
                    Country,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "substance",
                            "strengthRatio",
                            "strengthRatioRange",
                            "strengthQuantity",
                            "measurementPoint",
                            "country",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#substance: Option<Box<fhirbolt_model::r5::types::CodeableReference>> =
                    None;
                let mut r#strength : Option < fhirbolt_model :: r5 :: resources :: IngredientSubstanceStrengthReferenceStrengthStrength > = None ;
                let mut r#measurement_point: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#country: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Substance => {
                            if r#substance.is_some() {
                                return Err(serde::de::Error::duplicate_field("substance"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableReference>,
                            > = self.0.transmute();
                            r#substance = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::StrengthRatio => {
                            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthReferenceStrengthStrength as _Enum;
                            if r#strength.is_some() {
                                return Err(serde::de::Error::duplicate_field("strengthRatio"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Ratio>,
                            > = self.0.transmute();
                            r#strength =
                                Some(_Enum::Ratio(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::StrengthRatioRange => {
                            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthReferenceStrengthStrength as _Enum;
                            if r#strength.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "strengthRatioRange",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::RatioRange>,
                            > = self.0.transmute();
                            r#strength = Some(_Enum::RatioRange(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::StrengthQuantity => {
                            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthReferenceStrengthStrength as _Enum;
                            if r#strength.is_some() {
                                return Err(serde::de::Error::duplicate_field("strengthQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#strength =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::MeasurementPoint => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#measurement_point.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "measurementPoint",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#measurement_point.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "measurementPoint",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#measurement_point =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MeasurementPointPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#measurement_point.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_measurementPoint",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("measurementPoint");
                            }
                        }
                        Field::Country => {
                            if self.0.from == crate::context::Format::Json {
                                if r#country.is_some() {
                                    return Err(serde::de::Error::duplicate_field("country"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#country = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#country.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                Ok(IngredientSubstanceStrengthReferenceStrength {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#substance: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#substance.unwrap_or(Default::default())
                    } else {
                        r#substance.ok_or(serde::de::Error::missing_field("substance"))?
                    },
                    r#strength: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#strength.unwrap_or(Default::default())
                    } else {
                        r#strength.ok_or(serde::de::Error::missing_field("strength[x]"))?
                    },
                    r#measurement_point,
                    r#country: r#country.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<IngredientSubstanceStrengthReferenceStrength>>
{
    type Value = Box<IngredientSubstanceStrengthReferenceStrength>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<IngredientSubstanceStrengthReferenceStrength>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<IngredientSubstanceStrengthReferenceStrength>>
{
    type Value = Vec<IngredientSubstanceStrengthReferenceStrength>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<IngredientSubstanceStrengthReferenceStrength>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<IngredientSubstanceStrengthReferenceStrength>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    IngredientSubstanceStrengthReferenceStrength,
                > = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::IngredientSubstanceStrength;
impl serde::ser::Serialize for SerializationContext<&IngredientSubstanceStrength> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Ingredient.substance.strength", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        {
            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthPresentation as _Enum;
            if let Some(some) = self.value.r#presentation.as_ref() {
                match some {
                    _Enum::Ratio(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("presentationRatio", ctx)
                        })?;
                    }
                    _Enum::RatioRange(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("presentationRatioRange", ctx)
                        })?;
                    }
                    _Enum::CodeableConcept(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("presentationCodeableConcept", ctx)
                        })?;
                    }
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("presentationQuantity", ctx)
                        })?;
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("presentation is invalid"))
                    }
                }
            }
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#text_presentation.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("textPresentation", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_textPresentation", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#text_presentation.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("textPresentation", ctx))?;
        }
        {
            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthConcentration as _Enum;
            if let Some(some) = self.value.r#concentration.as_ref() {
                match some {
                    _Enum::Ratio(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("concentrationRatio", ctx)
                        })?;
                    }
                    _Enum::RatioRange(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("concentrationRatioRange", ctx)
                        })?;
                    }
                    _Enum::CodeableConcept(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("concentrationCodeableConcept", ctx)
                        })?;
                    }
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("concentrationQuantity", ctx)
                        })?;
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("concentration is invalid"))
                    }
                }
            }
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#text_concentration.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("textConcentration", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_textConcentration", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#text_concentration.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("textConcentration", ctx))?;
        }
        if let Some(some) = self.value.r#basis.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("basis", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#measurement_point.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("measurementPoint", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_measurementPoint", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#measurement_point.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("measurementPoint", ctx))?;
        }
        if !self.value.r#country.is_empty() {
            self.with_context(&self.value.r#country, |ctx| {
                state.serialize_entry("country", ctx)
            })?;
        }
        if !self.value.r#reference_strength.is_empty() {
            self.with_context(&self.value.r#reference_strength, |ctx| {
                state.serialize_entry("referenceStrength", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<IngredientSubstanceStrength>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<IngredientSubstanceStrength>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<IngredientSubstanceStrength>
{
    type Value = IngredientSubstanceStrength;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<IngredientSubstanceStrength>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = IngredientSubstanceStrength;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("IngredientSubstanceStrength")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<IngredientSubstanceStrength, V::Error>
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
                    #[serde(rename = "presentationRatio")]
                    PresentationRatio,
                    #[serde(rename = "presentationRatioRange")]
                    PresentationRatioRange,
                    #[serde(rename = "presentationCodeableConcept")]
                    PresentationCodeableConcept,
                    #[serde(rename = "presentationQuantity")]
                    PresentationQuantity,
                    #[serde(rename = "textPresentation")]
                    TextPresentation,
                    #[serde(rename = "_textPresentation")]
                    TextPresentationPrimitiveElement,
                    #[serde(rename = "concentrationRatio")]
                    ConcentrationRatio,
                    #[serde(rename = "concentrationRatioRange")]
                    ConcentrationRatioRange,
                    #[serde(rename = "concentrationCodeableConcept")]
                    ConcentrationCodeableConcept,
                    #[serde(rename = "concentrationQuantity")]
                    ConcentrationQuantity,
                    #[serde(rename = "textConcentration")]
                    TextConcentration,
                    #[serde(rename = "_textConcentration")]
                    TextConcentrationPrimitiveElement,
                    #[serde(rename = "basis")]
                    Basis,
                    #[serde(rename = "measurementPoint")]
                    MeasurementPoint,
                    #[serde(rename = "_measurementPoint")]
                    MeasurementPointPrimitiveElement,
                    #[serde(rename = "country")]
                    Country,
                    #[serde(rename = "referenceStrength")]
                    ReferenceStrength,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "presentationRatio",
                            "presentationRatioRange",
                            "presentationCodeableConcept",
                            "presentationQuantity",
                            "textPresentation",
                            "concentrationRatio",
                            "concentrationRatioRange",
                            "concentrationCodeableConcept",
                            "concentrationQuantity",
                            "textConcentration",
                            "basis",
                            "measurementPoint",
                            "country",
                            "referenceStrength",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#presentation: Option<
                    fhirbolt_model::r5::resources::IngredientSubstanceStrengthPresentation,
                > = None;
                let mut r#text_presentation: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#concentration: Option<
                    fhirbolt_model::r5::resources::IngredientSubstanceStrengthConcentration,
                > = None;
                let mut r#text_concentration: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#basis: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#measurement_point: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#country: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#reference_strength: Option<
                    Vec<
                        fhirbolt_model::r5::resources::IngredientSubstanceStrengthReferenceStrength,
                    >,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PresentationRatio => {
                            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthPresentation as _Enum;
                            if r#presentation.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentationRatio"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Ratio>,
                            > = self.0.transmute();
                            r#presentation =
                                Some(_Enum::Ratio(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PresentationRatioRange => {
                            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthPresentation as _Enum;
                            if r#presentation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "presentationRatioRange",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::RatioRange>,
                            > = self.0.transmute();
                            r#presentation = Some(_Enum::RatioRange(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PresentationCodeableConcept => {
                            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthPresentation as _Enum;
                            if r#presentation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "presentationCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#presentation = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PresentationQuantity => {
                            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthPresentation as _Enum;
                            if r#presentation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "presentationQuantity",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#presentation =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::TextPresentation => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#text_presentation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "textPresentation",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#text_presentation.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "textPresentation",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#text_presentation =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TextPresentationPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#text_presentation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_textPresentation",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("textPresentation");
                            }
                        }
                        Field::ConcentrationRatio => {
                            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthConcentration as _Enum;
                            if r#concentration.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "concentrationRatio",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Ratio>,
                            > = self.0.transmute();
                            r#concentration =
                                Some(_Enum::Ratio(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ConcentrationRatioRange => {
                            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthConcentration as _Enum;
                            if r#concentration.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "concentrationRatioRange",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::RatioRange>,
                            > = self.0.transmute();
                            r#concentration = Some(_Enum::RatioRange(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ConcentrationCodeableConcept => {
                            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthConcentration as _Enum;
                            if r#concentration.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "concentrationCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#concentration = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ConcentrationQuantity => {
                            use fhirbolt_model::r5::resources::IngredientSubstanceStrengthConcentration as _Enum;
                            if r#concentration.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "concentrationQuantity",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#concentration =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::TextConcentration => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#text_concentration.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "textConcentration",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#text_concentration.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "textConcentration",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#text_concentration =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TextConcentrationPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#text_concentration.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_textConcentration",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("textConcentration");
                            }
                        }
                        Field::Basis => {
                            if r#basis.is_some() {
                                return Err(serde::de::Error::duplicate_field("basis"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#basis = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::MeasurementPoint => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#measurement_point.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "measurementPoint",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#measurement_point.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "measurementPoint",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#measurement_point =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MeasurementPointPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#measurement_point.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_measurementPoint",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("measurementPoint");
                            }
                        }
                        Field::Country => {
                            if self.0.from == crate::context::Format::Json {
                                if r#country.is_some() {
                                    return Err(serde::de::Error::duplicate_field("country"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#country = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#country.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ReferenceStrength => {
                            if self.0.from == crate::context::Format::Json {
                                if r#reference_strength.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceStrength",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: IngredientSubstanceStrengthReferenceStrength >> = self . 0 . transmute () ;
                                r#reference_strength =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#reference_strength.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: IngredientSubstanceStrengthReferenceStrength > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                Ok(IngredientSubstanceStrength {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#presentation,
                    r#text_presentation,
                    r#concentration,
                    r#text_concentration,
                    r#basis,
                    r#measurement_point,
                    r#country: r#country.unwrap_or(vec![]),
                    r#reference_strength: r#reference_strength.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<IngredientSubstanceStrength>>
{
    type Value = Box<IngredientSubstanceStrength>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<IngredientSubstanceStrength>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<IngredientSubstanceStrength>>
{
    type Value = Vec<IngredientSubstanceStrength>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<IngredientSubstanceStrength>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<IngredientSubstanceStrength>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<IngredientSubstanceStrength> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::IngredientSubstance;
impl serde::ser::Serialize for SerializationContext<&IngredientSubstance> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Ingredient.substance", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.value.r#code.id.as_deref() == Some("$invalid") {
            return missing_field_error("code");
        } else {
            self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        }
        if !self.value.r#strength.is_empty() {
            self.with_context(&self.value.r#strength, |ctx| {
                state.serialize_entry("strength", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<IngredientSubstance>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<IngredientSubstance>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<IngredientSubstance> {
    type Value = IngredientSubstance;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<IngredientSubstance>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = IngredientSubstance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("IngredientSubstance")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<IngredientSubstance, V::Error>
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
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "strength")]
                    Strength,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "code", "strength"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#code: Option<Box<fhirbolt_model::r5::types::CodeableReference>> = None;
                let mut r#strength: Option<
                    Vec<fhirbolt_model::r5::resources::IngredientSubstanceStrength>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableReference>,
                            > = self.0.transmute();
                            r#code = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Strength => {
                            if self.0.from == crate::context::Format::Json {
                                if r#strength.is_some() {
                                    return Err(serde::de::Error::duplicate_field("strength"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::IngredientSubstanceStrength>,
                                > = self.0.transmute();
                                r#strength = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#strength.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::IngredientSubstanceStrength,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                Ok(IngredientSubstance {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#code.unwrap_or(Default::default())
                    } else {
                        r#code.ok_or(serde::de::Error::missing_field("code"))?
                    },
                    r#strength: r#strength.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<IngredientSubstance>>
{
    type Value = Box<IngredientSubstance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<IngredientSubstance>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<IngredientSubstance>>
{
    type Value = Vec<IngredientSubstance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<IngredientSubstance>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<IngredientSubstance>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<IngredientSubstance> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::Ingredient;
impl crate::Resource for Ingredient {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&Ingredient> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Ingredient", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Ingredient")?;
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("id", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| state.serialize_entry("_id", ctx))?;
                }
            }
        } else if let Some(some) = self.value.r#id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("id", ctx))?;
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meta", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("implicitRules", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_implicitRules", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#implicit_rules.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("language", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_language", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#language.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
        }
        if let Some(some) = self.value.r#text.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
        }
        if !self.value.r#contained.is_empty() {
            self.with_context(&self.value.r#contained, |ctx| {
                state.serialize_entry("contained", ctx)
            })?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#identifier.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("identifier", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#status.id.as_deref() == Some("$invalid") {
                return missing_field_error("status");
            }
            if let Some(some) = self.value.r#status.value.as_ref().map(Ok) {
                state.serialize_entry("status", &some?)?;
            }
            if self.value.r#status.id.is_some() || !self.value.r#status.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#status.id.as_ref(),
                    extension: &self.value.r#status.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_status", ctx)
                })?;
            }
        } else if self.value.r#status.id.as_deref() == Some("$invalid") {
            return missing_field_error("status");
        } else {
            self.with_context(&self.value.r#status, |ctx| {
                state.serialize_entry("status", ctx)
            })?;
        }
        if !self.value.r#for.is_empty() {
            self.with_context(&self.value.r#for, |ctx| state.serialize_entry("for", ctx))?;
        }
        if self.value.r#role.id.as_deref() == Some("$invalid") {
            return missing_field_error("role");
        } else {
            self.with_context(&self.value.r#role, |ctx| state.serialize_entry("role", ctx))?;
        }
        if !self.value.r#function.is_empty() {
            self.with_context(&self.value.r#function, |ctx| {
                state.serialize_entry("function", ctx)
            })?;
        }
        if let Some(some) = self.value.r#group.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("group", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#allergenic_indicator.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("allergenicIndicator", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_allergenicIndicator", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#allergenic_indicator.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("allergenicIndicator", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#comment.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("comment", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_comment", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#comment.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("comment", ctx))?;
        }
        if !self.value.r#manufacturer.is_empty() {
            self.with_context(&self.value.r#manufacturer, |ctx| {
                state.serialize_entry("manufacturer", ctx)
            })?;
        }
        if self.value.r#substance.id.as_deref() == Some("$invalid") {
            return missing_field_error("substance");
        } else {
            self.with_context(&self.value.r#substance, |ctx| {
                state.serialize_entry("substance", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Ingredient>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Ingredient>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<Ingredient> {
    type Value = Ingredient;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Ingredient> {
    type Value = Ingredient;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Ingredient>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Ingredient;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Ingredient")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Ingredient, V::Error>
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
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "for")]
                    For,
                    #[serde(rename = "role")]
                    Role,
                    #[serde(rename = "function")]
                    Function,
                    #[serde(rename = "group")]
                    Group,
                    #[serde(rename = "allergenicIndicator")]
                    AllergenicIndicator,
                    #[serde(rename = "_allergenicIndicator")]
                    AllergenicIndicatorPrimitiveElement,
                    #[serde(rename = "comment")]
                    Comment,
                    #[serde(rename = "_comment")]
                    CommentPrimitiveElement,
                    #[serde(rename = "manufacturer")]
                    Manufacturer,
                    #[serde(rename = "substance")]
                    Substance,
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
                            "status",
                            "for",
                            "role",
                            "function",
                            "group",
                            "allergenicIndicator",
                            "comment",
                            "manufacturer",
                            "substance",
                        ],
                    ))
                }
                let mut r#id: Option<Box<fhirbolt_model::r5::types::Id>> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r5::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#identifier: Option<Box<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#for: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#role: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#function: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#group: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#allergenic_indicator: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#comment: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#manufacturer: Option<
                    Vec<fhirbolt_model::r5::resources::IngredientManufacturer>,
                > = None;
                let mut r#substance: Option<fhirbolt_model::r5::resources::IngredientSubstance> =
                    None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "Ingredient" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Ingredient",
                                ));
                            }
                        }
                        Field::Id => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::Id>,
                                > = self.0.transmute();
                                r#id = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                Box<fhirbolt_model::r5::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#language = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                Box<fhirbolt_model::r5::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::Resource,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Identifier>,
                            > = self.0.transmute();
                            r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Status => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#status = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::For => {
                            if self.0.from == crate::context::Format::Json {
                                if r#for.is_some() {
                                    return Err(serde::de::Error::duplicate_field("for"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#for = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#for.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Role => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#role = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Function => {
                            if self.0.from == crate::context::Format::Json {
                                if r#function.is_some() {
                                    return Err(serde::de::Error::duplicate_field("function"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#function = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#function.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Group => {
                            if r#group.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#group = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::AllergenicIndicator => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#allergenic_indicator.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "allergenicIndicator",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#allergenic_indicator.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "allergenicIndicator",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#allergenic_indicator =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AllergenicIndicatorPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#allergenic_indicator.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_allergenicIndicator",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("allergenicIndicator");
                            }
                        }
                        Field::Comment => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#comment.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comment"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#comment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comment"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Markdown,
                                > = self.0.transmute();
                                r#comment = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CommentPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#comment.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_comment"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("comment");
                            }
                        }
                        Field::Manufacturer => {
                            if self.0.from == crate::context::Format::Json {
                                if r#manufacturer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manufacturer"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::IngredientManufacturer>,
                                > = self.0.transmute();
                                r#manufacturer = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#manufacturer.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::IngredientManufacturer,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Substance => {
                            if r#substance.is_some() {
                                return Err(serde::de::Error::duplicate_field("substance"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::IngredientSubstance,
                            > = self.0.transmute();
                            r#substance = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(Ingredient {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier,
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        r#status.ok_or(serde::de::Error::missing_field("status"))?
                    },
                    r#for: r#for.unwrap_or(vec![]),
                    r#role: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#role.unwrap_or(Default::default())
                    } else {
                        r#role.ok_or(serde::de::Error::missing_field("role"))?
                    },
                    r#function: r#function.unwrap_or(vec![]),
                    r#group,
                    r#allergenic_indicator,
                    r#comment,
                    r#manufacturer: r#manufacturer.unwrap_or(vec![]),
                    r#substance: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#substance.unwrap_or(Default::default())
                    } else {
                        r#substance.ok_or(serde::de::Error::missing_field("substance"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Ingredient>> {
    type Value = Box<Ingredient>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Ingredient>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Ingredient>> {
    type Value = Vec<Ingredient>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Ingredient>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Ingredient>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Ingredient> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
