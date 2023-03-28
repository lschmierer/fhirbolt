// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "The date when or period to which this information refers."]
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
#[doc = "Additional data or information such as resources, documents, images etc. including references to the data or the actual inclusion of the data."]
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
#[doc = "The nature of illness or problem in a coded form or as a reference to an external defined Condition."]
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
#[doc = "The code or reference to a Procedure resource which identifies the clinical intervention performed."]
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
#[doc = "The physical location of the accident event."]
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
#[doc = "The date or dates when the service or product was supplied, performed or completed."]
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
#[doc = "Where the product or service was provided."]
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
#[doc = "The date or dates when the service or product was supplied, performed or completed."]
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
#[doc = "Where the product or service was provided."]
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
#[doc = "The quantity of the benefit which is permitted under the coverage."]
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
#[doc = "The quantity of the benefit which have been consumed to date."]
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
#[doc = "Other claims which are related to this claim such as prior submissions or claims for related services or for the same event."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitRelated {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Reference to a related claim."]
    pub r#claim: Option<Box<super::super::types::Reference>>,
    #[doc = "A code to convey how the claims are related."]
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An alternate organizational reference to the case or file to which this particular claim pertains."]
    pub r#reference: Option<Box<super::super::types::Identifier>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitRelated {
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
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitRelated {
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
            #[serde(rename = "claim")]
            Claim,
            #[serde(rename = "relationship")]
            Relationship,
            #[serde(rename = "reference")]
            Reference,
            Unknown(std::string::String),
        }
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
                            Field::Claim => {
                                if r#claim.is_some() {
                                    return Err(serde::de::Error::duplicate_field("claim"));
                                }
                                r#claim = Some(map_access.next_value()?);
                            }
                            Field::Relationship => {
                                if r#relationship.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relationship"));
                                }
                                r#relationship = Some(map_access.next_value()?);
                            }
                            Field::Reference => {
                                if r#reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                r#reference = Some(map_access.next_value()?);
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
                                        "claim",
                                        "relationship",
                                        "reference",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The party to be reimbursed for cost of the products and services according to the terms of the policy."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitPayee {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of Party to be reimbursed: Subscriber, provider, other."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Reference to the individual or organization to whom any payment will be made."]
    pub r#party: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitPayee {
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
            if let Some(some) = self.r#party.as_ref() {
                state.serialize_entry("party", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitPayee {
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
            #[serde(rename = "party")]
            Party,
            Unknown(std::string::String),
        }
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
                            Field::Party => {
                                if r#party.is_some() {
                                    return Err(serde::de::Error::duplicate_field("party"));
                                }
                                r#party = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "type", "party"],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitPayee {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#party,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The members of the team who provided the products and services."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitCareTeam {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify care team entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "Member of the team who provided the product or service."]
    pub r#provider: Box<super::super::types::Reference>,
    #[doc = "The party who is billing and/or responsible for the claimed products or services."]
    pub r#responsible: Option<super::super::types::Boolean>,
    #[doc = "The lead, assisting or supervising practitioner and their discipline if a multidisciplinary team."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The qualification of the practitioner which is applicable for this service."]
    pub r#qualification: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitCareTeam {
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
                if let Some(some) = self.r#sequence.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sequence", &some)?;
                }
                if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#sequence.id.as_ref(),
                        extension: &self.r#sequence.extension,
                    };
                    state.serialize_entry("_sequence", &primitive_element)?;
                }
            } else {
                state.serialize_entry("sequence", &self.r#sequence)?;
            }
            state.serialize_entry("provider", &self.r#provider)?;
            if _ctx.output_json {
                if let Some(some) = self.r#responsible.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("responsible", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_responsible", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#responsible.as_ref() {
                    state.serialize_entry("responsible", some)?;
                }
            }
            if let Some(some) = self.r#role.as_ref() {
                state.serialize_entry("role", some)?;
            }
            if let Some(some) = self.r#qualification.as_ref() {
                state.serialize_entry("qualification", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitCareTeam {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "provider")]
            Provider,
            #[serde(rename = "responsible")]
            Responsible,
            #[serde(rename = "_responsible")]
            ResponsiblePrimitiveElement,
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "qualification")]
            Qualification,
            Unknown(std::string::String),
        }
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
                            Field::Sequence => {
                                if _ctx.from_json {
                                    let some = r#sequence.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    r#sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::SequencePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "provider",
                                            "responsible",
                                            "role",
                                            "qualification",
                                        ],
                                    ));
                                }
                            }
                            Field::Provider => {
                                if r#provider.is_some() {
                                    return Err(serde::de::Error::duplicate_field("provider"));
                                }
                                r#provider = Some(map_access.next_value()?);
                            }
                            Field::Responsible => {
                                if _ctx.from_json {
                                    let some = r#responsible.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "responsible",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#responsible.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "responsible",
                                        ));
                                    }
                                    r#responsible = Some(map_access.next_value()?);
                                }
                            }
                            Field::ResponsiblePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#responsible.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_responsible",
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
                                        "responsible",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "provider",
                                            "responsible",
                                            "role",
                                            "qualification",
                                        ],
                                    ));
                                }
                            }
                            Field::Role => {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                r#role = Some(map_access.next_value()?);
                            }
                            Field::Qualification => {
                                if r#qualification.is_some() {
                                    return Err(serde::de::Error::duplicate_field("qualification"));
                                }
                                r#qualification = Some(map_access.next_value()?);
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
                                        "sequence",
                                        "provider",
                                        "responsible",
                                        "role",
                                        "qualification",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitCareTeam {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#provider: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#provider.unwrap_or(Default::default())
                        } else {
                            r#provider.ok_or(serde::de::Error::missing_field("provider"))?
                        },
                        r#responsible,
                        r#role,
                        r#qualification,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Additional information codes regarding exceptions, special considerations, the condition, situation, prior or concurrent issues."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitSupportingInfo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify supporting information entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The general class of the information supplied: information; exception; accident, employment; onset, etc."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "System and code pertaining to the specific information regarding special conditions relating to the setting, treatment or patient  for which care is sought."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date when or period to which this information refers."]
    pub r#timing: Option<ExplanationOfBenefitSupportingInfoTiming>,
    #[doc = "Additional data or information such as resources, documents, images etc. including references to the data or the actual inclusion of the data."]
    pub r#value: Option<ExplanationOfBenefitSupportingInfoValue>,
    #[doc = "Provides the reason in the situation where a reason code is required in addition to the content."]
    pub r#reason: Option<Box<super::super::types::Coding>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitSupportingInfo {
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
                if let Some(some) = self.r#sequence.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sequence", &some)?;
                }
                if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#sequence.id.as_ref(),
                        extension: &self.r#sequence.extension,
                    };
                    state.serialize_entry("_sequence", &primitive_element)?;
                }
            } else {
                state.serialize_entry("sequence", &self.r#sequence)?;
            }
            state.serialize_entry("category", &self.r#category)?;
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if let Some(some) = self.r#timing.as_ref() {
                match some {
                    ExplanationOfBenefitSupportingInfoTiming::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("timingDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_timingDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("timingDate", value)?;
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
                    ExplanationOfBenefitSupportingInfoValue::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueString", value)?;
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
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitSupportingInfo {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "timingDate")]
            TimingDate,
            #[serde(rename = "_timingDate")]
            TimingDatePrimitiveElement,
            #[serde(rename = "timingPeriod")]
            TimingPeriod,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueAttachment")]
            ValueAttachment,
            #[serde(rename = "valueReference")]
            ValueReference,
            #[serde(rename = "reason")]
            Reason,
            Unknown(std::string::String),
        }
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
                            Field::Sequence => {
                                if _ctx.from_json {
                                    let some = r#sequence.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    r#sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::SequencePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "category",
                                            "code",
                                            "timingDate",
                                            "timingPeriod",
                                            "valueBoolean",
                                            "valueString",
                                            "valueQuantity",
                                            "valueAttachment",
                                            "valueReference",
                                            "reason",
                                        ],
                                    ));
                                }
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
                            Field::TimingDate => {
                                if _ctx.from_json {
                                    let r#enum = r#timing.get_or_insert(
                                        ExplanationOfBenefitSupportingInfoTiming::Date(
                                            Default::default(),
                                        ),
                                    );
                                    if let ExplanationOfBenefitSupportingInfoTiming::Date(variant) =
                                        r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "timingDate",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("timing[x]"));
                                    }
                                } else {
                                    if r#timing.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timingDate",
                                        ));
                                    }
                                    r#timing =
                                        Some(ExplanationOfBenefitSupportingInfoTiming::Date(
                                            map_access.next_value()?,
                                        ));
                                }
                            }
                            Field::TimingDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#timing.get_or_insert(
                                        ExplanationOfBenefitSupportingInfoTiming::Date(
                                            Default::default(),
                                        ),
                                    );
                                    if let ExplanationOfBenefitSupportingInfoTiming::Date(variant) =
                                        r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_timingDate",
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
                                            "_timing[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "timingDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "category",
                                            "code",
                                            "timingDate",
                                            "timingPeriod",
                                            "valueBoolean",
                                            "valueString",
                                            "valueQuantity",
                                            "valueAttachment",
                                            "valueReference",
                                            "reason",
                                        ],
                                    ));
                                }
                            }
                            Field::TimingPeriod => {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timingPeriod"));
                                }
                                r#timing = Some(ExplanationOfBenefitSupportingInfoTiming::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueBoolean => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ExplanationOfBenefitSupportingInfoValue::Boolean(
                                            Default::default(),
                                        ),
                                    );
                                    if let ExplanationOfBenefitSupportingInfoValue::Boolean(
                                        variant,
                                    ) = r#enum
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
                                    r#value =
                                        Some(ExplanationOfBenefitSupportingInfoValue::Boolean(
                                            map_access.next_value()?,
                                        ));
                                }
                            }
                            Field::ValueBooleanPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ExplanationOfBenefitSupportingInfoValue::Boolean(
                                            Default::default(),
                                        ),
                                    );
                                    if let ExplanationOfBenefitSupportingInfoValue::Boolean(
                                        variant,
                                    ) = r#enum
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
                                            "sequence",
                                            "category",
                                            "code",
                                            "timingDate",
                                            "timingPeriod",
                                            "valueBoolean",
                                            "valueString",
                                            "valueQuantity",
                                            "valueAttachment",
                                            "valueReference",
                                            "reason",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueString => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ExplanationOfBenefitSupportingInfoValue::String(
                                            Default::default(),
                                        ),
                                    );
                                    if let ExplanationOfBenefitSupportingInfoValue::String(
                                        variant,
                                    ) = r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueString",
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
                                            "valueString",
                                        ));
                                    }
                                    r#value =
                                        Some(ExplanationOfBenefitSupportingInfoValue::String(
                                            map_access.next_value()?,
                                        ));
                                }
                            }
                            Field::ValueStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ExplanationOfBenefitSupportingInfoValue::String(
                                            Default::default(),
                                        ),
                                    );
                                    if let ExplanationOfBenefitSupportingInfoValue::String(
                                        variant,
                                    ) = r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueString",
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
                                        "valueString",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "category",
                                            "code",
                                            "timingDate",
                                            "timingPeriod",
                                            "valueBoolean",
                                            "valueString",
                                            "valueQuantity",
                                            "valueAttachment",
                                            "valueReference",
                                            "reason",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value = Some(ExplanationOfBenefitSupportingInfoValue::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueAttachment => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAttachment",
                                    ));
                                }
                                r#value =
                                    Some(ExplanationOfBenefitSupportingInfoValue::Attachment(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::ValueReference => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueReference",
                                    ));
                                }
                                r#value = Some(ExplanationOfBenefitSupportingInfoValue::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Reason => {
                                if r#reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                r#reason = Some(map_access.next_value()?);
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
                                        "sequence",
                                        "category",
                                        "code",
                                        "timingDate",
                                        "timingPeriod",
                                        "valueBoolean",
                                        "valueString",
                                        "valueQuantity",
                                        "valueAttachment",
                                        "valueReference",
                                        "reason",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitSupportingInfo {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#category: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#category.unwrap_or(Default::default())
                        } else {
                            r#category.ok_or(serde::de::Error::missing_field("category"))?
                        },
                        r#code,
                        r#timing,
                        r#value,
                        r#reason,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Information about diagnoses relevant to the claim items."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitDiagnosis {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify diagnosis entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The nature of illness or problem in a coded form or as a reference to an external defined Condition."]
    pub r#diagnosis: ExplanationOfBenefitDiagnosisDiagnosis,
    #[doc = "When the condition was observed or the relative ranking."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indication of whether the diagnosis was present on admission to a facility."]
    pub r#on_admission: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A package billing code or bundle code used to group products and services to a particular health condition (such as heart attack) which is based on a predetermined grouping code system."]
    pub r#package_code: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitDiagnosis {
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
                if let Some(some) = self.r#sequence.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sequence", &some)?;
                }
                if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#sequence.id.as_ref(),
                        extension: &self.r#sequence.extension,
                    };
                    state.serialize_entry("_sequence", &primitive_element)?;
                }
            } else {
                state.serialize_entry("sequence", &self.r#sequence)?;
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
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitDiagnosis {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "diagnosisCodeableConcept")]
            DiagnosisCodeableConcept,
            #[serde(rename = "diagnosisReference")]
            DiagnosisReference,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "onAdmission")]
            OnAdmission,
            #[serde(rename = "packageCode")]
            PackageCode,
            Unknown(std::string::String),
        }
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
                            Field::Sequence => {
                                if _ctx.from_json {
                                    let some = r#sequence.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    r#sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::SequencePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "diagnosisCodeableConcept",
                                            "diagnosisReference",
                                            "type",
                                            "onAdmission",
                                            "packageCode",
                                        ],
                                    ));
                                }
                            }
                            Field::DiagnosisCodeableConcept => {
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
                            Field::DiagnosisReference => {
                                if r#diagnosis.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "diagnosisReference",
                                    ));
                                }
                                r#diagnosis =
                                    Some(ExplanationOfBenefitDiagnosisDiagnosis::Reference(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::OnAdmission => {
                                if r#on_admission.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onAdmission"));
                                }
                                r#on_admission = Some(map_access.next_value()?);
                            }
                            Field::PackageCode => {
                                if r#package_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("packageCode"));
                                }
                                r#package_code = Some(map_access.next_value()?);
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
                                        "sequence",
                                        "diagnosisCodeableConcept",
                                        "diagnosisReference",
                                        "type",
                                        "onAdmission",
                                        "packageCode",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitDiagnosis {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#diagnosis: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#diagnosis.unwrap_or(Default::default())
                        } else {
                            r#diagnosis.ok_or(serde::de::Error::missing_field("diagnosis[x]"))?
                        },
                        r#type: r#type.unwrap_or(vec![]),
                        r#on_admission,
                        r#package_code,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Procedures performed on the patient relevant to the billing items with the claim."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitProcedure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify procedure entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "When the condition was observed or the relative ranking."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Date and optionally time the procedure was performed."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The code or reference to a Procedure resource which identifies the clinical intervention performed."]
    pub r#procedure: ExplanationOfBenefitProcedureProcedure,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitProcedure {
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
                if let Some(some) = self.r#sequence.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sequence", &some)?;
                }
                if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#sequence.id.as_ref(),
                        extension: &self.r#sequence.extension,
                    };
                    state.serialize_entry("_sequence", &primitive_element)?;
                }
            } else {
                state.serialize_entry("sequence", &self.r#sequence)?;
            }
            if !self.r#type.is_empty() {
                state.serialize_entry("type", &self.r#type)?;
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
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitProcedure {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "procedureCodeableConcept")]
            ProcedureCodeableConcept,
            #[serde(rename = "procedureReference")]
            ProcedureReference,
            #[serde(rename = "udi")]
            Udi,
            Unknown(std::string::String),
        }
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
                            Field::Sequence => {
                                if _ctx.from_json {
                                    let some = r#sequence.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    r#sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::SequencePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "type",
                                            "date",
                                            "procedureCodeableConcept",
                                            "procedureReference",
                                            "udi",
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
                                            "sequence",
                                            "type",
                                            "date",
                                            "procedureCodeableConcept",
                                            "procedureReference",
                                            "udi",
                                        ],
                                    ));
                                }
                            }
                            Field::ProcedureCodeableConcept => {
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
                            Field::ProcedureReference => {
                                if r#procedure.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "procedureReference",
                                    ));
                                }
                                r#procedure =
                                    Some(ExplanationOfBenefitProcedureProcedure::Reference(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::Udi => {
                                if r#udi.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udi"));
                                }
                                r#udi = Some(map_access.next_value()?);
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
                                        "sequence",
                                        "type",
                                        "date",
                                        "procedureCodeableConcept",
                                        "procedureReference",
                                        "udi",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitProcedure {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#type: r#type.unwrap_or(vec![]),
                        r#date,
                        r#procedure: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#procedure.unwrap_or(Default::default())
                        } else {
                            r#procedure.ok_or(serde::de::Error::missing_field("procedure[x]"))?
                        },
                        r#udi: r#udi.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Financial instruments for reimbursement for the health care products and services specified on the claim."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitInsurance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A flag to indicate that this Coverage is to be used for adjudication of this claim when set to true."]
    pub r#focal: super::super::types::Boolean,
    #[doc = "Reference to the insurance card level information contained in the Coverage resource. The coverage issuing insurer will use these details to locate the patient's actual coverage within the insurer's information system."]
    pub r#coverage: Box<super::super::types::Reference>,
    #[doc = "Reference numbers previously provided by the insurer to the provider to be quoted on subsequent claims containing services or products related to the prior authorization."]
    pub r#pre_auth_ref: Vec<super::super::types::String>,
}
impl serde::ser::Serialize for ExplanationOfBenefitInsurance {
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
                if let Some(some) = self.r#focal.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("focal", &some)?;
                }
                if self.r#focal.id.is_some() || !self.r#focal.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#focal.id.as_ref(),
                        extension: &self.r#focal.extension,
                    };
                    state.serialize_entry("_focal", &primitive_element)?;
                }
            } else {
                state.serialize_entry("focal", &self.r#focal)?;
            }
            state.serialize_entry("coverage", &self.r#coverage)?;
            if _ctx.output_json {
                if !self.r#pre_auth_ref.is_empty() {
                    let values = self
                        .r#pre_auth_ref
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#pre_auth_ref.is_empty() {
                    state.serialize_entry("preAuthRef", &self.r#pre_auth_ref)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitInsurance {
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
            #[serde(rename = "focal")]
            Focal,
            #[serde(rename = "_focal")]
            FocalPrimitiveElement,
            #[serde(rename = "coverage")]
            Coverage,
            #[serde(rename = "preAuthRef")]
            PreAuthRef,
            #[serde(rename = "_preAuthRef")]
            PreAuthRefPrimitiveElement,
            Unknown(std::string::String),
        }
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
                            Field::Focal => {
                                if _ctx.from_json {
                                    let some = r#focal.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("focal"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#focal.is_some() {
                                        return Err(serde::de::Error::duplicate_field("focal"));
                                    }
                                    r#focal = Some(map_access.next_value()?);
                                }
                            }
                            Field::FocalPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "focal",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "focal",
                                            "coverage",
                                            "preAuthRef",
                                        ],
                                    ));
                                }
                            }
                            Field::Coverage => {
                                if r#coverage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("coverage"));
                                }
                                r#coverage = Some(map_access.next_value()?);
                            }
                            Field::PreAuthRef => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#pre_auth_ref.get_or_insert(
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
                                            "preAuthRef",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#pre_auth_ref.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "preAuthRef",
                                        ));
                                    }
                                    r#pre_auth_ref = Some(map_access.next_value()?);
                                }
                            }
                            Field::PreAuthRefPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#pre_auth_ref.get_or_insert(
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
                                            "_preAuthRef",
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
                                        "preAuthRef",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "focal",
                                            "coverage",
                                            "preAuthRef",
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
                                        "focal",
                                        "coverage",
                                        "preAuthRef",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitInsurance {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#focal: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#focal.unwrap_or(Default::default())
                        } else {
                            r#focal.ok_or(serde::de::Error::missing_field("focal"))?
                        },
                        r#coverage: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#coverage.unwrap_or(Default::default())
                        } else {
                            r#coverage.ok_or(serde::de::Error::missing_field("coverage"))?
                        },
                        r#pre_auth_ref: r#pre_auth_ref.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Details of a accident which resulted in injuries which required the products and services listed in the claim."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitAccident {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Date of an accident event  related to the products and services contained in the claim."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "The type or context of the accident event for the purposes of selection of potential insurance coverages and determination of coordination between insurers."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The physical location of the accident event."]
    pub r#location: Option<ExplanationOfBenefitAccidentLocation>,
}
impl serde::ser::Serialize for ExplanationOfBenefitAccident {
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
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitAccident {
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
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "locationAddress")]
            LocationAddress,
            #[serde(rename = "locationReference")]
            LocationReference,
            Unknown(std::string::String),
        }
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
                                            "date",
                                            "type",
                                            "locationAddress",
                                            "locationReference",
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
                            Field::LocationAddress => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "locationAddress",
                                    ));
                                }
                                r#location = Some(ExplanationOfBenefitAccidentLocation::Address(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::LocationReference => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "locationReference",
                                    ));
                                }
                                r#location = Some(ExplanationOfBenefitAccidentLocation::Reference(
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
                                        "date",
                                        "type",
                                        "locationAddress",
                                        "locationReference",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "If this item is a group then the values here are a summary of the adjudication of the detail items. If this item is a simple product or service then this is the result of the adjudication of this item."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitItemAdjudication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code to indicate the information type of this adjudication record. Information types may include: the value submitted, maximum values or percentages allowed or payable under the plan, amounts that the patient is responsible for in-aggregate or pertaining to this item, amounts paid by other coverages, and the benefit payable for this item."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "A code supporting the understanding of the adjudication result and explaining variance from expected amount."]
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Monetary amount associated with the category."]
    pub r#amount: Option<Box<super::super::types::Money>>,
    #[doc = "A non-monetary value associated with the category. Mutually exclusive to the amount element above."]
    pub r#value: Option<super::super::types::Decimal>,
}
impl serde::ser::Serialize for ExplanationOfBenefitItemAdjudication {
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
            state.serialize_entry("category", &self.r#category)?;
            if let Some(some) = self.r#reason.as_ref() {
                state.serialize_entry("reason", some)?;
            }
            if let Some(some) = self.r#amount.as_ref() {
                state.serialize_entry("amount", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("value", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_value", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#value.as_ref() {
                    state.serialize_entry("value", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitItemAdjudication {
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
            #[serde(rename = "reason")]
            Reason,
            #[serde(rename = "amount")]
            Amount,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
            Unknown(std::string::String),
        }
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
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Reason => {
                                if r#reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                r#reason = Some(map_access.next_value()?);
                            }
                            Field::Amount => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amount"));
                                }
                                r#amount = Some(map_access.next_value()?);
                            }
                            Field::Value => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
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
                                            "category",
                                            "reason",
                                            "amount",
                                            "value",
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
                                        "reason",
                                        "amount",
                                        "value",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitItemAdjudication {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#category: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#category.unwrap_or(Default::default())
                        } else {
                            r#category.ok_or(serde::de::Error::missing_field("category"))?
                        },
                        r#reason,
                        r#amount,
                        r#value,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Third-tier of goods and services."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitItemDetailSubDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A claim detail line. Either a simple (a product or service) or a 'group' of sub-details which are simple items."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The type of revenue or cost center providing the product and/or service."]
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}
impl serde::ser::Serialize for ExplanationOfBenefitItemDetailSubDetail {
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
                if let Some(some) = self.r#sequence.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sequence", &some)?;
                }
                if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#sequence.id.as_ref(),
                        extension: &self.r#sequence.extension,
                    };
                    state.serialize_entry("_sequence", &primitive_element)?;
                }
            } else {
                state.serialize_entry("sequence", &self.r#sequence)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#factor.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("factor", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_factor", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#factor.as_ref() {
                    state.serialize_entry("factor", some)?;
                }
            }
            if let Some(some) = self.r#net.as_ref() {
                state.serialize_entry("net", some)?;
            }
            if !self.r#udi.is_empty() {
                state.serialize_entry("udi", &self.r#udi)?;
            }
            if _ctx.output_json {
                if !self.r#note_number.is_empty() {
                    let values = self
                        .r#note_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#note_number.is_empty() {
                    state.serialize_entry("noteNumber", &self.r#note_number)?;
                }
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitItemDetailSubDetail {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "revenue")]
            Revenue,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "programCode")]
            ProgramCode,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "udi")]
            Udi,
            #[serde(rename = "noteNumber")]
            NoteNumber,
            #[serde(rename = "_noteNumber")]
            NoteNumberPrimitiveElement,
            #[serde(rename = "adjudication")]
            Adjudication,
            Unknown(std::string::String),
        }
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
                            Field::Sequence => {
                                if _ctx.from_json {
                                    let some = r#sequence.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    r#sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::SequencePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "noteNumber",
                                            "adjudication",
                                        ],
                                    ));
                                }
                            }
                            Field::Revenue => {
                                if r#revenue.is_some() {
                                    return Err(serde::de::Error::duplicate_field("revenue"));
                                }
                                r#revenue = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::ProductOrService => {
                                if r#product_or_service.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productOrService",
                                    ));
                                }
                                r#product_or_service = Some(map_access.next_value()?);
                            }
                            Field::Modifier => {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier = Some(map_access.next_value()?);
                            }
                            Field::ProgramCode => {
                                if r#program_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("programCode"));
                                }
                                r#program_code = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::UnitPrice => {
                                if r#unit_price.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unitPrice"));
                                }
                                r#unit_price = Some(map_access.next_value()?);
                            }
                            Field::Factor => {
                                if _ctx.from_json {
                                    let some = r#factor.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#factor.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    r#factor = Some(map_access.next_value()?);
                                }
                            }
                            Field::FactorPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "factor",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "noteNumber",
                                            "adjudication",
                                        ],
                                    ));
                                }
                            }
                            Field::Net => {
                                if r#net.is_some() {
                                    return Err(serde::de::Error::duplicate_field("net"));
                                }
                                r#net = Some(map_access.next_value()?);
                            }
                            Field::Udi => {
                                if r#udi.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udi"));
                                }
                                r#udi = Some(map_access.next_value()?);
                            }
                            Field::NoteNumber => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#note_number.get_or_insert(
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
                                            "noteNumber",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#note_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "noteNumber",
                                        ));
                                    }
                                    r#note_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::NoteNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#note_number.get_or_insert(
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
                                            "_noteNumber",
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
                                        "noteNumber",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "noteNumber",
                                            "adjudication",
                                        ],
                                    ));
                                }
                            }
                            Field::Adjudication => {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                r#adjudication = Some(map_access.next_value()?);
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
                                        "sequence",
                                        "revenue",
                                        "category",
                                        "productOrService",
                                        "modifier",
                                        "programCode",
                                        "quantity",
                                        "unitPrice",
                                        "factor",
                                        "net",
                                        "udi",
                                        "noteNumber",
                                        "adjudication",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitItemDetailSubDetail {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#revenue,
                        r#category,
                        r#product_or_service: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#product_or_service.unwrap_or(Default::default())
                        } else {
                            r#product_or_service
                                .ok_or(serde::de::Error::missing_field("productOrService"))?
                        },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Second-tier of goods and services."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitItemDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A claim detail line. Either a simple (a product or service) or a 'group' of sub-details which are simple items."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The type of revenue or cost center providing the product and/or service."]
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "Third-tier of goods and services."]
    pub r#sub_detail: Vec<ExplanationOfBenefitItemDetailSubDetail>,
}
impl serde::ser::Serialize for ExplanationOfBenefitItemDetail {
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
                if let Some(some) = self.r#sequence.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sequence", &some)?;
                }
                if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#sequence.id.as_ref(),
                        extension: &self.r#sequence.extension,
                    };
                    state.serialize_entry("_sequence", &primitive_element)?;
                }
            } else {
                state.serialize_entry("sequence", &self.r#sequence)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#factor.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("factor", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_factor", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#factor.as_ref() {
                    state.serialize_entry("factor", some)?;
                }
            }
            if let Some(some) = self.r#net.as_ref() {
                state.serialize_entry("net", some)?;
            }
            if !self.r#udi.is_empty() {
                state.serialize_entry("udi", &self.r#udi)?;
            }
            if _ctx.output_json {
                if !self.r#note_number.is_empty() {
                    let values = self
                        .r#note_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#note_number.is_empty() {
                    state.serialize_entry("noteNumber", &self.r#note_number)?;
                }
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            if !self.r#sub_detail.is_empty() {
                state.serialize_entry("subDetail", &self.r#sub_detail)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitItemDetail {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "revenue")]
            Revenue,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "programCode")]
            ProgramCode,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "udi")]
            Udi,
            #[serde(rename = "noteNumber")]
            NoteNumber,
            #[serde(rename = "_noteNumber")]
            NoteNumberPrimitiveElement,
            #[serde(rename = "adjudication")]
            Adjudication,
            #[serde(rename = "subDetail")]
            SubDetail,
            Unknown(std::string::String),
        }
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
                            Field::Sequence => {
                                if _ctx.from_json {
                                    let some = r#sequence.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    r#sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::SequencePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "noteNumber",
                                            "adjudication",
                                            "subDetail",
                                        ],
                                    ));
                                }
                            }
                            Field::Revenue => {
                                if r#revenue.is_some() {
                                    return Err(serde::de::Error::duplicate_field("revenue"));
                                }
                                r#revenue = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::ProductOrService => {
                                if r#product_or_service.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productOrService",
                                    ));
                                }
                                r#product_or_service = Some(map_access.next_value()?);
                            }
                            Field::Modifier => {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier = Some(map_access.next_value()?);
                            }
                            Field::ProgramCode => {
                                if r#program_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("programCode"));
                                }
                                r#program_code = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::UnitPrice => {
                                if r#unit_price.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unitPrice"));
                                }
                                r#unit_price = Some(map_access.next_value()?);
                            }
                            Field::Factor => {
                                if _ctx.from_json {
                                    let some = r#factor.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#factor.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    r#factor = Some(map_access.next_value()?);
                                }
                            }
                            Field::FactorPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "factor",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "noteNumber",
                                            "adjudication",
                                            "subDetail",
                                        ],
                                    ));
                                }
                            }
                            Field::Net => {
                                if r#net.is_some() {
                                    return Err(serde::de::Error::duplicate_field("net"));
                                }
                                r#net = Some(map_access.next_value()?);
                            }
                            Field::Udi => {
                                if r#udi.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udi"));
                                }
                                r#udi = Some(map_access.next_value()?);
                            }
                            Field::NoteNumber => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#note_number.get_or_insert(
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
                                            "noteNumber",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#note_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "noteNumber",
                                        ));
                                    }
                                    r#note_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::NoteNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#note_number.get_or_insert(
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
                                            "_noteNumber",
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
                                        "noteNumber",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "noteNumber",
                                            "adjudication",
                                            "subDetail",
                                        ],
                                    ));
                                }
                            }
                            Field::Adjudication => {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                r#adjudication = Some(map_access.next_value()?);
                            }
                            Field::SubDetail => {
                                if r#sub_detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subDetail"));
                                }
                                r#sub_detail = Some(map_access.next_value()?);
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
                                        "sequence",
                                        "revenue",
                                        "category",
                                        "productOrService",
                                        "modifier",
                                        "programCode",
                                        "quantity",
                                        "unitPrice",
                                        "factor",
                                        "net",
                                        "udi",
                                        "noteNumber",
                                        "adjudication",
                                        "subDetail",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitItemDetail {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#revenue,
                        r#category,
                        r#product_or_service: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#product_or_service.unwrap_or(Default::default())
                        } else {
                            r#product_or_service
                                .ok_or(serde::de::Error::missing_field("productOrService"))?
                        },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A claim line. Either a simple (a product or service) or a 'group' of details which can also be a simple items or groups of sub-details."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify item entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "Care team members related to this service or product."]
    pub r#care_team_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "Diagnoses applicable for this service or product."]
    pub r#diagnosis_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "Procedures applicable for this service or product."]
    pub r#procedure_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "Exceptions, special conditions and supporting information applicable for this service or product."]
    pub r#information_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The type of revenue or cost center providing the product and/or service."]
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date or dates when the service or product was supplied, performed or completed."]
    pub r#serviced: Option<ExplanationOfBenefitItemServiced>,
    #[doc = "Where the product or service was provided."]
    pub r#location: Option<ExplanationOfBenefitItemLocation>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    #[doc = "Physical service site on the patient (limb, tooth, etc.)."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A region or surface of the bodySite, e.g. limb region or tooth surface(s)."]
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A billed item may include goods or services provided in multiple encounters."]
    pub r#encounter: Vec<Box<super::super::types::Reference>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "If this item is a group then the values here are a summary of the adjudication of the detail items. If this item is a simple product or service then this is the result of the adjudication of this item."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "Second-tier of goods and services."]
    pub r#detail: Vec<ExplanationOfBenefitItemDetail>,
}
impl serde::ser::Serialize for ExplanationOfBenefitItem {
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
                if let Some(some) = self.r#sequence.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sequence", &some)?;
                }
                if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#sequence.id.as_ref(),
                        extension: &self.r#sequence.extension,
                    };
                    state.serialize_entry("_sequence", &primitive_element)?;
                }
            } else {
                state.serialize_entry("sequence", &self.r#sequence)?;
            }
            if _ctx.output_json {
                if !self.r#care_team_sequence.is_empty() {
                    let values = self
                        .r#care_team_sequence
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#care_team_sequence.is_empty() {
                    state.serialize_entry("careTeamSequence", &self.r#care_team_sequence)?;
                }
            }
            if _ctx.output_json {
                if !self.r#diagnosis_sequence.is_empty() {
                    let values = self
                        .r#diagnosis_sequence
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#diagnosis_sequence.is_empty() {
                    state.serialize_entry("diagnosisSequence", &self.r#diagnosis_sequence)?;
                }
            }
            if _ctx.output_json {
                if !self.r#procedure_sequence.is_empty() {
                    let values = self
                        .r#procedure_sequence
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#procedure_sequence.is_empty() {
                    state.serialize_entry("procedureSequence", &self.r#procedure_sequence)?;
                }
            }
            if _ctx.output_json {
                if !self.r#information_sequence.is_empty() {
                    let values = self
                        .r#information_sequence
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#information_sequence.is_empty() {
                    state.serialize_entry("informationSequence", &self.r#information_sequence)?;
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
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("servicedDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_servicedDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("servicedDate", value)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#factor.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("factor", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_factor", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#factor.as_ref() {
                    state.serialize_entry("factor", some)?;
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
            if _ctx.output_json {
                if !self.r#note_number.is_empty() {
                    let values = self
                        .r#note_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#note_number.is_empty() {
                    state.serialize_entry("noteNumber", &self.r#note_number)?;
                }
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            if !self.r#detail.is_empty() {
                state.serialize_entry("detail", &self.r#detail)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitItem {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "careTeamSequence")]
            CareTeamSequence,
            #[serde(rename = "_careTeamSequence")]
            CareTeamSequencePrimitiveElement,
            #[serde(rename = "diagnosisSequence")]
            DiagnosisSequence,
            #[serde(rename = "_diagnosisSequence")]
            DiagnosisSequencePrimitiveElement,
            #[serde(rename = "procedureSequence")]
            ProcedureSequence,
            #[serde(rename = "_procedureSequence")]
            ProcedureSequencePrimitiveElement,
            #[serde(rename = "informationSequence")]
            InformationSequence,
            #[serde(rename = "_informationSequence")]
            InformationSequencePrimitiveElement,
            #[serde(rename = "revenue")]
            Revenue,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "programCode")]
            ProgramCode,
            #[serde(rename = "servicedDate")]
            ServicedDate,
            #[serde(rename = "_servicedDate")]
            ServicedDatePrimitiveElement,
            #[serde(rename = "servicedPeriod")]
            ServicedPeriod,
            #[serde(rename = "locationCodeableConcept")]
            LocationCodeableConcept,
            #[serde(rename = "locationAddress")]
            LocationAddress,
            #[serde(rename = "locationReference")]
            LocationReference,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "udi")]
            Udi,
            #[serde(rename = "bodySite")]
            BodySite,
            #[serde(rename = "subSite")]
            SubSite,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "noteNumber")]
            NoteNumber,
            #[serde(rename = "_noteNumber")]
            NoteNumberPrimitiveElement,
            #[serde(rename = "adjudication")]
            Adjudication,
            #[serde(rename = "detail")]
            Detail,
            Unknown(std::string::String),
        }
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
                            Field::Sequence => {
                                if _ctx.from_json {
                                    let some = r#sequence.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sequence"));
                                    }
                                    r#sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::SequencePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "careTeamSequence",
                                            "diagnosisSequence",
                                            "procedureSequence",
                                            "informationSequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "bodySite",
                                            "subSite",
                                            "encounter",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::CareTeamSequence => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#care_team_sequence.get_or_insert(
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
                                            "careTeamSequence",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#care_team_sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "careTeamSequence",
                                        ));
                                    }
                                    r#care_team_sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::CareTeamSequencePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#care_team_sequence.get_or_insert(
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
                                            "_careTeamSequence",
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
                                        "careTeamSequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "careTeamSequence",
                                            "diagnosisSequence",
                                            "procedureSequence",
                                            "informationSequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "bodySite",
                                            "subSite",
                                            "encounter",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::DiagnosisSequence => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#diagnosis_sequence.get_or_insert(
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
                                            "diagnosisSequence",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#diagnosis_sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "diagnosisSequence",
                                        ));
                                    }
                                    r#diagnosis_sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::DiagnosisSequencePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#diagnosis_sequence.get_or_insert(
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
                                            "_diagnosisSequence",
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
                                        "diagnosisSequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "careTeamSequence",
                                            "diagnosisSequence",
                                            "procedureSequence",
                                            "informationSequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "bodySite",
                                            "subSite",
                                            "encounter",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::ProcedureSequence => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#procedure_sequence.get_or_insert(
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
                                            "procedureSequence",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#procedure_sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "procedureSequence",
                                        ));
                                    }
                                    r#procedure_sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::ProcedureSequencePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#procedure_sequence.get_or_insert(
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
                                            "_procedureSequence",
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
                                        "procedureSequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "careTeamSequence",
                                            "diagnosisSequence",
                                            "procedureSequence",
                                            "informationSequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "bodySite",
                                            "subSite",
                                            "encounter",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::InformationSequence => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#information_sequence.get_or_insert(
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
                                            "informationSequence",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#information_sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "informationSequence",
                                        ));
                                    }
                                    r#information_sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::InformationSequencePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#information_sequence.get_or_insert(
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
                                            "_informationSequence",
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
                                        "informationSequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "careTeamSequence",
                                            "diagnosisSequence",
                                            "procedureSequence",
                                            "informationSequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "bodySite",
                                            "subSite",
                                            "encounter",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::Revenue => {
                                if r#revenue.is_some() {
                                    return Err(serde::de::Error::duplicate_field("revenue"));
                                }
                                r#revenue = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::ProductOrService => {
                                if r#product_or_service.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productOrService",
                                    ));
                                }
                                r#product_or_service = Some(map_access.next_value()?);
                            }
                            Field::Modifier => {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier = Some(map_access.next_value()?);
                            }
                            Field::ProgramCode => {
                                if r#program_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("programCode"));
                                }
                                r#program_code = Some(map_access.next_value()?);
                            }
                            Field::ServicedDate => {
                                if _ctx.from_json {
                                    let r#enum = r#serviced.get_or_insert(
                                        ExplanationOfBenefitItemServiced::Date(Default::default()),
                                    );
                                    if let ExplanationOfBenefitItemServiced::Date(variant) = r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "servicedDate",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "serviced[x]",
                                        ));
                                    }
                                } else {
                                    if r#serviced.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "servicedDate",
                                        ));
                                    }
                                    r#serviced = Some(ExplanationOfBenefitItemServiced::Date(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ServicedDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#serviced.get_or_insert(
                                        ExplanationOfBenefitItemServiced::Date(Default::default()),
                                    );
                                    if let ExplanationOfBenefitItemServiced::Date(variant) = r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_servicedDate",
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
                                            "_serviced[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "servicedDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "careTeamSequence",
                                            "diagnosisSequence",
                                            "procedureSequence",
                                            "informationSequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "bodySite",
                                            "subSite",
                                            "encounter",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::ServicedPeriod => {
                                if r#serviced.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "servicedPeriod",
                                    ));
                                }
                                r#serviced = Some(ExplanationOfBenefitItemServiced::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::LocationCodeableConcept => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "locationCodeableConcept",
                                    ));
                                }
                                r#location =
                                    Some(ExplanationOfBenefitItemLocation::CodeableConcept(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::LocationAddress => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "locationAddress",
                                    ));
                                }
                                r#location = Some(ExplanationOfBenefitItemLocation::Address(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::LocationReference => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "locationReference",
                                    ));
                                }
                                r#location = Some(ExplanationOfBenefitItemLocation::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::UnitPrice => {
                                if r#unit_price.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unitPrice"));
                                }
                                r#unit_price = Some(map_access.next_value()?);
                            }
                            Field::Factor => {
                                if _ctx.from_json {
                                    let some = r#factor.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#factor.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    r#factor = Some(map_access.next_value()?);
                                }
                            }
                            Field::FactorPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "factor",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "careTeamSequence",
                                            "diagnosisSequence",
                                            "procedureSequence",
                                            "informationSequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "bodySite",
                                            "subSite",
                                            "encounter",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::Net => {
                                if r#net.is_some() {
                                    return Err(serde::de::Error::duplicate_field("net"));
                                }
                                r#net = Some(map_access.next_value()?);
                            }
                            Field::Udi => {
                                if r#udi.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udi"));
                                }
                                r#udi = Some(map_access.next_value()?);
                            }
                            Field::BodySite => {
                                if r#body_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bodySite"));
                                }
                                r#body_site = Some(map_access.next_value()?);
                            }
                            Field::SubSite => {
                                if r#sub_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subSite"));
                                }
                                r#sub_site = Some(map_access.next_value()?);
                            }
                            Field::Encounter => {
                                if r#encounter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("encounter"));
                                }
                                r#encounter = Some(map_access.next_value()?);
                            }
                            Field::NoteNumber => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#note_number.get_or_insert(
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
                                            "noteNumber",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#note_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "noteNumber",
                                        ));
                                    }
                                    r#note_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::NoteNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#note_number.get_or_insert(
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
                                            "_noteNumber",
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
                                        "noteNumber",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "careTeamSequence",
                                            "diagnosisSequence",
                                            "procedureSequence",
                                            "informationSequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "bodySite",
                                            "subSite",
                                            "encounter",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::Adjudication => {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                r#adjudication = Some(map_access.next_value()?);
                            }
                            Field::Detail => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detail"));
                                }
                                r#detail = Some(map_access.next_value()?);
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
                                        "sequence",
                                        "careTeamSequence",
                                        "diagnosisSequence",
                                        "procedureSequence",
                                        "informationSequence",
                                        "revenue",
                                        "category",
                                        "productOrService",
                                        "modifier",
                                        "programCode",
                                        "servicedDate",
                                        "servicedPeriod",
                                        "locationCodeableConcept",
                                        "locationAddress",
                                        "locationReference",
                                        "quantity",
                                        "unitPrice",
                                        "factor",
                                        "net",
                                        "udi",
                                        "bodySite",
                                        "subSite",
                                        "encounter",
                                        "noteNumber",
                                        "adjudication",
                                        "detail",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitItem {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#care_team_sequence: r#care_team_sequence.unwrap_or(vec![]),
                        r#diagnosis_sequence: r#diagnosis_sequence.unwrap_or(vec![]),
                        r#procedure_sequence: r#procedure_sequence.unwrap_or(vec![]),
                        r#information_sequence: r#information_sequence.unwrap_or(vec![]),
                        r#revenue,
                        r#category,
                        r#product_or_service: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#product_or_service.unwrap_or(Default::default())
                        } else {
                            r#product_or_service
                                .ok_or(serde::de::Error::missing_field("productOrService"))?
                        },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The third-tier service adjudications for payor added services."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitAddItemDetailSubDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}
