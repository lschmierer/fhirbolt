// Generated on 2023-04-05 by fhirbolt-codegen v0.1.0
#[doc = "Characteristics for quantitative results of this observation."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObservationDefinitionQuantitativeDetails {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Customary unit used to report quantitative results of observations conforming to this ObservationDefinition."]
    pub r#customary_unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "SI unit used to report quantitative results of observations conforming to this ObservationDefinition."]
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Factor for converting value expressed with SI unit to value expressed with customary unit."]
    pub r#conversion_factor: Option<super::super::types::Decimal>,
    #[doc = "Number of digits after decimal separator when the results of such observations are of type Quantity."]
    pub r#decimal_precision: Option<super::super::types::Integer>,
}
impl serde::ser::Serialize for ObservationDefinitionQuantitativeDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if let Some(some) = self.r#customary_unit.as_ref() {
                state.serialize_entry("customaryUnit", some)?;
            }
            if let Some(some) = self.r#unit.as_ref() {
                state.serialize_entry("unit", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#conversion_factor.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("conversionFactor", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_conversionFactor", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#conversion_factor.as_ref() {
                    state.serialize_entry("conversionFactor", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#decimal_precision.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("decimalPrecision", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_decimalPrecision", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#decimal_precision.as_ref() {
                    state.serialize_entry("decimalPrecision", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ObservationDefinitionQuantitativeDetails {
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
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#customary_unit: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#unit: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#conversion_factor: Option<super::super::types::Decimal> = None;
                let mut r#decimal_precision: Option<super::super::types::Integer> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::CustomaryUnit => {
                                if r#customary_unit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("customaryUnit"));
                                }
                                r#customary_unit = Some(map_access.next_value()?);
                            }
                            Field::Unit => {
                                if r#unit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unit"));
                                }
                                r#unit = Some(map_access.next_value()?);
                            }
                            Field::ConversionFactor => {
                                if _ctx.from_json {
                                    let some =
                                        r#conversion_factor.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "conversionFactor",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#conversion_factor.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "conversionFactor",
                                        ));
                                    }
                                    r#conversion_factor = Some(map_access.next_value()?);
                                }
                            }
                            Field::ConversionFactorPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#conversion_factor.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_conversionFactor",
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
                                        "conversionFactor",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "customaryUnit",
                                            "unit",
                                            "conversionFactor",
                                            "decimalPrecision",
                                        ],
                                    ));
                                }
                            }
                            Field::DecimalPrecision => {
                                if _ctx.from_json {
                                    let some =
                                        r#decimal_precision.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "decimalPrecision",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#decimal_precision.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "decimalPrecision",
                                        ));
                                    }
                                    r#decimal_precision = Some(map_access.next_value()?);
                                }
                            }
                            Field::DecimalPrecisionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#decimal_precision.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_decimalPrecision",
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
                                        "decimalPrecision",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "customaryUnit",
                                            "unit",
                                            "conversionFactor",
                                            "decimalPrecision",
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
                                        "customaryUnit",
                                        "unit",
                                        "conversionFactor",
                                        "decimalPrecision",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Multiple  ranges of results qualified by different contexts for ordinal or continuous observations conforming to this ObservationDefinition."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObservationDefinitionQualifiedInterval {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The category of interval of values for continuous or ordinal observations conforming to this ObservationDefinition."]
    pub r#category: Option<super::super::types::Code>,
    #[doc = "The low and high values determining the interval. There may be only one of the two."]
    pub r#range: Option<Box<super::super::types::Range>>,
    #[doc = "Codes to indicate the health context the range applies to. For example, the normal or therapeutic range."]
    pub r#context: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Codes to indicate the target population this reference range applies to."]
    pub r#applies_to: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Sex of the population the range applies to."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "The age at which this reference range is applicable. This is a neonatal age (e.g. number of weeks at term) if the meaning says so."]
    pub r#age: Option<Box<super::super::types::Range>>,
    #[doc = "The gestational age to which this reference range is applicable, in the context of pregnancy."]
    pub r#gestational_age: Option<Box<super::super::types::Range>>,
    #[doc = "Text based condition for which the reference range is valid."]
    pub r#condition: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ObservationDefinitionQualifiedInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
                if let Some(some) = self.r#category.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("category", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_category", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#category.as_ref() {
                    state.serialize_entry("category", some)?;
                }
            }
            if let Some(some) = self.r#range.as_ref() {
                state.serialize_entry("range", some)?;
            }
            if let Some(some) = self.r#context.as_ref() {
                state.serialize_entry("context", some)?;
            }
            if !self.r#applies_to.is_empty() {
                state.serialize_entry("appliesTo", &self.r#applies_to)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#gender.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("gender", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_gender", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#gender.as_ref() {
                    state.serialize_entry("gender", some)?;
                }
            }
            if let Some(some) = self.r#age.as_ref() {
                state.serialize_entry("age", some)?;
            }
            if let Some(some) = self.r#gestational_age.as_ref() {
                state.serialize_entry("gestationalAge", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#condition.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("condition", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_condition", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#condition.as_ref() {
                    state.serialize_entry("condition", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ObservationDefinitionQualifiedInterval {
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
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#category: Option<super::super::types::Code> = None;
                let mut r#range: Option<Box<super::super::types::Range>> = None;
                let mut r#context: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#applies_to: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#gender: Option<super::super::types::Code> = None;
                let mut r#age: Option<Box<super::super::types::Range>> = None;
                let mut r#gestational_age: Option<Box<super::super::types::Range>> = None;
                let mut r#condition: Option<super::super::types::String> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Category => {
                                if _ctx.from_json {
                                    let some = r#category.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("category"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#category.is_some() {
                                        return Err(serde::de::Error::duplicate_field("category"));
                                    }
                                    r#category = Some(map_access.next_value()?);
                                }
                            }
                            Field::CategoryPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#category.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_category"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "category",
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
                                    ));
                                }
                            }
                            Field::Range => {
                                if r#range.is_some() {
                                    return Err(serde::de::Error::duplicate_field("range"));
                                }
                                r#range = Some(map_access.next_value()?);
                            }
                            Field::Context => {
                                if r#context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("context"));
                                }
                                r#context = Some(map_access.next_value()?);
                            }
                            Field::AppliesTo => {
                                if _ctx.from_json {
                                    if r#applies_to.is_some() {
                                        return Err(serde::de::Error::duplicate_field("appliesTo"));
                                    }
                                    r#applies_to = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#applies_to.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Gender => {
                                if _ctx.from_json {
                                    let some = r#gender.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("gender"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#gender.is_some() {
                                        return Err(serde::de::Error::duplicate_field("gender"));
                                    }
                                    r#gender = Some(map_access.next_value()?);
                                }
                            }
                            Field::GenderPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#gender.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_gender"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "gender",
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
                                    ));
                                }
                            }
                            Field::Age => {
                                if r#age.is_some() {
                                    return Err(serde::de::Error::duplicate_field("age"));
                                }
                                r#age = Some(map_access.next_value()?);
                            }
                            Field::GestationalAge => {
                                if r#gestational_age.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "gestationalAge",
                                    ));
                                }
                                r#gestational_age = Some(map_access.next_value()?);
                            }
                            Field::Condition => {
                                if _ctx.from_json {
                                    let some = r#condition.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("condition"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#condition.is_some() {
                                        return Err(serde::de::Error::duplicate_field("condition"));
                                    }
                                    r#condition = Some(map_access.next_value()?);
                                }
                            }
                            Field::ConditionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#condition.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_condition",
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
                                        "condition",
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
                                        "category",
                                        "range",
                                        "context",
                                        "appliesTo",
                                        "gender",
                                        "age",
                                        "gestationalAge",
                                        "condition",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.\n\nIn a catalog of health-related services that use or produce observations and measurements, this resource describes the expected characteristics of these observation / measurements."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObservationDefinition {
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
    #[doc = "A code that classifies the general type of observation."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Describes what will be observed. Sometimes this is called the observation \"name\"."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "A unique identifier assigned to this ObservationDefinition artifact."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The data types allowed for the value element of the instance observations conforming to this ObservationDefinition."]
    pub r#permitted_data_type: Vec<super::super::types::Code>,
    #[doc = "Multiple results allowed for observations conforming to this ObservationDefinition."]
    pub r#multiple_results_allowed: Option<super::super::types::Boolean>,
    #[doc = "The method or technique used to perform the observation."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The preferred name to be used when reporting the results of observations conforming to this ObservationDefinition."]
    pub r#preferred_report_name: Option<super::super::types::String>,
    #[doc = "Characteristics for quantitative results of this observation."]
    pub r#quantitative_details: Option<ObservationDefinitionQuantitativeDetails>,
    #[doc = "Multiple  ranges of results qualified by different contexts for ordinal or continuous observations conforming to this ObservationDefinition."]
    pub r#qualified_interval: Vec<ObservationDefinitionQualifiedInterval>,
    #[doc = "The set of valid coded results for the observations  conforming to this ObservationDefinition."]
    pub r#valid_coded_value_set: Option<Box<super::super::types::Reference>>,
    #[doc = "The set of normal coded results for the observations conforming to this ObservationDefinition."]
    pub r#normal_coded_value_set: Option<Box<super::super::types::Reference>>,
    #[doc = "The set of abnormal coded results for the observation conforming to this ObservationDefinition."]
    pub r#abnormal_coded_value_set: Option<Box<super::super::types::Reference>>,
    #[doc = "The set of critical coded results for the observation conforming to this ObservationDefinition."]
    pub r#critical_coded_value_set: Option<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for ObservationDefinition {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for ObservationDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ObservationDefinition")?;
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
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            state.serialize_entry("code", &self.r#code)?;
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if _ctx.output_json {
                if !self.r#permitted_data_type.is_empty() {
                    let values = self
                        .r#permitted_data_type
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("permittedDataType", &values)?;
                    }
                    let requires_elements = self
                        .r#permitted_data_type
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#permitted_data_type
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_permittedDataType", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#permitted_data_type.is_empty() {
                    state.serialize_entry("permittedDataType", &self.r#permitted_data_type)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#multiple_results_allowed.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("multipleResultsAllowed", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_multipleResultsAllowed", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#multiple_results_allowed.as_ref() {
                    state.serialize_entry("multipleResultsAllowed", some)?;
                }
            }
            if let Some(some) = self.r#method.as_ref() {
                state.serialize_entry("method", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#preferred_report_name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("preferredReportName", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_preferredReportName", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#preferred_report_name.as_ref() {
                    state.serialize_entry("preferredReportName", some)?;
                }
            }
            if let Some(some) = self.r#quantitative_details.as_ref() {
                state.serialize_entry("quantitativeDetails", some)?;
            }
            if !self.r#qualified_interval.is_empty() {
                state.serialize_entry("qualifiedInterval", &self.r#qualified_interval)?;
            }
            if let Some(some) = self.r#valid_coded_value_set.as_ref() {
                state.serialize_entry("validCodedValueSet", some)?;
            }
            if let Some(some) = self.r#normal_coded_value_set.as_ref() {
                state.serialize_entry("normalCodedValueSet", some)?;
            }
            if let Some(some) = self.r#abnormal_coded_value_set.as_ref() {
                state.serialize_entry("abnormalCodedValueSet", some)?;
            }
            if let Some(some) = self.r#critical_coded_value_set.as_ref() {
                state.serialize_entry("criticalCodedValueSet", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ObservationDefinition {
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
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ObservationDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ObservationDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ObservationDefinition, V::Error>
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
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#permitted_data_type: Option<Vec<super::super::types::Code>> = None;
                let mut r#multiple_results_allowed: Option<super::super::types::Boolean> = None;
                let mut r#method: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#preferred_report_name: Option<super::super::types::String> = None;
                let mut r#quantitative_details: Option<ObservationDefinitionQuantitativeDetails> =
                    None;
                let mut r#qualified_interval: Option<Vec<ObservationDefinitionQualifiedInterval>> =
                    None;
                let mut r#valid_coded_value_set: Option<Box<super::super::types::Reference>> = None;
                let mut r#normal_coded_value_set: Option<Box<super::super::types::Reference>> =
                    None;
                let mut r#abnormal_coded_value_set: Option<Box<super::super::types::Reference>> =
                    None;
                let mut r#critical_coded_value_set: Option<Box<super::super::types::Reference>> =
                    None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
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
                                if _ctx.from_json {
                                    if r#contained.is_some() {
                                        return Err(serde::de::Error::duplicate_field("contained"));
                                    }
                                    r#contained = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#contained.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Category => {
                                if _ctx.from_json {
                                    if r#category.is_some() {
                                        return Err(serde::de::Error::duplicate_field("category"));
                                    }
                                    r#category = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#category.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if _ctx.from_json {
                                    if r#identifier.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "identifier",
                                        ));
                                    }
                                    r#identifier = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#identifier.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::PermittedDataType => {
                                if _ctx.from_json {
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
                                    let vec =
                                        r#permitted_data_type.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::PermittedDataTypePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
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
                                    return Err(serde::de::Error::unknown_field(
                                        "permittedDataType",
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
                                    ));
                                }
                            }
                            Field::MultipleResultsAllowed => {
                                if _ctx.from_json {
                                    let some = r#multiple_results_allowed
                                        .get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "multipleResultsAllowed",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#multiple_results_allowed.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "multipleResultsAllowed",
                                        ));
                                    }
                                    r#multiple_results_allowed = Some(map_access.next_value()?);
                                }
                            }
                            Field::MultipleResultsAllowedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#multiple_results_allowed
                                        .get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_multipleResultsAllowed",
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
                                        "multipleResultsAllowed",
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
                                    ));
                                }
                            }
                            Field::Method => {
                                if r#method.is_some() {
                                    return Err(serde::de::Error::duplicate_field("method"));
                                }
                                r#method = Some(map_access.next_value()?);
                            }
                            Field::PreferredReportName => {
                                if _ctx.from_json {
                                    let some =
                                        r#preferred_report_name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "preferredReportName",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#preferred_report_name.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "preferredReportName",
                                        ));
                                    }
                                    r#preferred_report_name = Some(map_access.next_value()?);
                                }
                            }
                            Field::PreferredReportNamePrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#preferred_report_name.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_preferredReportName",
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
                                        "preferredReportName",
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
                                    ));
                                }
                            }
                            Field::QuantitativeDetails => {
                                if r#quantitative_details.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "quantitativeDetails",
                                    ));
                                }
                                r#quantitative_details = Some(map_access.next_value()?);
                            }
                            Field::QualifiedInterval => {
                                if _ctx.from_json {
                                    if r#qualified_interval.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "qualifiedInterval",
                                        ));
                                    }
                                    r#qualified_interval = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#qualified_interval.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ValidCodedValueSet => {
                                if r#valid_coded_value_set.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "validCodedValueSet",
                                    ));
                                }
                                r#valid_coded_value_set = Some(map_access.next_value()?);
                            }
                            Field::NormalCodedValueSet => {
                                if r#normal_coded_value_set.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "normalCodedValueSet",
                                    ));
                                }
                                r#normal_coded_value_set = Some(map_access.next_value()?);
                            }
                            Field::AbnormalCodedValueSet => {
                                if r#abnormal_coded_value_set.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "abnormalCodedValueSet",
                                    ));
                                }
                                r#abnormal_coded_value_set = Some(map_access.next_value()?);
                            }
                            Field::CriticalCodedValueSet => {
                                if r#critical_coded_value_set.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "criticalCodedValueSet",
                                    ));
                                }
                                r#critical_coded_value_set = Some(map_access.next_value()?);
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
                                ));
                            },
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
                        r#code: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
