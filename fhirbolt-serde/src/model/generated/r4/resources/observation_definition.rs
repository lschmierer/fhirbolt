// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::resources::ObservationDefinitionQuantitativeDetails;
impl serde::ser::Serialize for SerializationContext<&ObservationDefinitionQuantitativeDetails> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ObservationDefinition.quantitativeDetails", field
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
        if let Some(some) = self.value.r#customary_unit.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("customaryUnit", ctx))?;
        }
        if let Some(some) = self.value.r#unit.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unit", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#conversion_factor.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("conversionFactor", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_conversionFactor", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#conversion_factor.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("conversionFactor", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#decimal_precision.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("decimalPrecision", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_decimalPrecision", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#decimal_precision.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("decimalPrecision", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<ObservationDefinitionQuantitativeDetails>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<ObservationDefinitionQuantitativeDetails>>
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
    for &mut DeserializationContext<ObservationDefinitionQuantitativeDetails>
{
    type Value = ObservationDefinitionQuantitativeDetails;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<ObservationDefinitionQuantitativeDetails>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ObservationDefinitionQuantitativeDetails;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ObservationDefinitionQuantitativeDetails")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ObservationDefinitionQuantitativeDetails, V::Error>
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
                    #[serde(rename = "customaryUnit")]
                    CustomaryUnit,
                    #[serde(rename = "unit")]
                    Unit,
                    #[serde(rename = "conversionFactor")]
                    ConversionFactor,
                    #[serde(rename = "_conversionFactor")]
                    ConversionFactorPrimitiveElement,
                    #[serde(rename = "decimalPrecision")]
                    DecimalPrecision,
                    #[serde(rename = "_decimalPrecision")]
                    DecimalPrecisionPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "customaryUnit",
                            "unit",
                            "conversionFactor",
                            "decimalPrecision",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#customary_unit: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#unit: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#conversion_factor: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#decimal_precision: Option<fhirbolt_model::r4::types::Integer> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CustomaryUnit => {
                            if r#customary_unit.is_some() {
                                return Err(serde::de::Error::duplicate_field("customaryUnit"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#customary_unit = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unit => {
                            if r#unit.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#unit = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ConversionFactor => {
                            if self.0.from_json {
                                let some = r#conversion_factor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "conversionFactor",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#conversion_factor.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "conversionFactor",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#conversion_factor =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ConversionFactorPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#conversion_factor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_conversionFactor",
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
                                return unknown_field_error("conversionFactor");
                            }
                        }
                        Field::DecimalPrecision => {
                            if self.0.from_json {
                                let some = r#decimal_precision.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "decimalPrecision",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#decimal_precision.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "decimalPrecision",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#decimal_precision =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DecimalPrecisionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#decimal_precision.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_decimalPrecision",
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
                                return unknown_field_error("decimalPrecision");
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
                Ok(ObservationDefinitionQuantitativeDetails {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#customary_unit,
                    r#unit,
                    r#conversion_factor,
                    r#decimal_precision,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ObservationDefinitionQuantitativeDetails>>
{
    type Value = Box<ObservationDefinitionQuantitativeDetails>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ObservationDefinitionQuantitativeDetails>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ObservationDefinitionQuantitativeDetails>>
{
    type Value = Vec<ObservationDefinitionQuantitativeDetails>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<ObservationDefinitionQuantitativeDetails>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ObservationDefinitionQuantitativeDetails>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    ObservationDefinitionQuantitativeDetails,
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
use fhirbolt_model::r4::resources::ObservationDefinitionQualifiedInterval;
impl serde::ser::Serialize for SerializationContext<&ObservationDefinitionQualifiedInterval> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ObservationDefinition.qualifiedInterval", field
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
        if self.output_json {
            if let Some(some) = self.value.r#category.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("category", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_category", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#category.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("category", ctx))?;
        }
        if let Some(some) = self.value.r#range.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("range", ctx))?;
        }
        if let Some(some) = self.value.r#context.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("context", ctx))?;
        }
        if !self.value.r#applies_to.is_empty() {
            self.with_context(&self.value.r#applies_to, |ctx| {
                state.serialize_entry("appliesTo", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#gender.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("gender", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_gender", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#gender.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("gender", ctx))?;
        }
        if let Some(some) = self.value.r#age.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("age", ctx))?;
        }
        if let Some(some) = self.value.r#gestational_age.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("gestationalAge", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#condition.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("condition", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_condition", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#condition.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("condition", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ObservationDefinitionQualifiedInterval>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ObservationDefinitionQualifiedInterval>> {
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
    for &mut DeserializationContext<ObservationDefinitionQualifiedInterval>
{
    type Value = ObservationDefinitionQualifiedInterval;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ObservationDefinitionQualifiedInterval>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ObservationDefinitionQualifiedInterval;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ObservationDefinitionQualifiedInterval")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ObservationDefinitionQualifiedInterval, V::Error>
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
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "_category")]
                    CategoryPrimitiveElement,
                    #[serde(rename = "range")]
                    Range,
                    #[serde(rename = "context")]
                    Context,
                    #[serde(rename = "appliesTo")]
                    AppliesTo,
                    #[serde(rename = "gender")]
                    Gender,
                    #[serde(rename = "_gender")]
                    GenderPrimitiveElement,
                    #[serde(rename = "age")]
                    Age,
                    #[serde(rename = "gestationalAge")]
                    GestationalAge,
                    #[serde(rename = "condition")]
                    Condition,
                    #[serde(rename = "_condition")]
                    ConditionPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "category",
                            "range",
                            "context",
                            "appliesTo",
                            "gender",
                            "age",
                            "gestationalAge",
                            "condition",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#category: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#range: Option<Box<fhirbolt_model::r4::types::Range>> = None;
                let mut r#context: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#applies_to: Option<Vec<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#gender: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#age: Option<Box<fhirbolt_model::r4::types::Range>> = None;
                let mut r#gestational_age: Option<Box<fhirbolt_model::r4::types::Range>> = None;
                let mut r#condition: Option<fhirbolt_model::r4::types::String> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Category => {
                            if self.0.from_json {
                                let some = r#category.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#category = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CategoryPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#category.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_category"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("category");
                            }
                        }
                        Field::Range => {
                            if r#range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Range>,
                            > = self.0.transmute();
                            r#range = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Context => {
                            if r#context.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#context = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::AppliesTo => {
                            if self.0.from_json {
                                if r#applies_to.is_some() {
                                    return Err(serde::de::Error::duplicate_field("appliesTo"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#applies_to = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#applies_to.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Gender => {
                            if self.0.from_json {
                                let some = r#gender.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("gender"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#gender.is_some() {
                                    return Err(serde::de::Error::duplicate_field("gender"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#gender = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::GenderPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#gender.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_gender"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("gender");
                            }
                        }
                        Field::Age => {
                            if r#age.is_some() {
                                return Err(serde::de::Error::duplicate_field("age"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Range>,
                            > = self.0.transmute();
                            r#age = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::GestationalAge => {
                            if r#gestational_age.is_some() {
                                return Err(serde::de::Error::duplicate_field("gestationalAge"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Range>,
                            > = self.0.transmute();
                            r#gestational_age = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Condition => {
                            if self.0.from_json {
                                let some = r#condition.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#condition = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ConditionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#condition.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_condition"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("condition");
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
                Ok(ObservationDefinitionQualifiedInterval {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category,
                    r#range,
                    r#context,
                    r#applies_to: r#applies_to.unwrap_or(vec![]),
                    r#gender,
                    r#age,
                    r#gestational_age,
                    r#condition,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ObservationDefinitionQualifiedInterval>>
{
    type Value = Box<ObservationDefinitionQualifiedInterval>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ObservationDefinitionQualifiedInterval>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ObservationDefinitionQualifiedInterval>>
{
    type Value = Vec<ObservationDefinitionQualifiedInterval>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<ObservationDefinitionQualifiedInterval>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ObservationDefinitionQualifiedInterval>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ObservationDefinitionQualifiedInterval> =
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
use fhirbolt_model::r4::resources::ObservationDefinition;
impl crate::Resource for ObservationDefinition {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize for SerializationContext<&ObservationDefinition> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ObservationDefinition", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ObservationDefinition")?;
        if self.output_json {
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
        if self.output_json {
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
        if self.output_json {
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
        if !self.value.r#category.is_empty() {
            self.with_context(&self.value.r#category, |ctx| {
                state.serialize_entry("category", ctx)
            })?;
        }
        if self.value.r#code.id.as_deref() == Some("$invalid") {
            return missing_field_error("code");
        } else {
            self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        }
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#permitted_data_type.is_empty() {
                let values = self
                    .value
                    .r#permitted_data_type
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("permittedDataType", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#permitted_data_type
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#permitted_data_type
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
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_permittedDataType", ctx)
                    })?;
                }
            }
        } else if !self.value.r#permitted_data_type.is_empty() {
            self.with_context(&self.value.r#permitted_data_type, |ctx| {
                state.serialize_entry("permittedDataType", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#multiple_results_allowed.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("multipleResultsAllowed", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_multipleResultsAllowed", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#multiple_results_allowed.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("multipleResultsAllowed", ctx)
            })?;
        }
        if let Some(some) = self.value.r#method.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("method", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#preferred_report_name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("preferredReportName", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_preferredReportName", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#preferred_report_name.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("preferredReportName", ctx)
            })?;
        }
        if let Some(some) = self.value.r#quantitative_details.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("quantitativeDetails", ctx)
            })?;
        }
        if !self.value.r#qualified_interval.is_empty() {
            self.with_context(&self.value.r#qualified_interval, |ctx| {
                state.serialize_entry("qualifiedInterval", ctx)
            })?;
        }
        if let Some(some) = self.value.r#valid_coded_value_set.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("validCodedValueSet", ctx))?;
        }
        if let Some(some) = self.value.r#normal_coded_value_set.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("normalCodedValueSet", ctx)
            })?;
        }
        if let Some(some) = self.value.r#abnormal_coded_value_set.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("abnormalCodedValueSet", ctx)
            })?;
        }
        if let Some(some) = self.value.r#critical_coded_value_set.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("criticalCodedValueSet", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ObservationDefinition>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ObservationDefinition>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<ObservationDefinition> {
    type Value = ObservationDefinition;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ObservationDefinition> {
    type Value = ObservationDefinition;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ObservationDefinition>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ObservationDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ObservationDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ObservationDefinition, V::Error>
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
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "permittedDataType")]
                    PermittedDataType,
                    #[serde(rename = "_permittedDataType")]
                    PermittedDataTypePrimitiveElement,
                    #[serde(rename = "multipleResultsAllowed")]
                    MultipleResultsAllowed,
                    #[serde(rename = "_multipleResultsAllowed")]
                    MultipleResultsAllowedPrimitiveElement,
                    #[serde(rename = "method")]
                    Method,
                    #[serde(rename = "preferredReportName")]
                    PreferredReportName,
                    #[serde(rename = "_preferredReportName")]
                    PreferredReportNamePrimitiveElement,
                    #[serde(rename = "quantitativeDetails")]
                    QuantitativeDetails,
                    #[serde(rename = "qualifiedInterval")]
                    QualifiedInterval,
                    #[serde(rename = "validCodedValueSet")]
                    ValidCodedValueSet,
                    #[serde(rename = "normalCodedValueSet")]
                    NormalCodedValueSet,
                    #[serde(rename = "abnormalCodedValueSet")]
                    AbnormalCodedValueSet,
                    #[serde(rename = "criticalCodedValueSet")]
                    CriticalCodedValueSet,
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
                            "category",
                            "code",
                            "identifier",
                            "permittedDataType",
                            "multipleResultsAllowed",
                            "method",
                            "preferredReportName",
                            "quantitativeDetails",
                            "qualifiedInterval",
                            "validCodedValueSet",
                            "normalCodedValueSet",
                            "abnormalCodedValueSet",
                            "criticalCodedValueSet",
                        ],
                    ))
                }
                let mut r#id: Option<Box<fhirbolt_model::r4::types::Id>> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r4::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#category: Option<Vec<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#code: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r4::types::Identifier>> = None;
                let mut r#permitted_data_type: Option<Vec<fhirbolt_model::r4::types::Code>> = None;
                let mut r#multiple_results_allowed: Option<fhirbolt_model::r4::types::Boolean> =
                    None;
                let mut r#method: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#preferred_report_name: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#quantitative_details: Option<
                    fhirbolt_model::r4::resources::ObservationDefinitionQuantitativeDetails,
                > = None;
                let mut r#qualified_interval: Option<
                    Vec<fhirbolt_model::r4::resources::ObservationDefinitionQualifiedInterval>,
                > = None;
                let mut r#valid_coded_value_set: Option<Box<fhirbolt_model::r4::types::Reference>> =
                    None;
                let mut r#normal_coded_value_set: Option<
                    Box<fhirbolt_model::r4::types::Reference>,
                > = None;
                let mut r#abnormal_coded_value_set: Option<
                    Box<fhirbolt_model::r4::types::Reference>,
                > = None;
                let mut r#critical_coded_value_set: Option<
                    Box<fhirbolt_model::r4::types::Reference>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "ObservationDefinition" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"ObservationDefinition",
                                ));
                            }
                        }
                        Field::Id => {
                            if self.0.from_json {
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
                                    Box<fhirbolt_model::r4::types::Id>,
                                > = self.0.transmute();
                                r#id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IdPrimitiveElement => {
                            if self.0.from_json {
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
                                Box<fhirbolt_model::r4::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r4::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            if self.0.from_json {
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
                            if self.0.from_json {
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
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#language = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from_json {
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
                                Box<fhirbolt_model::r4::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::Resource,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Category => {
                            if self.0.from_json {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#category = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#category.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#code = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PermittedDataType => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#permitted_data_type.get_or_insert(
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
                                        "permittedDataType",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#permitted_data_type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PermittedDataTypePrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#permitted_data_type.get_or_insert(
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
                                        "_permittedDataType",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("permittedDataType");
                            }
                        }
                        Field::MultipleResultsAllowed => {
                            if self.0.from_json {
                                let some =
                                    r#multiple_results_allowed.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "multipleResultsAllowed",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#multiple_results_allowed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "multipleResultsAllowed",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Boolean,
                                > = self.0.transmute();
                                r#multiple_results_allowed =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MultipleResultsAllowedPrimitiveElement => {
                            if self.0.from_json {
                                let some =
                                    r#multiple_results_allowed.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_multipleResultsAllowed",
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
                                return unknown_field_error("multipleResultsAllowed");
                            }
                        }
                        Field::Method => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#method = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::PreferredReportName => {
                            if self.0.from_json {
                                let some =
                                    r#preferred_report_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "preferredReportName",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#preferred_report_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "preferredReportName",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#preferred_report_name =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PreferredReportNamePrimitiveElement => {
                            if self.0.from_json {
                                let some =
                                    r#preferred_report_name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_preferredReportName",
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
                                return unknown_field_error("preferredReportName");
                            }
                        }
                        Field::QuantitativeDetails => {
                            if r#quantitative_details.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "quantitativeDetails",
                                ));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r4 :: resources :: ObservationDefinitionQuantitativeDetails > = self . 0 . transmute () ;
                            r#quantitative_details =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::QualifiedInterval => {
                            if self.0.from_json {
                                if r#qualified_interval.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "qualifiedInterval",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: ObservationDefinitionQualifiedInterval >> = self . 0 . transmute () ;
                                r#qualified_interval =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#qualified_interval.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4 :: resources :: ObservationDefinitionQualifiedInterval > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ValidCodedValueSet => {
                            if r#valid_coded_value_set.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "validCodedValueSet",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#valid_coded_value_set =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::NormalCodedValueSet => {
                            if r#normal_coded_value_set.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "normalCodedValueSet",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#normal_coded_value_set =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::AbnormalCodedValueSet => {
                            if r#abnormal_coded_value_set.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "abnormalCodedValueSet",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#abnormal_coded_value_set =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::CriticalCodedValueSet => {
                            if r#critical_coded_value_set.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "criticalCodedValueSet",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#critical_coded_value_set =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ObservationDefinition {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category: r#category.unwrap_or(vec![]),
                    r#code: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#code.unwrap_or(Default::default())
                    } else {
                        r#code.ok_or(serde::de::Error::missing_field("code"))?
                    },
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#permitted_data_type: r#permitted_data_type.unwrap_or(vec![]),
                    r#multiple_results_allowed,
                    r#method,
                    r#preferred_report_name,
                    r#quantitative_details,
                    r#qualified_interval: r#qualified_interval.unwrap_or(vec![]),
                    r#valid_coded_value_set,
                    r#normal_coded_value_set,
                    r#abnormal_coded_value_set,
                    r#critical_coded_value_set,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ObservationDefinition>>
{
    type Value = Box<ObservationDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ObservationDefinition>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ObservationDefinition>>
{
    type Value = Vec<ObservationDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ObservationDefinition>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ObservationDefinition>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ObservationDefinition> =
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
