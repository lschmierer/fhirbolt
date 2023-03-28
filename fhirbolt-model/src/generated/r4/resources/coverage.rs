// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "The amount due from the patient for the cost category."]
#[derive(Debug, Clone)]
pub enum CoverageCostToBeneficiaryValue {
    Quantity(Box<super::super::types::Quantity>),
    Money(Box<super::super::types::Money>),
    Invalid,
}
impl Default for CoverageCostToBeneficiaryValue {
    fn default() -> CoverageCostToBeneficiaryValue {
        CoverageCostToBeneficiaryValue::Invalid
    }
}
#[doc = "A suite of underwriter specific classifiers."]
#[derive(Default, Debug, Clone)]
pub struct CoverageClass {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of classification for which an insurer-specific class label or number and optional name is provided, for example may be used to identify a class of coverage or employer group, Policy, Plan."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The alphanumeric string value associated with the insurer issued label."]
    pub r#value: super::super::types::String,
    #[doc = "A short description for the class."]
    pub r#name: Option<super::super::types::String>,
}
impl serde::ser::Serialize for CoverageClass {
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
            state.serialize_entry("type", &self.r#type)?;
            if _ctx.output_json {
                if let Some(some) = self.r#value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("value", &some)?;
                }
                if self.r#value.id.is_some() || !self.r#value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#value.id.as_ref(),
                        extension: &self.r#value.extension,
                    };
                    state.serialize_entry("_value", &primitive_element)?;
                }
            } else {
                state.serialize_entry("value", &self.r#value)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("name", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_name", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#name.as_ref() {
                    state.serialize_entry("name", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CoverageClass {
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
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CoverageClass;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageClass")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CoverageClass, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<super::super::types::String> = None;
                let mut r#name: Option<super::super::types::String> = None;
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
                            Field::Value => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    r#value = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValuePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_value"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "value",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "value",
                                            "name",
                                        ],
                                    ));
                                }
                            }
                            Field::Name => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                }
                            }
                            Field::NamePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_name"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "name",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "value",
                                            "name",
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
                                        "type",
                                        "value",
                                        "name",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CoverageClass {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#value: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value"))?
                        },
                        r#name,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A suite of codes indicating exceptions or reductions to patient costs and their effective periods."]