impl serde::ser::Serialize for ExplanationOfBenefitAddItemDetailSubDetail {
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
            if _ctx.output_json {
                if let Some(some) = self.r#factor.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("factor", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_factor", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#factor.as_ref() {
                    state.serialize_entry("factor", some)?;
                }
            }
            if let Some(some) = self.r#net.as_ref() {
                state.serialize_entry("net", some)?;
            }
            if _ctx.output_json {
                if !self.r#note_number.is_empty() {
                    let values = self
                        .r#note_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#note_number.is_empty() {
                    state.serialize_entry("noteNumber", &self.r#note_number)?;
                }
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitAddItemDetailSubDetail {
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
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "noteNumber")]
            NoteNumber,
            #[serde(rename = "_noteNumber")]
            NoteNumberPrimitiveElement,
            #[serde(rename = "adjudication")]
            Adjudication,
            Unknown(std::string::String),
        }
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
                            Field::ProductOrService => {
                                if r#product_or_service.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productOrService",
                                    ));
                                }
                                r#product_or_service = Some(map_access.next_value()?);
                            }
                            Field::Modifier => {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::UnitPrice => {
                                if r#unit_price.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unitPrice"));
                                }
                                r#unit_price = Some(map_access.next_value()?);
                            }
                            Field::Factor => {
                                if _ctx.from_json {
                                    let some = r#factor.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#factor.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    r#factor = Some(map_access.next_value()?);
                                }
                            }
                            Field::FactorPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "factor",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "productOrService",
                                            "modifier",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "noteNumber",
                                            "adjudication",
                                        ],
                                    ));
                                }
                            }
                            Field::Net => {
                                if r#net.is_some() {
                                    return Err(serde::de::Error::duplicate_field("net"));
                                }
                                r#net = Some(map_access.next_value()?);
                            }
                            Field::NoteNumber => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#note_number.get_or_insert(
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
                                            "noteNumber",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#note_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "noteNumber",
                                        ));
                                    }
                                    r#note_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::NoteNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#note_number.get_or_insert(
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
                                            "_noteNumber",
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
                                        "noteNumber",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "productOrService",
                                            "modifier",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "noteNumber",
                                            "adjudication",
                                        ],
                                    ));
                                }
                            }
                            Field::Adjudication => {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                r#adjudication = Some(map_access.next_value()?);
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
                                        "productOrService",
                                        "modifier",
                                        "quantity",
                                        "unitPrice",
                                        "factor",
                                        "net",
                                        "noteNumber",
                                        "adjudication",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitAddItemDetailSubDetail {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#product_or_service: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#product_or_service.unwrap_or(Default::default())
                        } else {
                            r#product_or_service
                                .ok_or(serde::de::Error::missing_field("productOrService"))?
                        },
                        r#modifier: r#modifier.unwrap_or(vec![]),
                        r#quantity,
                        r#unit_price,
                        r#factor,
                        r#net,
                        r#note_number: r#note_number.unwrap_or(vec![]),
                        r#adjudication: r#adjudication.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The second-tier service adjudications for payor added services."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitAddItemDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "The third-tier service adjudications for payor added services."]
    pub r#sub_detail: Vec<ExplanationOfBenefitAddItemDetailSubDetail>,
}
impl serde::ser::Serialize for ExplanationOfBenefitAddItemDetail {
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
            if _ctx.output_json {
                if let Some(some) = self.r#factor.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("factor", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_factor", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#factor.as_ref() {
                    state.serialize_entry("factor", some)?;
                }
            }
            if let Some(some) = self.r#net.as_ref() {
                state.serialize_entry("net", some)?;
            }
            if _ctx.output_json {
                if !self.r#note_number.is_empty() {
                    let values = self
                        .r#note_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#note_number.is_empty() {
                    state.serialize_entry("noteNumber", &self.r#note_number)?;
                }
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            if !self.r#sub_detail.is_empty() {
                state.serialize_entry("subDetail", &self.r#sub_detail)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitAddItemDetail {
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
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "noteNumber")]
            NoteNumber,
            #[serde(rename = "_noteNumber")]
            NoteNumberPrimitiveElement,
            #[serde(rename = "adjudication")]
            Adjudication,
            #[serde(rename = "subDetail")]
            SubDetail,
            Unknown(std::string::String),
        }
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
                            Field::ProductOrService => {
                                if r#product_or_service.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productOrService",
                                    ));
                                }
                                r#product_or_service = Some(map_access.next_value()?);
                            }
                            Field::Modifier => {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::UnitPrice => {
                                if r#unit_price.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unitPrice"));
                                }
                                r#unit_price = Some(map_access.next_value()?);
                            }
                            Field::Factor => {
                                if _ctx.from_json {
                                    let some = r#factor.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#factor.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    r#factor = Some(map_access.next_value()?);
                                }
                            }
                            Field::FactorPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "factor",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "productOrService",
                                            "modifier",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "noteNumber",
                                            "adjudication",
                                            "subDetail",
                                        ],
                                    ));
                                }
                            }
                            Field::Net => {
                                if r#net.is_some() {
                                    return Err(serde::de::Error::duplicate_field("net"));
                                }
                                r#net = Some(map_access.next_value()?);
                            }
                            Field::NoteNumber => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#note_number.get_or_insert(
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
                                            "noteNumber",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#note_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "noteNumber",
                                        ));
                                    }
                                    r#note_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::NoteNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#note_number.get_or_insert(
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
                                            "_noteNumber",
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
                                        "noteNumber",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "productOrService",
                                            "modifier",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "noteNumber",
                                            "adjudication",
                                            "subDetail",
                                        ],
                                    ));
                                }
                            }
                            Field::Adjudication => {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                r#adjudication = Some(map_access.next_value()?);
                            }
                            Field::SubDetail => {
                                if r#sub_detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subDetail"));
                                }
                                r#sub_detail = Some(map_access.next_value()?);
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
                                        "productOrService",
                                        "modifier",
                                        "quantity",
                                        "unitPrice",
                                        "factor",
                                        "net",
                                        "noteNumber",
                                        "adjudication",
                                        "subDetail",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitAddItemDetail {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#product_or_service: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#product_or_service.unwrap_or(Default::default())
                        } else {
                            r#product_or_service
                                .ok_or(serde::de::Error::missing_field("productOrService"))?
                        },
                        r#modifier: r#modifier.unwrap_or(vec![]),
                        r#quantity,
                        r#unit_price,
                        r#factor,
                        r#net,
                        r#note_number: r#note_number.unwrap_or(vec![]),
                        r#adjudication: r#adjudication.unwrap_or(vec![]),
                        r#sub_detail: r#sub_detail.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The first-tier service adjudications for payor added product or service lines."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitAddItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Claim items which this service line is intended to replace."]
    pub r#item_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The sequence number of the details within the claim item which this line is intended to replace."]
    pub r#detail_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The sequence number of the sub-details woithin the details within the claim item which this line is intended to replace."]
    pub r#sub_detail_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The providers who are authorized for the services rendered to the patient."]
    pub r#provider: Vec<Box<super::super::types::Reference>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date or dates when the service or product was supplied, performed or completed."]
    pub r#serviced: Option<ExplanationOfBenefitAddItemServiced>,
    #[doc = "Where the product or service was provided."]
    pub r#location: Option<ExplanationOfBenefitAddItemLocation>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Physical service site on the patient (limb, tooth, etc.)."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A region or surface of the bodySite, e.g. limb region or tooth surface(s)."]
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The numbers associated with notes below which apply to the adjudication of this item."]
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    #[doc = "The adjudication results."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "The second-tier service adjudications for payor added services."]
    pub r#detail: Vec<ExplanationOfBenefitAddItemDetail>,
}
impl serde::ser::Serialize for ExplanationOfBenefitAddItem {
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
                if !self.r#item_sequence.is_empty() {
                    let values = self
                        .r#item_sequence
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#item_sequence.is_empty() {
                    state.serialize_entry("itemSequence", &self.r#item_sequence)?;
                }
            }
            if _ctx.output_json {
                if !self.r#detail_sequence.is_empty() {
                    let values = self
                        .r#detail_sequence
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#detail_sequence.is_empty() {
                    state.serialize_entry("detailSequence", &self.r#detail_sequence)?;
                }
            }
            if _ctx.output_json {
                if !self.r#sub_detail_sequence.is_empty() {
                    let values = self
                        .r#sub_detail_sequence
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#sub_detail_sequence.is_empty() {
                    state.serialize_entry("subDetailSequence", &self.r#sub_detail_sequence)?;
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
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("servicedDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_servicedDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("servicedDate", value)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#factor.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("factor", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_factor", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#factor.as_ref() {
                    state.serialize_entry("factor", some)?;
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
            if _ctx.output_json {
                if !self.r#note_number.is_empty() {
                    let values = self
                        .r#note_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#note_number.is_empty() {
                    state.serialize_entry("noteNumber", &self.r#note_number)?;
                }
            }
            if !self.r#adjudication.is_empty() {
                state.serialize_entry("adjudication", &self.r#adjudication)?;
            }
            if !self.r#detail.is_empty() {
                state.serialize_entry("detail", &self.r#detail)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitAddItem {
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
            #[serde(rename = "itemSequence")]
            ItemSequence,
            #[serde(rename = "_itemSequence")]
            ItemSequencePrimitiveElement,
            #[serde(rename = "detailSequence")]
            DetailSequence,
            #[serde(rename = "_detailSequence")]
            DetailSequencePrimitiveElement,
            #[serde(rename = "subDetailSequence")]
            SubDetailSequence,
            #[serde(rename = "_subDetailSequence")]
            SubDetailSequencePrimitiveElement,
            #[serde(rename = "provider")]
            Provider,
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "programCode")]
            ProgramCode,
            #[serde(rename = "servicedDate")]
            ServicedDate,
            #[serde(rename = "_servicedDate")]
            ServicedDatePrimitiveElement,
            #[serde(rename = "servicedPeriod")]
            ServicedPeriod,
            #[serde(rename = "locationCodeableConcept")]
            LocationCodeableConcept,
            #[serde(rename = "locationAddress")]
            LocationAddress,
            #[serde(rename = "locationReference")]
            LocationReference,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "bodySite")]
            BodySite,
            #[serde(rename = "subSite")]
            SubSite,
            #[serde(rename = "noteNumber")]
            NoteNumber,
            #[serde(rename = "_noteNumber")]
            NoteNumberPrimitiveElement,
            #[serde(rename = "adjudication")]
            Adjudication,
            #[serde(rename = "detail")]
            Detail,
            Unknown(std::string::String),
        }
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
                            Field::ItemSequence => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#item_sequence.get_or_insert(
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
                                            "itemSequence",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#item_sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "itemSequence",
                                        ));
                                    }
                                    r#item_sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::ItemSequencePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#item_sequence.get_or_insert(
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
                                            "_itemSequence",
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
                                        "itemSequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "itemSequence",
                                            "detailSequence",
                                            "subDetailSequence",
                                            "provider",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "bodySite",
                                            "subSite",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::DetailSequence => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#detail_sequence.get_or_insert(
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
                                            "detailSequence",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#detail_sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "detailSequence",
                                        ));
                                    }
                                    r#detail_sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::DetailSequencePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#detail_sequence.get_or_insert(
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
                                            "_detailSequence",
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
                                        "detailSequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "itemSequence",
                                            "detailSequence",
                                            "subDetailSequence",
                                            "provider",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "bodySite",
                                            "subSite",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::SubDetailSequence => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#sub_detail_sequence.get_or_insert(
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
                                            "subDetailSequence",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#sub_detail_sequence.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "subDetailSequence",
                                        ));
                                    }
                                    r#sub_detail_sequence = Some(map_access.next_value()?);
                                }
                            }
                            Field::SubDetailSequencePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#sub_detail_sequence.get_or_insert(
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
                                            "_subDetailSequence",
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
                                        "subDetailSequence",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "itemSequence",
                                            "detailSequence",
                                            "subDetailSequence",
                                            "provider",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "bodySite",
                                            "subSite",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::Provider => {
                                if r#provider.is_some() {
                                    return Err(serde::de::Error::duplicate_field("provider"));
                                }
                                r#provider = Some(map_access.next_value()?);
                            }
                            Field::ProductOrService => {
                                if r#product_or_service.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productOrService",
                                    ));
                                }
                                r#product_or_service = Some(map_access.next_value()?);
                            }
                            Field::Modifier => {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier = Some(map_access.next_value()?);
                            }
                            Field::ProgramCode => {
                                if r#program_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("programCode"));
                                }
                                r#program_code = Some(map_access.next_value()?);
                            }
                            Field::ServicedDate => {
                                if _ctx.from_json {
                                    let r#enum = r#serviced.get_or_insert(
                                        ExplanationOfBenefitAddItemServiced::Date(
                                            Default::default(),
                                        ),
                                    );
                                    if let ExplanationOfBenefitAddItemServiced::Date(variant) =
                                        r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "servicedDate",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "serviced[x]",
                                        ));
                                    }
                                } else {
                                    if r#serviced.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "servicedDate",
                                        ));
                                    }
                                    r#serviced = Some(ExplanationOfBenefitAddItemServiced::Date(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ServicedDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#serviced.get_or_insert(
                                        ExplanationOfBenefitAddItemServiced::Date(
                                            Default::default(),
                                        ),
                                    );
                                    if let ExplanationOfBenefitAddItemServiced::Date(variant) =
                                        r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_servicedDate",
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
                                            "_serviced[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "servicedDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "itemSequence",
                                            "detailSequence",
                                            "subDetailSequence",
                                            "provider",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "bodySite",
                                            "subSite",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::ServicedPeriod => {
                                if r#serviced.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "servicedPeriod",
                                    ));
                                }
                                r#serviced = Some(ExplanationOfBenefitAddItemServiced::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::LocationCodeableConcept => {
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
                            Field::LocationAddress => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "locationAddress",
                                    ));
                                }
                                r#location = Some(ExplanationOfBenefitAddItemLocation::Address(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::LocationReference => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "locationReference",
                                    ));
                                }
                                r#location = Some(ExplanationOfBenefitAddItemLocation::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::UnitPrice => {
                                if r#unit_price.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unitPrice"));
                                }
                                r#unit_price = Some(map_access.next_value()?);
                            }
                            Field::Factor => {
                                if _ctx.from_json {
                                    let some = r#factor.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#factor.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    r#factor = Some(map_access.next_value()?);
                                }
                            }
                            Field::FactorPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "factor",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "itemSequence",
                                            "detailSequence",
                                            "subDetailSequence",
                                            "provider",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "bodySite",
                                            "subSite",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::Net => {
                                if r#net.is_some() {
                                    return Err(serde::de::Error::duplicate_field("net"));
                                }
                                r#net = Some(map_access.next_value()?);
                            }
                            Field::BodySite => {
                                if r#body_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bodySite"));
                                }
                                r#body_site = Some(map_access.next_value()?);
                            }
                            Field::SubSite => {
                                if r#sub_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subSite"));
                                }
                                r#sub_site = Some(map_access.next_value()?);
                            }
                            Field::NoteNumber => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#note_number.get_or_insert(
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
                                            "noteNumber",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#note_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "noteNumber",
                                        ));
                                    }
                                    r#note_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::NoteNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#note_number.get_or_insert(
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
                                            "_noteNumber",
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
                                        "noteNumber",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "itemSequence",
                                            "detailSequence",
                                            "subDetailSequence",
                                            "provider",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "bodySite",
                                            "subSite",
                                            "noteNumber",
                                            "adjudication",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                            Field::Adjudication => {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                r#adjudication = Some(map_access.next_value()?);
                            }
                            Field::Detail => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detail"));
                                }
                                r#detail = Some(map_access.next_value()?);
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
                                        "itemSequence",
                                        "detailSequence",
                                        "subDetailSequence",
                                        "provider",
                                        "productOrService",
                                        "modifier",
                                        "programCode",
                                        "servicedDate",
                                        "servicedPeriod",
                                        "locationCodeableConcept",
                                        "locationAddress",
                                        "locationReference",
                                        "quantity",
                                        "unitPrice",
                                        "factor",
                                        "net",
                                        "bodySite",
                                        "subSite",
                                        "noteNumber",
                                        "adjudication",
                                        "detail",
                                    ],
                                ));
                            },
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
                        r#product_or_service: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#product_or_service.unwrap_or(Default::default())
                        } else {
                            r#product_or_service
                                .ok_or(serde::de::Error::missing_field("productOrService"))?
                        },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Categorized monetary totals for the adjudication."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitTotal {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code to indicate the information type of this adjudication record. Information types may include: the value submitted, maximum values or percentages allowed or payable under the plan, amounts that the patient is responsible for in aggregate or pertaining to this item, amounts paid by other coverages, and the benefit payable for this item."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "Monetary total amount associated with the category."]
    pub r#amount: Box<super::super::types::Money>,
}
impl serde::ser::Serialize for ExplanationOfBenefitTotal {
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
            state.serialize_entry("category", &self.r#category)?;
            state.serialize_entry("amount", &self.r#amount)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitTotal {
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
            #[serde(rename = "amount")]
            Amount,
            Unknown(std::string::String),
        }
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
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
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
                                    &["id", "extension", "modifierExtension", "category", "amount"],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitTotal {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#category: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#category.unwrap_or(Default::default())
                        } else {
                            r#category.ok_or(serde::de::Error::missing_field("category"))?
                        },
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
#[doc = "Payment details for the adjudication of the claim."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitPayment {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Whether this represents partial or complete payment of the benefits payable."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Total amount of all adjustments to this payment included in this transaction which are not related to this claim's adjudication."]
    pub r#adjustment: Option<Box<super::super::types::Money>>,
    #[doc = "Reason for the payment adjustment."]
    pub r#adjustment_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Estimated date the payment will be issued or the actual issue date of payment."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "Benefits payable less any payment adjustment."]
    pub r#amount: Option<Box<super::super::types::Money>>,
    #[doc = "Issuer's unique identifier for the payment instrument."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitPayment {
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
            if let Some(some) = self.r#adjustment.as_ref() {
                state.serialize_entry("adjustment", some)?;
            }
            if let Some(some) = self.r#adjustment_reason.as_ref() {
                state.serialize_entry("adjustmentReason", some)?;
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
            if let Some(some) = self.r#amount.as_ref() {
                state.serialize_entry("amount", some)?;
            }
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitPayment {
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
            #[serde(rename = "adjustment")]
            Adjustment,
            #[serde(rename = "adjustmentReason")]
            AdjustmentReason,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "amount")]
            Amount,
            #[serde(rename = "identifier")]
            Identifier,
            Unknown(std::string::String),
        }
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
                            Field::Adjustment => {
                                if r#adjustment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjustment"));
                                }
                                r#adjustment = Some(map_access.next_value()?);
                            }
                            Field::AdjustmentReason => {
                                if r#adjustment_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "adjustmentReason",
                                    ));
                                }
                                r#adjustment_reason = Some(map_access.next_value()?);
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
                                            "type",
                                            "adjustment",
                                            "adjustmentReason",
                                            "date",
                                            "amount",
                                            "identifier",
                                        ],
                                    ));
                                }
                            }
                            Field::Amount => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amount"));
                                }
                                r#amount = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
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
                                        "adjustment",
                                        "adjustmentReason",
                                        "date",
                                        "amount",
                                        "identifier",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A note that describes or explains adjudication results in a human readable form."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitProcessNote {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify a note entry."]
    pub r#number: Option<super::super::types::PositiveInt>,
    #[doc = "The business purpose of the note text."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "The explanation or description associated with the processing."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "A code to define the language used in the text of the note."]
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ExplanationOfBenefitProcessNote {
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
                if let Some(some) = self.r#number.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("number", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_number", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number.as_ref() {
                    state.serialize_entry("number", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("type", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_type", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#type.as_ref() {
                    state.serialize_entry("type", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            if let Some(some) = self.r#language.as_ref() {
                state.serialize_entry("language", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitProcessNote {
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
            #[serde(rename = "number")]
            Number,
            #[serde(rename = "_number")]
            NumberPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            #[serde(rename = "language")]
            Language,
            Unknown(std::string::String),
        }
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
                            Field::Number => {
                                if _ctx.from_json {
                                    let some = r#number.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("number"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#number.is_some() {
                                        return Err(serde::de::Error::duplicate_field("number"));
                                    }
                                    r#number = Some(map_access.next_value()?);
                                }
                            }
                            Field::NumberPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "number",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "number",
                                            "type",
                                            "text",
                                            "language",
                                        ],
                                    ));
                                }
                            }
                            Field::Type => {
                                if _ctx.from_json {
                                    let some = r#type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    r#type = Some(map_access.next_value()?);
                                }
                            }
                            Field::TypePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "type",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "number",
                                            "type",
                                            "text",
                                            "language",
                                        ],
                                    ));
                                }
                            }
                            Field::Text => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    r#text = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "text",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "number",
                                            "type",
                                            "text",
                                            "language",
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
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "number",
                                        "type",
                                        "text",
                                        "language",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Benefits Used to date."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitBenefitBalanceFinancial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Classification of benefit being provided."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The quantity of the benefit which is permitted under the coverage."]
    pub r#allowed: Option<ExplanationOfBenefitBenefitBalanceFinancialAllowed>,
    #[doc = "The quantity of the benefit which have been consumed to date."]
    pub r#used: Option<ExplanationOfBenefitBenefitBalanceFinancialUsed>,
}
impl serde::ser::Serialize for ExplanationOfBenefitBenefitBalanceFinancial {
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
            if let Some(some) = self.r#allowed.as_ref() {
                match some {
                    ExplanationOfBenefitBenefitBalanceFinancialAllowed::UnsignedInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("allowedUnsignedInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_allowedUnsignedInt", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("allowedUnsignedInt", value)?;
                        }
                    }
                    ExplanationOfBenefitBenefitBalanceFinancialAllowed::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("allowedString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_allowedString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("allowedString", value)?;
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
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("usedUnsignedInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_usedUnsignedInt", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("usedUnsignedInt", value)?;
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
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitBenefitBalanceFinancial {
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
            #[serde(rename = "allowedUnsignedInt")]
            AllowedUnsignedInt,
            #[serde(rename = "_allowedUnsignedInt")]
            AllowedUnsignedIntPrimitiveElement,
            #[serde(rename = "allowedString")]
            AllowedString,
            #[serde(rename = "_allowedString")]
            AllowedStringPrimitiveElement,
            #[serde(rename = "allowedMoney")]
            AllowedMoney,
            #[serde(rename = "usedUnsignedInt")]
            UsedUnsignedInt,
            #[serde(rename = "_usedUnsignedInt")]
            UsedUnsignedIntPrimitiveElement,
            #[serde(rename = "usedMoney")]
            UsedMoney,
            Unknown(std::string::String),
        }
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
                fhirbolt_shared :: serde_context :: de :: DESERIALIZATION_CONTEXT . with (| _ctx | { let _ctx = _ctx . borrow () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } , Field :: ModifierExtension => { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } , Field :: Type => { if r#type . is_some () { return Err (serde :: de :: Error :: duplicate_field ("type")) ; } r#type = Some (map_access . next_value () ?) ; } , Field :: AllowedUnsignedInt => { if _ctx . from_json { let r#enum = r#allowed . get_or_insert (ExplanationOfBenefitBenefitBalanceFinancialAllowed :: UnsignedInt (Default :: default ())) ; if let ExplanationOfBenefitBenefitBalanceFinancialAllowed :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("allowedUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("allowed[x]")) ; } } else { if r#allowed . is_some () { return Err (serde :: de :: Error :: duplicate_field ("allowedUnsignedInt")) ; } r#allowed = Some (ExplanationOfBenefitBenefitBalanceFinancialAllowed :: UnsignedInt (map_access . next_value () ?)) ; } } , Field :: AllowedUnsignedIntPrimitiveElement => { if _ctx . from_json { let r#enum = r#allowed . get_or_insert (ExplanationOfBenefitBenefitBalanceFinancialAllowed :: UnsignedInt (Default :: default ())) ; if let ExplanationOfBenefitBenefitBalanceFinancialAllowed :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_allowedUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_allowed[x]")) ; } } else { return Err (serde :: de :: Error :: unknown_field ("allowedUnsignedInt" , & ["id" , "extension" , "modifierExtension" , "type" , "allowedUnsignedInt" , "allowedString" , "allowedMoney" , "usedUnsignedInt" , "usedMoney" ,])) ; } } , Field :: AllowedString => { if _ctx . from_json { let r#enum = r#allowed . get_or_insert (ExplanationOfBenefitBenefitBalanceFinancialAllowed :: String (Default :: default ())) ; if let ExplanationOfBenefitBenefitBalanceFinancialAllowed :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("allowedString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("allowed[x]")) ; } } else { if r#allowed . is_some () { return Err (serde :: de :: Error :: duplicate_field ("allowedString")) ; } r#allowed = Some (ExplanationOfBenefitBenefitBalanceFinancialAllowed :: String (map_access . next_value () ?)) ; } } , Field :: AllowedStringPrimitiveElement => { if _ctx . from_json { let r#enum = r#allowed . get_or_insert (ExplanationOfBenefitBenefitBalanceFinancialAllowed :: String (Default :: default ())) ; if let ExplanationOfBenefitBenefitBalanceFinancialAllowed :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_allowedString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_allowed[x]")) ; } } else { return Err (serde :: de :: Error :: unknown_field ("allowedString" , & ["id" , "extension" , "modifierExtension" , "type" , "allowedUnsignedInt" , "allowedString" , "allowedMoney" , "usedUnsignedInt" , "usedMoney" ,])) ; } } , Field :: AllowedMoney => { if r#allowed . is_some () { return Err (serde :: de :: Error :: duplicate_field ("allowedMoney")) ; } r#allowed = Some (ExplanationOfBenefitBenefitBalanceFinancialAllowed :: Money (map_access . next_value () ?)) ; } , Field :: UsedUnsignedInt => { if _ctx . from_json { let r#enum = r#used . get_or_insert (ExplanationOfBenefitBenefitBalanceFinancialUsed :: UnsignedInt (Default :: default ())) ; if let ExplanationOfBenefitBenefitBalanceFinancialUsed :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("usedUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("used[x]")) ; } } else { if r#used . is_some () { return Err (serde :: de :: Error :: duplicate_field ("usedUnsignedInt")) ; } r#used = Some (ExplanationOfBenefitBenefitBalanceFinancialUsed :: UnsignedInt (map_access . next_value () ?)) ; } } , Field :: UsedUnsignedIntPrimitiveElement => { if _ctx . from_json { let r#enum = r#used . get_or_insert (ExplanationOfBenefitBenefitBalanceFinancialUsed :: UnsignedInt (Default :: default ())) ; if let ExplanationOfBenefitBenefitBalanceFinancialUsed :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_usedUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_used[x]")) ; } } else { return Err (serde :: de :: Error :: unknown_field ("usedUnsignedInt" , & ["id" , "extension" , "modifierExtension" , "type" , "allowedUnsignedInt" , "allowedString" , "allowedMoney" , "usedUnsignedInt" , "usedMoney" ,])) ; } } , Field :: UsedMoney => { if r#used . is_some () { return Err (serde :: de :: Error :: duplicate_field ("usedMoney")) ; } r#used = Some (ExplanationOfBenefitBenefitBalanceFinancialUsed :: Money (map_access . next_value () ?)) ; } , Field :: Unknown (key) => if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "type" , "allowedUnsignedInt" , "allowedString" , "allowedMoney" , "usedUnsignedInt" , "usedMoney" ,])) ; } } } Ok (ExplanationOfBenefitBenefitBalanceFinancial { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#type : if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Lax { r#type . unwrap_or (Default :: default ()) } else { r#type . ok_or (serde :: de :: Error :: missing_field ("type")) ? } , r#allowed , r#used , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Balance by Benefit Category."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefitBenefitBalance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "True if the indicated class of service is excluded from the plan, missing or False indicates the product or service is included in the coverage."]
    pub r#excluded: Option<super::super::types::Boolean>,
    #[doc = "A short name or tag for the benefit."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A richer description of the benefit or services covered."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Is a flag to indicate whether the benefits refer to in-network providers or out-of-network providers."]
    pub r#network: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates if the benefits apply to an individual or to the family."]
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The term or period of the values such as 'maximum lifetime benefit' or 'maximum annual visits'."]
    pub r#term: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Benefits Used to date."]
    pub r#financial: Vec<ExplanationOfBenefitBenefitBalanceFinancial>,
}
impl serde::ser::Serialize for ExplanationOfBenefitBenefitBalance {
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
            state.serialize_entry("category", &self.r#category)?;
            if _ctx.output_json {
                if let Some(some) = self.r#excluded.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("excluded", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_excluded", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#excluded.as_ref() {
                    state.serialize_entry("excluded", some)?;
                }
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
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefitBenefitBalance {
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
            #[serde(rename = "excluded")]
            Excluded,
            #[serde(rename = "_excluded")]
            ExcludedPrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "network")]
            Network,
            #[serde(rename = "unit")]
            Unit,
            #[serde(rename = "term")]
            Term,
            #[serde(rename = "financial")]
            Financial,
            Unknown(std::string::String),
        }
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
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Excluded => {
                                if _ctx.from_json {
                                    let some = r#excluded.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("excluded"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#excluded.is_some() {
                                        return Err(serde::de::Error::duplicate_field("excluded"));
                                    }
                                    r#excluded = Some(map_access.next_value()?);
                                }
                            }
                            Field::ExcludedPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "excluded",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "category",
                                            "excluded",
                                            "name",
                                            "description",
                                            "network",
                                            "unit",
                                            "term",
                                            "financial",
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
                                            "category",
                                            "excluded",
                                            "name",
                                            "description",
                                            "network",
                                            "unit",
                                            "term",
                                            "financial",
                                        ],
                                    ));
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
                                            "extension",
                                            "modifierExtension",
                                            "category",
                                            "excluded",
                                            "name",
                                            "description",
                                            "network",
                                            "unit",
                                            "term",
                                            "financial",
                                        ],
                                    ));
                                }
                            }
                            Field::Network => {
                                if r#network.is_some() {
                                    return Err(serde::de::Error::duplicate_field("network"));
                                }
                                r#network = Some(map_access.next_value()?);
                            }
                            Field::Unit => {
                                if r#unit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unit"));
                                }
                                r#unit = Some(map_access.next_value()?);
                            }
                            Field::Term => {
                                if r#term.is_some() {
                                    return Err(serde::de::Error::duplicate_field("term"));
                                }
                                r#term = Some(map_access.next_value()?);
                            }
                            Field::Financial => {
                                if r#financial.is_some() {
                                    return Err(serde::de::Error::duplicate_field("financial"));
                                }
                                r#financial = Some(map_access.next_value()?);
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
                                        "excluded",
                                        "name",
                                        "description",
                                        "network",
                                        "unit",
                                        "term",
                                        "financial",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ExplanationOfBenefitBenefitBalance {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#category: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#category.unwrap_or(Default::default())
                        } else {
                            r#category.ok_or(serde::de::Error::missing_field("category"))?
                        },
                        r#excluded,
                        r#name,
                        r#description,
                        r#network,
                        r#unit,
                        r#term,
                        r#financial: r#financial.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "This resource provides: the claim details; adjudication details from the processing of a Claim; and optionally account balance information, for informing the subscriber of the benefits provided."]
