// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "Quantitative value for this moiety."]
#[derive(Debug, Clone)]
pub enum SubstanceSpecificationMoietyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SubstanceSpecificationMoietyAmount {
    fn default() -> SubstanceSpecificationMoietyAmount {
        SubstanceSpecificationMoietyAmount::Invalid
    }
}
#[doc = "A substance upon which a defining property depends (e.g. for solubility: in water, in alcohol)."]
#[derive(Debug, Clone)]
pub enum SubstanceSpecificationPropertyDefiningSubstance {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for SubstanceSpecificationPropertyDefiningSubstance {
    fn default() -> SubstanceSpecificationPropertyDefiningSubstance {
        SubstanceSpecificationPropertyDefiningSubstance::Invalid
    }
}
#[doc = "Quantitative value for this property."]
#[derive(Debug, Clone)]
pub enum SubstanceSpecificationPropertyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SubstanceSpecificationPropertyAmount {
    fn default() -> SubstanceSpecificationPropertyAmount {
        SubstanceSpecificationPropertyAmount::Invalid
    }
}
#[doc = "A pointer to another substance, as a resource or just a representational code."]
#[derive(Debug, Clone)]
pub enum SubstanceSpecificationRelationshipSubstance {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for SubstanceSpecificationRelationshipSubstance {
    fn default() -> SubstanceSpecificationRelationshipSubstance {
        SubstanceSpecificationRelationshipSubstance::Invalid
    }
}
#[doc = "A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other."]
#[derive(Debug, Clone)]
pub enum SubstanceSpecificationRelationshipAmount {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SubstanceSpecificationRelationshipAmount {
    fn default() -> SubstanceSpecificationRelationshipAmount {
        SubstanceSpecificationRelationshipAmount::Invalid
    }
}
#[doc = "Moiety, for structural modifications."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationMoiety {
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
    #[doc = "Molecular formula."]
    pub r#molecular_formula: Option<super::super::types::String>,
    #[doc = "Quantitative value for this moiety."]
    pub r#amount: Option<SubstanceSpecificationMoietyAmount>,
}
impl serde::ser::Serialize for SubstanceSpecificationMoiety {
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
                    SubstanceSpecificationMoietyAmount::Quantity(ref value) => {
                        state.serialize_entry("amountQuantity", value)?;
                    }
                    SubstanceSpecificationMoietyAmount::String(ref value) => {
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
                    SubstanceSpecificationMoietyAmount::Invalid => {
                        return Err(serde::ser::Error::custom("amount is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSpecificationMoiety {
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
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSpecificationMoiety;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSpecificationMoiety")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSpecificationMoiety, V::Error>
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
                let mut r#amount: Option<SubstanceSpecificationMoietyAmount> = None;
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
                                r#amount = Some(SubstanceSpecificationMoietyAmount::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::AmountString => {
                                if _ctx.from_json {
                                    let r#enum = r#amount.get_or_insert(
                                        SubstanceSpecificationMoietyAmount::String(
                                            Default::default(),
                                        ),
                                    );
                                    if let SubstanceSpecificationMoietyAmount::String(variant) =
                                        r#enum
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
                                    r#amount = Some(SubstanceSpecificationMoietyAmount::String(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::AmountStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#amount.get_or_insert(
                                        SubstanceSpecificationMoietyAmount::String(
                                            Default::default(),
                                        ),
                                    );
                                    if let SubstanceSpecificationMoietyAmount::String(variant) =
                                        r#enum
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
                                        "role",
                                        "identifier",
                                        "name",
                                        "stereochemistry",
                                        "opticalActivity",
                                        "molecularFormula",
                                        "amountQuantity",
                                        "amountString",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceSpecificationMoiety {
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
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "General specifications for this substance, including how it is related to other substances."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A category for this property, e.g. Physical, Chemical, Enzymatic."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Property type e.g. viscosity, pH, isoelectric point."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Parameters that were used in the measurement of a property (e.g. for viscosity: measured at 20C with a pH of 7.1)."]
    pub r#parameters: Option<super::super::types::String>,
    #[doc = "A substance upon which a defining property depends (e.g. for solubility: in water, in alcohol)."]
    pub r#defining_substance: Option<SubstanceSpecificationPropertyDefiningSubstance>,
    #[doc = "Quantitative value for this property."]
    pub r#amount: Option<SubstanceSpecificationPropertyAmount>,
}
impl serde::ser::Serialize for SubstanceSpecificationProperty {
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
            if let Some(some) = self.r#category.as_ref() {
                state.serialize_entry("category", some)?;
            }
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#parameters.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("parameters", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_parameters", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#parameters.as_ref() {
                    state.serialize_entry("parameters", some)?;
                }
            }
            if let Some(some) = self.r#defining_substance.as_ref() {
                match some {
                    SubstanceSpecificationPropertyDefiningSubstance::Reference(ref value) => {
                        state.serialize_entry("definingSubstanceReference", value)?;
                    }
                    SubstanceSpecificationPropertyDefiningSubstance::CodeableConcept(ref value) => {
                        state.serialize_entry("definingSubstanceCodeableConcept", value)?;
                    }
                    SubstanceSpecificationPropertyDefiningSubstance::Invalid => {
                        return Err(serde::ser::Error::custom("defining_substance is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#amount.as_ref() {
                match some {
                    SubstanceSpecificationPropertyAmount::Quantity(ref value) => {
                        state.serialize_entry("amountQuantity", value)?;
                    }
                    SubstanceSpecificationPropertyAmount::String(ref value) => {
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
                    SubstanceSpecificationPropertyAmount::Invalid => {
                        return Err(serde::ser::Error::custom("amount is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSpecificationProperty {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "parameters")]
            Parameters,
            #[serde(rename = "_parameters")]
            ParametersPrimitiveElement,
            #[serde(rename = "definingSubstanceReference")]
            DefiningSubstanceReference,
            #[serde(rename = "definingSubstanceCodeableConcept")]
            DefiningSubstanceCodeableConcept,
            #[serde(rename = "amountQuantity")]
            AmountQuantity,
            #[serde(rename = "amountString")]
            AmountString,
            #[serde(rename = "_amountString")]
            AmountStringPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSpecificationProperty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSpecificationProperty")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSpecificationProperty, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#parameters: Option<super::super::types::String> = None;
                let mut r#defining_substance: Option<
                    SubstanceSpecificationPropertyDefiningSubstance,
                > = None;
                let mut r#amount: Option<SubstanceSpecificationPropertyAmount> = None;
                fhirbolt_shared :: serde_context :: de :: DESERIALIZATION_CONTEXT . with (| _ctx | { let _ctx = _ctx . get () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } , Field :: ModifierExtension => { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } , Field :: Category => { if r#category . is_some () { return Err (serde :: de :: Error :: duplicate_field ("category")) ; } r#category = Some (map_access . next_value () ?) ; } , Field :: Code => { if r#code . is_some () { return Err (serde :: de :: Error :: duplicate_field ("code")) ; } r#code = Some (map_access . next_value () ?) ; } , Field :: Parameters => { if _ctx . from_json { let some = r#parameters . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("parameters")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#parameters . is_some () { return Err (serde :: de :: Error :: duplicate_field ("parameters")) ; } r#parameters = Some (map_access . next_value () ?) ; } } , Field :: ParametersPrimitiveElement => { if _ctx . from_json { let some = r#parameters . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_parameters")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("parameters" , & ["id" , "extension" , "modifierExtension" , "category" , "code" , "parameters" , "definingSubstanceReference" , "definingSubstanceCodeableConcept" , "amountQuantity" , "amountString" ,])) ; } } , Field :: DefiningSubstanceReference => { if r#defining_substance . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definingSubstanceReference")) ; } r#defining_substance = Some (SubstanceSpecificationPropertyDefiningSubstance :: Reference (map_access . next_value () ?)) ; } , Field :: DefiningSubstanceCodeableConcept => { if r#defining_substance . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definingSubstanceCodeableConcept")) ; } r#defining_substance = Some (SubstanceSpecificationPropertyDefiningSubstance :: CodeableConcept (map_access . next_value () ?)) ; } , Field :: AmountQuantity => { if r#amount . is_some () { return Err (serde :: de :: Error :: duplicate_field ("amountQuantity")) ; } r#amount = Some (SubstanceSpecificationPropertyAmount :: Quantity (map_access . next_value () ?)) ; } , Field :: AmountString => { if _ctx . from_json { let r#enum = r#amount . get_or_insert (SubstanceSpecificationPropertyAmount :: String (Default :: default ())) ; if let SubstanceSpecificationPropertyAmount :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("amountString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("amount[x]")) ; } } else { if r#amount . is_some () { return Err (serde :: de :: Error :: duplicate_field ("amountString")) ; } r#amount = Some (SubstanceSpecificationPropertyAmount :: String (map_access . next_value () ?)) ; } } , Field :: AmountStringPrimitiveElement => { if _ctx . from_json { let r#enum = r#amount . get_or_insert (SubstanceSpecificationPropertyAmount :: String (Default :: default ())) ; if let SubstanceSpecificationPropertyAmount :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_amountString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_amount[x]")) ; } } else { return Err (serde :: de :: Error :: unknown_field ("amountString" , & ["id" , "extension" , "modifierExtension" , "category" , "code" , "parameters" , "definingSubstanceReference" , "definingSubstanceCodeableConcept" , "amountQuantity" , "amountString" ,])) ; } } , Field :: Unknown (key) => if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "category" , "code" , "parameters" , "definingSubstanceReference" , "definingSubstanceCodeableConcept" , "amountQuantity" , "amountString" ,])) ; } } } Ok (SubstanceSpecificationProperty { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#category , r#code , r#parameters , r#defining_substance , r#amount , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationStructureIsotopeMolecularWeight {
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
    pub r#amount: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for SubstanceSpecificationStructureIsotopeMolecularWeight {
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
            if let Some(some) = self.r#method.as_ref() {
                state.serialize_entry("method", some)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#amount.as_ref() {
                state.serialize_entry("amount", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSpecificationStructureIsotopeMolecularWeight {
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
            type Value = SubstanceSpecificationStructureIsotopeMolecularWeight;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSpecificationStructureIsotopeMolecularWeight")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSpecificationStructureIsotopeMolecularWeight, V::Error>
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
                    Ok(SubstanceSpecificationStructureIsotopeMolecularWeight {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#method,
                        r#type,
                        r#amount,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Applicable for single substances that contain a radionuclide or a non-natural isotopic ratio."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationStructureIsotope {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Substance identifier for each non-natural or radioisotope."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Substance name for each non-natural or radioisotope."]
    pub r#name: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of isotopic substitution present in a single substance."]
    pub r#substitution: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Half life - for a non-natural nuclide."]
    pub r#half_life: Option<Box<super::super::types::Quantity>>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
}
impl serde::ser::Serialize for SubstanceSpecificationStructureIsotope {
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
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            if let Some(some) = self.r#name.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if let Some(some) = self.r#substitution.as_ref() {
                state.serialize_entry("substitution", some)?;
            }
            if let Some(some) = self.r#half_life.as_ref() {
                state.serialize_entry("halfLife", some)?;
            }
            if let Some(some) = self.r#molecular_weight.as_ref() {
                state.serialize_entry("molecularWeight", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSpecificationStructureIsotope {
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "substitution")]
            Substitution,
            #[serde(rename = "halfLife")]
            HalfLife,
            #[serde(rename = "molecularWeight")]
            MolecularWeight,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSpecificationStructureIsotope;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSpecificationStructureIsotope")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSpecificationStructureIsotope, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#name: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#substitution: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#half_life: Option<Box<super::super::types::Quantity>> = None;
                let mut r#molecular_weight: Option<
                    SubstanceSpecificationStructureIsotopeMolecularWeight,
                > = None;
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
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Name => {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                r#name = Some(map_access.next_value()?);
                            }
                            Field::Substitution => {
                                if r#substitution.is_some() {
                                    return Err(serde::de::Error::duplicate_field("substitution"));
                                }
                                r#substitution = Some(map_access.next_value()?);
                            }
                            Field::HalfLife => {
                                if r#half_life.is_some() {
                                    return Err(serde::de::Error::duplicate_field("halfLife"));
                                }
                                r#half_life = Some(map_access.next_value()?);
                            }
                            Field::MolecularWeight => {
                                if r#molecular_weight.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "molecularWeight",
                                    ));
                                }
                                r#molecular_weight = Some(map_access.next_value()?);
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
                                        "identifier",
                                        "name",
                                        "substitution",
                                        "halfLife",
                                        "molecularWeight",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceSpecificationStructureIsotope {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier,
                        r#name,
                        r#substitution,
                        r#half_life,
                        r#molecular_weight,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Molecular structural representation."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationStructureRepresentation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of structure (e.g. Full, Partial, Representative)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The structural representation as text string in a format e.g. InChI, SMILES, MOLFILE, CDX."]
    pub r#representation: Option<super::super::types::String>,
    #[doc = "An attached file with the structural representation."]
    pub r#attachment: Option<Box<super::super::types::Attachment>>,
}
impl serde::ser::Serialize for SubstanceSpecificationStructureRepresentation {
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
            if let Some(some) = self.r#attachment.as_ref() {
                state.serialize_entry("attachment", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSpecificationStructureRepresentation {
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
            #[serde(rename = "attachment")]
            Attachment,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSpecificationStructureRepresentation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSpecificationStructureRepresentation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSpecificationStructureRepresentation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#representation: Option<super::super::types::String> = None;
                let mut r#attachment: Option<Box<super::super::types::Attachment>> = None;
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
                                            "attachment",
                                        ],
                                    ));
                                }
                            }
                            Field::Attachment => {
                                if r#attachment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("attachment"));
                                }
                                r#attachment = Some(map_access.next_value()?);
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
                                        "attachment",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceSpecificationStructureRepresentation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#representation,
                        r#attachment,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Structural information."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationStructure {
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
    #[doc = "Molecular formula."]
    pub r#molecular_formula: Option<super::super::types::String>,
    #[doc = "Specified per moiety according to the Hill system, i.e. first C, then H, then alphabetical, each moiety separated by a dot."]
    pub r#molecular_formula_by_moiety: Option<super::super::types::String>,
    #[doc = "Applicable for single substances that contain a radionuclide or a non-natural isotopic ratio."]
    pub r#isotope: Vec<SubstanceSpecificationStructureIsotope>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
    #[doc = "Molecular structural representation."]
    pub r#representation: Vec<SubstanceSpecificationStructureRepresentation>,
}
impl serde::ser::Serialize for SubstanceSpecificationStructure {
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
            if !self.r#isotope.is_empty() {
                state.serialize_entry("isotope", &self.r#isotope)?;
            }
            if let Some(some) = self.r#molecular_weight.as_ref() {
                state.serialize_entry("molecularWeight", some)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            if !self.r#representation.is_empty() {
                state.serialize_entry("representation", &self.r#representation)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSpecificationStructure {
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
            #[serde(rename = "isotope")]
            Isotope,
            #[serde(rename = "molecularWeight")]
            MolecularWeight,
            #[serde(rename = "source")]
            Source,
            #[serde(rename = "representation")]
            Representation,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSpecificationStructure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSpecificationStructure")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSpecificationStructure, V::Error>
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
                let mut r#isotope: Option<Vec<SubstanceSpecificationStructureIsotope>> = None;
                let mut r#molecular_weight: Option<
                    SubstanceSpecificationStructureIsotopeMolecularWeight,
                > = None;
                let mut r#source: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#representation: Option<
                    Vec<SubstanceSpecificationStructureRepresentation>,
                > = None;
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
                                            "isotope",
                                            "molecularWeight",
                                            "source",
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
                                            "isotope",
                                            "molecularWeight",
                                            "source",
                                            "representation",
                                        ],
                                    ));
                                }
                            }
                            Field::Isotope => {
                                if r#isotope.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isotope"));
                                }
                                r#isotope = Some(map_access.next_value()?);
                            }
                            Field::MolecularWeight => {
                                if r#molecular_weight.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "molecularWeight",
                                    ));
                                }
                                r#molecular_weight = Some(map_access.next_value()?);
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
                            }
                            Field::Representation => {
                                if r#representation.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "representation",
                                    ));
                                }
                                r#representation = Some(map_access.next_value()?);
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
                                        "isotope",
                                        "molecularWeight",
                                        "source",
                                        "representation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceSpecificationStructure {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#stereochemistry,
                        r#optical_activity,
                        r#molecular_formula,
                        r#molecular_formula_by_moiety,
                        r#isotope: r#isotope.unwrap_or(vec![]),
                        r#molecular_weight,
                        r#source: r#source.unwrap_or(vec![]),
                        r#representation: r#representation.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Codes associated with the substance."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationCode {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specific code."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Status of the code assignment."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date at which the code status is changed as part of the terminology maintenance."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "Any comment can be provided in this field, if necessary."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceSpecificationCode {
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
            if _ctx.output_json {
                if let Some(some) = self.r#comment.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("comment", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_comment", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#comment.as_ref() {
                    state.serialize_entry("comment", some)?;
                }
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSpecificationCode {
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
            #[serde(rename = "comment")]
            Comment,
            #[serde(rename = "_comment")]
            CommentPrimitiveElement,
            #[serde(rename = "source")]
            Source,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSpecificationCode;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSpecificationCode")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceSpecificationCode, V::Error>
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
                let mut r#comment: Option<super::super::types::String> = None;
                let mut r#source: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                                            "comment",
                                            "source",
                                        ],
                                    ));
                                }
                            }
                            Field::Comment => {
                                if _ctx.from_json {
                                    let some = r#comment.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#comment.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    r#comment = Some(map_access.next_value()?);
                                }
                            }
                            Field::CommentPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#comment.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_comment"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "comment",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "status",
                                            "statusDate",
                                            "comment",
                                            "source",
                                        ],
                                    ));
                                }
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
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
                                        "comment",
                                        "source",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceSpecificationCode {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code,
                        r#status,
                        r#status_date,
                        r#comment,
                        r#source: r#source.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Details of the official nature of this name."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationNameOfficial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Which authority uses this official name."]
    pub r#authority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the official name."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Date of official name change."]
    pub r#date: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for SubstanceSpecificationNameOfficial {
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
impl<'de> serde::de::Deserialize<'de> for SubstanceSpecificationNameOfficial {
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
            type Value = SubstanceSpecificationNameOfficial;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSpecificationNameOfficial")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSpecificationNameOfficial, V::Error>
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
                    Ok(SubstanceSpecificationNameOfficial {
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
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The actual name."]
    pub r#name: super::super::types::String,
    #[doc = "Name type."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the name."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If this is the preferred name for this substance."]
    pub r#preferred: Option<super::super::types::Boolean>,
    #[doc = "Language of the name."]
    pub r#language: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The use context of this name for example if there is a different name a drug active ingredient as opposed to a food colour additive."]
    pub r#domain: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The jurisdiction where this name applies."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A synonym of this name."]
    pub r#synonym: Vec<SubstanceSpecificationName>,
    #[doc = "A translation for this name."]
    pub r#translation: Vec<SubstanceSpecificationName>,
    #[doc = "Details of the official nature of this name."]
    pub r#official: Vec<SubstanceSpecificationNameOfficial>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceSpecificationName {
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
impl<'de> serde::de::Deserialize<'de> for SubstanceSpecificationName {
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
            type Value = SubstanceSpecificationName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSpecificationName")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceSpecificationName, V::Error>
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
                let mut r#synonym: Option<Vec<SubstanceSpecificationName>> = None;
                let mut r#translation: Option<Vec<SubstanceSpecificationName>> = None;
                let mut r#official: Option<Vec<SubstanceSpecificationNameOfficial>> = None;
                let mut r#source: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                r#language = Some(map_access.next_value()?);
                            }
                            Field::Domain => {
                                if r#domain.is_some() {
                                    return Err(serde::de::Error::duplicate_field("domain"));
                                }
                                r#domain = Some(map_access.next_value()?);
                            }
                            Field::Jurisdiction => {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                r#jurisdiction = Some(map_access.next_value()?);
                            }
                            Field::Synonym => {
                                if r#synonym.is_some() {
                                    return Err(serde::de::Error::duplicate_field("synonym"));
                                }
                                r#synonym = Some(map_access.next_value()?);
                            }
                            Field::Translation => {
                                if r#translation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("translation"));
                                }
                                r#translation = Some(map_access.next_value()?);
                            }
                            Field::Official => {
                                if r#official.is_some() {
                                    return Err(serde::de::Error::duplicate_field("official"));
                                }
                                r#official = Some(map_access.next_value()?);
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
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
                    Ok(SubstanceSpecificationName {
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
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationRelationship {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A pointer to another substance, as a resource or just a representational code."]
    pub r#substance: Option<SubstanceSpecificationRelationshipSubstance>,
    #[doc = "For example \"salt to parent\", \"active moiety\", \"starting material\"."]
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible substance relationships."]
    pub r#is_defining: Option<super::super::types::Boolean>,
    #[doc = "A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other."]
    pub r#amount: Option<SubstanceSpecificationRelationshipAmount>,
    #[doc = "For use when the numeric."]
    pub r#amount_ratio_low_limit: Option<Box<super::super::types::Ratio>>,
    #[doc = "An operator for the amount, for example \"average\", \"approximately\", \"less than\"."]
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceSpecificationRelationship {
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
            if let Some(some) = self.r#substance.as_ref() {
                match some {
                    SubstanceSpecificationRelationshipSubstance::Reference(ref value) => {
                        state.serialize_entry("substanceReference", value)?;
                    }
                    SubstanceSpecificationRelationshipSubstance::CodeableConcept(ref value) => {
                        state.serialize_entry("substanceCodeableConcept", value)?;
                    }
                    SubstanceSpecificationRelationshipSubstance::Invalid => {
                        return Err(serde::ser::Error::custom("substance is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#relationship.as_ref() {
                state.serialize_entry("relationship", some)?;
            }
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
                    SubstanceSpecificationRelationshipAmount::Quantity(ref value) => {
                        state.serialize_entry("amountQuantity", value)?;
                    }
                    SubstanceSpecificationRelationshipAmount::Range(ref value) => {
                        state.serialize_entry("amountRange", value)?;
                    }
                    SubstanceSpecificationRelationshipAmount::Ratio(ref value) => {
                        state.serialize_entry("amountRatio", value)?;
                    }
                    SubstanceSpecificationRelationshipAmount::String(ref value) => {
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
                    SubstanceSpecificationRelationshipAmount::Invalid => {
                        return Err(serde::ser::Error::custom("amount is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#amount_ratio_low_limit.as_ref() {
                state.serialize_entry("amountRatioLowLimit", some)?;
            }
            if let Some(some) = self.r#amount_type.as_ref() {
                state.serialize_entry("amountType", some)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSpecificationRelationship {
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
            #[serde(rename = "substanceReference")]
            SubstanceReference,
            #[serde(rename = "substanceCodeableConcept")]
            SubstanceCodeableConcept,
            #[serde(rename = "relationship")]
            Relationship,
            #[serde(rename = "isDefining")]
            IsDefining,
            #[serde(rename = "_isDefining")]
            IsDefiningPrimitiveElement,
            #[serde(rename = "amountQuantity")]
            AmountQuantity,
            #[serde(rename = "amountRange")]
            AmountRange,
            #[serde(rename = "amountRatio")]
            AmountRatio,
            #[serde(rename = "amountString")]
            AmountString,
            #[serde(rename = "_amountString")]
            AmountStringPrimitiveElement,
            #[serde(rename = "amountRatioLowLimit")]
            AmountRatioLowLimit,
            #[serde(rename = "amountType")]
            AmountType,
            #[serde(rename = "source")]
            Source,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSpecificationRelationship;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSpecificationRelationship")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceSpecificationRelationship, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#substance: Option<SubstanceSpecificationRelationshipSubstance> = None;
                let mut r#relationship: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#is_defining: Option<super::super::types::Boolean> = None;
                let mut r#amount: Option<SubstanceSpecificationRelationshipAmount> = None;
                let mut r#amount_ratio_low_limit: Option<Box<super::super::types::Ratio>> = None;
                let mut r#amount_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#source: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                            Field::SubstanceReference => {
                                if r#substance.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "substanceReference",
                                    ));
                                }
                                r#substance =
                                    Some(SubstanceSpecificationRelationshipSubstance::Reference(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::SubstanceCodeableConcept => {
                                if r#substance.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "substanceCodeableConcept",
                                    ));
                                }
                                r#substance = Some(
                                    SubstanceSpecificationRelationshipSubstance::CodeableConcept(
                                        map_access.next_value()?,
                                    ),
                                );
                            }
                            Field::Relationship => {
                                if r#relationship.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relationship"));
                                }
                                r#relationship = Some(map_access.next_value()?);
                            }
                            Field::IsDefining => {
                                if _ctx.from_json {
                                    let some = r#is_defining.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "isDefining",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#is_defining.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "isDefining",
                                        ));
                                    }
                                    r#is_defining = Some(map_access.next_value()?);
                                }
                            }
                            Field::IsDefiningPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#is_defining.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_isDefining",
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
                                        "isDefining",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "substanceReference",
                                            "substanceCodeableConcept",
                                            "relationship",
                                            "isDefining",
                                            "amountQuantity",
                                            "amountRange",
                                            "amountRatio",
                                            "amountString",
                                            "amountRatioLowLimit",
                                            "amountType",
                                            "source",
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
                                r#amount =
                                    Some(SubstanceSpecificationRelationshipAmount::Quantity(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::AmountRange => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountRange"));
                                }
                                r#amount = Some(SubstanceSpecificationRelationshipAmount::Range(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::AmountRatio => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountRatio"));
                                }
                                r#amount = Some(SubstanceSpecificationRelationshipAmount::Ratio(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::AmountString => {
                                if _ctx.from_json {
                                    let r#enum = r#amount.get_or_insert(
                                        SubstanceSpecificationRelationshipAmount::String(
                                            Default::default(),
                                        ),
                                    );
                                    if let SubstanceSpecificationRelationshipAmount::String(
                                        variant,
                                    ) = r#enum
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
                                    r#amount =
                                        Some(SubstanceSpecificationRelationshipAmount::String(
                                            map_access.next_value()?,
                                        ));
                                }
                            }
                            Field::AmountStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#amount.get_or_insert(
                                        SubstanceSpecificationRelationshipAmount::String(
                                            Default::default(),
                                        ),
                                    );
                                    if let SubstanceSpecificationRelationshipAmount::String(
                                        variant,
                                    ) = r#enum
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
                                            "substanceReference",
                                            "substanceCodeableConcept",
                                            "relationship",
                                            "isDefining",
                                            "amountQuantity",
                                            "amountRange",
                                            "amountRatio",
                                            "amountString",
                                            "amountRatioLowLimit",
                                            "amountType",
                                            "source",
                                        ],
                                    ));
                                }
                            }
                            Field::AmountRatioLowLimit => {
                                if r#amount_ratio_low_limit.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "amountRatioLowLimit",
                                    ));
                                }
                                r#amount_ratio_low_limit = Some(map_access.next_value()?);
                            }
                            Field::AmountType => {
                                if r#amount_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountType"));
                                }
                                r#amount_type = Some(map_access.next_value()?);
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
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
                                        "substanceReference",
                                        "substanceCodeableConcept",
                                        "relationship",
                                        "isDefining",
                                        "amountQuantity",
                                        "amountRange",
                                        "amountRatio",
                                        "amountString",
                                        "amountRatioLowLimit",
                                        "amountType",
                                        "source",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceSpecificationRelationship {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#substance,
                        r#relationship,
                        r#is_defining,
                        r#amount,
                        r#amount_ratio_low_limit,
                        r#amount_type,
                        r#source: r#source.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The detailed description of a substance, typically at a level beyond what is used for prescribing."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecification {
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
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "High level categorization, e.g. polymer or nucleic acid."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Status of substance within the catalogue e.g. approved."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If the substance applies to only human or veterinary use."]
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Textual description of the substance."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
    #[doc = "Textual comment about this record of a substance."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "Moiety, for structural modifications."]
    pub r#moiety: Vec<SubstanceSpecificationMoiety>,
    #[doc = "General specifications for this substance, including how it is related to other substances."]
    pub r#property: Vec<SubstanceSpecificationProperty>,
    #[doc = "General information detailing this substance."]
    pub r#reference_information: Option<Box<super::super::types::Reference>>,
    #[doc = "Structural information."]
    pub r#structure: Option<SubstanceSpecificationStructure>,
    #[doc = "Codes associated with the substance."]
    pub r#code: Vec<SubstanceSpecificationCode>,
    #[doc = "Names applicable to this substance."]
    pub r#name: Vec<SubstanceSpecificationName>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Vec<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    #[doc = "A link between this substance and another, with details of the relationship."]
    pub r#relationship: Vec<SubstanceSpecificationRelationship>,
    #[doc = "Data items specific to nucleic acids."]
    pub r#nucleic_acid: Option<Box<super::super::types::Reference>>,
    #[doc = "Data items specific to polymers."]
    pub r#polymer: Option<Box<super::super::types::Reference>>,
    #[doc = "Data items specific to proteins."]
    pub r#protein: Option<Box<super::super::types::Reference>>,
    #[doc = "Material or taxonomic/anatomical source for the substance."]
    pub r#source_material: Option<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for SubstanceSpecification {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for SubstanceSpecification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "SubstanceSpecification")?;
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
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if let Some(some) = self.r#domain.as_ref() {
                state.serialize_entry("domain", some)?;
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
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#comment.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("comment", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_comment", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#comment.as_ref() {
                    state.serialize_entry("comment", some)?;
                }
            }
            if !self.r#moiety.is_empty() {
                state.serialize_entry("moiety", &self.r#moiety)?;
            }
            if !self.r#property.is_empty() {
                state.serialize_entry("property", &self.r#property)?;
            }
            if let Some(some) = self.r#reference_information.as_ref() {
                state.serialize_entry("referenceInformation", some)?;
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
            if !self.r#molecular_weight.is_empty() {
                state.serialize_entry("molecularWeight", &self.r#molecular_weight)?;
            }
            if !self.r#relationship.is_empty() {
                state.serialize_entry("relationship", &self.r#relationship)?;
            }
            if let Some(some) = self.r#nucleic_acid.as_ref() {
                state.serialize_entry("nucleicAcid", some)?;
            }
            if let Some(some) = self.r#polymer.as_ref() {
                state.serialize_entry("polymer", some)?;
            }
            if let Some(some) = self.r#protein.as_ref() {
                state.serialize_entry("protein", some)?;
            }
            if let Some(some) = self.r#source_material.as_ref() {
                state.serialize_entry("sourceMaterial", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceSpecification {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "domain")]
            Domain,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "source")]
            Source,
            #[serde(rename = "comment")]
            Comment,
            #[serde(rename = "_comment")]
            CommentPrimitiveElement,
            #[serde(rename = "moiety")]
            Moiety,
            #[serde(rename = "property")]
            Property,
            #[serde(rename = "referenceInformation")]
            ReferenceInformation,
            #[serde(rename = "structure")]
            Structure,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "molecularWeight")]
            MolecularWeight,
            #[serde(rename = "relationship")]
            Relationship,
            #[serde(rename = "nucleicAcid")]
            NucleicAcid,
            #[serde(rename = "polymer")]
            Polymer,
            #[serde(rename = "protein")]
            Protein,
            #[serde(rename = "sourceMaterial")]
            SourceMaterial,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceSpecification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceSpecification")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceSpecification, V::Error>
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
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#domain: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#source: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#comment: Option<super::super::types::String> = None;
                let mut r#moiety: Option<Vec<SubstanceSpecificationMoiety>> = None;
                let mut r#property: Option<Vec<SubstanceSpecificationProperty>> = None;
                let mut r#reference_information: Option<Box<super::super::types::Reference>> = None;
                let mut r#structure: Option<SubstanceSpecificationStructure> = None;
                let mut r#code: Option<Vec<SubstanceSpecificationCode>> = None;
                let mut r#name: Option<Vec<SubstanceSpecificationName>> = None;
                let mut r#molecular_weight: Option<
                    Vec<SubstanceSpecificationStructureIsotopeMolecularWeight>,
                > = None;
                let mut r#relationship: Option<Vec<SubstanceSpecificationRelationship>> = None;
                let mut r#nucleic_acid: Option<Box<super::super::types::Reference>> = None;
                let mut r#polymer: Option<Box<super::super::types::Reference>> = None;
                let mut r#protein: Option<Box<super::super::types::Reference>> = None;
                let mut r#source_material: Option<Box<super::super::types::Reference>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "SubstanceSpecification" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"SubstanceSpecification",
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
                                            "type",
                                            "status",
                                            "domain",
                                            "description",
                                            "source",
                                            "comment",
                                            "moiety",
                                            "property",
                                            "referenceInformation",
                                            "structure",
                                            "code",
                                            "name",
                                            "molecularWeight",
                                            "relationship",
                                            "nucleicAcid",
                                            "polymer",
                                            "protein",
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
                                            "type",
                                            "status",
                                            "domain",
                                            "description",
                                            "source",
                                            "comment",
                                            "moiety",
                                            "property",
                                            "referenceInformation",
                                            "structure",
                                            "code",
                                            "name",
                                            "molecularWeight",
                                            "relationship",
                                            "nucleicAcid",
                                            "polymer",
                                            "protein",
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
                            Field::Domain => {
                                if r#domain.is_some() {
                                    return Err(serde::de::Error::duplicate_field("domain"));
                                }
                                r#domain = Some(map_access.next_value()?);
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
                                            "type",
                                            "status",
                                            "domain",
                                            "description",
                                            "source",
                                            "comment",
                                            "moiety",
                                            "property",
                                            "referenceInformation",
                                            "structure",
                                            "code",
                                            "name",
                                            "molecularWeight",
                                            "relationship",
                                            "nucleicAcid",
                                            "polymer",
                                            "protein",
                                            "sourceMaterial",
                                        ],
                                    ));
                                }
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
                            }
                            Field::Comment => {
                                if _ctx.from_json {
                                    let some = r#comment.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#comment.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    r#comment = Some(map_access.next_value()?);
                                }
                            }
                            Field::CommentPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#comment.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_comment"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "comment",
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
                                            "type",
                                            "status",
                                            "domain",
                                            "description",
                                            "source",
                                            "comment",
                                            "moiety",
                                            "property",
                                            "referenceInformation",
                                            "structure",
                                            "code",
                                            "name",
                                            "molecularWeight",
                                            "relationship",
                                            "nucleicAcid",
                                            "polymer",
                                            "protein",
                                            "sourceMaterial",
                                        ],
                                    ));
                                }
                            }
                            Field::Moiety => {
                                if r#moiety.is_some() {
                                    return Err(serde::de::Error::duplicate_field("moiety"));
                                }
                                r#moiety = Some(map_access.next_value()?);
                            }
                            Field::Property => {
                                if r#property.is_some() {
                                    return Err(serde::de::Error::duplicate_field("property"));
                                }
                                r#property = Some(map_access.next_value()?);
                            }
                            Field::ReferenceInformation => {
                                if r#reference_information.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceInformation",
                                    ));
                                }
                                r#reference_information = Some(map_access.next_value()?);
                            }
                            Field::Structure => {
                                if r#structure.is_some() {
                                    return Err(serde::de::Error::duplicate_field("structure"));
                                }
                                r#structure = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Name => {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                r#name = Some(map_access.next_value()?);
                            }
                            Field::MolecularWeight => {
                                if r#molecular_weight.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "molecularWeight",
                                    ));
                                }
                                r#molecular_weight = Some(map_access.next_value()?);
                            }
                            Field::Relationship => {
                                if r#relationship.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relationship"));
                                }
                                r#relationship = Some(map_access.next_value()?);
                            }
                            Field::NucleicAcid => {
                                if r#nucleic_acid.is_some() {
                                    return Err(serde::de::Error::duplicate_field("nucleicAcid"));
                                }
                                r#nucleic_acid = Some(map_access.next_value()?);
                            }
                            Field::Polymer => {
                                if r#polymer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("polymer"));
                                }
                                r#polymer = Some(map_access.next_value()?);
                            }
                            Field::Protein => {
                                if r#protein.is_some() {
                                    return Err(serde::de::Error::duplicate_field("protein"));
                                }
                                r#protein = Some(map_access.next_value()?);
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
                                        "type",
                                        "status",
                                        "domain",
                                        "description",
                                        "source",
                                        "comment",
                                        "moiety",
                                        "property",
                                        "referenceInformation",
                                        "structure",
                                        "code",
                                        "name",
                                        "molecularWeight",
                                        "relationship",
                                        "nucleicAcid",
                                        "polymer",
                                        "protein",
                                        "sourceMaterial",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceSpecification {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier,
                        r#type,
                        r#status,
                        r#domain,
                        r#description,
                        r#source: r#source.unwrap_or(vec![]),
                        r#comment,
                        r#moiety: r#moiety.unwrap_or(vec![]),
                        r#property: r#property.unwrap_or(vec![]),
                        r#reference_information,
                        r#structure,
                        r#code: r#code.unwrap_or(vec![]),
                        r#name: r#name.unwrap_or(vec![]),
                        r#molecular_weight: r#molecular_weight.unwrap_or(vec![]),
                        r#relationship: r#relationship.unwrap_or(vec![]),
                        r#nucleic_acid,
                        r#polymer,
                        r#protein,
                        r#source_material,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