#[derive(Default, Debug, Clone)]
pub struct CoverageCostToBeneficiaryException {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The code for the specific exception."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The timeframe during when the exception is in force."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for CoverageCostToBeneficiaryException {
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
            state.serialize_entry("type", &self.r#type)?;
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CoverageCostToBeneficiaryException {
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
            #[serde(rename = "period")]
            Period,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CoverageCostToBeneficiaryException;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageCostToBeneficiaryException")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CoverageCostToBeneficiaryException, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
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
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "type", "period"],
                                ));
                            },
                        }
                    }
                    Ok(CoverageCostToBeneficiaryException {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#period,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A suite of codes indicating the cost category and associated amount which have been detailed in the policy and may have been  included on the health card."]
#[derive(Default, Debug, Clone)]
pub struct CoverageCostToBeneficiary {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The category of patient centric costs associated with treatment."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The amount due from the patient for the cost category."]
    pub r#value: CoverageCostToBeneficiaryValue,
    #[doc = "A suite of codes indicating exceptions or reductions to patient costs and their effective periods."]
    pub r#exception: Vec<CoverageCostToBeneficiaryException>,
}
impl serde::ser::Serialize for CoverageCostToBeneficiary {
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
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            match self.r#value {
                CoverageCostToBeneficiaryValue::Quantity(ref value) => {
                    state.serialize_entry("valueQuantity", value)?;
                }
                CoverageCostToBeneficiaryValue::Money(ref value) => {
                    state.serialize_entry("valueMoney", value)?;
                }
                CoverageCostToBeneficiaryValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is a required field"))
                }
            }
            if !self.r#exception.is_empty() {
                state.serialize_entry("exception", &self.r#exception)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CoverageCostToBeneficiary {
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
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueMoney")]
            ValueMoney,
            #[serde(rename = "exception")]
            Exception,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CoverageCostToBeneficiary;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageCostToBeneficiary")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CoverageCostToBeneficiary, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<CoverageCostToBeneficiaryValue> = None;
                let mut r#exception: Option<Vec<CoverageCostToBeneficiaryException>> = None;
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
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value = Some(CoverageCostToBeneficiaryValue::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueMoney => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMoney"));
                                }
                                r#value = Some(CoverageCostToBeneficiaryValue::Money(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Exception => {
                                if r#exception.is_some() {
                                    return Err(serde::de::Error::duplicate_field("exception"));
                                }
                                r#exception = Some(map_access.next_value()?);
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
                                        "type",
                                        "valueQuantity",
                                        "valueMoney",
                                        "exception",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CoverageCostToBeneficiary {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#value: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                        },
                        r#exception: r#exception.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Financial instrument which may be used to reimburse or pay for health care products and services. Includes both insurance and self-payment.\n\nCoverage provides a link between covered parties (patients) and the payors of their healthcare costs (both insurance and self-pay)."]
#[derive(Default, Debug, Clone)]
pub struct Coverage {
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
    #[doc = "A unique identifier assigned to this coverage."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "The type of coverage: social program, medical plan, accident coverage (workers compensation, auto), group health or payment by an individual or organization."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The party who 'owns' the insurance policy."]
    pub r#policy_holder: Option<Box<super::super::types::Reference>>,
    #[doc = "The party who has signed-up for or 'owns' the contractual relationship to the policy or to whom the benefit of the policy for services rendered to them or their family is due."]
    pub r#subscriber: Option<Box<super::super::types::Reference>>,
    #[doc = "The insurer assigned ID for the Subscriber."]
    pub r#subscriber_id: Option<super::super::types::String>,
    #[doc = "The party who benefits from the insurance coverage; the patient when products and/or services are provided."]
    pub r#beneficiary: Box<super::super::types::Reference>,
    #[doc = "A unique identifier for a dependent under the coverage."]
    pub r#dependent: Option<super::super::types::String>,
    #[doc = "The relationship of beneficiary (patient) to the subscriber."]
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Time period during which the coverage is in force. A missing start date indicates the start date isn't known, a missing end date means the coverage is continuing to be in force."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The program or plan underwriter or payor including both insurance and non-insurance agreements, such as patient-pay agreements."]
    pub r#payor: Vec<Box<super::super::types::Reference>>,
    #[doc = "A suite of underwriter specific classifiers."]
    pub r#class: Vec<CoverageClass>,
    #[doc = "The order of applicability of this coverage relative to other coverages which are currently in force. Note, there may be gaps in the numbering and this does not imply primary, secondary etc. as the specific positioning of coverages depends upon the episode of care."]
    pub r#order: Option<super::super::types::PositiveInt>,
    #[doc = "The insurer-specific identifier for the insurer-defined network of providers to which the beneficiary may seek treatment which will be covered at the 'in-network' rate, otherwise 'out of network' terms and conditions apply."]
    pub r#network: Option<super::super::types::String>,
    #[doc = "A suite of codes indicating the cost category and associated amount which have been detailed in the policy and may have been  included on the health card."]
    pub r#cost_to_beneficiary: Vec<CoverageCostToBeneficiary>,
    #[doc = "When 'subrogation=true' this insurance instance has been included not for adjudication but to provide insurers with the details to recover costs."]
    pub r#subrogation: Option<super::super::types::Boolean>,
    #[doc = "The policy(s) which constitute this insurance coverage."]
    pub r#contract: Vec<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for Coverage {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for Coverage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Coverage")?;
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
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#policy_holder.as_ref() {
                state.serialize_entry("policyHolder", some)?;
            }
            if let Some(some) = self.r#subscriber.as_ref() {
                state.serialize_entry("subscriber", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#subscriber_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("subscriberId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_subscriberId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#subscriber_id.as_ref() {
                    state.serialize_entry("subscriberId", some)?;
                }
            }
            state.serialize_entry("beneficiary", &self.r#beneficiary)?;
            if _ctx.output_json {
                if let Some(some) = self.r#dependent.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("dependent", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_dependent", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#dependent.as_ref() {
                    state.serialize_entry("dependent", some)?;
                }
            }
            if let Some(some) = self.r#relationship.as_ref() {
                state.serialize_entry("relationship", some)?;
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            if !self.r#payor.is_empty() {
                state.serialize_entry("payor", &self.r#payor)?;
            }
            if !self.r#class.is_empty() {
                state.serialize_entry("class", &self.r#class)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#order.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("order", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_order", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#order.as_ref() {
                    state.serialize_entry("order", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#network.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("network", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_network", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#network.as_ref() {
                    state.serialize_entry("network", some)?;
                }
            }
            if !self.r#cost_to_beneficiary.is_empty() {
                state.serialize_entry("costToBeneficiary", &self.r#cost_to_beneficiary)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#subrogation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("subrogation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_subrogation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#subrogation.as_ref() {
                    state.serialize_entry("subrogation", some)?;
                }
            }
            if !self.r#contract.is_empty() {
                state.serialize_entry("contract", &self.r#contract)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Coverage {
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
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "policyHolder")]
            PolicyHolder,
            #[serde(rename = "subscriber")]
            Subscriber,
            #[serde(rename = "subscriberId")]
            SubscriberId,
            #[serde(rename = "_subscriberId")]
            SubscriberIdPrimitiveElement,
            #[serde(rename = "beneficiary")]
            Beneficiary,
            #[serde(rename = "dependent")]
            Dependent,
            #[serde(rename = "_dependent")]
            DependentPrimitiveElement,
            #[serde(rename = "relationship")]
            Relationship,
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "payor")]
            Payor,
            #[serde(rename = "class")]
            Class,
            #[serde(rename = "order")]
            Order,
            #[serde(rename = "_order")]
            OrderPrimitiveElement,
            #[serde(rename = "network")]
            Network,
            #[serde(rename = "_network")]
            NetworkPrimitiveElement,
            #[serde(rename = "costToBeneficiary")]
            CostToBeneficiary,
            #[serde(rename = "subrogation")]
            Subrogation,
            #[serde(rename = "_subrogation")]
            SubrogationPrimitiveElement,
            #[serde(rename = "contract")]
            Contract,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Coverage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Coverage")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Coverage, V::Error>
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
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#policy_holder: Option<Box<super::super::types::Reference>> = None;
                let mut r#subscriber: Option<Box<super::super::types::Reference>> = None;
                let mut r#subscriber_id: Option<super::super::types::String> = None;
                let mut r#beneficiary: Option<Box<super::super::types::Reference>> = None;
                let mut r#dependent: Option<super::super::types::String> = None;
                let mut r#relationship: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                let mut r#payor: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#class: Option<Vec<CoverageClass>> = None;
                let mut r#order: Option<super::super::types::PositiveInt> = None;
                let mut r#network: Option<super::super::types::String> = None;
                let mut r#cost_to_beneficiary: Option<Vec<CoverageCostToBeneficiary>> = None;
                let mut r#subrogation: Option<super::super::types::Boolean> = None;
                let mut r#contract: Option<Vec<Box<super::super::types::Reference>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Coverage" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Coverage",
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
                                            "identifier",
                                            "status",
                                            "type",
                                            "policyHolder",
                                            "subscriber",
                                            "subscriberId",
                                            "beneficiary",
                                            "dependent",
                                            "relationship",
                                            "period",
                                            "payor",
                                            "class",
                                            "order",
                                            "network",
                                            "costToBeneficiary",
                                            "subrogation",
                                            "contract",
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
                                            "identifier",
                                            "status",
                                            "type",
                                            "policyHolder",
                                            "subscriber",
                                            "subscriberId",
                                            "beneficiary",
                                            "dependent",
                                            "relationship",
                                            "period",
                                            "payor",
                                            "class",
                                            "order",
                                            "network",
                                            "costToBeneficiary",
                                            "subrogation",
                                            "contract",
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
                            Field::Status => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    r#status = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "status",
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
                                            "type",
                                            "policyHolder",
                                            "subscriber",
                                            "subscriberId",
                                            "beneficiary",
                                            "dependent",
                                            "relationship",
                                            "period",
                                            "payor",
                                            "class",
                                            "order",
                                            "network",
                                            "costToBeneficiary",
                                            "subrogation",
                                            "contract",
                                        ],
                                    ));
                                }
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::PolicyHolder => {
                                if r#policy_holder.is_some() {
                                    return Err(serde::de::Error::duplicate_field("policyHolder"));
                                }
                                r#policy_holder = Some(map_access.next_value()?);
                            }
                            Field::Subscriber => {
                                if r#subscriber.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subscriber"));
                                }
                                r#subscriber = Some(map_access.next_value()?);
                            }
                            Field::SubscriberId => {
                                if _ctx.from_json {
                                    let some = r#subscriber_id.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "subscriberId",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#subscriber_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "subscriberId",
                                        ));
                                    }
                                    r#subscriber_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::SubscriberIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#subscriber_id.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_subscriberId",
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
                                        "subscriberId",
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
                                            "type",
                                            "policyHolder",
                                            "subscriber",
                                            "subscriberId",
                                            "beneficiary",
                                            "dependent",
                                            "relationship",
                                            "period",
                                            "payor",
                                            "class",
                                            "order",
                                            "network",
                                            "costToBeneficiary",
                                            "subrogation",
                                            "contract",
                                        ],
                                    ));
                                }
                            }
                            Field::Beneficiary => {
                                if r#beneficiary.is_some() {
                                    return Err(serde::de::Error::duplicate_field("beneficiary"));
                                }
                                r#beneficiary = Some(map_access.next_value()?);
                            }
                            Field::Dependent => {
                                if _ctx.from_json {
                                    let some = r#dependent.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("dependent"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#dependent.is_some() {
                                        return Err(serde::de::Error::duplicate_field("dependent"));
                                    }
                                    r#dependent = Some(map_access.next_value()?);
                                }
                            }
                            Field::DependentPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#dependent.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_dependent",
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
                                        "dependent",
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
                                            "type",
                                            "policyHolder",
                                            "subscriber",
                                            "subscriberId",
                                            "beneficiary",
                                            "dependent",
                                            "relationship",
                                            "period",
                                            "payor",
                                            "class",
                                            "order",
                                            "network",
                                            "costToBeneficiary",
                                            "subrogation",
                                            "contract",
                                        ],
                                    ));
                                }
                            }
                            Field::Relationship => {
                                if r#relationship.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relationship"));
                                }
                                r#relationship = Some(map_access.next_value()?);
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Payor => {
                                if r#payor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("payor"));
                                }
                                r#payor = Some(map_access.next_value()?);
                            }
                            Field::Class => {
                                if r#class.is_some() {
                                    return Err(serde::de::Error::duplicate_field("class"));
                                }
                                r#class = Some(map_access.next_value()?);
                            }
                            Field::Order => {
                                if _ctx.from_json {
                                    let some = r#order.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("order"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#order.is_some() {
                                        return Err(serde::de::Error::duplicate_field("order"));
                                    }
                                    r#order = Some(map_access.next_value()?);
                                }
                            }
                            Field::OrderPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#order.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_order"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "order",
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
                                            "type",
                                            "policyHolder",
                                            "subscriber",
                                            "subscriberId",
                                            "beneficiary",
                                            "dependent",
                                            "relationship",
                                            "period",
                                            "payor",
                                            "class",
                                            "order",
                                            "network",
                                            "costToBeneficiary",
                                            "subrogation",
                                            "contract",
                                        ],
                                    ));
                                }
                            }
                            Field::Network => {
                                if _ctx.from_json {
                                    let some = r#network.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("network"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#network.is_some() {
                                        return Err(serde::de::Error::duplicate_field("network"));
                                    }
                                    r#network = Some(map_access.next_value()?);
                                }
                            }
                            Field::NetworkPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#network.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_network"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "network",
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
                                            "type",
                                            "policyHolder",
                                            "subscriber",
                                            "subscriberId",
                                            "beneficiary",
                                            "dependent",
                                            "relationship",
                                            "period",
                                            "payor",
                                            "class",
                                            "order",
                                            "network",
                                            "costToBeneficiary",
                                            "subrogation",
                                            "contract",
                                        ],
                                    ));
                                }
                            }
                            Field::CostToBeneficiary => {
                                if r#cost_to_beneficiary.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "costToBeneficiary",
                                    ));
                                }
                                r#cost_to_beneficiary = Some(map_access.next_value()?);
                            }
                            Field::Subrogation => {
                                if _ctx.from_json {
                                    let some = r#subrogation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "subrogation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#subrogation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "subrogation",
                                        ));
                                    }
                                    r#subrogation = Some(map_access.next_value()?);
                                }
                            }
                            Field::SubrogationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#subrogation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_subrogation",
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
                                        "subrogation",
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
                                            "type",
                                            "policyHolder",
                                            "subscriber",
                                            "subscriberId",
                                            "beneficiary",
                                            "dependent",
                                            "relationship",
                                            "period",
                                            "payor",
                                            "class",
                                            "order",
                                            "network",
                                            "costToBeneficiary",
                                            "subrogation",
                                            "contract",
                                        ],
                                    ));
                                }
                            }
                            Field::Contract => {
                                if r#contract.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contract"));
                                }
                                r#contract = Some(map_access.next_value()?);
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
                                        "identifier",
                                        "status",
                                        "type",
                                        "policyHolder",
                                        "subscriber",
                                        "subscriberId",
                                        "beneficiary",
                                        "dependent",
                                        "relationship",
                                        "period",
                                        "payor",
                                        "class",
                                        "order",
                                        "network",
                                        "costToBeneficiary",
                                        "subrogation",
                                        "contract",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Coverage {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#type,
                        r#policy_holder,
                        r#subscriber,
                        r#subscriber_id,
                        r#beneficiary: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#beneficiary.unwrap_or(Default::default())
                        } else {
                            r#beneficiary.ok_or(serde::de::Error::missing_field("beneficiary"))?
                        },
                        r#dependent,
                        r#relationship,
                        r#period,
                        r#payor: r#payor.unwrap_or(vec![]),
                        r#class: r#class.unwrap_or(vec![]),
                        r#order,
                        r#network,
                        r#cost_to_beneficiary: r#cost_to_beneficiary.unwrap_or(vec![]),
                        r#subrogation,
                        r#contract: r#contract.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
