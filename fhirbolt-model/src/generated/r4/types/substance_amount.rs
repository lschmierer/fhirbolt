// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "Used to capture quantitative values for a variety of elements. If only limits are given, the arithmetic mean would be the average. If only a single definite value for a given element is given, it would be captured in this field."]
#[derive(Debug, Clone)]
pub enum SubstanceAmountAmount {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SubstanceAmountAmount {
    fn default() -> SubstanceAmountAmount {
        SubstanceAmountAmount::Invalid
    }
}
#[doc = "Reference range of possible or expected values."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceAmountReferenceRange {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Lower limit possible or expected."]
    pub r#low_limit: Option<Box<super::super::types::Quantity>>,
    #[doc = "Upper limit possible or expected."]
    pub r#high_limit: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for SubstanceAmountReferenceRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if let Some(some) = self.r#low_limit.as_ref() {
                state.serialize_entry("lowLimit", some)?;
            }
            if let Some(some) = self.r#high_limit.as_ref() {
                state.serialize_entry("highLimit", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceAmountReferenceRange {
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
            #[serde(rename = "lowLimit")]
            LowLimit,
            #[serde(rename = "highLimit")]
            HighLimit,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceAmountReferenceRange;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceAmountReferenceRange")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceAmountReferenceRange, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#low_limit: Option<Box<super::super::types::Quantity>> = None;
                let mut r#high_limit: Option<Box<super::super::types::Quantity>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::LowLimit => {
                                if r#low_limit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lowLimit"));
                                }
                                r#low_limit = Some(map_access.next_value()?);
                            }
                            Field::HighLimit => {
                                if r#high_limit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("highLimit"));
                                }
                                r#high_limit = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "lowLimit", "highLimit"],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceAmountReferenceRange {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#low_limit,
                        r#high_limit,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Base StructureDefinition for SubstanceAmount Type: Chemical substances are a single substance type whose primary defining element is the molecular structure. Chemical substances shall be defined on the basis of their complete covalent molecular structure; the presence of a salt (counter-ion) and/or solvates (water, alcohols) is also captured. Purity, grade, physical form or particle size are not taken into account in the definition of a chemical substance or in the assignment of a Substance ID."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceAmount {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Used to capture quantitative values for a variety of elements. If only limits are given, the arithmetic mean would be the average. If only a single definite value for a given element is given, it would be captured in this field."]
    pub r#amount: Option<SubstanceAmountAmount>,
    #[doc = "Most elements that require a quantitative value will also have a field called amount type. Amount type should always be specified because the actual value of the amount is often dependent on it. EXAMPLE: In capturing the actual relative amounts of substances or molecular fragments it is essential to indicate whether the amount refers to a mole ratio or weight ratio. For any given element an effort should be made to use same the amount type for all related definitional elements."]
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A textual comment on a numeric value."]
    pub r#amount_text: Option<super::super::types::String>,
    #[doc = "Reference range of possible or expected values."]
    pub r#reference_range: Option<SubstanceAmountReferenceRange>,
}
impl serde::ser::Serialize for SubstanceAmount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if let Some(some) = self.r#amount.as_ref() {
                match some {
                    SubstanceAmountAmount::Quantity(ref value) => {
                        state.serialize_entry("amountQuantity", value)?;
                    }
                    SubstanceAmountAmount::Range(ref value) => {
                        state.serialize_entry("amountRange", value)?;
                    }
                    SubstanceAmountAmount::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("amountString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_amountString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("amountString", value)?;
                        }
                    }
                    SubstanceAmountAmount::Invalid => {
                        return Err(serde::ser::Error::custom("amount is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#amount_type.as_ref() {
                state.serialize_entry("amountType", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#amount_text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("amountText", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_amountText", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#amount_text.as_ref() {
                    state.serialize_entry("amountText", some)?;
                }
            }
            if let Some(some) = self.r#reference_range.as_ref() {
                state.serialize_entry("referenceRange", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceAmount {
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
            #[serde(rename = "amountQuantity")]
            AmountQuantity,
            #[serde(rename = "amountRange")]
            AmountRange,
            #[serde(rename = "amountString")]
            AmountString,
            #[serde(rename = "_amountString")]
            AmountStringPrimitiveElement,
            #[serde(rename = "amountType")]
            AmountType,
            #[serde(rename = "amountText")]
            AmountText,
            #[serde(rename = "_amountText")]
            AmountTextPrimitiveElement,
            #[serde(rename = "referenceRange")]
            ReferenceRange,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceAmount;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceAmount")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceAmount, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#amount: Option<SubstanceAmountAmount> = None;
                let mut r#amount_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#amount_text: Option<super::super::types::String> = None;
                let mut r#reference_range: Option<SubstanceAmountReferenceRange> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::AmountQuantity => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "amountQuantity",
                                    ));
                                }
                                r#amount =
                                    Some(SubstanceAmountAmount::Quantity(map_access.next_value()?));
                            }
                            Field::AmountRange => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountRange"));
                                }
                                r#amount =
                                    Some(SubstanceAmountAmount::Range(map_access.next_value()?));
                            }
                            Field::AmountString => {
                                if _ctx.from_json {
                                    let r#enum = r#amount.get_or_insert(
                                        SubstanceAmountAmount::String(Default::default()),
                                    );
                                    if let SubstanceAmountAmount::String(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "amountString",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("amount[x]"));
                                    }
                                } else {
                                    if r#amount.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "amountString",
                                        ));
                                    }
                                    r#amount = Some(SubstanceAmountAmount::String(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::AmountStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#amount.get_or_insert(
                                        SubstanceAmountAmount::String(Default::default()),
                                    );
                                    if let SubstanceAmountAmount::String(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_amountString",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_amount[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "amountString",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "amountQuantity",
                                            "amountRange",
                                            "amountString",
                                            "amountType",
                                            "amountText",
                                            "referenceRange",
                                        ],
                                    ));
                                }
                            }
                            Field::AmountType => {
                                if r#amount_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountType"));
                                }
                                r#amount_type = Some(map_access.next_value()?);
                            }
                            Field::AmountText => {
                                if _ctx.from_json {
                                    let some = r#amount_text.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "amountText",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#amount_text.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "amountText",
                                        ));
                                    }
                                    r#amount_text = Some(map_access.next_value()?);
                                }
                            }
                            Field::AmountTextPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#amount_text.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_amountText",
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
                                        "amountText",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "amountQuantity",
                                            "amountRange",
                                            "amountString",
                                            "amountType",
                                            "amountText",
                                            "referenceRange",
                                        ],
                                    ));
                                }
                            }
                            Field::ReferenceRange => {
                                if r#reference_range.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceRange",
                                    ));
                                }
                                r#reference_range = Some(map_access.next_value()?);
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
                                        "amountQuantity",
                                        "amountRange",
                                        "amountString",
                                        "amountType",
                                        "amountText",
                                        "referenceRange",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceAmount {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#amount,
                        r#amount_type,
                        r#amount_text,
                        r#reference_range,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
