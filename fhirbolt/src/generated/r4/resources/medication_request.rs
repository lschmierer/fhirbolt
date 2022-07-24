// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicationRequestReported {
    Boolean(Box<super::super::types::Boolean>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicationRequestReported {
    fn default() -> MedicationRequestReported {
        MedicationRequestReported::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum MedicationRequestMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicationRequestMedication {
    fn default() -> MedicationRequestMedication {
        MedicationRequestMedication::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum MedicationRequestSubstitutionAllowed {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for MedicationRequestSubstitutionAllowed {
    fn default() -> MedicationRequestSubstitutionAllowed {
        MedicationRequestSubstitutionAllowed::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationRequestDispenseRequestInitialFill {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#duration: Option<Box<super::super::types::Duration>>,
}
impl serde::ser::Serialize for MedicationRequestDispenseRequestInitialFill {
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
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#duration.as_ref() {
            state.serialize_entry("duration", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationRequestDispenseRequestInitialFill {
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
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "duration")]
            Duration,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationRequestDispenseRequestInitialFill;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationRequestDispenseRequestInitialFill")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationRequestDispenseRequestInitialFill, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#duration: Option<Box<super::super::types::Duration>> = None;
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
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        Field::Duration => {
                            if r#duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            r#duration = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationRequestDispenseRequestInitialFill {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#quantity,
                    r#duration,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationRequestDispenseRequest {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#initial_fill: Option<MedicationRequestDispenseRequestInitialFill>,
    pub r#dispense_interval: Option<Box<super::super::types::Duration>>,
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    pub r#number_of_repeats_allowed: Option<super::super::types::UnsignedInt>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#expected_supply_duration: Option<Box<super::super::types::Duration>>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MedicationRequestDispenseRequest {
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
        if let Some(some) = self.r#initial_fill.as_ref() {
            state.serialize_entry("initialFill", some)?;
        }
        if let Some(some) = self.r#dispense_interval.as_ref() {
            state.serialize_entry("dispenseInterval", some)?;
        }
        if let Some(some) = self.r#validity_period.as_ref() {
            state.serialize_entry("validityPeriod", some)?;
        }
        if let Some(some) = self.r#number_of_repeats_allowed.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("numberOfRepeatsAllowed", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_numberOfRepeatsAllowed", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#expected_supply_duration.as_ref() {
            state.serialize_entry("expectedSupplyDuration", some)?;
        }
        if let Some(some) = self.r#performer.as_ref() {
            state.serialize_entry("performer", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationRequestDispenseRequest {
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
            #[serde(rename = "initialFill")]
            InitialFill,
            #[serde(rename = "dispenseInterval")]
            DispenseInterval,
            #[serde(rename = "validityPeriod")]
            ValidityPeriod,
            #[serde(rename = "numberOfRepeatsAllowed")]
            NumberOfRepeatsAllowed,
            #[serde(rename = "_numberOfRepeatsAllowed")]
            NumberOfRepeatsAllowedPrimitiveElement,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "expectedSupplyDuration")]
            ExpectedSupplyDuration,
            #[serde(rename = "performer")]
            Performer,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationRequestDispenseRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationRequestDispenseRequest")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationRequestDispenseRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#initial_fill: Option<MedicationRequestDispenseRequestInitialFill> = None;
                let mut r#dispense_interval: Option<Box<super::super::types::Duration>> = None;
                let mut r#validity_period: Option<Box<super::super::types::Period>> = None;
                let mut r#number_of_repeats_allowed: Option<super::super::types::UnsignedInt> =
                    None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#expected_supply_duration: Option<Box<super::super::types::Duration>> =
                    None;
                let mut r#performer: Option<Box<super::super::types::Reference>> = None;
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
                        Field::InitialFill => {
                            if r#initial_fill.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialFill"));
                            }
                            r#initial_fill = Some(map_access.next_value()?);
                        }
                        Field::DispenseInterval => {
                            if r#dispense_interval.is_some() {
                                return Err(serde::de::Error::duplicate_field("dispenseInterval"));
                            }
                            r#dispense_interval = Some(map_access.next_value()?);
                        }
                        Field::ValidityPeriod => {
                            if r#validity_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("validityPeriod"));
                            }
                            r#validity_period = Some(map_access.next_value()?);
                        }
                        Field::NumberOfRepeatsAllowed => {
                            let some =
                                r#number_of_repeats_allowed.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "numberOfRepeatsAllowed",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NumberOfRepeatsAllowedPrimitiveElement => {
                            let some =
                                r#number_of_repeats_allowed.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_numberOfRepeatsAllowed",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        Field::ExpectedSupplyDuration => {
                            if r#expected_supply_duration.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "expectedSupplyDuration",
                                ));
                            }
                            r#expected_supply_duration = Some(map_access.next_value()?);
                        }
                        Field::Performer => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationRequestDispenseRequest {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#initial_fill,
                    r#dispense_interval,
                    r#validity_period,
                    r#number_of_repeats_allowed,
                    r#quantity,
                    r#expected_supply_duration,
                    r#performer,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationRequestSubstitution {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#allowed: MedicationRequestSubstitutionAllowed,
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for MedicationRequestSubstitution {
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
        match self.r#allowed {
            MedicationRequestSubstitutionAllowed::Boolean(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("allowedBoolean", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_allowedBoolean", &primitive_element)?;
                }
            }
            MedicationRequestSubstitutionAllowed::CodeableConcept(ref value) => {
                state.serialize_entry("allowedCodeableConcept", value)?;
            }
            MedicationRequestSubstitutionAllowed::Invalid => {
                return Err(serde::ser::Error::custom("allowed is a required field"))
            }
        }
        if let Some(some) = self.r#reason.as_ref() {
            state.serialize_entry("reason", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationRequestSubstitution {
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
            #[serde(rename = "allowedBoolean")]
            AllowedBoolean,
            #[serde(rename = "_allowedBoolean")]
            AllowedBooleanPrimitiveElement,
            #[serde(rename = "allowedCodeableConcept")]
            AllowedCodeableConcept,
            #[serde(rename = "reason")]
            Reason,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationRequestSubstitution;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationRequestSubstitution")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationRequestSubstitution, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#allowed: Option<MedicationRequestSubstitutionAllowed> = None;
                let mut r#reason: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        Field::AllowedBoolean => {
                            let r#enum = r#allowed.get_or_insert(
                                MedicationRequestSubstitutionAllowed::Boolean(Default::default()),
                            );
                            if let MedicationRequestSubstitutionAllowed::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "allowedBoolean",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("allowed[x]"));
                            }
                        }
                        Field::AllowedBooleanPrimitiveElement => {
                            let r#enum = r#allowed.get_or_insert(
                                MedicationRequestSubstitutionAllowed::Boolean(Default::default()),
                            );
                            if let MedicationRequestSubstitutionAllowed::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_allowedBoolean",
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
                        Field::AllowedCodeableConcept => {
                            if r#allowed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "allowedCodeableConcept",
                                ));
                            }
                            r#allowed =
                                Some(MedicationRequestSubstitutionAllowed::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                        }
                        Field::Reason => {
                            if r#reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            r#reason = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationRequestSubstitution {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#allowed: r#allowed.ok_or(serde::de::Error::missing_field("allowed[x]"))?,
                    r#reason,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationRequest {
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
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#intent: super::super::types::Code,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    pub r#reported: Option<MedicationRequestReported>,
    pub r#medication: MedicationRequestMedication,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    pub r#authored_on: Option<super::super::types::DateTime>,
    pub r#requester: Option<Box<super::super::types::Reference>>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#performer_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#course_of_therapy_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#insurance: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#dosage_instruction: Vec<Box<super::super::types::Dosage>>,
    pub r#dispense_request: Option<MedicationRequestDispenseRequest>,
    pub r#substitution: Option<MedicationRequestSubstitution>,
    pub r#prior_prescription: Option<Box<super::super::types::Reference>>,
    pub r#detected_issue: Vec<Box<super::super::types::Reference>>,
    pub r#event_history: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MedicationRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicationRequest")?;
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
        if let Some(some) = self.r#status_reason.as_ref() {
            state.serialize_entry("statusReason", some)?;
        }
        if let Some(some) = self.r#intent.value.as_ref() {
            state.serialize_entry("intent", some)?;
        }
        if self.r#intent.id.is_some() || !self.r#intent.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#intent.id,
                extension: &self.r#intent.extension,
            };
            state.serialize_entry("_intent", &primitive_element)?;
        }
        if !self.r#category.is_empty() {
            state.serialize_entry("category", &self.r#category)?;
        }
        if let Some(some) = self.r#priority.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("priority", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_priority", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#do_not_perform.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("doNotPerform", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_doNotPerform", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#reported.as_ref() {
            match some {
                MedicationRequestReported::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("reportedBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_reportedBoolean", &primitive_element)?;
                    }
                }
                MedicationRequestReported::Reference(ref value) => {
                    state.serialize_entry("reportedReference", value)?;
                }
                MedicationRequestReported::Invalid => {
                    return Err(serde::ser::Error::custom("reported is invalid"))
                }
            }
        }
        match self.r#medication {
            MedicationRequestMedication::CodeableConcept(ref value) => {
                state.serialize_entry("medicationCodeableConcept", value)?;
            }
            MedicationRequestMedication::Reference(ref value) => {
                state.serialize_entry("medicationReference", value)?;
            }
            MedicationRequestMedication::Invalid => {
                return Err(serde::ser::Error::custom("medication is a required field"))
            }
        }
        state.serialize_entry("subject", &self.r#subject)?;
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if !self.r#supporting_information.is_empty() {
            state.serialize_entry("supportingInformation", &self.r#supporting_information)?;
        }
        if let Some(some) = self.r#authored_on.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("authoredOn", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_authoredOn", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#requester.as_ref() {
            state.serialize_entry("requester", some)?;
        }
        if let Some(some) = self.r#performer.as_ref() {
            state.serialize_entry("performer", some)?;
        }
        if let Some(some) = self.r#performer_type.as_ref() {
            state.serialize_entry("performerType", some)?;
        }
        if let Some(some) = self.r#recorder.as_ref() {
            state.serialize_entry("recorder", some)?;
        }
        if !self.r#reason_code.is_empty() {
            state.serialize_entry("reasonCode", &self.r#reason_code)?;
        }
        if !self.r#reason_reference.is_empty() {
            state.serialize_entry("reasonReference", &self.r#reason_reference)?;
        }
        if !self.r#instantiates_canonical.is_empty() {
            let values: Vec<_> = self
                .r#instantiates_canonical
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiatesCanonical", &values)?;
            }
            let requires_elements = self
                .r#instantiates_canonical
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#instantiates_canonical
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
                state.serialize_entry("_instantiatesCanonical", &primitive_elements)?;
            }
        }
        if !self.r#instantiates_uri.is_empty() {
            let values: Vec<_> = self.r#instantiates_uri.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiatesUri", &values)?;
            }
            let requires_elements = self
                .r#instantiates_uri
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#instantiates_uri
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
                state.serialize_entry("_instantiatesUri", &primitive_elements)?;
            }
        }
        if !self.r#based_on.is_empty() {
            state.serialize_entry("basedOn", &self.r#based_on)?;
        }
        if let Some(some) = self.r#group_identifier.as_ref() {
            state.serialize_entry("groupIdentifier", some)?;
        }
        if let Some(some) = self.r#course_of_therapy_type.as_ref() {
            state.serialize_entry("courseOfTherapyType", some)?;
        }
        if !self.r#insurance.is_empty() {
            state.serialize_entry("insurance", &self.r#insurance)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if !self.r#dosage_instruction.is_empty() {
            state.serialize_entry("dosageInstruction", &self.r#dosage_instruction)?;
        }
        if let Some(some) = self.r#dispense_request.as_ref() {
            state.serialize_entry("dispenseRequest", some)?;
        }
        if let Some(some) = self.r#substitution.as_ref() {
            state.serialize_entry("substitution", some)?;
        }
        if let Some(some) = self.r#prior_prescription.as_ref() {
            state.serialize_entry("priorPrescription", some)?;
        }
        if !self.r#detected_issue.is_empty() {
            state.serialize_entry("detectedIssue", &self.r#detected_issue)?;
        }
        if !self.r#event_history.is_empty() {
            state.serialize_entry("eventHistory", &self.r#event_history)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationRequest {
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
            #[serde(rename = "statusReason")]
            StatusReason,
            #[serde(rename = "intent")]
            Intent,
            #[serde(rename = "_intent")]
            IntentPrimitiveElement,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "_priority")]
            PriorityPrimitiveElement,
            #[serde(rename = "doNotPerform")]
            DoNotPerform,
            #[serde(rename = "_doNotPerform")]
            DoNotPerformPrimitiveElement,
            #[serde(rename = "reportedBoolean")]
            ReportedBoolean,
            #[serde(rename = "_reportedBoolean")]
            ReportedBooleanPrimitiveElement,
            #[serde(rename = "reportedReference")]
            ReportedReference,
            #[serde(rename = "medicationCodeableConcept")]
            MedicationCodeableConcept,
            #[serde(rename = "medicationReference")]
            MedicationReference,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "supportingInformation")]
            SupportingInformation,
            #[serde(rename = "authoredOn")]
            AuthoredOn,
            #[serde(rename = "_authoredOn")]
            AuthoredOnPrimitiveElement,
            #[serde(rename = "requester")]
            Requester,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "performerType")]
            PerformerType,
            #[serde(rename = "recorder")]
            Recorder,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "instantiatesCanonical")]
            InstantiatesCanonical,
            #[serde(rename = "_instantiatesCanonical")]
            InstantiatesCanonicalPrimitiveElement,
            #[serde(rename = "instantiatesUri")]
            InstantiatesUri,
            #[serde(rename = "_instantiatesUri")]
            InstantiatesUriPrimitiveElement,
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "groupIdentifier")]
            GroupIdentifier,
            #[serde(rename = "courseOfTherapyType")]
            CourseOfTherapyType,
            #[serde(rename = "insurance")]
            Insurance,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "dosageInstruction")]
            DosageInstruction,
            #[serde(rename = "dispenseRequest")]
            DispenseRequest,
            #[serde(rename = "substitution")]
            Substitution,
            #[serde(rename = "priorPrescription")]
            PriorPrescription,
            #[serde(rename = "detectedIssue")]
            DetectedIssue,
            #[serde(rename = "eventHistory")]
            EventHistory,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationRequest")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicationRequest, V::Error>
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
                let mut r#status_reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#intent: Option<super::super::types::Code> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#priority: Option<super::super::types::Code> = None;
                let mut r#do_not_perform: Option<super::super::types::Boolean> = None;
                let mut r#reported: Option<MedicationRequestReported> = None;
                let mut r#medication: Option<MedicationRequestMedication> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#supporting_information: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#authored_on: Option<super::super::types::DateTime> = None;
                let mut r#requester: Option<Box<super::super::types::Reference>> = None;
                let mut r#performer: Option<Box<super::super::types::Reference>> = None;
                let mut r#performer_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#recorder: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#instantiates_canonical: Option<Vec<super::super::types::Canonical>> =
                    None;
                let mut r#instantiates_uri: Option<Vec<super::super::types::Uri>> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#group_identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#course_of_therapy_type: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#insurance: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#dosage_instruction: Option<Vec<Box<super::super::types::Dosage>>> = None;
                let mut r#dispense_request: Option<MedicationRequestDispenseRequest> = None;
                let mut r#substitution: Option<MedicationRequestSubstitution> = None;
                let mut r#prior_prescription: Option<Box<super::super::types::Reference>> = None;
                let mut r#detected_issue: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#event_history: Option<Vec<Box<super::super::types::Reference>>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicationRequest" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicationRequest",
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
                        Field::Status => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::StatusPrimitiveElement => {
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
                        Field::StatusReason => {
                            if r#status_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusReason"));
                            }
                            r#status_reason = Some(map_access.next_value()?);
                        }
                        Field::Intent => {
                            let some = r#intent.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("intent"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IntentPrimitiveElement => {
                            let some = r#intent.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_intent"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        Field::Priority => {
                            let some = r#priority.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PriorityPrimitiveElement => {
                            let some = r#priority.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_priority"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::DoNotPerform => {
                            let some = r#do_not_perform.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("doNotPerform"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DoNotPerformPrimitiveElement => {
                            let some = r#do_not_perform.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_doNotPerform"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ReportedBoolean => {
                            let r#enum = r#reported.get_or_insert(
                                MedicationRequestReported::Boolean(Default::default()),
                            );
                            if let MedicationRequestReported::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "reportedBoolean",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("reported[x]"));
                            }
                        }
                        Field::ReportedBooleanPrimitiveElement => {
                            let r#enum = r#reported.get_or_insert(
                                MedicationRequestReported::Boolean(Default::default()),
                            );
                            if let MedicationRequestReported::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_reportedBoolean",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_reported[x]"));
                            }
                        }
                        Field::ReportedReference => {
                            if r#reported.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportedReference"));
                            }
                            r#reported = Some(MedicationRequestReported::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        Field::MedicationCodeableConcept => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "medicationCodeableConcept",
                                ));
                            }
                            r#medication = Some(MedicationRequestMedication::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        Field::MedicationReference => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "medicationReference",
                                ));
                            }
                            r#medication = Some(MedicationRequestMedication::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        Field::Encounter => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(map_access.next_value()?);
                        }
                        Field::SupportingInformation => {
                            if r#supporting_information.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "supportingInformation",
                                ));
                            }
                            r#supporting_information = Some(map_access.next_value()?);
                        }
                        Field::AuthoredOn => {
                            let some = r#authored_on.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("authoredOn"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::AuthoredOnPrimitiveElement => {
                            let some = r#authored_on.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_authoredOn"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Requester => {
                            if r#requester.is_some() {
                                return Err(serde::de::Error::duplicate_field("requester"));
                            }
                            r#requester = Some(map_access.next_value()?);
                        }
                        Field::Performer => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(map_access.next_value()?);
                        }
                        Field::PerformerType => {
                            if r#performer_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("performerType"));
                            }
                            r#performer_type = Some(map_access.next_value()?);
                        }
                        Field::Recorder => {
                            if r#recorder.is_some() {
                                return Err(serde::de::Error::duplicate_field("recorder"));
                            }
                            r#recorder = Some(map_access.next_value()?);
                        }
                        Field::ReasonCode => {
                            if r#reason_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonCode"));
                            }
                            r#reason_code = Some(map_access.next_value()?);
                        }
                        Field::ReasonReference => {
                            if r#reason_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonReference"));
                            }
                            r#reason_reference = Some(map_access.next_value()?);
                        }
                        Field::InstantiatesCanonical => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#instantiates_canonical.get_or_insert(
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
                                    "instantiatesCanonical",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::InstantiatesCanonicalPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#instantiates_canonical.get_or_insert(
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
                                    "_instantiatesCanonical",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::InstantiatesUri => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#instantiates_uri.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("instantiatesUri"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::InstantiatesUriPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#instantiates_uri.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("_instantiatesUri"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::BasedOn => {
                            if r#based_on.is_some() {
                                return Err(serde::de::Error::duplicate_field("basedOn"));
                            }
                            r#based_on = Some(map_access.next_value()?);
                        }
                        Field::GroupIdentifier => {
                            if r#group_identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupIdentifier"));
                            }
                            r#group_identifier = Some(map_access.next_value()?);
                        }
                        Field::CourseOfTherapyType => {
                            if r#course_of_therapy_type.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "courseOfTherapyType",
                                ));
                            }
                            r#course_of_therapy_type = Some(map_access.next_value()?);
                        }
                        Field::Insurance => {
                            if r#insurance.is_some() {
                                return Err(serde::de::Error::duplicate_field("insurance"));
                            }
                            r#insurance = Some(map_access.next_value()?);
                        }
                        Field::Note => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        Field::DosageInstruction => {
                            if r#dosage_instruction.is_some() {
                                return Err(serde::de::Error::duplicate_field("dosageInstruction"));
                            }
                            r#dosage_instruction = Some(map_access.next_value()?);
                        }
                        Field::DispenseRequest => {
                            if r#dispense_request.is_some() {
                                return Err(serde::de::Error::duplicate_field("dispenseRequest"));
                            }
                            r#dispense_request = Some(map_access.next_value()?);
                        }
                        Field::Substitution => {
                            if r#substitution.is_some() {
                                return Err(serde::de::Error::duplicate_field("substitution"));
                            }
                            r#substitution = Some(map_access.next_value()?);
                        }
                        Field::PriorPrescription => {
                            if r#prior_prescription.is_some() {
                                return Err(serde::de::Error::duplicate_field("priorPrescription"));
                            }
                            r#prior_prescription = Some(map_access.next_value()?);
                        }
                        Field::DetectedIssue => {
                            if r#detected_issue.is_some() {
                                return Err(serde::de::Error::duplicate_field("detectedIssue"));
                            }
                            r#detected_issue = Some(map_access.next_value()?);
                        }
                        Field::EventHistory => {
                            if r#event_history.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventHistory"));
                            }
                            r#event_history = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationRequest {
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
                    r#status_reason,
                    r#intent: r#intent.ok_or(serde::de::Error::missing_field("intent"))?,
                    r#category: r#category.unwrap_or(vec![]),
                    r#priority,
                    r#do_not_perform,
                    r#reported,
                    r#medication: r#medication
                        .ok_or(serde::de::Error::missing_field("medication[x]"))?,
                    r#subject: r#subject.ok_or(serde::de::Error::missing_field("subject"))?,
                    r#encounter,
                    r#supporting_information: r#supporting_information.unwrap_or(vec![]),
                    r#authored_on,
                    r#requester,
                    r#performer,
                    r#performer_type,
                    r#recorder,
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                    r#instantiates_canonical: r#instantiates_canonical.unwrap_or(vec![]),
                    r#instantiates_uri: r#instantiates_uri.unwrap_or(vec![]),
                    r#based_on: r#based_on.unwrap_or(vec![]),
                    r#group_identifier,
                    r#course_of_therapy_type,
                    r#insurance: r#insurance.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#dosage_instruction: r#dosage_instruction.unwrap_or(vec![]),
                    r#dispense_request,
                    r#substitution,
                    r#prior_prescription,
                    r#detected_issue: r#detected_issue.unwrap_or(vec![]),
                    r#event_history: r#event_history.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
