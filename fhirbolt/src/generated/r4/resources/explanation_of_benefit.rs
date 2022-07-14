// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitSupportingInfoTiming {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ExplanationOfBenefitSupportingInfoTiming {
    fn default() -> ExplanationOfBenefitSupportingInfoTiming {
        ExplanationOfBenefitSupportingInfoTiming::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitSupportingInfoValue {
    Boolean(Box<super::super::types::Boolean>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ExplanationOfBenefitSupportingInfoValue {
    fn default() -> ExplanationOfBenefitSupportingInfoValue {
        ExplanationOfBenefitSupportingInfoValue::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitDiagnosisDiagnosis {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ExplanationOfBenefitDiagnosisDiagnosis {
    fn default() -> ExplanationOfBenefitDiagnosisDiagnosis {
        ExplanationOfBenefitDiagnosisDiagnosis::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitProcedureProcedure {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ExplanationOfBenefitProcedureProcedure {
    fn default() -> ExplanationOfBenefitProcedureProcedure {
        ExplanationOfBenefitProcedureProcedure::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitAccidentLocation {
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ExplanationOfBenefitAccidentLocation {
    fn default() -> ExplanationOfBenefitAccidentLocation {
        ExplanationOfBenefitAccidentLocation::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ExplanationOfBenefitItemServiced {
    fn default() -> ExplanationOfBenefitItemServiced {
        ExplanationOfBenefitItemServiced::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ExplanationOfBenefitItemLocation {
    fn default() -> ExplanationOfBenefitItemLocation {
        ExplanationOfBenefitItemLocation::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitAddItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ExplanationOfBenefitAddItemServiced {
    fn default() -> ExplanationOfBenefitAddItemServiced {
        ExplanationOfBenefitAddItemServiced::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitAddItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ExplanationOfBenefitAddItemLocation {
    fn default() -> ExplanationOfBenefitAddItemLocation {
        ExplanationOfBenefitAddItemLocation::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialAllowed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    String(Box<super::super::types::String>),
    Money(Box<super::super::types::Money>),
    Invalid,
}
impl Default for ExplanationOfBenefitBenefitBalanceFinancialAllowed {
    fn default() -> ExplanationOfBenefitBenefitBalanceFinancialAllowed {
        ExplanationOfBenefitBenefitBalanceFinancialAllowed::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialUsed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    Money(Box<super::super::types::Money>),
    Invalid,
}
impl Default for ExplanationOfBenefitBenefitBalanceFinancialUsed {
    fn default() -> ExplanationOfBenefitBenefitBalanceFinancialUsed {
        ExplanationOfBenefitBenefitBalanceFinancialUsed::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitRelated {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#claim: Option<Box<super::super::types::Reference>>,
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reference: Option<Box<super::super::types::Identifier>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitRelated {
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
        if let Some(some) = self.r#claim.as_ref() {
            state.serialize_entry("claim", some)?;
        }
        if let Some(some) = self.r#relationship.as_ref() {
            state.serialize_entry("relationship", some)?;
        }
        if let Some(some) = self.r#reference.as_ref() {
            state.serialize_entry("reference", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitRelated {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitRelated;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitRelated")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitRelated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#claim: Option<Box<super::super::types::Reference>> = None;
                let mut r#relationship: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#reference: Option<Box<super::super::types::Identifier>> = None;
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
                        "claim" => {
                            if r#claim.is_some() {
                                return Err(serde::de::Error::duplicate_field("claim"));
                            }
                            r#claim = Some(map_access.next_value()?);
                        }
                        "relationship" => {
                            if r#relationship.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationship"));
                            }
                            r#relationship = Some(map_access.next_value()?);
                        }
                        "reference" => {
                            if r#reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            r#reference = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "claim",
                                    "relationship",
                                    "reference",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitRelated {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#claim,
                    r#relationship,
                    r#reference,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitPayee {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#party: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitPayee {
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
        if let Some(some) = self.r#party.as_ref() {
            state.serialize_entry("party", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitPayee {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitPayee;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitPayee")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ExplanationOfBenefitPayee, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#party: Option<Box<super::super::types::Reference>> = None;
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
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "party" => {
                            if r#party.is_some() {
                                return Err(serde::de::Error::duplicate_field("party"));
                            }
                            r#party = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "type", "party"],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitPayee {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#party,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitCareTeam {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#provider: Box<super::super::types::Reference>,
    pub r#responsible: Option<super::super::types::Boolean>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#qualification: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitCareTeam {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            state.serialize_entry("sequence", some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        state.serialize_entry("provider", &self.r#provider)?;
        if let Some(some) = self.r#responsible.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("responsible", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_responsible", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#role.as_ref() {
            state.serialize_entry("role", some)?;
        }
        if let Some(some) = self.r#qualification.as_ref() {
            state.serialize_entry("qualification", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitCareTeam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitCareTeam;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitCareTeam")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitCareTeam, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#provider: Option<Box<super::super::types::Reference>> = None;
                let mut r#responsible: Option<super::super::types::Boolean> = None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#qualification: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        "sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "provider" => {
                            if r#provider.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            r#provider = Some(map_access.next_value()?);
                        }
                        "responsible" => {
                            let some = r#responsible.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("responsible"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_responsible" => {
                            let some = r#responsible.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_responsible"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "role" => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some(map_access.next_value()?);
                        }
                        "qualification" => {
                            if r#qualification.is_some() {
                                return Err(serde::de::Error::duplicate_field("qualification"));
                            }
                            r#qualification = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "sequence",
                                    "provider",
                                    "responsible",
                                    "role",
                                    "qualification",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitCareTeam {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence: r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?,
                    r#provider: r#provider.ok_or(serde::de::Error::missing_field("provider"))?,
                    r#responsible,
                    r#role,
                    r#qualification,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitSupportingInfo {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#timing: Option<ExplanationOfBenefitSupportingInfoTiming>,
    pub r#value: Option<ExplanationOfBenefitSupportingInfoValue>,
    pub r#reason: Option<Box<super::super::types::Coding>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitSupportingInfo {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            state.serialize_entry("sequence", some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        state.serialize_entry("category", &self.r#category)?;
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if let Some(some) = self.r#timing.as_ref() {
            match some {
                ExplanationOfBenefitSupportingInfoTiming::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("timingDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_timingDate", &primitive_element)?;
                    }
                }
                ExplanationOfBenefitSupportingInfoTiming::Period(ref value) => {
                    state.serialize_entry("timingPeriod", value)?;
                }
                ExplanationOfBenefitSupportingInfoTiming::Invalid => {
                    return Err(serde::ser::Error::custom("timing is invalid"))
                }
            }
        }
        if let Some(some) = self.r#value.as_ref() {
            match some {
                ExplanationOfBenefitSupportingInfoValue::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueBoolean", &primitive_element)?;
                    }
                }
                ExplanationOfBenefitSupportingInfoValue::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueString", &primitive_element)?;
                    }
                }
                ExplanationOfBenefitSupportingInfoValue::Quantity(ref value) => {
                    state.serialize_entry("valueQuantity", value)?;
                }
                ExplanationOfBenefitSupportingInfoValue::Attachment(ref value) => {
                    state.serialize_entry("valueAttachment", value)?;
                }
                ExplanationOfBenefitSupportingInfoValue::Reference(ref value) => {
                    state.serialize_entry("valueReference", value)?;
                }
                ExplanationOfBenefitSupportingInfoValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is invalid"))
                }
            }
        }
        if let Some(some) = self.r#reason.as_ref() {
            state.serialize_entry("reason", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitSupportingInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitSupportingInfo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitSupportingInfo")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitSupportingInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#timing: Option<ExplanationOfBenefitSupportingInfoTiming> = None;
                let mut r#value: Option<ExplanationOfBenefitSupportingInfoValue> = None;
                let mut r#reason: Option<Box<super::super::types::Coding>> = None;
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
                        "sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
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
                        "timingDate" => {
                            let r#enum = r#timing.get_or_insert(
                                ExplanationOfBenefitSupportingInfoTiming::Date(Default::default()),
                            );
                            if let ExplanationOfBenefitSupportingInfoTiming::Date(variant) = r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timingDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("timing[x]"));
                            }
                        }
                        "_timingDate" => {
                            let r#enum = r#timing.get_or_insert(
                                ExplanationOfBenefitSupportingInfoTiming::Date(Default::default()),
                            );
                            if let ExplanationOfBenefitSupportingInfoTiming::Date(variant) = r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_timingDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_timing[x]"));
                            }
                        }
                        "timingPeriod" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingPeriod"));
                            }
                            r#timing = Some(ExplanationOfBenefitSupportingInfoTiming::Period(
                                map_access.next_value()?,
                            ));
                        }
                        "valueBoolean" => {
                            let r#enum =
                                r#value.get_or_insert(
                                    ExplanationOfBenefitSupportingInfoValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                            if let ExplanationOfBenefitSupportingInfoValue::Boolean(variant) =
                                r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueBoolean" => {
                            let r#enum =
                                r#value.get_or_insert(
                                    ExplanationOfBenefitSupportingInfoValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                            if let ExplanationOfBenefitSupportingInfoValue::Boolean(variant) =
                                r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueBoolean"));
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
                        }
                        "valueString" => {
                            let r#enum = r#value.get_or_insert(
                                ExplanationOfBenefitSupportingInfoValue::String(Default::default()),
                            );
                            if let ExplanationOfBenefitSupportingInfoValue::String(variant) = r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueString" => {
                            let r#enum = r#value.get_or_insert(
                                ExplanationOfBenefitSupportingInfoValue::String(Default::default()),
                            );
                            if let ExplanationOfBenefitSupportingInfoValue::String(variant) = r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueString"));
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
                        }
                        "valueQuantity" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some(ExplanationOfBenefitSupportingInfoValue::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                        "valueAttachment" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some(ExplanationOfBenefitSupportingInfoValue::Attachment(
                                map_access.next_value()?,
                            ));
                        }
                        "valueReference" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some(ExplanationOfBenefitSupportingInfoValue::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        "reason" => {
                            if r#reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            r#reason = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "sequence",
                                    "category",
                                    "code",
                                    "timing",
                                    "value",
                                    "reason",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitSupportingInfo {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence: r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?,
                    r#category: r#category.ok_or(serde::de::Error::missing_field("category"))?,
                    r#code,
                    r#timing,
                    r#value,
                    r#reason,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitDiagnosis {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#diagnosis: ExplanationOfBenefitDiagnosisDiagnosis,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#on_admission: Option<Box<super::super::types::CodeableConcept>>,
    pub r#package_code: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitDiagnosis {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            state.serialize_entry("sequence", some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        match self.r#diagnosis {
            ExplanationOfBenefitDiagnosisDiagnosis::CodeableConcept(ref value) => {
                state.serialize_entry("diagnosisCodeableConcept", value)?;
            }
            ExplanationOfBenefitDiagnosisDiagnosis::Reference(ref value) => {
                state.serialize_entry("diagnosisReference", value)?;
            }
            ExplanationOfBenefitDiagnosisDiagnosis::Invalid => {
                return Err(serde::ser::Error::custom("diagnosis is a required field"))
            }
        }
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
        }
        if let Some(some) = self.r#on_admission.as_ref() {
            state.serialize_entry("onAdmission", some)?;
        }
        if let Some(some) = self.r#package_code.as_ref() {
            state.serialize_entry("packageCode", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitDiagnosis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitDiagnosis;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitDiagnosis")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitDiagnosis, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#diagnosis: Option<ExplanationOfBenefitDiagnosisDiagnosis> = None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#on_admission: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#package_code: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        "sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "diagnosisCodeableConcept" => {
                            if r#diagnosis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "diagnosisCodeableConcept",
                                ));
                            }
                            r#diagnosis =
                                Some(ExplanationOfBenefitDiagnosisDiagnosis::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                        }
                        "diagnosisReference" => {
                            if r#diagnosis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "diagnosisReference",
                                ));
                            }
                            r#diagnosis = Some(ExplanationOfBenefitDiagnosisDiagnosis::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "onAdmission" => {
                            if r#on_admission.is_some() {
                                return Err(serde::de::Error::duplicate_field("onAdmission"));
                            }
                            r#on_admission = Some(map_access.next_value()?);
                        }
                        "packageCode" => {
                            if r#package_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageCode"));
                            }
                            r#package_code = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "sequence",
                                    "diagnosis",
                                    "type",
                                    "on_admission",
                                    "package_code",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitDiagnosis {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence: r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?,
                    r#diagnosis: r#diagnosis.ok_or(serde::de::Error::missing_field("diagnosis"))?,
                    r#type: r#type.unwrap_or(vec![]),
                    r#on_admission,
                    r#package_code,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitProcedure {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#procedure: ExplanationOfBenefitProcedureProcedure,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitProcedure {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            state.serialize_entry("sequence", some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
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
        match self.r#procedure {
            ExplanationOfBenefitProcedureProcedure::CodeableConcept(ref value) => {
                state.serialize_entry("procedureCodeableConcept", value)?;
            }
            ExplanationOfBenefitProcedureProcedure::Reference(ref value) => {
                state.serialize_entry("procedureReference", value)?;
            }
            ExplanationOfBenefitProcedureProcedure::Invalid => {
                return Err(serde::ser::Error::custom("procedure is a required field"))
            }
        }
        if !self.r#udi.is_empty() {
            state.serialize_entry("udi", &self.r#udi)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitProcedure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitProcedure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitProcedure")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitProcedure, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#procedure: Option<ExplanationOfBenefitProcedureProcedure> = None;
                let mut r#udi: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                        "sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "date" => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_date" => {
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
                        "procedureCodeableConcept" => {
                            if r#procedure.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "procedureCodeableConcept",
                                ));
                            }
                            r#procedure =
                                Some(ExplanationOfBenefitProcedureProcedure::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                        }
                        "procedureReference" => {
                            if r#procedure.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "procedureReference",
                                ));
                            }
                            r#procedure = Some(ExplanationOfBenefitProcedureProcedure::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        "udi" => {
                            if r#udi.is_some() {
                                return Err(serde::de::Error::duplicate_field("udi"));
                            }
                            r#udi = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "sequence",
                                    "type",
                                    "date",
                                    "procedure",
                                    "udi",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitProcedure {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence: r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?,
                    r#type: r#type.unwrap_or(vec![]),
                    r#date,
                    r#procedure: r#procedure.ok_or(serde::de::Error::missing_field("procedure"))?,
                    r#udi: r#udi.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitInsurance {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#focal: super::super::types::Boolean,
    pub r#coverage: Box<super::super::types::Reference>,
    pub r#pre_auth_ref: Vec<super::super::types::String>,
}
impl serde::ser::Serialize for ExplanationOfBenefitInsurance {
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
        if let Some(some) = self.r#focal.value.as_ref() {
            state.serialize_entry("focal", some)?;
        }
        if self.r#focal.id.is_some() || !self.r#focal.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#focal.id,
                extension: &self.r#focal.extension,
            };
            state.serialize_entry("_focal", &primitive_element)?;
        }
        state.serialize_entry("coverage", &self.r#coverage)?;
        if !self.r#pre_auth_ref.is_empty() {
            let values: Vec<_> = self.r#pre_auth_ref.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("preAuthRef", &values)?;
            }
            let requires_elements = self
                .r#pre_auth_ref
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#pre_auth_ref
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
                state.serialize_entry("_preAuthRef", &primitive_elements)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitInsurance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitInsurance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitInsurance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitInsurance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#focal: Option<super::super::types::Boolean> = None;
                let mut r#coverage: Option<Box<super::super::types::Reference>> = None;
                let mut r#pre_auth_ref: Option<Vec<super::super::types::String>> = None;
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
                        "focal" => {
                            let some = r#focal.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("focal"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_focal" => {
                            let some = r#focal.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_focal"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "coverage" => {
                            if r#coverage.is_some() {
                                return Err(serde::de::Error::duplicate_field("coverage"));
                            }
                            r#coverage = Some(map_access.next_value()?);
                        }
                        "preAuthRef" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#pre_auth_ref.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("preAuthRef"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_preAuthRef" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#pre_auth_ref.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_preAuthRef"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "focal",
                                    "coverage",
                                    "pre_auth_ref",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitInsurance {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#focal: r#focal.ok_or(serde::de::Error::missing_field("focal"))?,
                    r#coverage: r#coverage.ok_or(serde::de::Error::missing_field("coverage"))?,
                    r#pre_auth_ref: r#pre_auth_ref.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitAccident {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::Date>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#location: Option<ExplanationOfBenefitAccidentLocation>,
}
impl serde::ser::Serialize for ExplanationOfBenefitAccident {
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
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#location.as_ref() {
            match some {
                ExplanationOfBenefitAccidentLocation::Address(ref value) => {
                    state.serialize_entry("locationAddress", value)?;
                }
                ExplanationOfBenefitAccidentLocation::Reference(ref value) => {
                    state.serialize_entry("locationReference", value)?;
                }
                ExplanationOfBenefitAccidentLocation::Invalid => {
                    return Err(serde::ser::Error::custom("location is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitAccident {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitAccident;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitAccident")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitAccident, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#date: Option<super::super::types::Date> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#location: Option<ExplanationOfBenefitAccidentLocation> = None;
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
                        "date" => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_date" => {
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
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "locationAddress" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationAddress"));
                            }
                            r#location = Some(ExplanationOfBenefitAccidentLocation::Address(
                                map_access.next_value()?,
                            ));
                        }
                        "locationReference" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationReference"));
                            }
                            r#location = Some(ExplanationOfBenefitAccidentLocation::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "date",
                                    "type",
                                    "location",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitAccident {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#date,
                    r#type,
                    r#location,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitItemAdjudication {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Option<Box<super::super::types::Money>>,
    pub r#value: Option<super::super::types::Decimal>,
}
impl serde::ser::Serialize for ExplanationOfBenefitItemAdjudication {
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
        state.serialize_entry("category", &self.r#category)?;
        if let Some(some) = self.r#reason.as_ref() {
            state.serialize_entry("reason", some)?;
        }
        if let Some(some) = self.r#amount.as_ref() {
            state.serialize_entry("amount", some)?;
        }
        if let Some(some) = self.r#value.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("value", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_value", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitItemAdjudication {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitItemAdjudication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitItemAdjudication")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitItemAdjudication, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#amount: Option<Box<super::super::types::Money>> = None;
                let mut r#value: Option<super::super::types::Decimal> = None;
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
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "reason" => {
                            if r#reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            r#reason = Some(map_access.next_value()?);
                        }
                        "amount" => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(map_access.next_value()?);
                        }
                        "value" => {
                            let some = r#value.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_value" => {
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
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "category",
                                    "reason",
                                    "amount",
                                    "value",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitItemAdjudication {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category: r#category.ok_or(serde::de::Error::missing_field("category"))?,
                    r#reason,
                    r#amount,
                    r#value,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitItemDetailSubDetail {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}
impl serde::ser::Serialize for ExplanationOfBenefitItemDetailSubDetail {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            state.serialize_entry("sequence", some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        if let Some(some) = self.r#revenue.as_ref() {
            state.serialize_entry("revenue", some)?;
        }
        if let Some(some) = self.r#category.as_ref() {
            state.serialize_entry("category", some)?;
        }
        state.serialize_entry("productOrService", &self.r#product_or_service)?;
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
        }
        if !self.r#program_code.is_empty() {
            state.serialize_entry("programCode", &self.r#program_code)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#unit_price.as_ref() {
            state.serialize_entry("unitPrice", some)?;
        }
        if let Some(some) = self.r#factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("factor", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if !self.r#udi.is_empty() {
            state.serialize_entry("udi", &self.r#udi)?;
        }
        if !self.r#note_number.is_empty() {
            let values: Vec<_> = self.r#note_number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("noteNumber", &values)?;
            }
            let requires_elements = self
                .r#note_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#note_number
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
                state.serialize_entry("_noteNumber", &primitive_elements)?;
            }
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitItemDetailSubDetail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitItemDetailSubDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitItemDetailSubDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitItemDetailSubDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#revenue: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#program_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#udi: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note_number: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>> = None;
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
                        "sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "revenue" => {
                            if r#revenue.is_some() {
                                return Err(serde::de::Error::duplicate_field("revenue"));
                            }
                            r#revenue = Some(map_access.next_value()?);
                        }
                        "category" => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "productOrService" => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            r#product_or_service = Some(map_access.next_value()?);
                        }
                        "modifier" => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            r#modifier = Some(map_access.next_value()?);
                        }
                        "programCode" => {
                            if r#program_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("programCode"));
                            }
                            r#program_code = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "unitPrice" => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            r#unit_price = Some(map_access.next_value()?);
                        }
                        "factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_factor"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "net" => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            r#net = Some(map_access.next_value()?);
                        }
                        "udi" => {
                            if r#udi.is_some() {
                                return Err(serde::de::Error::duplicate_field("udi"));
                            }
                            r#udi = Some(map_access.next_value()?);
                        }
                        "noteNumber" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#note_number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("noteNumber"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_noteNumber" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#note_number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_noteNumber"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "adjudication" => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "sequence",
                                    "revenue",
                                    "category",
                                    "product_or_service",
                                    "modifier",
                                    "program_code",
                                    "quantity",
                                    "unit_price",
                                    "factor",
                                    "net",
                                    "udi",
                                    "note_number",
                                    "adjudication",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitItemDetailSubDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence: r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?,
                    r#revenue,
                    r#category,
                    r#product_or_service: r#product_or_service
                        .ok_or(serde::de::Error::missing_field("product_or_service"))?,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#program_code: r#program_code.unwrap_or(vec![]),
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#net,
                    r#udi: r#udi.unwrap_or(vec![]),
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitItemDetail {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    pub r#sub_detail: Vec<ExplanationOfBenefitItemDetailSubDetail>,
}
impl serde::ser::Serialize for ExplanationOfBenefitItemDetail {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            state.serialize_entry("sequence", some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        if let Some(some) = self.r#revenue.as_ref() {
            state.serialize_entry("revenue", some)?;
        }
        if let Some(some) = self.r#category.as_ref() {
            state.serialize_entry("category", some)?;
        }
        state.serialize_entry("productOrService", &self.r#product_or_service)?;
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
        }
        if !self.r#program_code.is_empty() {
            state.serialize_entry("programCode", &self.r#program_code)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#unit_price.as_ref() {
            state.serialize_entry("unitPrice", some)?;
        }
        if let Some(some) = self.r#factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("factor", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if !self.r#udi.is_empty() {
            state.serialize_entry("udi", &self.r#udi)?;
        }
        if !self.r#note_number.is_empty() {
            let values: Vec<_> = self.r#note_number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("noteNumber", &values)?;
            }
            let requires_elements = self
                .r#note_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#note_number
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
                state.serialize_entry("_noteNumber", &primitive_elements)?;
            }
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        if !self.r#sub_detail.is_empty() {
            state.serialize_entry("subDetail", &self.r#sub_detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitItemDetail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitItemDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitItemDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitItemDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#revenue: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#program_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#udi: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note_number: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>> = None;
                let mut r#sub_detail: Option<Vec<ExplanationOfBenefitItemDetailSubDetail>> = None;
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
                        "sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "revenue" => {
                            if r#revenue.is_some() {
                                return Err(serde::de::Error::duplicate_field("revenue"));
                            }
                            r#revenue = Some(map_access.next_value()?);
                        }
                        "category" => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "productOrService" => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            r#product_or_service = Some(map_access.next_value()?);
                        }
                        "modifier" => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            r#modifier = Some(map_access.next_value()?);
                        }
                        "programCode" => {
                            if r#program_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("programCode"));
                            }
                            r#program_code = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "unitPrice" => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            r#unit_price = Some(map_access.next_value()?);
                        }
                        "factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_factor"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "net" => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            r#net = Some(map_access.next_value()?);
                        }
                        "udi" => {
                            if r#udi.is_some() {
                                return Err(serde::de::Error::duplicate_field("udi"));
                            }
                            r#udi = Some(map_access.next_value()?);
                        }
                        "noteNumber" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#note_number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("noteNumber"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_noteNumber" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#note_number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_noteNumber"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "adjudication" => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                        "subDetail" => {
                            if r#sub_detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("subDetail"));
                            }
                            r#sub_detail = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "sequence",
                                    "revenue",
                                    "category",
                                    "product_or_service",
                                    "modifier",
                                    "program_code",
                                    "quantity",
                                    "unit_price",
                                    "factor",
                                    "net",
                                    "udi",
                                    "note_number",
                                    "adjudication",
                                    "sub_detail",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitItemDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence: r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?,
                    r#revenue,
                    r#category,
                    r#product_or_service: r#product_or_service
                        .ok_or(serde::de::Error::missing_field("product_or_service"))?,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#program_code: r#program_code.unwrap_or(vec![]),
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#net,
                    r#udi: r#udi.unwrap_or(vec![]),
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#sub_detail: r#sub_detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitItem {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#care_team_sequence: Vec<super::super::types::PositiveInt>,
    pub r#diagnosis_sequence: Vec<super::super::types::PositiveInt>,
    pub r#procedure_sequence: Vec<super::super::types::PositiveInt>,
    pub r#information_sequence: Vec<super::super::types::PositiveInt>,
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#serviced: Option<ExplanationOfBenefitItemServiced>,
    pub r#location: Option<ExplanationOfBenefitItemLocation>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#encounter: Vec<Box<super::super::types::Reference>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    pub r#detail: Vec<ExplanationOfBenefitItemDetail>,
}
impl serde::ser::Serialize for ExplanationOfBenefitItem {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            state.serialize_entry("sequence", some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        if !self.r#care_team_sequence.is_empty() {
            let values: Vec<_> = self.r#care_team_sequence.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("careTeamSequence", &values)?;
            }
            let requires_elements = self
                .r#care_team_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#care_team_sequence
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
                state.serialize_entry("_careTeamSequence", &primitive_elements)?;
            }
        }
        if !self.r#diagnosis_sequence.is_empty() {
            let values: Vec<_> = self.r#diagnosis_sequence.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("diagnosisSequence", &values)?;
            }
            let requires_elements = self
                .r#diagnosis_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#diagnosis_sequence
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
                state.serialize_entry("_diagnosisSequence", &primitive_elements)?;
            }
        }
        if !self.r#procedure_sequence.is_empty() {
            let values: Vec<_> = self.r#procedure_sequence.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("procedureSequence", &values)?;
            }
            let requires_elements = self
                .r#procedure_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#procedure_sequence
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
                state.serialize_entry("_procedureSequence", &primitive_elements)?;
            }
        }
        if !self.r#information_sequence.is_empty() {
            let values: Vec<_> = self
                .r#information_sequence
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("informationSequence", &values)?;
            }
            let requires_elements = self
                .r#information_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#information_sequence
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
                state.serialize_entry("_informationSequence", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#revenue.as_ref() {
            state.serialize_entry("revenue", some)?;
        }
        if let Some(some) = self.r#category.as_ref() {
            state.serialize_entry("category", some)?;
        }
        state.serialize_entry("productOrService", &self.r#product_or_service)?;
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
        }
        if !self.r#program_code.is_empty() {
            state.serialize_entry("programCode", &self.r#program_code)?;
        }
        if let Some(some) = self.r#serviced.as_ref() {
            match some {
                ExplanationOfBenefitItemServiced::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("servicedDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_servicedDate", &primitive_element)?;
                    }
                }
                ExplanationOfBenefitItemServiced::Period(ref value) => {
                    state.serialize_entry("servicedPeriod", value)?;
                }
                ExplanationOfBenefitItemServiced::Invalid => {
                    return Err(serde::ser::Error::custom("serviced is invalid"))
                }
            }
        }
        if let Some(some) = self.r#location.as_ref() {
            match some {
                ExplanationOfBenefitItemLocation::CodeableConcept(ref value) => {
                    state.serialize_entry("locationCodeableConcept", value)?;
                }
                ExplanationOfBenefitItemLocation::Address(ref value) => {
                    state.serialize_entry("locationAddress", value)?;
                }
                ExplanationOfBenefitItemLocation::Reference(ref value) => {
                    state.serialize_entry("locationReference", value)?;
                }
                ExplanationOfBenefitItemLocation::Invalid => {
                    return Err(serde::ser::Error::custom("location is invalid"))
                }
            }
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#unit_price.as_ref() {
            state.serialize_entry("unitPrice", some)?;
        }
        if let Some(some) = self.r#factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("factor", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if !self.r#udi.is_empty() {
            state.serialize_entry("udi", &self.r#udi)?;
        }
        if let Some(some) = self.r#body_site.as_ref() {
            state.serialize_entry("bodySite", some)?;
        }
        if !self.r#sub_site.is_empty() {
            state.serialize_entry("subSite", &self.r#sub_site)?;
        }
        if !self.r#encounter.is_empty() {
            state.serialize_entry("encounter", &self.r#encounter)?;
        }
        if !self.r#note_number.is_empty() {
            let values: Vec<_> = self.r#note_number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("noteNumber", &values)?;
            }
            let requires_elements = self
                .r#note_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#note_number
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
                state.serialize_entry("_noteNumber", &primitive_elements)?;
            }
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        if !self.r#detail.is_empty() {
            state.serialize_entry("detail", &self.r#detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitItem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ExplanationOfBenefitItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#care_team_sequence: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#diagnosis_sequence: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#procedure_sequence: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#information_sequence: Option<Vec<super::super::types::PositiveInt>> =
                    None;
                let mut r#revenue: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#program_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#serviced: Option<ExplanationOfBenefitItemServiced> = None;
                let mut r#location: Option<ExplanationOfBenefitItemLocation> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#udi: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#body_site: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_site: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#encounter: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note_number: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>> = None;
                let mut r#detail: Option<Vec<ExplanationOfBenefitItemDetail>> = None;
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
                        "sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_sequence" => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "careTeamSequence" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#care_team_sequence
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("careTeamSequence"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_careTeamSequence" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#care_team_sequence
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
                                return Err(serde::de::Error::duplicate_field("_careTeamSequence"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "diagnosisSequence" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#diagnosis_sequence
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("diagnosisSequence"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_diagnosisSequence" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#diagnosis_sequence
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
                                    "_diagnosisSequence",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "procedureSequence" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#procedure_sequence
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("procedureSequence"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_procedureSequence" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#procedure_sequence
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
                                    "_procedureSequence",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "informationSequence" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#information_sequence
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field(
                                    "informationSequence",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_informationSequence" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#information_sequence
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
                                    "_informationSequence",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "revenue" => {
                            if r#revenue.is_some() {
                                return Err(serde::de::Error::duplicate_field("revenue"));
                            }
                            r#revenue = Some(map_access.next_value()?);
                        }
                        "category" => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "productOrService" => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            r#product_or_service = Some(map_access.next_value()?);
                        }
                        "modifier" => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            r#modifier = Some(map_access.next_value()?);
                        }
                        "programCode" => {
                            if r#program_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("programCode"));
                            }
                            r#program_code = Some(map_access.next_value()?);
                        }
                        "servicedDate" => {
                            let r#enum = r#serviced.get_or_insert(
                                ExplanationOfBenefitItemServiced::Date(Default::default()),
                            );
                            if let ExplanationOfBenefitItemServiced::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("servicedDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("serviced[x]"));
                            }
                        }
                        "_servicedDate" => {
                            let r#enum = r#serviced.get_or_insert(
                                ExplanationOfBenefitItemServiced::Date(Default::default()),
                            );
                            if let ExplanationOfBenefitItemServiced::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_servicedDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_serviced[x]"));
                            }
                        }
                        "servicedPeriod" => {
                            if r#serviced.is_some() {
                                return Err(serde::de::Error::duplicate_field("servicedPeriod"));
                            }
                            r#serviced = Some(ExplanationOfBenefitItemServiced::Period(
                                map_access.next_value()?,
                            ));
                        }
                        "locationCodeableConcept" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "locationCodeableConcept",
                                ));
                            }
                            r#location = Some(ExplanationOfBenefitItemLocation::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "locationAddress" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationAddress"));
                            }
                            r#location = Some(ExplanationOfBenefitItemLocation::Address(
                                map_access.next_value()?,
                            ));
                        }
                        "locationReference" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationReference"));
                            }
                            r#location = Some(ExplanationOfBenefitItemLocation::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "unitPrice" => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            r#unit_price = Some(map_access.next_value()?);
                        }
                        "factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_factor"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "net" => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            r#net = Some(map_access.next_value()?);
                        }
                        "udi" => {
                            if r#udi.is_some() {
                                return Err(serde::de::Error::duplicate_field("udi"));
                            }
                            r#udi = Some(map_access.next_value()?);
                        }
                        "bodySite" => {
                            if r#body_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodySite"));
                            }
                            r#body_site = Some(map_access.next_value()?);
                        }
                        "subSite" => {
                            if r#sub_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("subSite"));
                            }
                            r#sub_site = Some(map_access.next_value()?);
                        }
                        "encounter" => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(map_access.next_value()?);
                        }
                        "noteNumber" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#note_number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("noteNumber"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_noteNumber" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#note_number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_noteNumber"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "adjudication" => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                        "detail" => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detail"));
                            }
                            r#detail = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "sequence",
                                    "care_team_sequence",
                                    "diagnosis_sequence",
                                    "procedure_sequence",
                                    "information_sequence",
                                    "revenue",
                                    "category",
                                    "product_or_service",
                                    "modifier",
                                    "program_code",
                                    "serviced",
                                    "location",
                                    "quantity",
                                    "unit_price",
                                    "factor",
                                    "net",
                                    "udi",
                                    "body_site",
                                    "sub_site",
                                    "encounter",
                                    "note_number",
                                    "adjudication",
                                    "detail",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitItem {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence: r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?,
                    r#care_team_sequence: r#care_team_sequence.unwrap_or(vec![]),
                    r#diagnosis_sequence: r#diagnosis_sequence.unwrap_or(vec![]),
                    r#procedure_sequence: r#procedure_sequence.unwrap_or(vec![]),
                    r#information_sequence: r#information_sequence.unwrap_or(vec![]),
                    r#revenue,
                    r#category,
                    r#product_or_service: r#product_or_service
                        .ok_or(serde::de::Error::missing_field("product_or_service"))?,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#program_code: r#program_code.unwrap_or(vec![]),
                    r#serviced,
                    r#location,
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#net,
                    r#udi: r#udi.unwrap_or(vec![]),
                    r#body_site,
                    r#sub_site: r#sub_site.unwrap_or(vec![]),
                    r#encounter: r#encounter.unwrap_or(vec![]),
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#detail: r#detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitAddItemDetailSubDetail {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}
impl serde::ser::Serialize for ExplanationOfBenefitAddItemDetailSubDetail {
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
        state.serialize_entry("productOrService", &self.r#product_or_service)?;
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#unit_price.as_ref() {
            state.serialize_entry("unitPrice", some)?;
        }
        if let Some(some) = self.r#factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("factor", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if !self.r#note_number.is_empty() {
            let values: Vec<_> = self.r#note_number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("noteNumber", &values)?;
            }
            let requires_elements = self
                .r#note_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#note_number
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
                state.serialize_entry("_noteNumber", &primitive_elements)?;
            }
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitAddItemDetailSubDetail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitAddItemDetailSubDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitAddItemDetailSubDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitAddItemDetailSubDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#note_number: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>> = None;
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
                        "productOrService" => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            r#product_or_service = Some(map_access.next_value()?);
                        }
                        "modifier" => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            r#modifier = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "unitPrice" => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            r#unit_price = Some(map_access.next_value()?);
                        }
                        "factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_factor"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "net" => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            r#net = Some(map_access.next_value()?);
                        }
                        "noteNumber" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#note_number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("noteNumber"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_noteNumber" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#note_number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_noteNumber"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "adjudication" => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "product_or_service",
                                    "modifier",
                                    "quantity",
                                    "unit_price",
                                    "factor",
                                    "net",
                                    "note_number",
                                    "adjudication",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitAddItemDetailSubDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#product_or_service: r#product_or_service
                        .ok_or(serde::de::Error::missing_field("product_or_service"))?,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#net,
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitAddItemDetail {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    pub r#sub_detail: Vec<ExplanationOfBenefitAddItemDetailSubDetail>,
}
impl serde::ser::Serialize for ExplanationOfBenefitAddItemDetail {
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
        state.serialize_entry("productOrService", &self.r#product_or_service)?;
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#unit_price.as_ref() {
            state.serialize_entry("unitPrice", some)?;
        }
        if let Some(some) = self.r#factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("factor", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if !self.r#note_number.is_empty() {
            let values: Vec<_> = self.r#note_number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("noteNumber", &values)?;
            }
            let requires_elements = self
                .r#note_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#note_number
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
                state.serialize_entry("_noteNumber", &primitive_elements)?;
            }
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        if !self.r#sub_detail.is_empty() {
            state.serialize_entry("subDetail", &self.r#sub_detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitAddItemDetail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitAddItemDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitAddItemDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitAddItemDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#note_number: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>> = None;
                let mut r#sub_detail: Option<Vec<ExplanationOfBenefitAddItemDetailSubDetail>> =
                    None;
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
                        "productOrService" => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            r#product_or_service = Some(map_access.next_value()?);
                        }
                        "modifier" => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            r#modifier = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "unitPrice" => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            r#unit_price = Some(map_access.next_value()?);
                        }
                        "factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_factor"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "net" => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            r#net = Some(map_access.next_value()?);
                        }
                        "noteNumber" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#note_number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("noteNumber"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_noteNumber" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#note_number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_noteNumber"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "adjudication" => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                        "subDetail" => {
                            if r#sub_detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("subDetail"));
                            }
                            r#sub_detail = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "product_or_service",
                                    "modifier",
                                    "quantity",
                                    "unit_price",
                                    "factor",
                                    "net",
                                    "note_number",
                                    "adjudication",
                                    "sub_detail",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitAddItemDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#product_or_service: r#product_or_service
                        .ok_or(serde::de::Error::missing_field("product_or_service"))?,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#net,
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#sub_detail: r#sub_detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitAddItem {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#item_sequence: Vec<super::super::types::PositiveInt>,
    pub r#detail_sequence: Vec<super::super::types::PositiveInt>,
    pub r#sub_detail_sequence: Vec<super::super::types::PositiveInt>,
    pub r#provider: Vec<Box<super::super::types::Reference>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#serviced: Option<ExplanationOfBenefitAddItemServiced>,
    pub r#location: Option<ExplanationOfBenefitAddItemLocation>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    pub r#detail: Vec<ExplanationOfBenefitAddItemDetail>,
}
impl serde::ser::Serialize for ExplanationOfBenefitAddItem {
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
        if !self.r#item_sequence.is_empty() {
            let values: Vec<_> = self.r#item_sequence.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("itemSequence", &values)?;
            }
            let requires_elements = self
                .r#item_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#item_sequence
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
                state.serialize_entry("_itemSequence", &primitive_elements)?;
            }
        }
        if !self.r#detail_sequence.is_empty() {
            let values: Vec<_> = self.r#detail_sequence.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("detailSequence", &values)?;
            }
            let requires_elements = self
                .r#detail_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#detail_sequence
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
                state.serialize_entry("_detailSequence", &primitive_elements)?;
            }
        }
        if !self.r#sub_detail_sequence.is_empty() {
            let values: Vec<_> = self
                .r#sub_detail_sequence
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("subDetailSequence", &values)?;
            }
            let requires_elements = self
                .r#sub_detail_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#sub_detail_sequence
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
                state.serialize_entry("_subDetailSequence", &primitive_elements)?;
            }
        }
        if !self.r#provider.is_empty() {
            state.serialize_entry("provider", &self.r#provider)?;
        }
        state.serialize_entry("productOrService", &self.r#product_or_service)?;
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
        }
        if !self.r#program_code.is_empty() {
            state.serialize_entry("programCode", &self.r#program_code)?;
        }
        if let Some(some) = self.r#serviced.as_ref() {
            match some {
                ExplanationOfBenefitAddItemServiced::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("servicedDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_servicedDate", &primitive_element)?;
                    }
                }
                ExplanationOfBenefitAddItemServiced::Period(ref value) => {
                    state.serialize_entry("servicedPeriod", value)?;
                }
                ExplanationOfBenefitAddItemServiced::Invalid => {
                    return Err(serde::ser::Error::custom("serviced is invalid"))
                }
            }
        }
        if let Some(some) = self.r#location.as_ref() {
            match some {
                ExplanationOfBenefitAddItemLocation::CodeableConcept(ref value) => {
                    state.serialize_entry("locationCodeableConcept", value)?;
                }
                ExplanationOfBenefitAddItemLocation::Address(ref value) => {
                    state.serialize_entry("locationAddress", value)?;
                }
                ExplanationOfBenefitAddItemLocation::Reference(ref value) => {
                    state.serialize_entry("locationReference", value)?;
                }
                ExplanationOfBenefitAddItemLocation::Invalid => {
                    return Err(serde::ser::Error::custom("location is invalid"))
                }
            }
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#unit_price.as_ref() {
            state.serialize_entry("unitPrice", some)?;
        }
        if let Some(some) = self.r#factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("factor", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if let Some(some) = self.r#body_site.as_ref() {
            state.serialize_entry("bodySite", some)?;
        }
        if !self.r#sub_site.is_empty() {
            state.serialize_entry("subSite", &self.r#sub_site)?;
        }
        if !self.r#note_number.is_empty() {
            let values: Vec<_> = self.r#note_number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("noteNumber", &values)?;
            }
            let requires_elements = self
                .r#note_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#note_number
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
                state.serialize_entry("_noteNumber", &primitive_elements)?;
            }
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        if !self.r#detail.is_empty() {
            state.serialize_entry("detail", &self.r#detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitAddItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitAddItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitAddItem")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitAddItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#item_sequence: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#detail_sequence: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#sub_detail_sequence: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#provider: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#program_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#serviced: Option<ExplanationOfBenefitAddItemServiced> = None;
                let mut r#location: Option<ExplanationOfBenefitAddItemLocation> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#body_site: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_site: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#note_number: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>> = None;
                let mut r#detail: Option<Vec<ExplanationOfBenefitAddItemDetail>> = None;
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
                        "itemSequence" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#item_sequence.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("itemSequence"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_itemSequence" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#item_sequence.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_itemSequence"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "detailSequence" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#detail_sequence.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("detailSequence"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_detailSequence" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#detail_sequence.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_detailSequence"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "subDetailSequence" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#sub_detail_sequence
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("subDetailSequence"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_subDetailSequence" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#sub_detail_sequence
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
                                    "_subDetailSequence",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "provider" => {
                            if r#provider.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            r#provider = Some(map_access.next_value()?);
                        }
                        "productOrService" => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            r#product_or_service = Some(map_access.next_value()?);
                        }
                        "modifier" => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            r#modifier = Some(map_access.next_value()?);
                        }
                        "programCode" => {
                            if r#program_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("programCode"));
                            }
                            r#program_code = Some(map_access.next_value()?);
                        }
                        "servicedDate" => {
                            let r#enum = r#serviced.get_or_insert(
                                ExplanationOfBenefitAddItemServiced::Date(Default::default()),
                            );
                            if let ExplanationOfBenefitAddItemServiced::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("servicedDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("serviced[x]"));
                            }
                        }
                        "_servicedDate" => {
                            let r#enum = r#serviced.get_or_insert(
                                ExplanationOfBenefitAddItemServiced::Date(Default::default()),
                            );
                            if let ExplanationOfBenefitAddItemServiced::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_servicedDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_serviced[x]"));
                            }
                        }
                        "servicedPeriod" => {
                            if r#serviced.is_some() {
                                return Err(serde::de::Error::duplicate_field("servicedPeriod"));
                            }
                            r#serviced = Some(ExplanationOfBenefitAddItemServiced::Period(
                                map_access.next_value()?,
                            ));
                        }
                        "locationCodeableConcept" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "locationCodeableConcept",
                                ));
                            }
                            r#location =
                                Some(ExplanationOfBenefitAddItemLocation::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                        }
                        "locationAddress" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationAddress"));
                            }
                            r#location = Some(ExplanationOfBenefitAddItemLocation::Address(
                                map_access.next_value()?,
                            ));
                        }
                        "locationReference" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationReference"));
                            }
                            r#location = Some(ExplanationOfBenefitAddItemLocation::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "unitPrice" => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            r#unit_price = Some(map_access.next_value()?);
                        }
                        "factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_factor"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "net" => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            r#net = Some(map_access.next_value()?);
                        }
                        "bodySite" => {
                            if r#body_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodySite"));
                            }
                            r#body_site = Some(map_access.next_value()?);
                        }
                        "subSite" => {
                            if r#sub_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("subSite"));
                            }
                            r#sub_site = Some(map_access.next_value()?);
                        }
                        "noteNumber" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#note_number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("noteNumber"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_noteNumber" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#note_number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_noteNumber"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "adjudication" => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                        "detail" => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detail"));
                            }
                            r#detail = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "item_sequence",
                                    "detail_sequence",
                                    "sub_detail_sequence",
                                    "provider",
                                    "product_or_service",
                                    "modifier",
                                    "program_code",
                                    "serviced",
                                    "location",
                                    "quantity",
                                    "unit_price",
                                    "factor",
                                    "net",
                                    "body_site",
                                    "sub_site",
                                    "note_number",
                                    "adjudication",
                                    "detail",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitAddItem {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#item_sequence: r#item_sequence.unwrap_or(vec![]),
                    r#detail_sequence: r#detail_sequence.unwrap_or(vec![]),
                    r#sub_detail_sequence: r#sub_detail_sequence.unwrap_or(vec![]),
                    r#provider: r#provider.unwrap_or(vec![]),
                    r#product_or_service: r#product_or_service
                        .ok_or(serde::de::Error::missing_field("product_or_service"))?,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#program_code: r#program_code.unwrap_or(vec![]),
                    r#serviced,
                    r#location,
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#net,
                    r#body_site,
                    r#sub_site: r#sub_site.unwrap_or(vec![]),
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#detail: r#detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitTotal {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#amount: Box<super::super::types::Money>,
}
impl serde::ser::Serialize for ExplanationOfBenefitTotal {
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
        state.serialize_entry("category", &self.r#category)?;
        state.serialize_entry("amount", &self.r#amount)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitTotal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitTotal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitTotal")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ExplanationOfBenefitTotal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#amount: Option<Box<super::super::types::Money>> = None;
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
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "amount" => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "category",
                                    "amount",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitTotal {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category: r#category.ok_or(serde::de::Error::missing_field("category"))?,
                    r#amount: r#amount.ok_or(serde::de::Error::missing_field("amount"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitPayment {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#adjustment: Option<Box<super::super::types::Money>>,
    pub r#adjustment_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::Date>,
    pub r#amount: Option<Box<super::super::types::Money>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitPayment {
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
        if let Some(some) = self.r#adjustment.as_ref() {
            state.serialize_entry("adjustment", some)?;
        }
        if let Some(some) = self.r#adjustment_reason.as_ref() {
            state.serialize_entry("adjustmentReason", some)?;
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
        if let Some(some) = self.r#amount.as_ref() {
            state.serialize_entry("amount", some)?;
        }
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitPayment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitPayment;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitPayment")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitPayment, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#adjustment: Option<Box<super::super::types::Money>> = None;
                let mut r#adjustment_reason: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#date: Option<super::super::types::Date> = None;
                let mut r#amount: Option<Box<super::super::types::Money>> = None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
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
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "adjustment" => {
                            if r#adjustment.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjustment"));
                            }
                            r#adjustment = Some(map_access.next_value()?);
                        }
                        "adjustmentReason" => {
                            if r#adjustment_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjustmentReason"));
                            }
                            r#adjustment_reason = Some(map_access.next_value()?);
                        }
                        "date" => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_date" => {
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
                        "amount" => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(map_access.next_value()?);
                        }
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "type",
                                    "adjustment",
                                    "adjustment_reason",
                                    "date",
                                    "amount",
                                    "identifier",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitPayment {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#adjustment,
                    r#adjustment_reason,
                    r#date,
                    r#amount,
                    r#identifier,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitProcessNote {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#number: Option<super::super::types::PositiveInt>,
    pub r#type: Option<super::super::types::Code>,
    pub r#text: Option<super::super::types::String>,
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitProcessNote {
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
        if let Some(some) = self.r#number.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("number", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_number", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_type", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_text", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#language.as_ref() {
            state.serialize_entry("language", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitProcessNote {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitProcessNote;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitProcessNote")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitProcessNote, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#number: Option<super::super::types::PositiveInt> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#language: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        "number" => {
                            let some = r#number.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_number" => {
                            let some = r#number.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_number"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "type" => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_type" => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_text"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "language" => {
                            if r#language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            r#language = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "number",
                                    "type",
                                    "text",
                                    "language",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitProcessNote {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#number,
                    r#type,
                    r#text,
                    r#language,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitBenefitBalanceFinancial {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#allowed: Option<ExplanationOfBenefitBenefitBalanceFinancialAllowed>,
    pub r#used: Option<ExplanationOfBenefitBenefitBalanceFinancialUsed>,
}
impl serde::ser::Serialize for ExplanationOfBenefitBenefitBalanceFinancial {
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
        state.serialize_entry("type", &self.r#type)?;
        if let Some(some) = self.r#allowed.as_ref() {
            match some {
                ExplanationOfBenefitBenefitBalanceFinancialAllowed::UnsignedInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("allowedUnsignedInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_allowedUnsignedInt", &primitive_element)?;
                    }
                }
                ExplanationOfBenefitBenefitBalanceFinancialAllowed::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("allowedString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_allowedString", &primitive_element)?;
                    }
                }
                ExplanationOfBenefitBenefitBalanceFinancialAllowed::Money(ref value) => {
                    state.serialize_entry("allowedMoney", value)?;
                }
                ExplanationOfBenefitBenefitBalanceFinancialAllowed::Invalid => {
                    return Err(serde::ser::Error::custom("allowed is invalid"))
                }
            }
        }
        if let Some(some) = self.r#used.as_ref() {
            match some {
                ExplanationOfBenefitBenefitBalanceFinancialUsed::UnsignedInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("usedUnsignedInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_usedUnsignedInt", &primitive_element)?;
                    }
                }
                ExplanationOfBenefitBenefitBalanceFinancialUsed::Money(ref value) => {
                    state.serialize_entry("usedMoney", value)?;
                }
                ExplanationOfBenefitBenefitBalanceFinancialUsed::Invalid => {
                    return Err(serde::ser::Error::custom("used is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitBenefitBalanceFinancial {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitBenefitBalanceFinancial;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitBenefitBalanceFinancial")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitBenefitBalanceFinancial, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#allowed: Option<ExplanationOfBenefitBenefitBalanceFinancialAllowed> =
                    None;
                let mut r#used: Option<ExplanationOfBenefitBenefitBalanceFinancialUsed> = None;
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
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "allowedUnsignedInt" => {
                            let r#enum = r#allowed.get_or_insert(
                                ExplanationOfBenefitBenefitBalanceFinancialAllowed::UnsignedInt(
                                    Default::default(),
                                ),
                            );
                            if let ExplanationOfBenefitBenefitBalanceFinancialAllowed::UnsignedInt(
                                variant,
                            ) = r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "allowedUnsignedInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("allowed[x]"));
                            }
                        }
                        "_allowedUnsignedInt" => {
                            let r#enum = r#allowed.get_or_insert(
                                ExplanationOfBenefitBenefitBalanceFinancialAllowed::UnsignedInt(
                                    Default::default(),
                                ),
                            );
                            if let ExplanationOfBenefitBenefitBalanceFinancialAllowed::UnsignedInt(
                                variant,
                            ) = r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_allowedUnsignedInt",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_allowed[x]"));
                            }
                        }
                        "allowedString" => {
                            let r#enum = r#allowed.get_or_insert(
                                ExplanationOfBenefitBenefitBalanceFinancialAllowed::String(
                                    Default::default(),
                                ),
                            );
                            if let ExplanationOfBenefitBenefitBalanceFinancialAllowed::String(
                                variant,
                            ) = r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("allowedString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("allowed[x]"));
                            }
                        }
                        "_allowedString" => {
                            let r#enum = r#allowed.get_or_insert(
                                ExplanationOfBenefitBenefitBalanceFinancialAllowed::String(
                                    Default::default(),
                                ),
                            );
                            if let ExplanationOfBenefitBenefitBalanceFinancialAllowed::String(
                                variant,
                            ) = r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_allowedString",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_allowed[x]"));
                            }
                        }
                        "allowedMoney" => {
                            if r#allowed.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedMoney"));
                            }
                            r#allowed =
                                Some(ExplanationOfBenefitBenefitBalanceFinancialAllowed::Money(
                                    map_access.next_value()?,
                                ));
                        }
                        "usedUnsignedInt" => {
                            let r#enum = r#used.get_or_insert(
                                ExplanationOfBenefitBenefitBalanceFinancialUsed::UnsignedInt(
                                    Default::default(),
                                ),
                            );
                            if let ExplanationOfBenefitBenefitBalanceFinancialUsed::UnsignedInt(
                                variant,
                            ) = r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "usedUnsignedInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("used[x]"));
                            }
                        }
                        "_usedUnsignedInt" => {
                            let r#enum = r#used.get_or_insert(
                                ExplanationOfBenefitBenefitBalanceFinancialUsed::UnsignedInt(
                                    Default::default(),
                                ),
                            );
                            if let ExplanationOfBenefitBenefitBalanceFinancialUsed::UnsignedInt(
                                variant,
                            ) = r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_usedUnsignedInt",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_used[x]"));
                            }
                        }
                        "usedMoney" => {
                            if r#used.is_some() {
                                return Err(serde::de::Error::duplicate_field("usedMoney"));
                            }
                            r#used = Some(ExplanationOfBenefitBenefitBalanceFinancialUsed::Money(
                                map_access.next_value()?,
                            ));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "type",
                                    "allowed",
                                    "used",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitBenefitBalanceFinancial {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#allowed,
                    r#used,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitBenefitBalance {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#excluded: Option<super::super::types::Boolean>,
    pub r#name: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#network: Option<Box<super::super::types::CodeableConcept>>,
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    pub r#term: Option<Box<super::super::types::CodeableConcept>>,
    pub r#financial: Vec<ExplanationOfBenefitBenefitBalanceFinancial>,
}
impl serde::ser::Serialize for ExplanationOfBenefitBenefitBalance {
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
        state.serialize_entry("category", &self.r#category)?;
        if let Some(some) = self.r#excluded.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("excluded", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_excluded", &primitive_element)?;
            }
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
        if let Some(some) = self.r#network.as_ref() {
            state.serialize_entry("network", some)?;
        }
        if let Some(some) = self.r#unit.as_ref() {
            state.serialize_entry("unit", some)?;
        }
        if let Some(some) = self.r#term.as_ref() {
            state.serialize_entry("term", some)?;
        }
        if !self.r#financial.is_empty() {
            state.serialize_entry("financial", &self.r#financial)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitBenefitBalance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefitBenefitBalance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitBenefitBalance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitBenefitBalance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#excluded: Option<super::super::types::Boolean> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#network: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#unit: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#term: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#financial: Option<Vec<ExplanationOfBenefitBenefitBalanceFinancial>> =
                    None;
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
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "excluded" => {
                            let some = r#excluded.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("excluded"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_excluded" => {
                            let some = r#excluded.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_excluded"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_name" => {
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
                        "description" => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_description" => {
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
                        "network" => {
                            if r#network.is_some() {
                                return Err(serde::de::Error::duplicate_field("network"));
                            }
                            r#network = Some(map_access.next_value()?);
                        }
                        "unit" => {
                            if r#unit.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            r#unit = Some(map_access.next_value()?);
                        }
                        "term" => {
                            if r#term.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            r#term = Some(map_access.next_value()?);
                        }
                        "financial" => {
                            if r#financial.is_some() {
                                return Err(serde::de::Error::duplicate_field("financial"));
                            }
                            r#financial = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "category",
                                    "excluded",
                                    "name",
                                    "description",
                                    "network",
                                    "unit",
                                    "term",
                                    "financial",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefitBenefitBalance {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category: r#category.ok_or(serde::de::Error::missing_field("category"))?,
                    r#excluded,
                    r#name,
                    r#description,
                    r#network,
                    r#unit,
                    r#term,
                    r#financial: r#financial.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefit {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#status: super::super::types::Code,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#use: super::super::types::Code,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#billable_period: Option<Box<super::super::types::Period>>,
    pub r#created: super::super::types::DateTime,
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    pub r#insurer: Box<super::super::types::Reference>,
    pub r#provider: Box<super::super::types::Reference>,
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#funds_reserve_requested: Option<Box<super::super::types::CodeableConcept>>,
    pub r#funds_reserve: Option<Box<super::super::types::CodeableConcept>>,
    pub r#related: Vec<ExplanationOfBenefitRelated>,
    pub r#prescription: Option<Box<super::super::types::Reference>>,
    pub r#original_prescription: Option<Box<super::super::types::Reference>>,
    pub r#payee: Option<ExplanationOfBenefitPayee>,
    pub r#referral: Option<Box<super::super::types::Reference>>,
    pub r#facility: Option<Box<super::super::types::Reference>>,
    pub r#claim: Option<Box<super::super::types::Reference>>,
    pub r#claim_response: Option<Box<super::super::types::Reference>>,
    pub r#outcome: super::super::types::Code,
    pub r#disposition: Option<super::super::types::String>,
    pub r#pre_auth_ref: Vec<super::super::types::String>,
    pub r#pre_auth_ref_period: Vec<Box<super::super::types::Period>>,
    pub r#care_team: Vec<ExplanationOfBenefitCareTeam>,
    pub r#supporting_info: Vec<ExplanationOfBenefitSupportingInfo>,
    pub r#diagnosis: Vec<ExplanationOfBenefitDiagnosis>,
    pub r#procedure: Vec<ExplanationOfBenefitProcedure>,
    pub r#precedence: Option<super::super::types::PositiveInt>,
    pub r#insurance: Vec<ExplanationOfBenefitInsurance>,
    pub r#accident: Option<ExplanationOfBenefitAccident>,
    pub r#item: Vec<ExplanationOfBenefitItem>,
    pub r#add_item: Vec<ExplanationOfBenefitAddItem>,
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    pub r#total: Vec<ExplanationOfBenefitTotal>,
    pub r#payment: Option<ExplanationOfBenefitPayment>,
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#form: Option<Box<super::super::types::Attachment>>,
    pub r#process_note: Vec<ExplanationOfBenefitProcessNote>,
    pub r#benefit_period: Option<Box<super::super::types::Period>>,
    pub r#benefit_balance: Vec<ExplanationOfBenefitBenefitBalance>,
}
impl serde::ser::Serialize for ExplanationOfBenefit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ExplanationOfBenefit")?;
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
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if let Some(some) = self.r#status.value.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        state.serialize_entry("type", &self.r#type)?;
        if let Some(some) = self.r#sub_type.as_ref() {
            state.serialize_entry("subType", some)?;
        }
        if let Some(some) = self.r#use.value.as_ref() {
            state.serialize_entry("use", some)?;
        }
        if self.r#use.id.is_some() || !self.r#use.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#use.id,
                extension: &self.r#use.extension,
            };
            state.serialize_entry("_use", &primitive_element)?;
        }
        state.serialize_entry("patient", &self.r#patient)?;
        if let Some(some) = self.r#billable_period.as_ref() {
            state.serialize_entry("billablePeriod", some)?;
        }
        if let Some(some) = self.r#created.value.as_ref() {
            state.serialize_entry("created", some)?;
        }
        if self.r#created.id.is_some() || !self.r#created.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#created.id,
                extension: &self.r#created.extension,
            };
            state.serialize_entry("_created", &primitive_element)?;
        }
        if let Some(some) = self.r#enterer.as_ref() {
            state.serialize_entry("enterer", some)?;
        }
        state.serialize_entry("insurer", &self.r#insurer)?;
        state.serialize_entry("provider", &self.r#provider)?;
        if let Some(some) = self.r#priority.as_ref() {
            state.serialize_entry("priority", some)?;
        }
        if let Some(some) = self.r#funds_reserve_requested.as_ref() {
            state.serialize_entry("fundsReserveRequested", some)?;
        }
        if let Some(some) = self.r#funds_reserve.as_ref() {
            state.serialize_entry("fundsReserve", some)?;
        }
        if !self.r#related.is_empty() {
            state.serialize_entry("related", &self.r#related)?;
        }
        if let Some(some) = self.r#prescription.as_ref() {
            state.serialize_entry("prescription", some)?;
        }
        if let Some(some) = self.r#original_prescription.as_ref() {
            state.serialize_entry("originalPrescription", some)?;
        }
        if let Some(some) = self.r#payee.as_ref() {
            state.serialize_entry("payee", some)?;
        }
        if let Some(some) = self.r#referral.as_ref() {
            state.serialize_entry("referral", some)?;
        }
        if let Some(some) = self.r#facility.as_ref() {
            state.serialize_entry("facility", some)?;
        }
        if let Some(some) = self.r#claim.as_ref() {
            state.serialize_entry("claim", some)?;
        }
        if let Some(some) = self.r#claim_response.as_ref() {
            state.serialize_entry("claimResponse", some)?;
        }
        if let Some(some) = self.r#outcome.value.as_ref() {
            state.serialize_entry("outcome", some)?;
        }
        if self.r#outcome.id.is_some() || !self.r#outcome.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#outcome.id,
                extension: &self.r#outcome.extension,
            };
            state.serialize_entry("_outcome", &primitive_element)?;
        }
        if let Some(some) = self.r#disposition.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("disposition", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_disposition", &primitive_element)?;
            }
        }
        if !self.r#pre_auth_ref.is_empty() {
            let values: Vec<_> = self.r#pre_auth_ref.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("preAuthRef", &values)?;
            }
            let requires_elements = self
                .r#pre_auth_ref
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#pre_auth_ref
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
                state.serialize_entry("_preAuthRef", &primitive_elements)?;
            }
        }
        if !self.r#pre_auth_ref_period.is_empty() {
            state.serialize_entry("preAuthRefPeriod", &self.r#pre_auth_ref_period)?;
        }
        if !self.r#care_team.is_empty() {
            state.serialize_entry("careTeam", &self.r#care_team)?;
        }
        if !self.r#supporting_info.is_empty() {
            state.serialize_entry("supportingInfo", &self.r#supporting_info)?;
        }
        if !self.r#diagnosis.is_empty() {
            state.serialize_entry("diagnosis", &self.r#diagnosis)?;
        }
        if !self.r#procedure.is_empty() {
            state.serialize_entry("procedure", &self.r#procedure)?;
        }
        if let Some(some) = self.r#precedence.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("precedence", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_precedence", &primitive_element)?;
            }
        }
        if !self.r#insurance.is_empty() {
            state.serialize_entry("insurance", &self.r#insurance)?;
        }
        if let Some(some) = self.r#accident.as_ref() {
            state.serialize_entry("accident", some)?;
        }
        if !self.r#item.is_empty() {
            state.serialize_entry("item", &self.r#item)?;
        }
        if !self.r#add_item.is_empty() {
            state.serialize_entry("addItem", &self.r#add_item)?;
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        if !self.r#total.is_empty() {
            state.serialize_entry("total", &self.r#total)?;
        }
        if let Some(some) = self.r#payment.as_ref() {
            state.serialize_entry("payment", some)?;
        }
        if let Some(some) = self.r#form_code.as_ref() {
            state.serialize_entry("formCode", some)?;
        }
        if let Some(some) = self.r#form.as_ref() {
            state.serialize_entry("form", some)?;
        }
        if !self.r#process_note.is_empty() {
            state.serialize_entry("processNote", &self.r#process_note)?;
        }
        if let Some(some) = self.r#benefit_period.as_ref() {
            state.serialize_entry("benefitPeriod", some)?;
        }
        if !self.r#benefit_balance.is_empty() {
            state.serialize_entry("benefitBalance", &self.r#benefit_balance)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExplanationOfBenefit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefit")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ExplanationOfBenefit, V::Error>
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
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#use: Option<super::super::types::Code> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#billable_period: Option<Box<super::super::types::Period>> = None;
                let mut r#created: Option<super::super::types::DateTime> = None;
                let mut r#enterer: Option<Box<super::super::types::Reference>> = None;
                let mut r#insurer: Option<Box<super::super::types::Reference>> = None;
                let mut r#provider: Option<Box<super::super::types::Reference>> = None;
                let mut r#priority: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#funds_reserve_requested: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#funds_reserve: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#related: Option<Vec<ExplanationOfBenefitRelated>> = None;
                let mut r#prescription: Option<Box<super::super::types::Reference>> = None;
                let mut r#original_prescription: Option<Box<super::super::types::Reference>> = None;
                let mut r#payee: Option<ExplanationOfBenefitPayee> = None;
                let mut r#referral: Option<Box<super::super::types::Reference>> = None;
                let mut r#facility: Option<Box<super::super::types::Reference>> = None;
                let mut r#claim: Option<Box<super::super::types::Reference>> = None;
                let mut r#claim_response: Option<Box<super::super::types::Reference>> = None;
                let mut r#outcome: Option<super::super::types::Code> = None;
                let mut r#disposition: Option<super::super::types::String> = None;
                let mut r#pre_auth_ref: Option<Vec<super::super::types::String>> = None;
                let mut r#pre_auth_ref_period: Option<Vec<Box<super::super::types::Period>>> = None;
                let mut r#care_team: Option<Vec<ExplanationOfBenefitCareTeam>> = None;
                let mut r#supporting_info: Option<Vec<ExplanationOfBenefitSupportingInfo>> = None;
                let mut r#diagnosis: Option<Vec<ExplanationOfBenefitDiagnosis>> = None;
                let mut r#procedure: Option<Vec<ExplanationOfBenefitProcedure>> = None;
                let mut r#precedence: Option<super::super::types::PositiveInt> = None;
                let mut r#insurance: Option<Vec<ExplanationOfBenefitInsurance>> = None;
                let mut r#accident: Option<ExplanationOfBenefitAccident> = None;
                let mut r#item: Option<Vec<ExplanationOfBenefitItem>> = None;
                let mut r#add_item: Option<Vec<ExplanationOfBenefitAddItem>> = None;
                let mut r#adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>> = None;
                let mut r#total: Option<Vec<ExplanationOfBenefitTotal>> = None;
                let mut r#payment: Option<ExplanationOfBenefitPayment> = None;
                let mut r#form_code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#form: Option<Box<super::super::types::Attachment>> = None;
                let mut r#process_note: Option<Vec<ExplanationOfBenefitProcessNote>> = None;
                let mut r#benefit_period: Option<Box<super::super::types::Period>> = None;
                let mut r#benefit_balance: Option<Vec<ExplanationOfBenefitBenefitBalance>> = None;
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
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
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
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "subType" => {
                            if r#sub_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("subType"));
                            }
                            r#sub_type = Some(map_access.next_value()?);
                        }
                        "use" => {
                            let some = r#use.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("use"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_use" => {
                            let some = r#use.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_use"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "patient" => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            r#patient = Some(map_access.next_value()?);
                        }
                        "billablePeriod" => {
                            if r#billable_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("billablePeriod"));
                            }
                            r#billable_period = Some(map_access.next_value()?);
                        }
                        "created" => {
                            let some = r#created.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_created" => {
                            let some = r#created.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_created"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "enterer" => {
                            if r#enterer.is_some() {
                                return Err(serde::de::Error::duplicate_field("enterer"));
                            }
                            r#enterer = Some(map_access.next_value()?);
                        }
                        "insurer" => {
                            if r#insurer.is_some() {
                                return Err(serde::de::Error::duplicate_field("insurer"));
                            }
                            r#insurer = Some(map_access.next_value()?);
                        }
                        "provider" => {
                            if r#provider.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            r#provider = Some(map_access.next_value()?);
                        }
                        "priority" => {
                            if r#priority.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            r#priority = Some(map_access.next_value()?);
                        }
                        "fundsReserveRequested" => {
                            if r#funds_reserve_requested.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fundsReserveRequested",
                                ));
                            }
                            r#funds_reserve_requested = Some(map_access.next_value()?);
                        }
                        "fundsReserve" => {
                            if r#funds_reserve.is_some() {
                                return Err(serde::de::Error::duplicate_field("fundsReserve"));
                            }
                            r#funds_reserve = Some(map_access.next_value()?);
                        }
                        "related" => {
                            if r#related.is_some() {
                                return Err(serde::de::Error::duplicate_field("related"));
                            }
                            r#related = Some(map_access.next_value()?);
                        }
                        "prescription" => {
                            if r#prescription.is_some() {
                                return Err(serde::de::Error::duplicate_field("prescription"));
                            }
                            r#prescription = Some(map_access.next_value()?);
                        }
                        "originalPrescription" => {
                            if r#original_prescription.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "originalPrescription",
                                ));
                            }
                            r#original_prescription = Some(map_access.next_value()?);
                        }
                        "payee" => {
                            if r#payee.is_some() {
                                return Err(serde::de::Error::duplicate_field("payee"));
                            }
                            r#payee = Some(map_access.next_value()?);
                        }
                        "referral" => {
                            if r#referral.is_some() {
                                return Err(serde::de::Error::duplicate_field("referral"));
                            }
                            r#referral = Some(map_access.next_value()?);
                        }
                        "facility" => {
                            if r#facility.is_some() {
                                return Err(serde::de::Error::duplicate_field("facility"));
                            }
                            r#facility = Some(map_access.next_value()?);
                        }
                        "claim" => {
                            if r#claim.is_some() {
                                return Err(serde::de::Error::duplicate_field("claim"));
                            }
                            r#claim = Some(map_access.next_value()?);
                        }
                        "claimResponse" => {
                            if r#claim_response.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimResponse"));
                            }
                            r#claim_response = Some(map_access.next_value()?);
                        }
                        "outcome" => {
                            let some = r#outcome.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcome"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_outcome" => {
                            let some = r#outcome.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_outcome"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "disposition" => {
                            let some = r#disposition.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("disposition"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_disposition" => {
                            let some = r#disposition.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_disposition"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "preAuthRef" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#pre_auth_ref.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("preAuthRef"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_preAuthRef" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#pre_auth_ref.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_preAuthRef"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "preAuthRefPeriod" => {
                            if r#pre_auth_ref_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("preAuthRefPeriod"));
                            }
                            r#pre_auth_ref_period = Some(map_access.next_value()?);
                        }
                        "careTeam" => {
                            if r#care_team.is_some() {
                                return Err(serde::de::Error::duplicate_field("careTeam"));
                            }
                            r#care_team = Some(map_access.next_value()?);
                        }
                        "supportingInfo" => {
                            if r#supporting_info.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportingInfo"));
                            }
                            r#supporting_info = Some(map_access.next_value()?);
                        }
                        "diagnosis" => {
                            if r#diagnosis.is_some() {
                                return Err(serde::de::Error::duplicate_field("diagnosis"));
                            }
                            r#diagnosis = Some(map_access.next_value()?);
                        }
                        "procedure" => {
                            if r#procedure.is_some() {
                                return Err(serde::de::Error::duplicate_field("procedure"));
                            }
                            r#procedure = Some(map_access.next_value()?);
                        }
                        "precedence" => {
                            let some = r#precedence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("precedence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_precedence" => {
                            let some = r#precedence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_precedence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "insurance" => {
                            if r#insurance.is_some() {
                                return Err(serde::de::Error::duplicate_field("insurance"));
                            }
                            r#insurance = Some(map_access.next_value()?);
                        }
                        "accident" => {
                            if r#accident.is_some() {
                                return Err(serde::de::Error::duplicate_field("accident"));
                            }
                            r#accident = Some(map_access.next_value()?);
                        }
                        "item" => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field("item"));
                            }
                            r#item = Some(map_access.next_value()?);
                        }
                        "addItem" => {
                            if r#add_item.is_some() {
                                return Err(serde::de::Error::duplicate_field("addItem"));
                            }
                            r#add_item = Some(map_access.next_value()?);
                        }
                        "adjudication" => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                        "total" => {
                            if r#total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            r#total = Some(map_access.next_value()?);
                        }
                        "payment" => {
                            if r#payment.is_some() {
                                return Err(serde::de::Error::duplicate_field("payment"));
                            }
                            r#payment = Some(map_access.next_value()?);
                        }
                        "formCode" => {
                            if r#form_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("formCode"));
                            }
                            r#form_code = Some(map_access.next_value()?);
                        }
                        "form" => {
                            if r#form.is_some() {
                                return Err(serde::de::Error::duplicate_field("form"));
                            }
                            r#form = Some(map_access.next_value()?);
                        }
                        "processNote" => {
                            if r#process_note.is_some() {
                                return Err(serde::de::Error::duplicate_field("processNote"));
                            }
                            r#process_note = Some(map_access.next_value()?);
                        }
                        "benefitPeriod" => {
                            if r#benefit_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("benefitPeriod"));
                            }
                            r#benefit_period = Some(map_access.next_value()?);
                        }
                        "benefitBalance" => {
                            if r#benefit_balance.is_some() {
                                return Err(serde::de::Error::duplicate_field("benefitBalance"));
                            }
                            r#benefit_balance = Some(map_access.next_value()?);
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
                                    "identifier",
                                    "status",
                                    "type",
                                    "sub_type",
                                    "use",
                                    "patient",
                                    "billable_period",
                                    "created",
                                    "enterer",
                                    "insurer",
                                    "provider",
                                    "priority",
                                    "funds_reserve_requested",
                                    "funds_reserve",
                                    "related",
                                    "prescription",
                                    "original_prescription",
                                    "payee",
                                    "referral",
                                    "facility",
                                    "claim",
                                    "claim_response",
                                    "outcome",
                                    "disposition",
                                    "pre_auth_ref",
                                    "pre_auth_ref_period",
                                    "care_team",
                                    "supporting_info",
                                    "diagnosis",
                                    "procedure",
                                    "precedence",
                                    "insurance",
                                    "accident",
                                    "item",
                                    "add_item",
                                    "adjudication",
                                    "total",
                                    "payment",
                                    "form_code",
                                    "form",
                                    "process_note",
                                    "benefit_period",
                                    "benefit_balance",
                                ],
                            ))
                        }
                    }
                }
                Ok(ExplanationOfBenefit {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#sub_type,
                    r#use: r#use.ok_or(serde::de::Error::missing_field("use"))?,
                    r#patient: r#patient.ok_or(serde::de::Error::missing_field("patient"))?,
                    r#billable_period,
                    r#created: r#created.ok_or(serde::de::Error::missing_field("created"))?,
                    r#enterer,
                    r#insurer: r#insurer.ok_or(serde::de::Error::missing_field("insurer"))?,
                    r#provider: r#provider.ok_or(serde::de::Error::missing_field("provider"))?,
                    r#priority,
                    r#funds_reserve_requested,
                    r#funds_reserve,
                    r#related: r#related.unwrap_or(vec![]),
                    r#prescription,
                    r#original_prescription,
                    r#payee,
                    r#referral,
                    r#facility,
                    r#claim,
                    r#claim_response,
                    r#outcome: r#outcome.ok_or(serde::de::Error::missing_field("outcome"))?,
                    r#disposition,
                    r#pre_auth_ref: r#pre_auth_ref.unwrap_or(vec![]),
                    r#pre_auth_ref_period: r#pre_auth_ref_period.unwrap_or(vec![]),
                    r#care_team: r#care_team.unwrap_or(vec![]),
                    r#supporting_info: r#supporting_info.unwrap_or(vec![]),
                    r#diagnosis: r#diagnosis.unwrap_or(vec![]),
                    r#procedure: r#procedure.unwrap_or(vec![]),
                    r#precedence,
                    r#insurance: r#insurance.unwrap_or(vec![]),
                    r#accident,
                    r#item: r#item.unwrap_or(vec![]),
                    r#add_item: r#add_item.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#total: r#total.unwrap_or(vec![]),
                    r#payment,
                    r#form_code,
                    r#form,
                    r#process_note: r#process_note.unwrap_or(vec![]),
                    r#benefit_period,
                    r#benefit_balance: r#benefit_balance.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