#[derive(Default, Debug, Clone)]
pub struct ExplanationOfBenefit {
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
    #[doc = "A unique identifier assigned to this explanation of benefit."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "The category of claim, e.g. oral, pharmacy, vision, institutional, professional."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A finer grained suite of claim type codes which may convey additional information such as Inpatient vs Outpatient and/or a specialty service."]
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code to indicate whether the nature of the request is: to request adjudication of products and services previously rendered; or requesting authorization and adjudication for provision in the future; or requesting the non-binding adjudication of the listed products and services which could be provided in the future."]
    pub r#use: super::super::types::Code,
    #[doc = "The party to whom the professional services and/or products have been supplied or are being considered and for whom actual for forecast reimbursement is sought."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The period for which charges are being submitted."]
    pub r#billable_period: Option<Box<super::super::types::Period>>,
    #[doc = "The date this resource was created."]
    pub r#created: super::super::types::DateTime,
    #[doc = "Individual who created the claim, predetermination or preauthorization."]
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    #[doc = "The party responsible for authorization, adjudication and reimbursement."]
    pub r#insurer: Box<super::super::types::Reference>,
    #[doc = "The provider which is responsible for the claim, predetermination or preauthorization."]
    pub r#provider: Box<super::super::types::Reference>,
    #[doc = "The provider-required urgency of processing the request. Typical values include: stat, routine deferred."]
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code to indicate whether and for whom funds are to be reserved for future claims."]
    pub r#funds_reserve_requested: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code, used only on a response to a preauthorization, to indicate whether the benefits payable have been reserved and for whom."]
    pub r#funds_reserve: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Other claims which are related to this claim such as prior submissions or claims for related services or for the same event."]
    pub r#related: Vec<ExplanationOfBenefitRelated>,
    #[doc = "Prescription to support the dispensing of pharmacy, device or vision products."]
    pub r#prescription: Option<Box<super::super::types::Reference>>,
    #[doc = "Original prescription which has been superseded by this prescription to support the dispensing of pharmacy services, medications or products."]
    pub r#original_prescription: Option<Box<super::super::types::Reference>>,
    #[doc = "The party to be reimbursed for cost of the products and services according to the terms of the policy."]
    pub r#payee: Option<ExplanationOfBenefitPayee>,
    #[doc = "A reference to a referral resource."]
    pub r#referral: Option<Box<super::super::types::Reference>>,
    #[doc = "Facility where the services were provided."]
    pub r#facility: Option<Box<super::super::types::Reference>>,
    #[doc = "The business identifier for the instance of the adjudication request: claim predetermination or preauthorization."]
    pub r#claim: Option<Box<super::super::types::Reference>>,
    #[doc = "The business identifier for the instance of the adjudication response: claim, predetermination or preauthorization response."]
    pub r#claim_response: Option<Box<super::super::types::Reference>>,
    #[doc = "The outcome of the claim, predetermination, or preauthorization processing."]
    pub r#outcome: super::super::types::Code,
    #[doc = "A human readable description of the status of the adjudication."]
    pub r#disposition: Option<super::super::types::String>,
    #[doc = "Reference from the Insurer which is used in later communications which refers to this adjudication."]
    pub r#pre_auth_ref: Vec<super::super::types::String>,
    #[doc = "The timeframe during which the supplied preauthorization reference may be quoted on claims to obtain the adjudication as provided."]
    pub r#pre_auth_ref_period: Vec<Box<super::super::types::Period>>,
    #[doc = "The members of the team who provided the products and services."]
    pub r#care_team: Vec<ExplanationOfBenefitCareTeam>,
    #[doc = "Additional information codes regarding exceptions, special considerations, the condition, situation, prior or concurrent issues."]
    pub r#supporting_info: Vec<ExplanationOfBenefitSupportingInfo>,
    #[doc = "Information about diagnoses relevant to the claim items."]
    pub r#diagnosis: Vec<ExplanationOfBenefitDiagnosis>,
    #[doc = "Procedures performed on the patient relevant to the billing items with the claim."]
    pub r#procedure: Vec<ExplanationOfBenefitProcedure>,
    #[doc = "This indicates the relative order of a series of EOBs related to different coverages for the same suite of services."]
    pub r#precedence: Option<super::super::types::PositiveInt>,
    #[doc = "Financial instruments for reimbursement for the health care products and services specified on the claim."]
    pub r#insurance: Vec<ExplanationOfBenefitInsurance>,
    #[doc = "Details of a accident which resulted in injuries which required the products and services listed in the claim."]
    pub r#accident: Option<ExplanationOfBenefitAccident>,
    #[doc = "A claim line. Either a simple (a product or service) or a 'group' of details which can also be a simple items or groups of sub-details."]
    pub r#item: Vec<ExplanationOfBenefitItem>,
    #[doc = "The first-tier service adjudications for payor added product or service lines."]
    pub r#add_item: Vec<ExplanationOfBenefitAddItem>,
    #[doc = "The adjudication results which are presented at the header level rather than at the line-item or add-item levels."]
    pub r#adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    #[doc = "Categorized monetary totals for the adjudication."]
    pub r#total: Vec<ExplanationOfBenefitTotal>,
    #[doc = "Payment details for the adjudication of the claim."]
    pub r#payment: Option<ExplanationOfBenefitPayment>,
    #[doc = "A code for the form to be used for printing the content."]
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The actual form, by reference or inclusion, for printing the content or an EOB."]
    pub r#form: Option<Box<super::super::types::Attachment>>,
    #[doc = "A note that describes or explains adjudication results in a human readable form."]
    pub r#process_note: Vec<ExplanationOfBenefitProcessNote>,
    #[doc = "The term of the benefits documented in this response."]
    pub r#benefit_period: Option<Box<super::super::types::Period>>,
    #[doc = "Balance by Benefit Category."]
    pub r#benefit_balance: Vec<ExplanationOfBenefitBenefitBalance>,
}
impl crate::AnyResource for ExplanationOfBenefit {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for ExplanationOfBenefit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ExplanationOfBenefit")?;
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
            state.serialize_entry("type", &self.r#type)?;
            if let Some(some) = self.r#sub_type.as_ref() {
                state.serialize_entry("subType", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#use.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("use", &some)?;
                }
                if self.r#use.id.is_some() || !self.r#use.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#use.id.as_ref(),
                        extension: &self.r#use.extension,
                    };
                    state.serialize_entry("_use", &primitive_element)?;
                }
            } else {
                state.serialize_entry("use", &self.r#use)?;
            }
            state.serialize_entry("patient", &self.r#patient)?;
            if let Some(some) = self.r#billable_period.as_ref() {
                state.serialize_entry("billablePeriod", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#created.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("created", &some)?;
                }
                if self.r#created.id.is_some() || !self.r#created.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#created.id.as_ref(),
                        extension: &self.r#created.extension,
                    };
                    state.serialize_entry("_created", &primitive_element)?;
                }
            } else {
                state.serialize_entry("created", &self.r#created)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#outcome.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("outcome", &some)?;
                }
                if self.r#outcome.id.is_some() || !self.r#outcome.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#outcome.id.as_ref(),
                        extension: &self.r#outcome.extension,
                    };
                    state.serialize_entry("_outcome", &primitive_element)?;
                }
            } else {
                state.serialize_entry("outcome", &self.r#outcome)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#disposition.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("disposition", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_disposition", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#disposition.as_ref() {
                    state.serialize_entry("disposition", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#pre_auth_ref.is_empty() {
                    let values = self
                        .r#pre_auth_ref
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#pre_auth_ref.is_empty() {
                    state.serialize_entry("preAuthRef", &self.r#pre_auth_ref)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#precedence.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("precedence", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_precedence", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#precedence.as_ref() {
                    state.serialize_entry("precedence", some)?;
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
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ExplanationOfBenefit {
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
            #[serde(rename = "subType")]
            SubType,
            #[serde(rename = "use")]
            Use,
            #[serde(rename = "_use")]
            UsePrimitiveElement,
            #[serde(rename = "patient")]
            Patient,
            #[serde(rename = "billablePeriod")]
            BillablePeriod,
            #[serde(rename = "created")]
            Created,
            #[serde(rename = "_created")]
            CreatedPrimitiveElement,
            #[serde(rename = "enterer")]
            Enterer,
            #[serde(rename = "insurer")]
            Insurer,
            #[serde(rename = "provider")]
            Provider,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "fundsReserveRequested")]
            FundsReserveRequested,
            #[serde(rename = "fundsReserve")]
            FundsReserve,
            #[serde(rename = "related")]
            Related,
            #[serde(rename = "prescription")]
            Prescription,
            #[serde(rename = "originalPrescription")]
            OriginalPrescription,
            #[serde(rename = "payee")]
            Payee,
            #[serde(rename = "referral")]
            Referral,
            #[serde(rename = "facility")]
            Facility,
            #[serde(rename = "claim")]
            Claim,
            #[serde(rename = "claimResponse")]
            ClaimResponse,
            #[serde(rename = "outcome")]
            Outcome,
            #[serde(rename = "_outcome")]
            OutcomePrimitiveElement,
            #[serde(rename = "disposition")]
            Disposition,
            #[serde(rename = "_disposition")]
            DispositionPrimitiveElement,
            #[serde(rename = "preAuthRef")]
            PreAuthRef,
            #[serde(rename = "_preAuthRef")]
            PreAuthRefPrimitiveElement,
            #[serde(rename = "preAuthRefPeriod")]
            PreAuthRefPeriod,
            #[serde(rename = "careTeam")]
            CareTeam,
            #[serde(rename = "supportingInfo")]
            SupportingInfo,
            #[serde(rename = "diagnosis")]
            Diagnosis,
            #[serde(rename = "procedure")]
            Procedure,
            #[serde(rename = "precedence")]
            Precedence,
            #[serde(rename = "_precedence")]
            PrecedencePrimitiveElement,
            #[serde(rename = "insurance")]
            Insurance,
            #[serde(rename = "accident")]
            Accident,
            #[serde(rename = "item")]
            Item,
            #[serde(rename = "addItem")]
            AddItem,
            #[serde(rename = "adjudication")]
            Adjudication,
            #[serde(rename = "total")]
            Total,
            #[serde(rename = "payment")]
            Payment,
            #[serde(rename = "formCode")]
            FormCode,
            #[serde(rename = "form")]
            Form,
            #[serde(rename = "processNote")]
            ProcessNote,
            #[serde(rename = "benefitPeriod")]
            BenefitPeriod,
            #[serde(rename = "benefitBalance")]
            BenefitBalance,
            Unknown(std::string::String),
        }
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
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "ExplanationOfBenefit" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"ExplanationOfBenefit",
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
                                            "subType",
                                            "use",
                                            "patient",
                                            "billablePeriod",
                                            "created",
                                            "enterer",
                                            "insurer",
                                            "provider",
                                            "priority",
                                            "fundsReserveRequested",
                                            "fundsReserve",
                                            "related",
                                            "prescription",
                                            "originalPrescription",
                                            "payee",
                                            "referral",
                                            "facility",
                                            "claim",
                                            "claimResponse",
                                            "outcome",
                                            "disposition",
                                            "preAuthRef",
                                            "preAuthRefPeriod",
                                            "careTeam",
                                            "supportingInfo",
                                            "diagnosis",
                                            "procedure",
                                            "precedence",
                                            "insurance",
                                            "accident",
                                            "item",
                                            "addItem",
                                            "adjudication",
                                            "total",
                                            "payment",
                                            "formCode",
                                            "form",
                                            "processNote",
                                            "benefitPeriod",
                                            "benefitBalance",
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
                                            "subType",
                                            "use",
                                            "patient",
                                            "billablePeriod",
                                            "created",
                                            "enterer",
                                            "insurer",
                                            "provider",
                                            "priority",
                                            "fundsReserveRequested",
                                            "fundsReserve",
                                            "related",
                                            "prescription",
                                            "originalPrescription",
                                            "payee",
                                            "referral",
                                            "facility",
                                            "claim",
                                            "claimResponse",
                                            "outcome",
                                            "disposition",
                                            "preAuthRef",
                                            "preAuthRefPeriod",
                                            "careTeam",
                                            "supportingInfo",
                                            "diagnosis",
                                            "procedure",
                                            "precedence",
                                            "insurance",
                                            "accident",
                                            "item",
                                            "addItem",
                                            "adjudication",
                                            "total",
                                            "payment",
                                            "formCode",
                                            "form",
                                            "processNote",
                                            "benefitPeriod",
                                            "benefitBalance",
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
                                            "subType",
                                            "use",
                                            "patient",
                                            "billablePeriod",
                                            "created",
                                            "enterer",
                                            "insurer",
                                            "provider",
                                            "priority",
                                            "fundsReserveRequested",
                                            "fundsReserve",
                                            "related",
                                            "prescription",
                                            "originalPrescription",
                                            "payee",
                                            "referral",
                                            "facility",
                                            "claim",
                                            "claimResponse",
                                            "outcome",
                                            "disposition",
                                            "preAuthRef",
                                            "preAuthRefPeriod",
                                            "careTeam",
                                            "supportingInfo",
                                            "diagnosis",
                                            "procedure",
                                            "precedence",
                                            "insurance",
                                            "accident",
                                            "item",
                                            "addItem",
                                            "adjudication",
                                            "total",
                                            "payment",
                                            "formCode",
                                            "form",
                                            "processNote",
                                            "benefitPeriod",
                                            "benefitBalance",
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
                            Field::SubType => {
                                if r#sub_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subType"));
                                }
                                r#sub_type = Some(map_access.next_value()?);
                            }
                            Field::Use => {
                                if _ctx.from_json {
                                    let some = r#use.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("use"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#use.is_some() {
                                        return Err(serde::de::Error::duplicate_field("use"));
                                    }
                                    r#use = Some(map_access.next_value()?);
                                }
                            }
                            Field::UsePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "use",
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
                                            "subType",
                                            "use",
                                            "patient",
                                            "billablePeriod",
                                            "created",
                                            "enterer",
                                            "insurer",
                                            "provider",
                                            "priority",
                                            "fundsReserveRequested",
                                            "fundsReserve",
                                            "related",
                                            "prescription",
                                            "originalPrescription",
                                            "payee",
                                            "referral",
                                            "facility",
                                            "claim",
                                            "claimResponse",
                                            "outcome",
                                            "disposition",
                                            "preAuthRef",
                                            "preAuthRefPeriod",
                                            "careTeam",
                                            "supportingInfo",
                                            "diagnosis",
                                            "procedure",
                                            "precedence",
                                            "insurance",
                                            "accident",
                                            "item",
                                            "addItem",
                                            "adjudication",
                                            "total",
                                            "payment",
                                            "formCode",
                                            "form",
                                            "processNote",
                                            "benefitPeriod",
                                            "benefitBalance",
                                        ],
                                    ));
                                }
                            }
                            Field::Patient => {
                                if r#patient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patient"));
                                }
                                r#patient = Some(map_access.next_value()?);
                            }
                            Field::BillablePeriod => {
                                if r#billable_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "billablePeriod",
                                    ));
                                }
                                r#billable_period = Some(map_access.next_value()?);
                            }
                            Field::Created => {
                                if _ctx.from_json {
                                    let some = r#created.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("created"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#created.is_some() {
                                        return Err(serde::de::Error::duplicate_field("created"));
                                    }
                                    r#created = Some(map_access.next_value()?);
                                }
                            }
                            Field::CreatedPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "created",
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
                                            "subType",
                                            "use",
                                            "patient",
                                            "billablePeriod",
                                            "created",
                                            "enterer",
                                            "insurer",
                                            "provider",
                                            "priority",
                                            "fundsReserveRequested",
                                            "fundsReserve",
                                            "related",
                                            "prescription",
                                            "originalPrescription",
                                            "payee",
                                            "referral",
                                            "facility",
                                            "claim",
                                            "claimResponse",
                                            "outcome",
                                            "disposition",
                                            "preAuthRef",
                                            "preAuthRefPeriod",
                                            "careTeam",
                                            "supportingInfo",
                                            "diagnosis",
                                            "procedure",
                                            "precedence",
                                            "insurance",
                                            "accident",
                                            "item",
                                            "addItem",
                                            "adjudication",
                                            "total",
                                            "payment",
                                            "formCode",
                                            "form",
                                            "processNote",
                                            "benefitPeriod",
                                            "benefitBalance",
                                        ],
                                    ));
                                }
                            }
                            Field::Enterer => {
                                if r#enterer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("enterer"));
                                }
                                r#enterer = Some(map_access.next_value()?);
                            }
                            Field::Insurer => {
                                if r#insurer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("insurer"));
                                }
                                r#insurer = Some(map_access.next_value()?);
                            }
                            Field::Provider => {
                                if r#provider.is_some() {
                                    return Err(serde::de::Error::duplicate_field("provider"));
                                }
                                r#provider = Some(map_access.next_value()?);
                            }
                            Field::Priority => {
                                if r#priority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("priority"));
                                }
                                r#priority = Some(map_access.next_value()?);
                            }
                            Field::FundsReserveRequested => {
                                if r#funds_reserve_requested.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fundsReserveRequested",
                                    ));
                                }
                                r#funds_reserve_requested = Some(map_access.next_value()?);
                            }
                            Field::FundsReserve => {
                                if r#funds_reserve.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fundsReserve"));
                                }
                                r#funds_reserve = Some(map_access.next_value()?);
                            }
                            Field::Related => {
                                if r#related.is_some() {
                                    return Err(serde::de::Error::duplicate_field("related"));
                                }
                                r#related = Some(map_access.next_value()?);
                            }
                            Field::Prescription => {
                                if r#prescription.is_some() {
                                    return Err(serde::de::Error::duplicate_field("prescription"));
                                }
                                r#prescription = Some(map_access.next_value()?);
                            }
                            Field::OriginalPrescription => {
                                if r#original_prescription.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "originalPrescription",
                                    ));
                                }
                                r#original_prescription = Some(map_access.next_value()?);
                            }
                            Field::Payee => {
                                if r#payee.is_some() {
                                    return Err(serde::de::Error::duplicate_field("payee"));
                                }
                                r#payee = Some(map_access.next_value()?);
                            }
                            Field::Referral => {
                                if r#referral.is_some() {
                                    return Err(serde::de::Error::duplicate_field("referral"));
                                }
                                r#referral = Some(map_access.next_value()?);
                            }
                            Field::Facility => {
                                if r#facility.is_some() {
                                    return Err(serde::de::Error::duplicate_field("facility"));
                                }
                                r#facility = Some(map_access.next_value()?);
                            }
                            Field::Claim => {
                                if r#claim.is_some() {
                                    return Err(serde::de::Error::duplicate_field("claim"));
                                }
                                r#claim = Some(map_access.next_value()?);
                            }
                            Field::ClaimResponse => {
                                if r#claim_response.is_some() {
                                    return Err(serde::de::Error::duplicate_field("claimResponse"));
                                }
                                r#claim_response = Some(map_access.next_value()?);
                            }
                            Field::Outcome => {
                                if _ctx.from_json {
                                    let some = r#outcome.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("outcome"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#outcome.is_some() {
                                        return Err(serde::de::Error::duplicate_field("outcome"));
                                    }
                                    r#outcome = Some(map_access.next_value()?);
                                }
                            }
                            Field::OutcomePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "outcome",
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
                                            "subType",
                                            "use",
                                            "patient",
                                            "billablePeriod",
                                            "created",
                                            "enterer",
                                            "insurer",
                                            "provider",
                                            "priority",
                                            "fundsReserveRequested",
                                            "fundsReserve",
                                            "related",
                                            "prescription",
                                            "originalPrescription",
                                            "payee",
                                            "referral",
                                            "facility",
                                            "claim",
                                            "claimResponse",
                                            "outcome",
                                            "disposition",
                                            "preAuthRef",
                                            "preAuthRefPeriod",
                                            "careTeam",
                                            "supportingInfo",
                                            "diagnosis",
                                            "procedure",
                                            "precedence",
                                            "insurance",
                                            "accident",
                                            "item",
                                            "addItem",
                                            "adjudication",
                                            "total",
                                            "payment",
                                            "formCode",
                                            "form",
                                            "processNote",
                                            "benefitPeriod",
                                            "benefitBalance",
                                        ],
                                    ));
                                }
                            }
                            Field::Disposition => {
                                if _ctx.from_json {
                                    let some = r#disposition.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "disposition",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#disposition.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "disposition",
                                        ));
                                    }
                                    r#disposition = Some(map_access.next_value()?);
                                }
                            }
                            Field::DispositionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#disposition.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_disposition",
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
                                        "disposition",
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
                                            "subType",
                                            "use",
                                            "patient",
                                            "billablePeriod",
                                            "created",
                                            "enterer",
                                            "insurer",
                                            "provider",
                                            "priority",
                                            "fundsReserveRequested",
                                            "fundsReserve",
                                            "related",
                                            "prescription",
                                            "originalPrescription",
                                            "payee",
                                            "referral",
                                            "facility",
                                            "claim",
                                            "claimResponse",
                                            "outcome",
                                            "disposition",
                                            "preAuthRef",
                                            "preAuthRefPeriod",
                                            "careTeam",
                                            "supportingInfo",
                                            "diagnosis",
                                            "procedure",
                                            "precedence",
                                            "insurance",
                                            "accident",
                                            "item",
                                            "addItem",
                                            "adjudication",
                                            "total",
                                            "payment",
                                            "formCode",
                                            "form",
                                            "processNote",
                                            "benefitPeriod",
                                            "benefitBalance",
                                        ],
                                    ));
                                }
                            }
                            Field::PreAuthRef => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#pre_auth_ref.get_or_insert(
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
                                            "preAuthRef",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#pre_auth_ref.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "preAuthRef",
                                        ));
                                    }
                                    r#pre_auth_ref = Some(map_access.next_value()?);
                                }
                            }
                            Field::PreAuthRefPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#pre_auth_ref.get_or_insert(
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
                                            "_preAuthRef",
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
                                        "preAuthRef",
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
                                            "subType",
                                            "use",
                                            "patient",
                                            "billablePeriod",
                                            "created",
                                            "enterer",
                                            "insurer",
                                            "provider",
                                            "priority",
                                            "fundsReserveRequested",
                                            "fundsReserve",
                                            "related",
                                            "prescription",
                                            "originalPrescription",
                                            "payee",
                                            "referral",
                                            "facility",
                                            "claim",
                                            "claimResponse",
                                            "outcome",
                                            "disposition",
                                            "preAuthRef",
                                            "preAuthRefPeriod",
                                            "careTeam",
                                            "supportingInfo",
                                            "diagnosis",
                                            "procedure",
                                            "precedence",
                                            "insurance",
                                            "accident",
                                            "item",
                                            "addItem",
                                            "adjudication",
                                            "total",
                                            "payment",
                                            "formCode",
                                            "form",
                                            "processNote",
                                            "benefitPeriod",
                                            "benefitBalance",
                                        ],
                                    ));
                                }
                            }
                            Field::PreAuthRefPeriod => {
                                if r#pre_auth_ref_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "preAuthRefPeriod",
                                    ));
                                }
                                r#pre_auth_ref_period = Some(map_access.next_value()?);
                            }
                            Field::CareTeam => {
                                if r#care_team.is_some() {
                                    return Err(serde::de::Error::duplicate_field("careTeam"));
                                }
                                r#care_team = Some(map_access.next_value()?);
                            }
                            Field::SupportingInfo => {
                                if r#supporting_info.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "supportingInfo",
                                    ));
                                }
                                r#supporting_info = Some(map_access.next_value()?);
                            }
                            Field::Diagnosis => {
                                if r#diagnosis.is_some() {
                                    return Err(serde::de::Error::duplicate_field("diagnosis"));
                                }
                                r#diagnosis = Some(map_access.next_value()?);
                            }
                            Field::Procedure => {
                                if r#procedure.is_some() {
                                    return Err(serde::de::Error::duplicate_field("procedure"));
                                }
                                r#procedure = Some(map_access.next_value()?);
                            }
                            Field::Precedence => {
                                if _ctx.from_json {
                                    let some = r#precedence.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "precedence",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#precedence.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "precedence",
                                        ));
                                    }
                                    r#precedence = Some(map_access.next_value()?);
                                }
                            }
                            Field::PrecedencePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#precedence.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_precedence",
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
                                        "precedence",
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
                                            "subType",
                                            "use",
                                            "patient",
                                            "billablePeriod",
                                            "created",
                                            "enterer",
                                            "insurer",
                                            "provider",
                                            "priority",
                                            "fundsReserveRequested",
                                            "fundsReserve",
                                            "related",
                                            "prescription",
                                            "originalPrescription",
                                            "payee",
                                            "referral",
                                            "facility",
                                            "claim",
                                            "claimResponse",
                                            "outcome",
                                            "disposition",
                                            "preAuthRef",
                                            "preAuthRefPeriod",
                                            "careTeam",
                                            "supportingInfo",
                                            "diagnosis",
                                            "procedure",
                                            "precedence",
                                            "insurance",
                                            "accident",
                                            "item",
                                            "addItem",
                                            "adjudication",
                                            "total",
                                            "payment",
                                            "formCode",
                                            "form",
                                            "processNote",
                                            "benefitPeriod",
                                            "benefitBalance",
                                        ],
                                    ));
                                }
                            }
                            Field::Insurance => {
                                if r#insurance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("insurance"));
                                }
                                r#insurance = Some(map_access.next_value()?);
                            }
                            Field::Accident => {
                                if r#accident.is_some() {
                                    return Err(serde::de::Error::duplicate_field("accident"));
                                }
                                r#accident = Some(map_access.next_value()?);
                            }
                            Field::Item => {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("item"));
                                }
                                r#item = Some(map_access.next_value()?);
                            }
                            Field::AddItem => {
                                if r#add_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("addItem"));
                                }
                                r#add_item = Some(map_access.next_value()?);
                            }
                            Field::Adjudication => {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                r#adjudication = Some(map_access.next_value()?);
                            }
                            Field::Total => {
                                if r#total.is_some() {
                                    return Err(serde::de::Error::duplicate_field("total"));
                                }
                                r#total = Some(map_access.next_value()?);
                            }
                            Field::Payment => {
                                if r#payment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("payment"));
                                }
                                r#payment = Some(map_access.next_value()?);
                            }
                            Field::FormCode => {
                                if r#form_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("formCode"));
                                }
                                r#form_code = Some(map_access.next_value()?);
                            }
                            Field::Form => {
                                if r#form.is_some() {
                                    return Err(serde::de::Error::duplicate_field("form"));
                                }
                                r#form = Some(map_access.next_value()?);
                            }
                            Field::ProcessNote => {
                                if r#process_note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("processNote"));
                                }
                                r#process_note = Some(map_access.next_value()?);
                            }
                            Field::BenefitPeriod => {
                                if r#benefit_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("benefitPeriod"));
                                }
                                r#benefit_period = Some(map_access.next_value()?);
                            }
                            Field::BenefitBalance => {
                                if r#benefit_balance.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "benefitBalance",
                                    ));
                                }
                                r#benefit_balance = Some(map_access.next_value()?);
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
                                        "subType",
                                        "use",
                                        "patient",
                                        "billablePeriod",
                                        "created",
                                        "enterer",
                                        "insurer",
                                        "provider",
                                        "priority",
                                        "fundsReserveRequested",
                                        "fundsReserve",
                                        "related",
                                        "prescription",
                                        "originalPrescription",
                                        "payee",
                                        "referral",
                                        "facility",
                                        "claim",
                                        "claimResponse",
                                        "outcome",
                                        "disposition",
                                        "preAuthRef",
                                        "preAuthRefPeriod",
                                        "careTeam",
                                        "supportingInfo",
                                        "diagnosis",
                                        "procedure",
                                        "precedence",
                                        "insurance",
                                        "accident",
                                        "item",
                                        "addItem",
                                        "adjudication",
                                        "total",
                                        "payment",
                                        "formCode",
                                        "form",
                                        "processNote",
                                        "benefitPeriod",
                                        "benefitBalance",
                                    ],
                                ));
                            },
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
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#sub_type,
                        r#use: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#use.unwrap_or(Default::default())
                        } else {
                            r#use.ok_or(serde::de::Error::missing_field("use"))?
                        },
                        r#patient: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#patient.unwrap_or(Default::default())
                        } else {
                            r#patient.ok_or(serde::de::Error::missing_field("patient"))?
                        },
                        r#billable_period,
                        r#created: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#created.unwrap_or(Default::default())
                        } else {
                            r#created.ok_or(serde::de::Error::missing_field("created"))?
                        },
                        r#enterer,
                        r#insurer: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#insurer.unwrap_or(Default::default())
                        } else {
                            r#insurer.ok_or(serde::de::Error::missing_field("insurer"))?
                        },
                        r#provider: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#provider.unwrap_or(Default::default())
                        } else {
                            r#provider.ok_or(serde::de::Error::missing_field("provider"))?
                        },
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
                        r#outcome: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#outcome.unwrap_or(Default::default())
                        } else {
                            r#outcome.ok_or(serde::de::Error::missing_field("outcome"))?
                        },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
