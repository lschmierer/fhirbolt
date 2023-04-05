// Generated on 2023-04-05 by fhirbolt-codegen v0.1.0
#[doc = "Quantitative value for this moiety."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceDefinitionMoietyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SubstanceDefinitionMoietyAmount {
    fn default() -> SubstanceDefinitionMoietyAmount {
        SubstanceDefinitionMoietyAmount::Invalid
    }
}
#[doc = "A value for the property."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceDefinitionPropertyValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Date(Box<super::super::types::Date>),
    Boolean(Box<super::super::types::Boolean>),
    Attachment(Box<super::super::types::Attachment>),
    Invalid,
}
impl Default for SubstanceDefinitionPropertyValue {
    fn default() -> SubstanceDefinitionPropertyValue {
        SubstanceDefinitionPropertyValue::Invalid
    }
}
#[doc = "A pointer to another substance, as a resource or just a representational code."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceDefinitionRelationshipSubstanceDefinition {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for SubstanceDefinitionRelationshipSubstanceDefinition {
    fn default() -> SubstanceDefinitionRelationshipSubstanceDefinition {
        SubstanceDefinitionRelationshipSubstanceDefinition::Invalid
    }
}
#[doc = "A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other."]
#[derive(Debug, Clone, PartialEq)]
pub enum SubstanceDefinitionRelationshipAmount {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SubstanceDefinitionRelationshipAmount {
    fn default() -> SubstanceDefinitionRelationshipAmount {
        SubstanceDefinitionRelationshipAmount::Invalid
    }
}
#[doc = "Moiety, for structural modifications."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionMoiety {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Role that the moiety is playing."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifier by which this moiety substance is known."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Textual name for this moiety substance."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Stereochemistry type."]
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Optical activity type."]
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Molecular formula for this moiety of this substance, typically using the Hill system."]
    pub r#molecular_formula: Option<super::super::types::String>,
    #[doc = "Quantitative value for this moiety."]
    pub r#amount: Option<SubstanceDefinitionMoietyAmount>,
    #[doc = "The measurement type of the quantitative value. In capturing the actual relative amounts of substances or molecular fragments it may be necessary to indicate whether the amount refers to, for example, a mole ratio or weight ratio."]
    pub r#measurement_type: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubstanceDefinitionMoiety {
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
            if let Some(some) = self.r#role.as_ref() {
                state.serialize_entry("role", some)?;
            }
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
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
            if let Some(some) = self.r#stereochemistry.as_ref() {
                state.serialize_entry("stereochemistry", some)?;
            }
            if let Some(some) = self.r#optical_activity.as_ref() {
                state.serialize_entry("opticalActivity", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#molecular_formula.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("molecularFormula", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_molecularFormula", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#molecular_formula.as_ref() {
                    state.serialize_entry("molecularFormula", some)?;
                }
            }
            if let Some(some) = self.r#amount.as_ref() {
                match some {
                    SubstanceDefinitionMoietyAmount::Quantity(ref value) => {
                        state.serialize_entry("amountQuantity", value)?;
                    }
                    SubstanceDefinitionMoietyAmount::String(ref value) => {
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
                    SubstanceDefinitionMoietyAmount::Invalid => {
                        return Err(serde::ser::Error::custom("amount is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#measurement_type.as_ref() {
                state.serialize_entry("measurementType", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceDefinitionMoiety {
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
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "stereochemistry")]
            Stereochemistry,
            #[serde(rename = "opticalActivity")]
            OpticalActivity,
            #[serde(rename = "molecularFormula")]
            MolecularFormula,
            #[serde(rename = "_molecularFormula")]
            MolecularFormulaPrimitiveElement,
            #[serde(rename = "amountQuantity")]
            AmountQuantity,
            #[serde(rename = "amountString")]
            AmountString,
            #[serde(rename = "_amountString")]
            AmountStringPrimitiveElement,
            #[serde(rename = "measurementType")]
            MeasurementType,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceDefinitionMoiety;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceDefinitionMoiety")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceDefinitionMoiety, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#stereochemistry: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#optical_activity: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#molecular_formula: Option<super::super::types::String> = None;
                let mut r#amount: Option<SubstanceDefinitionMoietyAmount> = None;
                let mut r#measurement_type: Option<Box<super::super::types::CodeableConcept>> =
                    None;
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
                            Field::Role => {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                r#role = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
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
                                            "role",
                                            "identifier",
                                            "name",
                                            "stereochemistry",
                                            "opticalActivity",
                                            "molecularFormula",
                                            "amountQuantity",
                                            "amountString",
                                            "measurementType",
                                        ],
                                    ));
                                }
                            }
                            Field::Stereochemistry => {
                                if r#stereochemistry.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "stereochemistry",
                                    ));
                                }
                                r#stereochemistry = Some(map_access.next_value()?);
                            }
                            Field::OpticalActivity => {
                                if r#optical_activity.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "opticalActivity",
                                    ));
                                }
                                r#optical_activity = Some(map_access.next_value()?);
                            }
                            Field::MolecularFormula => {
                                if _ctx.from_json {
                                    let some =
                                        r#molecular_formula.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "molecularFormula",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#molecular_formula.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "molecularFormula",
                                        ));
                                    }
                                    r#molecular_formula = Some(map_access.next_value()?);
                                }
                            }
                            Field::MolecularFormulaPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#molecular_formula.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_molecularFormula",
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
                                        "molecularFormula",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "role",
                                            "identifier",
                                            "name",
                                            "stereochemistry",
                                            "opticalActivity",
                                            "molecularFormula",
                                            "amountQuantity",
                                            "amountString",
                                            "measurementType",
                                        ],
                                    ));
                                }
                            }
                            Field::AmountQuantity => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "amountQuantity",
                                    ));
                                }
                                r#amount = Some(SubstanceDefinitionMoietyAmount::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::AmountString => {
                                if _ctx.from_json {
                                    let r#enum = r#amount.get_or_insert(
                                        SubstanceDefinitionMoietyAmount::String(Default::default()),
                                    );
                                    if let SubstanceDefinitionMoietyAmount::String(variant) = r#enum
                                    {
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
                                    r#amount = Some(SubstanceDefinitionMoietyAmount::String(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::AmountStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#amount.get_or_insert(
                                        SubstanceDefinitionMoietyAmount::String(Default::default()),
                                    );
                                    if let SubstanceDefinitionMoietyAmount::String(variant) = r#enum
                                    {
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
                                            "role",
                                            "identifier",
                                            "name",
                                            "stereochemistry",
                                            "opticalActivity",
                                            "molecularFormula",
                                            "amountQuantity",
                                            "amountString",
                                            "measurementType",
                                        ],
                                    ));
                                }
                            }
                            Field::MeasurementType => {
                                if r#measurement_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "measurementType",
                                    ));
                                }
                                r#measurement_type = Some(map_access.next_value()?);
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
                                        "role",
                                        "identifier",
                                        "name",
                                        "stereochemistry",
                                        "opticalActivity",
                                        "molecularFormula",
                                        "amountQuantity",
                                        "amountString",
                                        "measurementType",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceDefinitionMoiety {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#role,
                        r#identifier,
                        r#name,
                        r#stereochemistry,
                        r#optical_activity,
                        r#molecular_formula,
                        r#amount,
                        r#measurement_type,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "General specifications for this substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code expressing the type of property."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A value for the property."]
    pub r#value: Option<SubstanceDefinitionPropertyValue>,
}
impl serde::ser::Serialize for SubstanceDefinitionProperty {
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
            if let Some(some) = self.r#value.as_ref() {
                match some {
                    SubstanceDefinitionPropertyValue::CodeableConcept(ref value) => {
                        state.serialize_entry("valueCodeableConcept", value)?;
                    }
                    SubstanceDefinitionPropertyValue::Quantity(ref value) => {
                        state.serialize_entry("valueQuantity", value)?;
                    }
                    SubstanceDefinitionPropertyValue::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueDate", value)?;
                        }
                    }
                    SubstanceDefinitionPropertyValue::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueBoolean", value)?;
                        }
                    }
                    SubstanceDefinitionPropertyValue::Attachment(ref value) => {
                        state.serialize_entry("valueAttachment", value)?;
                    }
                    SubstanceDefinitionPropertyValue::Invalid => {
                        return Err(serde::ser::Error::custom("value is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceDefinitionProperty {
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
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueDate")]
            ValueDate,
            #[serde(rename = "_valueDate")]
            ValueDatePrimitiveElement,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueAttachment")]
            ValueAttachment,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceDefinitionProperty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceDefinitionProperty")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceDefinitionProperty, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<SubstanceDefinitionPropertyValue> = None;
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
                                r#value = Some(SubstanceDefinitionPropertyValue::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value = Some(SubstanceDefinitionPropertyValue::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueDate => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        SubstanceDefinitionPropertyValue::Date(Default::default()),
                                    );
                                    if let SubstanceDefinitionPropertyValue::Date(variant) = r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueDate",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("value[x]"));
                                    }
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueDate"));
                                    }
                                    r#value = Some(SubstanceDefinitionPropertyValue::Date(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        SubstanceDefinitionPropertyValue::Date(Default::default()),
                                    );
                                    if let SubstanceDefinitionPropertyValue::Date(variant) = r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueDate",
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "valueCodeableConcept",
                                            "valueQuantity",
                                            "valueDate",
                                            "valueBoolean",
                                            "valueAttachment",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueBoolean => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        SubstanceDefinitionPropertyValue::Boolean(
                                            Default::default(),
                                        ),
                                    );
                                    if let SubstanceDefinitionPropertyValue::Boolean(variant) =
                                        r#enum
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
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    r#value = Some(SubstanceDefinitionPropertyValue::Boolean(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueBooleanPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        SubstanceDefinitionPropertyValue::Boolean(
                                            Default::default(),
                                        ),
                                    );
                                    if let SubstanceDefinitionPropertyValue::Boolean(variant) =
                                        r#enum
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueBoolean",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "valueCodeableConcept",
                                            "valueQuantity",
                                            "valueDate",
                                            "valueBoolean",
                                            "valueAttachment",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueAttachment => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAttachment",
                                    ));
                                }
                                r#value = Some(SubstanceDefinitionPropertyValue::Attachment(
                                    map_access.next_value()?,
                                ));
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
                                        "valueCodeableConcept",
                                        "valueQuantity",
                                        "valueDate",
                                        "valueBoolean",
                                        "valueAttachment",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceDefinitionProperty {
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
                        r#value,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionMolecularWeight {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The method by which the molecular weight was determined."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Type of molecular weight such as exact, average (also known as. number average), weight average."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to capture quantitative values for a variety of elements. If only limits are given, the arithmetic mean would be the average. If only a single definite value for a given element is given, it would be captured in this field."]
    pub r#amount: Box<super::super::types::Quantity>,
}
impl serde::ser::Serialize for SubstanceDefinitionMolecularWeight {
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
            if let Some(some) = self.r#method.as_ref() {
                state.serialize_entry("method", some)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            state.serialize_entry("amount", &self.r#amount)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceDefinitionMolecularWeight {
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
            #[serde(rename = "method")]
            Method,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "amount")]
            Amount,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceDefinitionMolecularWeight;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceDefinitionMolecularWeight")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceDefinitionMolecularWeight, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#method: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#amount: Option<Box<super::super::types::Quantity>> = None;
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
                            Field::Method => {
                                if r#method.is_some() {
                                    return Err(serde::de::Error::duplicate_field("method"));
                                }
                                r#method = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Amount => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amount"));
                                }
                                r#amount = Some(map_access.next_value()?);
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
                                        "method",
                                        "type",
                                        "amount",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceDefinitionMolecularWeight {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#method,
                        r#type,
                        r#amount: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#amount.unwrap_or(Default::default())
                        } else {
                            r#amount.ok_or(serde::de::Error::missing_field("amount"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A depiction of the structure or characterization of the substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionStructureRepresentation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of structural representation (e.g. full, partial)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The structural representation or characterization as a text string in a standard format."]
    pub r#representation: Option<super::super::types::String>,
    #[doc = "The format of the representation e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB, mmCIF. The logical content type rather than the physical file format of a document."]
    pub r#format: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An attached file with the structural representation or characterization e.g. a molecular structure graphic of the substance, a JCAMP or AnIML file."]
    pub r#document: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceDefinitionStructureRepresentation {
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
            if _ctx.output_json {
                if let Some(some) = self.r#representation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("representation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_representation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#representation.as_ref() {
                    state.serialize_entry("representation", some)?;
                }
            }
            if let Some(some) = self.r#format.as_ref() {
                state.serialize_entry("format", some)?;
            }
            if let Some(some) = self.r#document.as_ref() {
                state.serialize_entry("document", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceDefinitionStructureRepresentation {
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
            #[serde(rename = "representation")]
            Representation,
            #[serde(rename = "_representation")]
            RepresentationPrimitiveElement,
            #[serde(rename = "format")]
            Format,
            #[serde(rename = "document")]
            Document,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceDefinitionStructureRepresentation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceDefinitionStructureRepresentation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceDefinitionStructureRepresentation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#representation: Option<super::super::types::String> = None;
                let mut r#format: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#document: Option<Box<super::super::types::Reference>> = None;
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Representation => {
                                if _ctx.from_json {
                                    let some = r#representation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "representation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#representation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "representation",
                                        ));
                                    }
                                    r#representation = Some(map_access.next_value()?);
                                }
                            }
                            Field::RepresentationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#representation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_representation",
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
                                        "representation",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "representation",
                                            "format",
                                            "document",
                                        ],
                                    ));
                                }
                            }
                            Field::Format => {
                                if r#format.is_some() {
                                    return Err(serde::de::Error::duplicate_field("format"));
                                }
                                r#format = Some(map_access.next_value()?);
                            }
                            Field::Document => {
                                if r#document.is_some() {
                                    return Err(serde::de::Error::duplicate_field("document"));
                                }
                                r#document = Some(map_access.next_value()?);
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
                                        "representation",
                                        "format",
                                        "document",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceDefinitionStructureRepresentation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#representation,
                        r#format,
                        r#document,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Structural information."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionStructure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Stereochemistry type."]
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Optical activity type."]
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Molecular formula of this substance, typically using the Hill system."]
    pub r#molecular_formula: Option<super::super::types::String>,
    #[doc = "Specified per moiety according to the Hill system, i.e. first C, then H, then alphabetical, each moiety separated by a dot."]
    pub r#molecular_formula_by_moiety: Option<super::super::types::String>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Option<SubstanceDefinitionMolecularWeight>,
    #[doc = "The method used to elucidate the structure or characterization of the drug substance. Examples: X-ray, HPLC, NMR, Peptide mapping, Ligand binding assay."]
    pub r#technique: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The source of information about the structure."]
    pub r#source_document: Vec<Box<super::super::types::Reference>>,
    #[doc = "A depiction of the structure or characterization of the substance."]
    pub r#representation: Vec<SubstanceDefinitionStructureRepresentation>,
}
impl serde::ser::Serialize for SubstanceDefinitionStructure {
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
            if let Some(some) = self.r#stereochemistry.as_ref() {
                state.serialize_entry("stereochemistry", some)?;
            }
            if let Some(some) = self.r#optical_activity.as_ref() {
                state.serialize_entry("opticalActivity", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#molecular_formula.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("molecularFormula", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_molecularFormula", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#molecular_formula.as_ref() {
                    state.serialize_entry("molecularFormula", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#molecular_formula_by_moiety.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("molecularFormulaByMoiety", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_molecularFormulaByMoiety", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#molecular_formula_by_moiety.as_ref() {
                    state.serialize_entry("molecularFormulaByMoiety", some)?;
                }
            }
            if let Some(some) = self.r#molecular_weight.as_ref() {
                state.serialize_entry("molecularWeight", some)?;
            }
            if !self.r#technique.is_empty() {
                state.serialize_entry("technique", &self.r#technique)?;
            }
            if !self.r#source_document.is_empty() {
                state.serialize_entry("sourceDocument", &self.r#source_document)?;
            }
            if !self.r#representation.is_empty() {
                state.serialize_entry("representation", &self.r#representation)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceDefinitionStructure {
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
            #[serde(rename = "stereochemistry")]
            Stereochemistry,
            #[serde(rename = "opticalActivity")]
            OpticalActivity,
            #[serde(rename = "molecularFormula")]
            MolecularFormula,
            #[serde(rename = "_molecularFormula")]
            MolecularFormulaPrimitiveElement,
            #[serde(rename = "molecularFormulaByMoiety")]
            MolecularFormulaByMoiety,
            #[serde(rename = "_molecularFormulaByMoiety")]
            MolecularFormulaByMoietyPrimitiveElement,
            #[serde(rename = "molecularWeight")]
            MolecularWeight,
            #[serde(rename = "technique")]
            Technique,
            #[serde(rename = "sourceDocument")]
            SourceDocument,
            #[serde(rename = "representation")]
            Representation,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceDefinitionStructure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceDefinitionStructure")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceDefinitionStructure, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#stereochemistry: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#optical_activity: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#molecular_formula: Option<super::super::types::String> = None;
                let mut r#molecular_formula_by_moiety: Option<super::super::types::String> = None;
                let mut r#molecular_weight: Option<SubstanceDefinitionMolecularWeight> = None;
                let mut r#technique: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#source_document: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#representation: Option<Vec<SubstanceDefinitionStructureRepresentation>> =
                    None;
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
                            Field::Stereochemistry => {
                                if r#stereochemistry.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "stereochemistry",
                                    ));
                                }
                                r#stereochemistry = Some(map_access.next_value()?);
                            }
                            Field::OpticalActivity => {
                                if r#optical_activity.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "opticalActivity",
                                    ));
                                }
                                r#optical_activity = Some(map_access.next_value()?);
                            }
                            Field::MolecularFormula => {
                                if _ctx.from_json {
                                    let some =
                                        r#molecular_formula.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "molecularFormula",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#molecular_formula.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "molecularFormula",
                                        ));
                                    }
                                    r#molecular_formula = Some(map_access.next_value()?);
                                }
                            }
                            Field::MolecularFormulaPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#molecular_formula.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_molecularFormula",
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
                                        "molecularFormula",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "stereochemistry",
                                            "opticalActivity",
                                            "molecularFormula",
                                            "molecularFormulaByMoiety",
                                            "molecularWeight",
                                            "technique",
                                            "sourceDocument",
                                            "representation",
                                        ],
                                    ));
                                }
                            }
                            Field::MolecularFormulaByMoiety => {
                                if _ctx.from_json {
                                    let some = r#molecular_formula_by_moiety
                                        .get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "molecularFormulaByMoiety",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#molecular_formula_by_moiety.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "molecularFormulaByMoiety",
                                        ));
                                    }
                                    r#molecular_formula_by_moiety = Some(map_access.next_value()?);
                                }
                            }
                            Field::MolecularFormulaByMoietyPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#molecular_formula_by_moiety
                                        .get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_molecularFormulaByMoiety",
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
                                        "molecularFormulaByMoiety",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "stereochemistry",
                                            "opticalActivity",
                                            "molecularFormula",
                                            "molecularFormulaByMoiety",
                                            "molecularWeight",
                                            "technique",
                                            "sourceDocument",
                                            "representation",
                                        ],
                                    ));
                                }
                            }
                            Field::MolecularWeight => {
                                if r#molecular_weight.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "molecularWeight",
                                    ));
                                }
                                r#molecular_weight = Some(map_access.next_value()?);
                            }
                            Field::Technique => {
                                if _ctx.from_json {
                                    if r#technique.is_some() {
                                        return Err(serde::de::Error::duplicate_field("technique"));
                                    }
                                    r#technique = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#technique.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::SourceDocument => {
                                if _ctx.from_json {
                                    if r#source_document.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "sourceDocument",
                                        ));
                                    }
                                    r#source_document = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#source_document.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Representation => {
                                if _ctx.from_json {
                                    if r#representation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "representation",
                                        ));
                                    }
                                    r#representation = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#representation.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
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
                                        "stereochemistry",
                                        "opticalActivity",
                                        "molecularFormula",
                                        "molecularFormulaByMoiety",
                                        "molecularWeight",
                                        "technique",
                                        "sourceDocument",
                                        "representation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceDefinitionStructure {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#stereochemistry,
                        r#optical_activity,
                        r#molecular_formula,
                        r#molecular_formula_by_moiety,
                        r#molecular_weight,
                        r#technique: r#technique.unwrap_or(vec![]),
                        r#source_document: r#source_document.unwrap_or(vec![]),
                        r#representation: r#representation.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Codes associated with the substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionCode {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specific code."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Status of the code assignment, for example 'provisional', 'approved'."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date at which the code status was changed as part of the terminology maintenance."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "Any comment can be provided in this field, if necessary."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceDefinitionCode {
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
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("statusDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_statusDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status_date.as_ref() {
                    state.serialize_entry("statusDate", some)?;
                }
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceDefinitionCode {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "statusDate")]
            StatusDate,
            #[serde(rename = "_statusDate")]
            StatusDatePrimitiveElement,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "source")]
            Source,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceDefinitionCode;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceDefinitionCode")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceDefinitionCode, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status_date: Option<super::super::types::DateTime> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#source: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value()?);
                            }
                            Field::StatusDate => {
                                if _ctx.from_json {
                                    let some = r#status_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "statusDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "statusDate",
                                        ));
                                    }
                                    r#status_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#status_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_statusDate",
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
                                        "statusDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "status",
                                            "statusDate",
                                            "note",
                                            "source",
                                        ],
                                    ));
                                }
                            }
                            Field::Note => {
                                if _ctx.from_json {
                                    if r#note.is_some() {
                                        return Err(serde::de::Error::duplicate_field("note"));
                                    }
                                    r#note = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#note.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Source => {
                                if _ctx.from_json {
                                    if r#source.is_some() {
                                        return Err(serde::de::Error::duplicate_field("source"));
                                    }
                                    r#source = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#source.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
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
                                        "code",
                                        "status",
                                        "statusDate",
                                        "note",
                                        "source",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceDefinitionCode {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code,
                        r#status,
                        r#status_date,
                        r#note: r#note.unwrap_or(vec![]),
                        r#source: r#source.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Details of the official nature of this name."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionNameOfficial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Which authority uses this official name."]
    pub r#authority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the official name, for example 'draft', 'active', 'retired'."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Date of the official name change."]
    pub r#date: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for SubstanceDefinitionNameOfficial {
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
            if let Some(some) = self.r#authority.as_ref() {
                state.serialize_entry("authority", some)?;
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("date", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_date", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#date.as_ref() {
                    state.serialize_entry("date", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceDefinitionNameOfficial {
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
            #[serde(rename = "authority")]
            Authority,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceDefinitionNameOfficial;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceDefinitionNameOfficial")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceDefinitionNameOfficial, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#authority: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
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
                            Field::Authority => {
                                if r#authority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authority"));
                                }
                                r#authority = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value()?);
                            }
                            Field::Date => {
                                if _ctx.from_json {
                                    let some = r#date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("date"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#date.is_some() {
                                        return Err(serde::de::Error::duplicate_field("date"));
                                    }
                                    r#date = Some(map_access.next_value()?);
                                }
                            }
                            Field::DatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_date"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "date",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "authority",
                                            "status",
                                            "date",
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
                                        "authority",
                                        "status",
                                        "date",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceDefinitionNameOfficial {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#authority,
                        r#status,
                        r#date,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Names applicable to this substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The actual name."]
    pub r#name: super::super::types::String,
    #[doc = "Name type, for example 'systematic',  'scientific, 'brand'."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the name, for example 'current', 'proposed'."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If this is the preferred name for this substance."]
    pub r#preferred: Option<super::super::types::Boolean>,
    #[doc = "Human language that the name is written in."]
    pub r#language: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The use context of this name for example if there is a different name a drug active ingredient as opposed to a food colour additive."]
    pub r#domain: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The jurisdiction where this name applies."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A synonym of this particular name, by which the substance is also known."]
    pub r#synonym: Vec<SubstanceDefinitionName>,
    #[doc = "A translation for this name into another human language."]
    pub r#translation: Vec<SubstanceDefinitionName>,
    #[doc = "Details of the official nature of this name."]
    pub r#official: Vec<SubstanceDefinitionNameOfficial>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceDefinitionName {
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
                if let Some(some) = self.r#name.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#name.id.as_ref(),
                        extension: &self.r#name.extension,
                    };
                    state.serialize_entry("_name", &primitive_element)?;
                }
            } else {
                state.serialize_entry("name", &self.r#name)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#preferred.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("preferred", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_preferred", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#preferred.as_ref() {
                    state.serialize_entry("preferred", some)?;
                }
            }
            if !self.r#language.is_empty() {
                state.serialize_entry("language", &self.r#language)?;
            }
            if !self.r#domain.is_empty() {
                state.serialize_entry("domain", &self.r#domain)?;
            }
            if !self.r#jurisdiction.is_empty() {
                state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
            }
            if !self.r#synonym.is_empty() {
                state.serialize_entry("synonym", &self.r#synonym)?;
            }
            if !self.r#translation.is_empty() {
                state.serialize_entry("translation", &self.r#translation)?;
            }
            if !self.r#official.is_empty() {
                state.serialize_entry("official", &self.r#official)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceDefinitionName {
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
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "preferred")]
            Preferred,
            #[serde(rename = "_preferred")]
            PreferredPrimitiveElement,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "domain")]
            Domain,
            #[serde(rename = "jurisdiction")]
            Jurisdiction,
            #[serde(rename = "synonym")]
            Synonym,
            #[serde(rename = "translation")]
            Translation,
            #[serde(rename = "official")]
            Official,
            #[serde(rename = "source")]
            Source,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceDefinitionName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceDefinitionName")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceDefinitionName, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#preferred: Option<super::super::types::Boolean> = None;
                let mut r#language: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#domain: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#jurisdiction: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#synonym: Option<Vec<SubstanceDefinitionName>> = None;
                let mut r#translation: Option<Vec<SubstanceDefinitionName>> = None;
                let mut r#official: Option<Vec<SubstanceDefinitionNameOfficial>> = None;
                let mut r#source: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                                            "name",
                                            "type",
                                            "status",
                                            "preferred",
                                            "language",
                                            "domain",
                                            "jurisdiction",
                                            "synonym",
                                            "translation",
                                            "official",
                                            "source",
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
                            Field::Status => {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value()?);
                            }
                            Field::Preferred => {
                                if _ctx.from_json {
                                    let some = r#preferred.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("preferred"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#preferred.is_some() {
                                        return Err(serde::de::Error::duplicate_field("preferred"));
                                    }
                                    r#preferred = Some(map_access.next_value()?);
                                }
                            }
                            Field::PreferredPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#preferred.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_preferred",
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
                                        "preferred",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "type",
                                            "status",
                                            "preferred",
                                            "language",
                                            "domain",
                                            "jurisdiction",
                                            "synonym",
                                            "translation",
                                            "official",
                                            "source",
                                        ],
                                    ));
                                }
                            }
                            Field::Language => {
                                if _ctx.from_json {
                                    if r#language.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    r#language = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#language.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Domain => {
                                if _ctx.from_json {
                                    if r#domain.is_some() {
                                        return Err(serde::de::Error::duplicate_field("domain"));
                                    }
                                    r#domain = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#domain.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Jurisdiction => {
                                if _ctx.from_json {
                                    if r#jurisdiction.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "jurisdiction",
                                        ));
                                    }
                                    r#jurisdiction = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#jurisdiction.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Synonym => {
                                if _ctx.from_json {
                                    if r#synonym.is_some() {
                                        return Err(serde::de::Error::duplicate_field("synonym"));
                                    }
                                    r#synonym = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#synonym.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Translation => {
                                if _ctx.from_json {
                                    if r#translation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "translation",
                                        ));
                                    }
                                    r#translation = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#translation.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Official => {
                                if _ctx.from_json {
                                    if r#official.is_some() {
                                        return Err(serde::de::Error::duplicate_field("official"));
                                    }
                                    r#official = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#official.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Source => {
                                if _ctx.from_json {
                                    if r#source.is_some() {
                                        return Err(serde::de::Error::duplicate_field("source"));
                                    }
                                    r#source = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#source.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
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
                                        "name",
                                        "type",
                                        "status",
                                        "preferred",
                                        "language",
                                        "domain",
                                        "jurisdiction",
                                        "synonym",
                                        "translation",
                                        "official",
                                        "source",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceDefinitionName {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#type,
                        r#status,
                        r#preferred,
                        r#language: r#language.unwrap_or(vec![]),
                        r#domain: r#domain.unwrap_or(vec![]),
                        r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                        r#synonym: r#synonym.unwrap_or(vec![]),
                        r#translation: r#translation.unwrap_or(vec![]),
                        r#official: r#official.unwrap_or(vec![]),
                        r#source: r#source.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A link between this substance and another, with details of the relationship."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionRelationship {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A pointer to another substance, as a resource or just a representational code."]
    pub r#substance_definition: Option<SubstanceDefinitionRelationshipSubstanceDefinition>,
    #[doc = "For example \"salt to parent\", \"active moiety\", \"starting material\", \"polymorph\", \"impurity of\"."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible substance relationships."]
    pub r#is_defining: Option<super::super::types::Boolean>,
    #[doc = "A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other."]
    pub r#amount: Option<SubstanceDefinitionRelationshipAmount>,
    #[doc = "For use when the numeric has an uncertain range."]
    pub r#ratio_high_limit_amount: Option<Box<super::super::types::Ratio>>,
    #[doc = "An operator for the amount, for example \"average\", \"approximately\", \"less than\"."]
    pub r#comparator: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceDefinitionRelationship {
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
            if let Some(some) = self.r#substance_definition.as_ref() {
                match some {
                    SubstanceDefinitionRelationshipSubstanceDefinition::Reference(ref value) => {
                        state.serialize_entry("substanceDefinitionReference", value)?;
                    }
                    SubstanceDefinitionRelationshipSubstanceDefinition::CodeableConcept(
                        ref value,
                    ) => {
                        state.serialize_entry("substanceDefinitionCodeableConcept", value)?;
                    }
                    SubstanceDefinitionRelationshipSubstanceDefinition::Invalid => {
                        return Err(serde::ser::Error::custom("substance_definition is invalid"))
                    }
                }
            }
            state.serialize_entry("type", &self.r#type)?;
            if _ctx.output_json {
                if let Some(some) = self.r#is_defining.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("isDefining", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_isDefining", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#is_defining.as_ref() {
                    state.serialize_entry("isDefining", some)?;
                }
            }
            if let Some(some) = self.r#amount.as_ref() {
                match some {
                    SubstanceDefinitionRelationshipAmount::Quantity(ref value) => {
                        state.serialize_entry("amountQuantity", value)?;
                    }
                    SubstanceDefinitionRelationshipAmount::Ratio(ref value) => {
                        state.serialize_entry("amountRatio", value)?;
                    }
                    SubstanceDefinitionRelationshipAmount::String(ref value) => {
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
                    SubstanceDefinitionRelationshipAmount::Invalid => {
                        return Err(serde::ser::Error::custom("amount is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#ratio_high_limit_amount.as_ref() {
                state.serialize_entry("ratioHighLimitAmount", some)?;
            }
            if let Some(some) = self.r#comparator.as_ref() {
                state.serialize_entry("comparator", some)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceDefinitionRelationship {
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
            #[serde(rename = "substanceDefinitionReference")]
            SubstanceDefinitionReference,
            #[serde(rename = "substanceDefinitionCodeableConcept")]
            SubstanceDefinitionCodeableConcept,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "isDefining")]
            IsDefining,
            #[serde(rename = "_isDefining")]
            IsDefiningPrimitiveElement,
            #[serde(rename = "amountQuantity")]
            AmountQuantity,
            #[serde(rename = "amountRatio")]
            AmountRatio,
            #[serde(rename = "amountString")]
            AmountString,
            #[serde(rename = "_amountString")]
            AmountStringPrimitiveElement,
            #[serde(rename = "ratioHighLimitAmount")]
            RatioHighLimitAmount,
            #[serde(rename = "comparator")]
            Comparator,
            #[serde(rename = "source")]
            Source,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceDefinitionRelationship;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceDefinitionRelationship")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceDefinitionRelationship, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#substance_definition: Option<
                    SubstanceDefinitionRelationshipSubstanceDefinition,
                > = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#is_defining: Option<super::super::types::Boolean> = None;
                let mut r#amount: Option<SubstanceDefinitionRelationshipAmount> = None;
                let mut r#ratio_high_limit_amount: Option<Box<super::super::types::Ratio>> = None;
                let mut r#comparator: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#source: Option<Vec<Box<super::super::types::Reference>>> = None;
                fhirbolt_shared :: serde_context :: de :: DESERIALIZATION_CONTEXT . with (| _ctx | { let _ctx = _ctx . borrow () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if _ctx . from_json { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } else { let vec = r#extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value () ?) ; } } , Field :: ModifierExtension => { if _ctx . from_json { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } else { let vec = r#modifier_extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value () ?) ; } } , Field :: SubstanceDefinitionReference => { if r#substance_definition . is_some () { return Err (serde :: de :: Error :: duplicate_field ("substanceDefinitionReference")) ; } r#substance_definition = Some (SubstanceDefinitionRelationshipSubstanceDefinition :: Reference (map_access . next_value () ?)) ; } , Field :: SubstanceDefinitionCodeableConcept => { if r#substance_definition . is_some () { return Err (serde :: de :: Error :: duplicate_field ("substanceDefinitionCodeableConcept")) ; } r#substance_definition = Some (SubstanceDefinitionRelationshipSubstanceDefinition :: CodeableConcept (map_access . next_value () ?)) ; } , Field :: Type => { if r#type . is_some () { return Err (serde :: de :: Error :: duplicate_field ("type")) ; } r#type = Some (map_access . next_value () ?) ; } , Field :: IsDefining => { if _ctx . from_json { let some = r#is_defining . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("isDefining")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#is_defining . is_some () { return Err (serde :: de :: Error :: duplicate_field ("isDefining")) ; } r#is_defining = Some (map_access . next_value () ?) ; } } , Field :: IsDefiningPrimitiveElement => { if _ctx . from_json { let some = r#is_defining . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_isDefining")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("isDefining" , & ["id" , "extension" , "modifierExtension" , "substanceDefinitionReference" , "substanceDefinitionCodeableConcept" , "type" , "isDefining" , "amountQuantity" , "amountRatio" , "amountString" , "ratioHighLimitAmount" , "comparator" , "source" ,])) ; } } , Field :: AmountQuantity => { if r#amount . is_some () { return Err (serde :: de :: Error :: duplicate_field ("amountQuantity")) ; } r#amount = Some (SubstanceDefinitionRelationshipAmount :: Quantity (map_access . next_value () ?)) ; } , Field :: AmountRatio => { if r#amount . is_some () { return Err (serde :: de :: Error :: duplicate_field ("amountRatio")) ; } r#amount = Some (SubstanceDefinitionRelationshipAmount :: Ratio (map_access . next_value () ?)) ; } , Field :: AmountString => { if _ctx . from_json { let r#enum = r#amount . get_or_insert (SubstanceDefinitionRelationshipAmount :: String (Default :: default ())) ; if let SubstanceDefinitionRelationshipAmount :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("amountString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("amount[x]")) ; } } else { if r#amount . is_some () { return Err (serde :: de :: Error :: duplicate_field ("amountString")) ; } r#amount = Some (SubstanceDefinitionRelationshipAmount :: String (map_access . next_value () ?)) ; } } , Field :: AmountStringPrimitiveElement => { if _ctx . from_json { let r#enum = r#amount . get_or_insert (SubstanceDefinitionRelationshipAmount :: String (Default :: default ())) ; if let SubstanceDefinitionRelationshipAmount :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_amountString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_amount[x]")) ; } } else { return Err (serde :: de :: Error :: unknown_field ("amountString" , & ["id" , "extension" , "modifierExtension" , "substanceDefinitionReference" , "substanceDefinitionCodeableConcept" , "type" , "isDefining" , "amountQuantity" , "amountRatio" , "amountString" , "ratioHighLimitAmount" , "comparator" , "source" ,])) ; } } , Field :: RatioHighLimitAmount => { if r#ratio_high_limit_amount . is_some () { return Err (serde :: de :: Error :: duplicate_field ("ratioHighLimitAmount")) ; } r#ratio_high_limit_amount = Some (map_access . next_value () ?) ; } , Field :: Comparator => { if r#comparator . is_some () { return Err (serde :: de :: Error :: duplicate_field ("comparator")) ; } r#comparator = Some (map_access . next_value () ?) ; } , Field :: Source => { if _ctx . from_json { if r#source . is_some () { return Err (serde :: de :: Error :: duplicate_field ("source")) ; } r#source = Some (map_access . next_value () ?) ; } else { let vec = r#source . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value () ?) ; } } , Field :: Unknown (key) => if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "substanceDefinitionReference" , "substanceDefinitionCodeableConcept" , "type" , "isDefining" , "amountQuantity" , "amountRatio" , "amountString" , "ratioHighLimitAmount" , "comparator" , "source" ,])) ; } } } Ok (SubstanceDefinitionRelationship { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#substance_definition , r#type : if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Lax { r#type . unwrap_or (Default :: default ()) } else { r#type . ok_or (serde :: de :: Error :: missing_field ("type")) ? } , r#is_defining , r#amount , r#ratio_high_limit_amount , r#comparator , r#source : r#source . unwrap_or (vec ! []) , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Material or taxonomic/anatomical source for the substance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionSourceMaterial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A classification that provides the origin of the raw material. Example: cat hair would be an Animal source type."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The genus of an organism, typically referring to the Latin epithet of the genus element of the plant/animal scientific name."]
    pub r#genus: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The species of an organism, typically referring to the Latin epithet of the species of the plant/animal."]
    pub r#species: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An anatomical origin of the source material within an organism."]
    pub r#part: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The country or countries where the material is harvested."]
    pub r#country_of_origin: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubstanceDefinitionSourceMaterial {
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
            if let Some(some) = self.r#genus.as_ref() {
                state.serialize_entry("genus", some)?;
            }
            if let Some(some) = self.r#species.as_ref() {
                state.serialize_entry("species", some)?;
            }
            if let Some(some) = self.r#part.as_ref() {
                state.serialize_entry("part", some)?;
            }
            if !self.r#country_of_origin.is_empty() {
                state.serialize_entry("countryOfOrigin", &self.r#country_of_origin)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceDefinitionSourceMaterial {
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
            #[serde(rename = "genus")]
            Genus,
            #[serde(rename = "species")]
            Species,
            #[serde(rename = "part")]
            Part,
            #[serde(rename = "countryOfOrigin")]
            CountryOfOrigin,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceDefinitionSourceMaterial;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceDefinitionSourceMaterial")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceDefinitionSourceMaterial, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#genus: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#species: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#part: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#country_of_origin: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Genus => {
                                if r#genus.is_some() {
                                    return Err(serde::de::Error::duplicate_field("genus"));
                                }
                                r#genus = Some(map_access.next_value()?);
                            }
                            Field::Species => {
                                if r#species.is_some() {
                                    return Err(serde::de::Error::duplicate_field("species"));
                                }
                                r#species = Some(map_access.next_value()?);
                            }
                            Field::Part => {
                                if r#part.is_some() {
                                    return Err(serde::de::Error::duplicate_field("part"));
                                }
                                r#part = Some(map_access.next_value()?);
                            }
                            Field::CountryOfOrigin => {
                                if _ctx.from_json {
                                    if r#country_of_origin.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "countryOfOrigin",
                                        ));
                                    }
                                    r#country_of_origin = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#country_of_origin.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
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
                                        "genus",
                                        "species",
                                        "part",
                                        "countryOfOrigin",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceDefinitionSourceMaterial {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#genus,
                        r#species,
                        r#part,
                        r#country_of_origin: r#country_of_origin.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The detailed description of a substance, typically at a level beyond what is used for prescribing."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubstanceDefinition {
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
    #[doc = "Identifier by which this substance is known."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A business level version identifier of the substance."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Status of substance within the catalogue e.g. active, retired."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A high level categorization, e.g. polymer or nucleic acid, or food, chemical, biological, or a lower level such as the general types of polymer (linear or branch chain) or type of impurity (process related or contaminant)."]
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "If the substance applies to human or veterinary use."]
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The quality standard, established benchmark, to which substance complies (e.g. USP/NF, Ph. Eur, JP, BP, Company Standard)."]
    pub r#grade: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Textual description of the substance."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Supporting literature."]
    pub r#information_source: Vec<Box<super::super::types::Reference>>,
    #[doc = "Textual comment about the substance's catalogue or registry record."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "The entity that creates, makes, produces or fabricates the substance. This is a set of potential manufacturers but is not necessarily comprehensive."]
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    #[doc = "An entity that is the source for the substance. It may be different from the manufacturer. Supplier is synonymous to a distributor."]
    pub r#supplier: Vec<Box<super::super::types::Reference>>,
    #[doc = "Moiety, for structural modifications."]
    pub r#moiety: Vec<SubstanceDefinitionMoiety>,
    #[doc = "General specifications for this substance."]
    pub r#property: Vec<SubstanceDefinitionProperty>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Vec<SubstanceDefinitionMolecularWeight>,
    #[doc = "Structural information."]
    pub r#structure: Option<SubstanceDefinitionStructure>,
    #[doc = "Codes associated with the substance."]
    pub r#code: Vec<SubstanceDefinitionCode>,
    #[doc = "Names applicable to this substance."]
    pub r#name: Vec<SubstanceDefinitionName>,
    #[doc = "A link between this substance and another, with details of the relationship."]
    pub r#relationship: Vec<SubstanceDefinitionRelationship>,
    #[doc = "Material or taxonomic/anatomical source for the substance."]
    pub r#source_material: Option<SubstanceDefinitionSourceMaterial>,
}
impl crate::AnyResource for SubstanceDefinition {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4B;
}
impl serde::ser::Serialize for SubstanceDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "SubstanceDefinition")?;
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
                if let Some(some) = self.r#version.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("version", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_version", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#version.as_ref() {
                    state.serialize_entry("version", some)?;
                }
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if !self.r#classification.is_empty() {
                state.serialize_entry("classification", &self.r#classification)?;
            }
            if let Some(some) = self.r#domain.as_ref() {
                state.serialize_entry("domain", some)?;
            }
            if !self.r#grade.is_empty() {
                state.serialize_entry("grade", &self.r#grade)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            if !self.r#information_source.is_empty() {
                state.serialize_entry("informationSource", &self.r#information_source)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#manufacturer.is_empty() {
                state.serialize_entry("manufacturer", &self.r#manufacturer)?;
            }
            if !self.r#supplier.is_empty() {
                state.serialize_entry("supplier", &self.r#supplier)?;
            }
            if !self.r#moiety.is_empty() {
                state.serialize_entry("moiety", &self.r#moiety)?;
            }
            if !self.r#property.is_empty() {
                state.serialize_entry("property", &self.r#property)?;
            }
            if !self.r#molecular_weight.is_empty() {
                state.serialize_entry("molecularWeight", &self.r#molecular_weight)?;
            }
            if let Some(some) = self.r#structure.as_ref() {
                state.serialize_entry("structure", some)?;
            }
            if !self.r#code.is_empty() {
                state.serialize_entry("code", &self.r#code)?;
            }
            if !self.r#name.is_empty() {
                state.serialize_entry("name", &self.r#name)?;
            }
            if !self.r#relationship.is_empty() {
                state.serialize_entry("relationship", &self.r#relationship)?;
            }
            if let Some(some) = self.r#source_material.as_ref() {
                state.serialize_entry("sourceMaterial", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceDefinition {
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
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "classification")]
            Classification,
            #[serde(rename = "domain")]
            Domain,
            #[serde(rename = "grade")]
            Grade,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "informationSource")]
            InformationSource,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "manufacturer")]
            Manufacturer,
            #[serde(rename = "supplier")]
            Supplier,
            #[serde(rename = "moiety")]
            Moiety,
            #[serde(rename = "property")]
            Property,
            #[serde(rename = "molecularWeight")]
            MolecularWeight,
            #[serde(rename = "structure")]
            Structure,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "relationship")]
            Relationship,
            #[serde(rename = "sourceMaterial")]
            SourceMaterial,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceDefinition, V::Error>
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
                let mut r#version: Option<super::super::types::String> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#classification: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#domain: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#grade: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#information_source: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#manufacturer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#supplier: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#moiety: Option<Vec<SubstanceDefinitionMoiety>> = None;
                let mut r#property: Option<Vec<SubstanceDefinitionProperty>> = None;
                let mut r#molecular_weight: Option<Vec<SubstanceDefinitionMolecularWeight>> = None;
                let mut r#structure: Option<SubstanceDefinitionStructure> = None;
                let mut r#code: Option<Vec<SubstanceDefinitionCode>> = None;
                let mut r#name: Option<Vec<SubstanceDefinitionName>> = None;
                let mut r#relationship: Option<Vec<SubstanceDefinitionRelationship>> = None;
                let mut r#source_material: Option<SubstanceDefinitionSourceMaterial> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "SubstanceDefinition" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"SubstanceDefinition",
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
                                            "version",
                                            "status",
                                            "classification",
                                            "domain",
                                            "grade",
                                            "description",
                                            "informationSource",
                                            "note",
                                            "manufacturer",
                                            "supplier",
                                            "moiety",
                                            "property",
                                            "molecularWeight",
                                            "structure",
                                            "code",
                                            "name",
                                            "relationship",
                                            "sourceMaterial",
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
                                            "version",
                                            "status",
                                            "classification",
                                            "domain",
                                            "grade",
                                            "description",
                                            "informationSource",
                                            "note",
                                            "manufacturer",
                                            "supplier",
                                            "moiety",
                                            "property",
                                            "molecularWeight",
                                            "structure",
                                            "code",
                                            "name",
                                            "relationship",
                                            "sourceMaterial",
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
                            Field::Version => {
                                if _ctx.from_json {
                                    let some = r#version.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#version.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    r#version = Some(map_access.next_value()?);
                                }
                            }
                            Field::VersionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#version.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_version"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "version",
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
                                            "version",
                                            "status",
                                            "classification",
                                            "domain",
                                            "grade",
                                            "description",
                                            "informationSource",
                                            "note",
                                            "manufacturer",
                                            "supplier",
                                            "moiety",
                                            "property",
                                            "molecularWeight",
                                            "structure",
                                            "code",
                                            "name",
                                            "relationship",
                                            "sourceMaterial",
                                        ],
                                    ));
                                }
                            }
                            Field::Status => {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value()?);
                            }
                            Field::Classification => {
                                if _ctx.from_json {
                                    if r#classification.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "classification",
                                        ));
                                    }
                                    r#classification = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#classification.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Domain => {
                                if r#domain.is_some() {
                                    return Err(serde::de::Error::duplicate_field("domain"));
                                }
                                r#domain = Some(map_access.next_value()?);
                            }
                            Field::Grade => {
                                if _ctx.from_json {
                                    if r#grade.is_some() {
                                        return Err(serde::de::Error::duplicate_field("grade"));
                                    }
                                    r#grade = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#grade.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
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
                                        "description",
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
                                            "version",
                                            "status",
                                            "classification",
                                            "domain",
                                            "grade",
                                            "description",
                                            "informationSource",
                                            "note",
                                            "manufacturer",
                                            "supplier",
                                            "moiety",
                                            "property",
                                            "molecularWeight",
                                            "structure",
                                            "code",
                                            "name",
                                            "relationship",
                                            "sourceMaterial",
                                        ],
                                    ));
                                }
                            }
                            Field::InformationSource => {
                                if _ctx.from_json {
                                    if r#information_source.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "informationSource",
                                        ));
                                    }
                                    r#information_source = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#information_source.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Note => {
                                if _ctx.from_json {
                                    if r#note.is_some() {
                                        return Err(serde::de::Error::duplicate_field("note"));
                                    }
                                    r#note = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#note.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Manufacturer => {
                                if _ctx.from_json {
                                    if r#manufacturer.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "manufacturer",
                                        ));
                                    }
                                    r#manufacturer = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#manufacturer.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Supplier => {
                                if _ctx.from_json {
                                    if r#supplier.is_some() {
                                        return Err(serde::de::Error::duplicate_field("supplier"));
                                    }
                                    r#supplier = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#supplier.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Moiety => {
                                if _ctx.from_json {
                                    if r#moiety.is_some() {
                                        return Err(serde::de::Error::duplicate_field("moiety"));
                                    }
                                    r#moiety = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#moiety.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Property => {
                                if _ctx.from_json {
                                    if r#property.is_some() {
                                        return Err(serde::de::Error::duplicate_field("property"));
                                    }
                                    r#property = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#property.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::MolecularWeight => {
                                if _ctx.from_json {
                                    if r#molecular_weight.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "molecularWeight",
                                        ));
                                    }
                                    r#molecular_weight = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#molecular_weight.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Structure => {
                                if r#structure.is_some() {
                                    return Err(serde::de::Error::duplicate_field("structure"));
                                }
                                r#structure = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if _ctx.from_json {
                                    if r#code.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    r#code = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#code.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Name => {
                                if _ctx.from_json {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#name.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Relationship => {
                                if _ctx.from_json {
                                    if r#relationship.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "relationship",
                                        ));
                                    }
                                    r#relationship = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#relationship.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::SourceMaterial => {
                                if r#source_material.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "sourceMaterial",
                                    ));
                                }
                                r#source_material = Some(map_access.next_value()?);
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
                                        "version",
                                        "status",
                                        "classification",
                                        "domain",
                                        "grade",
                                        "description",
                                        "informationSource",
                                        "note",
                                        "manufacturer",
                                        "supplier",
                                        "moiety",
                                        "property",
                                        "molecularWeight",
                                        "structure",
                                        "code",
                                        "name",
                                        "relationship",
                                        "sourceMaterial",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceDefinition {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#version,
                        r#status,
                        r#classification: r#classification.unwrap_or(vec![]),
                        r#domain,
                        r#grade: r#grade.unwrap_or(vec![]),
                        r#description,
                        r#information_source: r#information_source.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                        r#manufacturer: r#manufacturer.unwrap_or(vec![]),
                        r#supplier: r#supplier.unwrap_or(vec![]),
                        r#moiety: r#moiety.unwrap_or(vec![]),
                        r#property: r#property.unwrap_or(vec![]),
                        r#molecular_weight: r#molecular_weight.unwrap_or(vec![]),
                        r#structure,
                        r#code: r#code.unwrap_or(vec![]),
                        r#name: r#name.unwrap_or(vec![]),
                        r#relationship: r#relationship.unwrap_or(vec![]),
                        r#source_material,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
