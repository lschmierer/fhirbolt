// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct ObservationDefinitionQuantitativeDetails {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#customary_unit: Option<Box<super::super::types::CodeableConcept>>,
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    pub r#conversion_factor: Option<super::super::types::Decimal>,
    pub r#decimal_precision: Option<super::super::types::Integer>,
}
impl serde::ser::Serialize for ObservationDefinitionQuantitativeDetails {
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
        if let Some(some) = self.r#customary_unit.as_ref() {
            state.serialize_entry("customaryUnit", some)?;
        }
        if let Some(some) = self.r#unit.as_ref() {
            state.serialize_entry("unit", some)?;
        }
        if let Some(some) = self.r#conversion_factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("conversionFactor", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_conversionFactor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#decimal_precision.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("decimalPrecision", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_decimalPrecision", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ObservationDefinitionQuantitativeDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                        "customaryUnit" => {
                            if r#customary_unit.is_some() {
                                return Err(serde::de::Error::duplicate_field("customaryUnit"));
                            }
                            r#customary_unit = Some(map_access.next_value()?);
                        }
                        "unit" => {
                            if r#unit.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            r#unit = Some(map_access.next_value()?);
                        }
                        "conversionFactor" => {
                            let some = r#conversion_factor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversionFactor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_conversionFactor" => {
                            let some = r#conversion_factor.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_conversionFactor"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "decimalPrecision" => {
                            let some = r#decimal_precision.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("decimalPrecision"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_decimalPrecision" => {
                            let some = r#decimal_precision.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_decimalPrecision"));
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
                                    "customary_unit",
                                    "unit",
                                    "conversion_factor",
                                    "decimal_precision",
                                ],
                            ))
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
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ObservationDefinitionQualifiedInterval {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Option<super::super::types::Code>,
    pub r#range: Option<Box<super::super::types::Range>>,
    pub r#context: Option<Box<super::super::types::CodeableConcept>>,
    pub r#applies_to: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#age: Option<Box<super::super::types::Range>>,
    pub r#gestational_age: Option<Box<super::super::types::Range>>,
    pub r#condition: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ObservationDefinitionQualifiedInterval {
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
        if let Some(some) = self.r#category.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("category", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_category", &primitive_element)?;
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
        if let Some(some) = self.r#gender.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("gender", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_gender", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#age.as_ref() {
            state.serialize_entry("age", some)?;
        }
        if let Some(some) = self.r#gestational_age.as_ref() {
            state.serialize_entry("gestationalAge", some)?;
        }
        if let Some(some) = self.r#condition.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("condition", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_condition", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ObservationDefinitionQualifiedInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                        "category" => {
                            let some = r#category.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_category" => {
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
                        }
                        "range" => {
                            if r#range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            r#range = Some(map_access.next_value()?);
                        }
                        "context" => {
                            if r#context.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            r#context = Some(map_access.next_value()?);
                        }
                        "appliesTo" => {
                            if r#applies_to.is_some() {
                                return Err(serde::de::Error::duplicate_field("appliesTo"));
                            }
                            r#applies_to = Some(map_access.next_value()?);
                        }
                        "gender" => {
                            let some = r#gender.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("gender"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_gender" => {
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
                        }
                        "age" => {
                            if r#age.is_some() {
                                return Err(serde::de::Error::duplicate_field("age"));
                            }
                            r#age = Some(map_access.next_value()?);
                        }
                        "gestationalAge" => {
                            if r#gestational_age.is_some() {
                                return Err(serde::de::Error::duplicate_field("gestationalAge"));
                            }
                            r#gestational_age = Some(map_access.next_value()?);
                        }
                        "condition" => {
                            let some = r#condition.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_condition" => {
                            let some = r#condition.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_condition"));
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
                                    "category",
                                    "range",
                                    "context",
                                    "applies_to",
                                    "gender",
                                    "age",
                                    "gestational_age",
                                    "condition",
                                ],
                            ))
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
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ObservationDefinition {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#permitted_data_type: Vec<super::super::types::Code>,
    pub r#multiple_results_allowed: Option<super::super::types::Boolean>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#preferred_report_name: Option<super::super::types::String>,
    pub r#quantitative_details: Option<ObservationDefinitionQuantitativeDetails>,
    pub r#qualified_interval: Vec<ObservationDefinitionQualifiedInterval>,
    pub r#valid_coded_value_set: Option<Box<super::super::types::Reference>>,
    pub r#normal_coded_value_set: Option<Box<super::super::types::Reference>>,
    pub r#abnormal_coded_value_set: Option<Box<super::super::types::Reference>>,
    pub r#critical_coded_value_set: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ObservationDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ObservationDefinition")?;
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
        if !self.r#category.is_empty() {
            state.serialize_entry("category", &self.r#category)?;
        }
        state.serialize_entry("code", &self.r#code)?;
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if !self.r#permitted_data_type.is_empty() {
            let values: Vec<_> = self
                .r#permitted_data_type
                .iter()
                .map(|v| &v.value)
                .collect();
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
                                id: &e.id,
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
        if let Some(some) = self.r#multiple_results_allowed.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("multipleResultsAllowed", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_multipleResultsAllowed", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#method.as_ref() {
            state.serialize_entry("method", some)?;
        }
        if let Some(some) = self.r#preferred_report_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("preferredReportName", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_preferredReportName", &primitive_element)?;
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
    }
}
impl<'de> serde::de::Deserialize<'de> for ObservationDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
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
                        "category" => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "code" => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "permittedDataType" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#permitted_data_type
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("permittedDataType"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_permittedDataType" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#permitted_data_type
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
                                    "_permittedDataType",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "multipleResultsAllowed" => {
                            let some = r#multiple_results_allowed.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "multipleResultsAllowed",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_multipleResultsAllowed" => {
                            let some = r#multiple_results_allowed.get_or_insert(Default::default());
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
                        }
                        "method" => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            r#method = Some(map_access.next_value()?);
                        }
                        "preferredReportName" => {
                            let some = r#preferred_report_name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "preferredReportName",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_preferredReportName" => {
                            let some = r#preferred_report_name.get_or_insert(Default::default());
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
                        }
                        "quantitativeDetails" => {
                            if r#quantitative_details.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "quantitativeDetails",
                                ));
                            }
                            r#quantitative_details = Some(map_access.next_value()?);
                        }
                        "qualifiedInterval" => {
                            if r#qualified_interval.is_some() {
                                return Err(serde::de::Error::duplicate_field("qualifiedInterval"));
                            }
                            r#qualified_interval = Some(map_access.next_value()?);
                        }
                        "validCodedValueSet" => {
                            if r#valid_coded_value_set.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "validCodedValueSet",
                                ));
                            }
                            r#valid_coded_value_set = Some(map_access.next_value()?);
                        }
                        "normalCodedValueSet" => {
                            if r#normal_coded_value_set.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "normalCodedValueSet",
                                ));
                            }
                            r#normal_coded_value_set = Some(map_access.next_value()?);
                        }
                        "abnormalCodedValueSet" => {
                            if r#abnormal_coded_value_set.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "abnormalCodedValueSet",
                                ));
                            }
                            r#abnormal_coded_value_set = Some(map_access.next_value()?);
                        }
                        "criticalCodedValueSet" => {
                            if r#critical_coded_value_set.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "criticalCodedValueSet",
                                ));
                            }
                            r#critical_coded_value_set = Some(map_access.next_value()?);
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
                                    "category",
                                    "code",
                                    "identifier",
                                    "permitted_data_type",
                                    "multiple_results_allowed",
                                    "method",
                                    "preferred_report_name",
                                    "quantitative_details",
                                    "qualified_interval",
                                    "valid_coded_value_set",
                                    "normal_coded_value_set",
                                    "abnormal_coded_value_set",
                                    "critical_coded_value_set",
                                ],
                            ))
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
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
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
        deserializer.deserialize_map(Visitor)
    }
}
