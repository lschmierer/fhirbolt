// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ClaimSupportingInfoTiming {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
impl Default for ClaimSupportingInfoTiming {
    fn default() -> ClaimSupportingInfoTiming {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ClaimSupportingInfoValue {
    Boolean(Box<super::super::types::Boolean>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ClaimSupportingInfoValue {
    fn default() -> ClaimSupportingInfoValue {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ClaimDiagnosisDiagnosis {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ClaimDiagnosisDiagnosis {
    fn default() -> ClaimDiagnosisDiagnosis {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ClaimProcedureProcedure {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ClaimProcedureProcedure {
    fn default() -> ClaimProcedureProcedure {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ClaimAccidentLocation {
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ClaimAccidentLocation {
    fn default() -> ClaimAccidentLocation {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ClaimItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
}
impl Default for ClaimItemServiced {
    fn default() -> ClaimItemServiced {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ClaimItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ClaimItemLocation {
    fn default() -> ClaimItemLocation {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimRelated {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#claim: Option<Box<super::super::types::Reference>>,
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reference: Option<Box<super::super::types::Identifier>>,
}
impl serde::ser::Serialize for ClaimRelated {
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
impl<'de> serde::de::Deserialize<'de> for ClaimRelated {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimRelated;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimRelated")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimRelated, V::Error>
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
                Ok(ClaimRelated {
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
pub struct ClaimPayee {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#party: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ClaimPayee {
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
        if let Some(some) = self.r#party.as_ref() {
            state.serialize_entry("party", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimPayee {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimPayee;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimPayee")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimPayee, V::Error>
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
                Ok(ClaimPayee {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#party,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimCareTeam {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#provider: Box<super::super::types::Reference>,
    pub r#responsible: Option<super::super::types::Boolean>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#qualification: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClaimCareTeam {
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
impl<'de> serde::de::Deserialize<'de> for ClaimCareTeam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimCareTeam;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimCareTeam")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimCareTeam, V::Error>
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
                Ok(ClaimCareTeam {
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
pub struct ClaimSupportingInfo {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#timing: Option<ClaimSupportingInfoTiming>,
    pub r#value: Option<ClaimSupportingInfoValue>,
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClaimSupportingInfo {
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
                ClaimSupportingInfoTiming::Date(ref value) => {
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
                ClaimSupportingInfoTiming::Period(ref value) => {
                    state.serialize_entry("timingPeriod", value)?;
                }
            }
        }
        if let Some(some) = self.r#value.as_ref() {
            match some {
                ClaimSupportingInfoValue::Boolean(ref value) => {
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
                ClaimSupportingInfoValue::String(ref value) => {
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
                ClaimSupportingInfoValue::Quantity(ref value) => {
                    state.serialize_entry("valueQuantity", value)?;
                }
                ClaimSupportingInfoValue::Attachment(ref value) => {
                    state.serialize_entry("valueAttachment", value)?;
                }
                ClaimSupportingInfoValue::Reference(ref value) => {
                    state.serialize_entry("valueReference", value)?;
                }
            }
        }
        if let Some(some) = self.r#reason.as_ref() {
            state.serialize_entry("reason", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimSupportingInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimSupportingInfo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimSupportingInfo")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimSupportingInfo, V::Error>
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
                let mut r#timing: Option<ClaimSupportingInfoTiming> = None;
                let mut r#value: Option<ClaimSupportingInfoValue> = None;
                let mut r#reason: Option<Box<super::super::types::CodeableConcept>> = None;
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
                            let r#enum = r#timing
                                .get_or_insert(ClaimSupportingInfoTiming::Date(Default::default()));
                            if let ClaimSupportingInfoTiming::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timingDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("timing[x]"));
                            }
                        }
                        "_timingDate" => {
                            let r#enum = r#timing
                                .get_or_insert(ClaimSupportingInfoTiming::Date(Default::default()));
                            if let ClaimSupportingInfoTiming::Date(variant) = r#enum {
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
                            r#timing =
                                Some(ClaimSupportingInfoTiming::Period(map_access.next_value()?));
                        }
                        "valueBoolean" => {
                            let r#enum = r#value.get_or_insert(ClaimSupportingInfoValue::Boolean(
                                Default::default(),
                            ));
                            if let ClaimSupportingInfoValue::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueBoolean" => {
                            let r#enum = r#value.get_or_insert(ClaimSupportingInfoValue::Boolean(
                                Default::default(),
                            ));
                            if let ClaimSupportingInfoValue::Boolean(variant) = r#enum {
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
                            let r#enum = r#value.get_or_insert(ClaimSupportingInfoValue::String(
                                Default::default(),
                            ));
                            if let ClaimSupportingInfoValue::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueString" => {
                            let r#enum = r#value.get_or_insert(ClaimSupportingInfoValue::String(
                                Default::default(),
                            ));
                            if let ClaimSupportingInfoValue::String(variant) = r#enum {
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
                            r#value =
                                Some(ClaimSupportingInfoValue::Quantity(map_access.next_value()?));
                        }
                        "valueAttachment" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some(ClaimSupportingInfoValue::Attachment(
                                map_access.next_value()?,
                            ));
                        }
                        "valueReference" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some(ClaimSupportingInfoValue::Reference(
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
                Ok(ClaimSupportingInfo {
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
pub struct ClaimDiagnosis {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#diagnosis: ClaimDiagnosisDiagnosis,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#on_admission: Option<Box<super::super::types::CodeableConcept>>,
    pub r#package_code: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClaimDiagnosis {
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
            ClaimDiagnosisDiagnosis::CodeableConcept(ref value) => {
                state.serialize_entry("diagnosisCodeableConcept", value)?;
            }
            ClaimDiagnosisDiagnosis::Reference(ref value) => {
                state.serialize_entry("diagnosisReference", value)?;
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
impl<'de> serde::de::Deserialize<'de> for ClaimDiagnosis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimDiagnosis;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimDiagnosis")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimDiagnosis, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#diagnosis: Option<ClaimDiagnosisDiagnosis> = None;
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
                            r#diagnosis = Some(ClaimDiagnosisDiagnosis::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "diagnosisReference" => {
                            if r#diagnosis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "diagnosisReference",
                                ));
                            }
                            r#diagnosis =
                                Some(ClaimDiagnosisDiagnosis::Reference(map_access.next_value()?));
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
                Ok(ClaimDiagnosis {
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
pub struct ClaimProcedure {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#procedure: ClaimProcedureProcedure,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ClaimProcedure {
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
            ClaimProcedureProcedure::CodeableConcept(ref value) => {
                state.serialize_entry("procedureCodeableConcept", value)?;
            }
            ClaimProcedureProcedure::Reference(ref value) => {
                state.serialize_entry("procedureReference", value)?;
            }
        }
        if !self.r#udi.is_empty() {
            state.serialize_entry("udi", &self.r#udi)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimProcedure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimProcedure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimProcedure")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimProcedure, V::Error>
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
                let mut r#procedure: Option<ClaimProcedureProcedure> = None;
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
                            r#procedure = Some(ClaimProcedureProcedure::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "procedureReference" => {
                            if r#procedure.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "procedureReference",
                                ));
                            }
                            r#procedure =
                                Some(ClaimProcedureProcedure::Reference(map_access.next_value()?));
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
                Ok(ClaimProcedure {
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
pub struct ClaimInsurance {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#focal: super::super::types::Boolean,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#coverage: Box<super::super::types::Reference>,
    pub r#business_arrangement: Option<super::super::types::String>,
    pub r#pre_auth_ref: Vec<super::super::types::String>,
    pub r#claim_response: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ClaimInsurance {
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
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        state.serialize_entry("coverage", &self.r#coverage)?;
        if let Some(some) = self.r#business_arrangement.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("businessArrangement", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_businessArrangement", &primitive_element)?;
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
        if let Some(some) = self.r#claim_response.as_ref() {
            state.serialize_entry("claimResponse", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimInsurance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimInsurance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimInsurance")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimInsurance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#focal: Option<super::super::types::Boolean> = None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#coverage: Option<Box<super::super::types::Reference>> = None;
                let mut r#business_arrangement: Option<super::super::types::String> = None;
                let mut r#pre_auth_ref: Option<Vec<super::super::types::String>> = None;
                let mut r#claim_response: Option<Box<super::super::types::Reference>> = None;
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
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "coverage" => {
                            if r#coverage.is_some() {
                                return Err(serde::de::Error::duplicate_field("coverage"));
                            }
                            r#coverage = Some(map_access.next_value()?);
                        }
                        "businessArrangement" => {
                            let some = r#business_arrangement.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "businessArrangement",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_businessArrangement" => {
                            let some = r#business_arrangement.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_businessArrangement",
                                ));
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
                        "claimResponse" => {
                            if r#claim_response.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimResponse"));
                            }
                            r#claim_response = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "sequence",
                                    "focal",
                                    "identifier",
                                    "coverage",
                                    "business_arrangement",
                                    "pre_auth_ref",
                                    "claim_response",
                                ],
                            ))
                        }
                    }
                }
                Ok(ClaimInsurance {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence: r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?,
                    r#focal: r#focal.ok_or(serde::de::Error::missing_field("focal"))?,
                    r#identifier,
                    r#coverage: r#coverage.ok_or(serde::de::Error::missing_field("coverage"))?,
                    r#business_arrangement,
                    r#pre_auth_ref: r#pre_auth_ref.unwrap_or(vec![]),
                    r#claim_response,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimAccident {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: super::super::types::Date,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#location: Option<ClaimAccidentLocation>,
}
impl serde::ser::Serialize for ClaimAccident {
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
        if let Some(some) = self.r#date.value.as_ref() {
            state.serialize_entry("date", some)?;
        }
        if self.r#date.id.is_some() || !self.r#date.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#date.id,
                extension: &self.r#date.extension,
            };
            state.serialize_entry("_date", &primitive_element)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#location.as_ref() {
            match some {
                ClaimAccidentLocation::Address(ref value) => {
                    state.serialize_entry("locationAddress", value)?;
                }
                ClaimAccidentLocation::Reference(ref value) => {
                    state.serialize_entry("locationReference", value)?;
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimAccident {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimAccident;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimAccident")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimAccident, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#date: Option<super::super::types::Date> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#location: Option<ClaimAccidentLocation> = None;
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
                            r#location =
                                Some(ClaimAccidentLocation::Address(map_access.next_value()?));
                        }
                        "locationReference" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationReference"));
                            }
                            r#location =
                                Some(ClaimAccidentLocation::Reference(map_access.next_value()?));
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
                Ok(ClaimAccident {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#date: r#date.ok_or(serde::de::Error::missing_field("date"))?,
                    r#type,
                    r#location,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimItemDetailSubDetail {
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
}
impl serde::ser::Serialize for ClaimItemDetailSubDetail {
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
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimItemDetailSubDetail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimItemDetailSubDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimItemDetailSubDetail")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimItemDetailSubDetail, V::Error>
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
                                ],
                            ))
                        }
                    }
                }
                Ok(ClaimItemDetailSubDetail {
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimItemDetail {
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
    pub r#sub_detail: Vec<ClaimItemDetailSubDetail>,
}
impl serde::ser::Serialize for ClaimItemDetail {
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
        if !self.r#sub_detail.is_empty() {
            state.serialize_entry("subDetail", &self.r#sub_detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimItemDetail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimItemDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimItemDetail")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimItemDetail, V::Error>
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
                let mut r#sub_detail: Option<Vec<ClaimItemDetailSubDetail>> = None;
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
                                    "sub_detail",
                                ],
                            ))
                        }
                    }
                }
                Ok(ClaimItemDetail {
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
                    r#sub_detail: r#sub_detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimItem {
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
    pub r#serviced: Option<ClaimItemServiced>,
    pub r#location: Option<ClaimItemLocation>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#encounter: Vec<Box<super::super::types::Reference>>,
    pub r#detail: Vec<ClaimItemDetail>,
}
impl serde::ser::Serialize for ClaimItem {
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
                ClaimItemServiced::Date(ref value) => {
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
                ClaimItemServiced::Period(ref value) => {
                    state.serialize_entry("servicedPeriod", value)?;
                }
            }
        }
        if let Some(some) = self.r#location.as_ref() {
            match some {
                ClaimItemLocation::CodeableConcept(ref value) => {
                    state.serialize_entry("locationCodeableConcept", value)?;
                }
                ClaimItemLocation::Address(ref value) => {
                    state.serialize_entry("locationAddress", value)?;
                }
                ClaimItemLocation::Reference(ref value) => {
                    state.serialize_entry("locationReference", value)?;
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
        if !self.r#detail.is_empty() {
            state.serialize_entry("detail", &self.r#detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimItem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimItem, V::Error>
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
                let mut r#serviced: Option<ClaimItemServiced> = None;
                let mut r#location: Option<ClaimItemLocation> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#udi: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#body_site: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_site: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#encounter: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#detail: Option<Vec<ClaimItemDetail>> = None;
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
                            let r#enum = r#serviced
                                .get_or_insert(ClaimItemServiced::Date(Default::default()));
                            if let ClaimItemServiced::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("servicedDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("serviced[x]"));
                            }
                        }
                        "_servicedDate" => {
                            let r#enum = r#serviced
                                .get_or_insert(ClaimItemServiced::Date(Default::default()));
                            if let ClaimItemServiced::Date(variant) = r#enum {
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
                            r#serviced = Some(ClaimItemServiced::Period(map_access.next_value()?));
                        }
                        "locationCodeableConcept" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "locationCodeableConcept",
                                ));
                            }
                            r#location =
                                Some(ClaimItemLocation::CodeableConcept(map_access.next_value()?));
                        }
                        "locationAddress" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationAddress"));
                            }
                            r#location = Some(ClaimItemLocation::Address(map_access.next_value()?));
                        }
                        "locationReference" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationReference"));
                            }
                            r#location =
                                Some(ClaimItemLocation::Reference(map_access.next_value()?));
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
                                    "detail",
                                ],
                            ))
                        }
                    }
                }
                Ok(ClaimItem {
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
                    r#detail: r#detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Claim {
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
    pub r#insurer: Option<Box<super::super::types::Reference>>,
    pub r#provider: Box<super::super::types::Reference>,
    pub r#priority: Box<super::super::types::CodeableConcept>,
    pub r#funds_reserve: Option<Box<super::super::types::CodeableConcept>>,
    pub r#related: Vec<ClaimRelated>,
    pub r#prescription: Option<Box<super::super::types::Reference>>,
    pub r#original_prescription: Option<Box<super::super::types::Reference>>,
    pub r#payee: Option<ClaimPayee>,
    pub r#referral: Option<Box<super::super::types::Reference>>,
    pub r#facility: Option<Box<super::super::types::Reference>>,
    pub r#care_team: Vec<ClaimCareTeam>,
    pub r#supporting_info: Vec<ClaimSupportingInfo>,
    pub r#diagnosis: Vec<ClaimDiagnosis>,
    pub r#procedure: Vec<ClaimProcedure>,
    pub r#insurance: Vec<ClaimInsurance>,
    pub r#accident: Option<ClaimAccident>,
    pub r#item: Vec<ClaimItem>,
    pub r#total: Option<Box<super::super::types::Money>>,
}
impl serde::ser::Serialize for Claim {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Claim")?;
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
        if let Some(some) = self.r#insurer.as_ref() {
            state.serialize_entry("insurer", some)?;
        }
        state.serialize_entry("provider", &self.r#provider)?;
        state.serialize_entry("priority", &self.r#priority)?;
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
        if !self.r#insurance.is_empty() {
            state.serialize_entry("insurance", &self.r#insurance)?;
        }
        if let Some(some) = self.r#accident.as_ref() {
            state.serialize_entry("accident", some)?;
        }
        if !self.r#item.is_empty() {
            state.serialize_entry("item", &self.r#item)?;
        }
        if let Some(some) = self.r#total.as_ref() {
            state.serialize_entry("total", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Claim {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Claim;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Claim")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Claim, V::Error>
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
                let mut r#funds_reserve: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#related: Option<Vec<ClaimRelated>> = None;
                let mut r#prescription: Option<Box<super::super::types::Reference>> = None;
                let mut r#original_prescription: Option<Box<super::super::types::Reference>> = None;
                let mut r#payee: Option<ClaimPayee> = None;
                let mut r#referral: Option<Box<super::super::types::Reference>> = None;
                let mut r#facility: Option<Box<super::super::types::Reference>> = None;
                let mut r#care_team: Option<Vec<ClaimCareTeam>> = None;
                let mut r#supporting_info: Option<Vec<ClaimSupportingInfo>> = None;
                let mut r#diagnosis: Option<Vec<ClaimDiagnosis>> = None;
                let mut r#procedure: Option<Vec<ClaimProcedure>> = None;
                let mut r#insurance: Option<Vec<ClaimInsurance>> = None;
                let mut r#accident: Option<ClaimAccident> = None;
                let mut r#item: Option<Vec<ClaimItem>> = None;
                let mut r#total: Option<Box<super::super::types::Money>> = None;
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
                        "total" => {
                            if r#total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            r#total = Some(map_access.next_value()?);
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
                                    "funds_reserve",
                                    "related",
                                    "prescription",
                                    "original_prescription",
                                    "payee",
                                    "referral",
                                    "facility",
                                    "care_team",
                                    "supporting_info",
                                    "diagnosis",
                                    "procedure",
                                    "insurance",
                                    "accident",
                                    "item",
                                    "total",
                                ],
                            ))
                        }
                    }
                }
                Ok(Claim {
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
                    r#insurer,
                    r#provider: r#provider.ok_or(serde::de::Error::missing_field("provider"))?,
                    r#priority: r#priority.ok_or(serde::de::Error::missing_field("priority"))?,
                    r#funds_reserve,
                    r#related: r#related.unwrap_or(vec![]),
                    r#prescription,
                    r#original_prescription,
                    r#payee,
                    r#referral,
                    r#facility,
                    r#care_team: r#care_team.unwrap_or(vec![]),
                    r#supporting_info: r#supporting_info.unwrap_or(vec![]),
                    r#diagnosis: r#diagnosis.unwrap_or(vec![]),
                    r#procedure: r#procedure.unwrap_or(vec![]),
                    r#insurance: r#insurance.unwrap_or(vec![]),
                    r#accident,
                    r#item: r#item.unwrap_or(vec![]),
                    r#total,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
