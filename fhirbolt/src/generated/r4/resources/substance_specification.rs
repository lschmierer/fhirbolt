// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
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
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationMoiety {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#name: Option<super::super::types::String>,
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
    pub r#molecular_formula: Option<super::super::types::String>,
    pub r#amount: Option<SubstanceSpecificationMoietyAmount>,
}
impl serde::ser::Serialize for SubstanceSpecificationMoiety {
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
        if let Some(some) = self.r#role.as_ref() {
            state.serialize_entry("role", some)?;
        }
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#stereochemistry.as_ref() {
            state.serialize_entry("stereochemistry", some)?;
        }
        if let Some(some) = self.r#optical_activity.as_ref() {
            state.serialize_entry("opticalActivity", some)?;
        }
        if let Some(some) = self.r#molecular_formula.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("molecularFormula", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_molecularFormula", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#amount.as_ref() {
            match some {
                SubstanceSpecificationMoietyAmount::Quantity(ref value) => {
                    state.serialize_entry("amountQuantity", value)?;
                }
                SubstanceSpecificationMoietyAmount::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("amountString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_amountString", &primitive_element)?;
                    }
                }
                SubstanceSpecificationMoietyAmount::Invalid => {
                    return Err(serde::ser::Error::custom("amount is invalid"))
                }
            }
        }
        state.end()
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NamePrimitiveElement => {
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
                        }
                        Field::Stereochemistry => {
                            if r#stereochemistry.is_some() {
                                return Err(serde::de::Error::duplicate_field("stereochemistry"));
                            }
                            r#stereochemistry = Some(map_access.next_value()?);
                        }
                        Field::OpticalActivity => {
                            if r#optical_activity.is_some() {
                                return Err(serde::de::Error::duplicate_field("opticalActivity"));
                            }
                            r#optical_activity = Some(map_access.next_value()?);
                        }
                        Field::MolecularFormula => {
                            let some = r#molecular_formula.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("molecularFormula"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::MolecularFormulaPrimitiveElement => {
                            let some = r#molecular_formula.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_molecularFormula"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::AmountQuantity => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountQuantity"));
                            }
                            r#amount = Some(SubstanceSpecificationMoietyAmount::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                        Field::AmountString => {
                            let r#enum = r#amount.get_or_insert(
                                SubstanceSpecificationMoietyAmount::String(Default::default()),
                            );
                            if let SubstanceSpecificationMoietyAmount::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("amount[x]"));
                            }
                        }
                        Field::AmountStringPrimitiveElement => {
                            let r#enum = r#amount.get_or_insert(
                                SubstanceSpecificationMoietyAmount::String(Default::default()),
                            );
                            if let SubstanceSpecificationMoietyAmount::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_amountString"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_amount[x]"));
                            }
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationProperty {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#parameters: Option<super::super::types::String>,
    pub r#defining_substance: Option<SubstanceSpecificationPropertyDefiningSubstance>,
    pub r#amount: Option<SubstanceSpecificationPropertyAmount>,
}
impl serde::ser::Serialize for SubstanceSpecificationProperty {
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
            state.serialize_entry("category", some)?;
        }
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if let Some(some) = self.r#parameters.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("parameters", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_parameters", &primitive_element)?;
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
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("amountString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_amountString", &primitive_element)?;
                    }
                }
                SubstanceSpecificationPropertyAmount::Invalid => {
                    return Err(serde::ser::Error::custom("amount is invalid"))
                }
            }
        }
        state.end()
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
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
                        Field::Parameters => {
                            let some = r#parameters.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ParametersPrimitiveElement => {
                            let some = r#parameters.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_parameters"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::DefiningSubstanceReference => {
                            if r#defining_substance.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "definingSubstanceReference",
                                ));
                            }
                            r#defining_substance =
                                Some(SubstanceSpecificationPropertyDefiningSubstance::Reference(
                                    map_access.next_value()?,
                                ));
                        }
                        Field::DefiningSubstanceCodeableConcept => {
                            if r#defining_substance.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "definingSubstanceCodeableConcept",
                                ));
                            }
                            r#defining_substance = Some(
                                SubstanceSpecificationPropertyDefiningSubstance::CodeableConcept(
                                    map_access.next_value()?,
                                ),
                            );
                        }
                        Field::AmountQuantity => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountQuantity"));
                            }
                            r#amount = Some(SubstanceSpecificationPropertyAmount::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                        Field::AmountString => {
                            let r#enum = r#amount.get_or_insert(
                                SubstanceSpecificationPropertyAmount::String(Default::default()),
                            );
                            if let SubstanceSpecificationPropertyAmount::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("amount[x]"));
                            }
                        }
                        Field::AmountStringPrimitiveElement => {
                            let r#enum = r#amount.get_or_insert(
                                SubstanceSpecificationPropertyAmount::String(Default::default()),
                            );
                            if let SubstanceSpecificationPropertyAmount::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_amountString"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_amount[x]"));
                            }
                        }
                    }
                }
                Ok(SubstanceSpecificationProperty {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category,
                    r#code,
                    r#parameters,
                    r#defining_substance,
                    r#amount,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationStructureIsotopeMolecularWeight {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for SubstanceSpecificationStructureIsotopeMolecularWeight {
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationStructureIsotope {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#name: Option<Box<super::super::types::CodeableConcept>>,
    pub r#substitution: Option<Box<super::super::types::CodeableConcept>>,
    pub r#half_life: Option<Box<super::super::types::Quantity>>,
    pub r#molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
}
impl serde::ser::Serialize for SubstanceSpecificationStructureIsotope {
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                                return Err(serde::de::Error::duplicate_field("molecularWeight"));
                            }
                            r#molecular_weight = Some(map_access.next_value()?);
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationStructureRepresentation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#representation: Option<super::super::types::String>,
    pub r#attachment: Option<Box<super::super::types::Attachment>>,
}
impl serde::ser::Serialize for SubstanceSpecificationStructureRepresentation {
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
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#representation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("representation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_representation", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#attachment.as_ref() {
            state.serialize_entry("attachment", some)?;
        }
        state.end()
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                            let some = r#representation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("representation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RepresentationPrimitiveElement => {
                            let some = r#representation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_representation"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Attachment => {
                            if r#attachment.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachment"));
                            }
                            r#attachment = Some(map_access.next_value()?);
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationStructure {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
    pub r#molecular_formula: Option<super::super::types::String>,
    pub r#molecular_formula_by_moiety: Option<super::super::types::String>,
    pub r#isotope: Vec<SubstanceSpecificationStructureIsotope>,
    pub r#molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#representation: Vec<SubstanceSpecificationStructureRepresentation>,
}
impl serde::ser::Serialize for SubstanceSpecificationStructure {
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
        if let Some(some) = self.r#stereochemistry.as_ref() {
            state.serialize_entry("stereochemistry", some)?;
        }
        if let Some(some) = self.r#optical_activity.as_ref() {
            state.serialize_entry("opticalActivity", some)?;
        }
        if let Some(some) = self.r#molecular_formula.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("molecularFormula", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_molecularFormula", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#molecular_formula_by_moiety.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("molecularFormulaByMoiety", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_molecularFormulaByMoiety", &primitive_element)?;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Stereochemistry => {
                            if r#stereochemistry.is_some() {
                                return Err(serde::de::Error::duplicate_field("stereochemistry"));
                            }
                            r#stereochemistry = Some(map_access.next_value()?);
                        }
                        Field::OpticalActivity => {
                            if r#optical_activity.is_some() {
                                return Err(serde::de::Error::duplicate_field("opticalActivity"));
                            }
                            r#optical_activity = Some(map_access.next_value()?);
                        }
                        Field::MolecularFormula => {
                            let some = r#molecular_formula.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("molecularFormula"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::MolecularFormulaPrimitiveElement => {
                            let some = r#molecular_formula.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_molecularFormula"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::MolecularFormulaByMoiety => {
                            let some =
                                r#molecular_formula_by_moiety.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "molecularFormulaByMoiety",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::MolecularFormulaByMoietyPrimitiveElement => {
                            let some =
                                r#molecular_formula_by_moiety.get_or_insert(Default::default());
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
                        }
                        Field::Isotope => {
                            if r#isotope.is_some() {
                                return Err(serde::de::Error::duplicate_field("isotope"));
                            }
                            r#isotope = Some(map_access.next_value()?);
                        }
                        Field::MolecularWeight => {
                            if r#molecular_weight.is_some() {
                                return Err(serde::de::Error::duplicate_field("molecularWeight"));
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
                                return Err(serde::de::Error::duplicate_field("representation"));
                            }
                            r#representation = Some(map_access.next_value()?);
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationCode {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status_date: Option<super::super::types::DateTime>,
    pub r#comment: Option<super::super::types::String>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceSpecificationCode {
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
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if let Some(some) = self.r#status.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if let Some(some) = self.r#status_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("statusDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_statusDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#comment.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("comment", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_comment", &primitive_element)?;
            }
        }
        if !self.r#source.is_empty() {
            state.serialize_entry("source", &self.r#source)?;
        }
        state.end()
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                            let some = r#status_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::StatusDatePrimitiveElement => {
                            let some = r#status_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_statusDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Comment => {
                            let some = r#comment.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::CommentPrimitiveElement => {
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
                        }
                        Field::Source => {
                            if r#source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            r#source = Some(map_access.next_value()?);
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationNameOfficial {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#authority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for SubstanceSpecificationNameOfficial {
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
        if let Some(some) = self.r#authority.as_ref() {
            state.serialize_entry("authority", some)?;
        }
        if let Some(some) = self.r#status.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("date", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_date", &primitive_element)?;
            }
        }
        state.end()
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                            let some = r#date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DatePrimitiveElement => {
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
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationName {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#preferred: Option<super::super::types::Boolean>,
    pub r#language: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#domain: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#synonym: Vec<SubstanceSpecificationName>,
    pub r#translation: Vec<SubstanceSpecificationName>,
    pub r#official: Vec<SubstanceSpecificationNameOfficial>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceSpecificationName {
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
        if let Some(some) = self.r#name.value.as_ref() {
            state.serialize_entry("name", some)?;
        }
        if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#name.id,
                extension: &self.r#name.extension,
            };
            state.serialize_entry("_name", &primitive_element)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#status.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if let Some(some) = self.r#preferred.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("preferred", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_preferred", &primitive_element)?;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Name => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NamePrimitiveElement => {
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
                            let some = r#preferred.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferred"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PreferredPrimitiveElement => {
                            let some = r#preferred.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_preferred"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
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
                    }
                }
                Ok(SubstanceSpecificationName {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: r#name.ok_or(serde::de::Error::missing_field("name"))?,
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecificationRelationship {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#substance: Option<SubstanceSpecificationRelationshipSubstance>,
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    pub r#is_defining: Option<super::super::types::Boolean>,
    pub r#amount: Option<SubstanceSpecificationRelationshipAmount>,
    pub r#amount_ratio_low_limit: Option<Box<super::super::types::Ratio>>,
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceSpecificationRelationship {
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
        if let Some(some) = self.r#is_defining.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isDefining", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isDefining", &primitive_element)?;
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
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("amountString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_amountString", &primitive_element)?;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                            let some = r#is_defining.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefining"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IsDefiningPrimitiveElement => {
                            let some = r#is_defining.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_isDefining"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::AmountQuantity => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountQuantity"));
                            }
                            r#amount = Some(SubstanceSpecificationRelationshipAmount::Quantity(
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
                            let r#enum =
                                r#amount.get_or_insert(
                                    SubstanceSpecificationRelationshipAmount::String(
                                        Default::default(),
                                    ),
                                );
                            if let SubstanceSpecificationRelationshipAmount::String(variant) =
                                r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("amount[x]"));
                            }
                        }
                        Field::AmountStringPrimitiveElement => {
                            let r#enum =
                                r#amount.get_or_insert(
                                    SubstanceSpecificationRelationshipAmount::String(
                                        Default::default(),
                                    ),
                                );
                            if let SubstanceSpecificationRelationshipAmount::String(variant) =
                                r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_amountString"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_amount[x]"));
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceSpecification {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
    pub r#comment: Option<super::super::types::String>,
    pub r#moiety: Vec<SubstanceSpecificationMoiety>,
    pub r#property: Vec<SubstanceSpecificationProperty>,
    pub r#reference_information: Option<Box<super::super::types::Reference>>,
    pub r#structure: Option<SubstanceSpecificationStructure>,
    pub r#code: Vec<SubstanceSpecificationCode>,
    pub r#name: Vec<SubstanceSpecificationName>,
    pub r#molecular_weight: Vec<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    pub r#relationship: Vec<SubstanceSpecificationRelationship>,
    pub r#nucleic_acid: Option<Box<super::super::types::Reference>>,
    pub r#polymer: Option<Box<super::super::types::Reference>>,
    pub r#protein: Option<Box<super::super::types::Reference>>,
    pub r#source_material: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceSpecification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SubstanceSpecification")?;
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
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if !self.r#source.is_empty() {
            state.serialize_entry("source", &self.r#source)?;
        }
        if let Some(some) = self.r#comment.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("comment", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_comment", &primitive_element)?;
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
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
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
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRulesPrimitiveElement => {
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
                        Field::Language => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DescriptionPrimitiveElement => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Source => {
                            if r#source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            r#source = Some(map_access.next_value()?);
                        }
                        Field::Comment => {
                            let some = r#comment.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::CommentPrimitiveElement => {
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
                                return Err(serde::de::Error::duplicate_field("molecularWeight"));
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
                                return Err(serde::de::Error::duplicate_field("sourceMaterial"));
                            }
                            r#source_material = Some(map_access.next_value()?);
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
