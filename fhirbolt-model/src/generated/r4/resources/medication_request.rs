// Generated on 2022-10-13 by fhirbolt-codegen v0.1.0
#[doc = "Indicates if this record was captured as a secondary 'reported' record rather than as an original primary source-of-truth record.  It may also indicate the source of the report."]
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
#[doc = "Identifies the medication being requested. This is a link to a resource that represents the medication which may be the details of the medication or simply an attribute carrying a code that identifies the medication from a known list of medications."]
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
#[doc = "True if the prescriber allows a different drug to be dispensed from what was prescribed."]
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
#[doc = "Indicates the quantity or duration for the first dispense of the medication."]
#[derive(Default, Debug, Clone)]
pub struct MedicationRequestDispenseRequestInitialFill {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The amount or quantity to provide as part of the first dispense."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The length of time that the first dispense is expected to last."]
    pub r#duration: Option<Box<super::super::types::Duration>>,
}
impl crate::AnyResource for MedicationRequestDispenseRequestInitialFill {}
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
            Unknown(String),
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
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "quantity",
                                            "duration",
                                        ],
                                    ));
                                }
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Indicates the specific details for the dispense or medication supply part of a medication request (also known as a Medication Prescription or Medication Order).  Note that this information is not always sent with the order.  There may be in some settings (e.g. hospitals) institutional or system support for completing the dispense details in the pharmacy department."]
#[derive(Default, Debug, Clone)]
pub struct MedicationRequestDispenseRequest {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates the quantity or duration for the first dispense of the medication."]
    pub r#initial_fill: Option<MedicationRequestDispenseRequestInitialFill>,
    #[doc = "The minimum period of time that must occur between dispenses of the medication."]
    pub r#dispense_interval: Option<Box<super::super::types::Duration>>,
    #[doc = "This indicates the validity period of a prescription (stale dating the Prescription)."]
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    #[doc = "An integer indicating the number of times, in addition to the original dispense, (aka refills or repeats) that the patient can receive the prescribed medication. Usage Notes: This integer does not include the original order dispense. This means that if an order indicates dispense 30 tablets plus \"3 repeats\", then the order can be dispensed a total of 4 times and the patient can receive a total of 120 tablets.  A prescriber may explicitly say that zero refills are permitted after the initial dispense."]
    pub r#number_of_repeats_allowed: Option<super::super::types::UnsignedInt>,
    #[doc = "The amount that is to be dispensed for one fill."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Identifies the period time over which the supplied product is expected to be used, or the length of time the dispense is expected to last."]
    pub r#expected_supply_duration: Option<Box<super::super::types::Duration>>,
    #[doc = "Indicates the intended dispensing Organization specified by the prescriber."]
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
                let some = Ok(some)?;
                state.serialize_entry("numberOfRepeatsAllowed", &some)?;
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
            Unknown(String),
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
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::InitialFill => {
                                if r#initial_fill.is_some() {
                                    return Err(serde::de::Error::duplicate_field("initialFill"));
                                }
                                r#initial_fill = Some(map_access.next_value()?);
                            }
                            Field::DispenseInterval => {
                                if r#dispense_interval.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dispenseInterval",
                                    ));
                                }
                                r#dispense_interval = Some(map_access.next_value()?);
                            }
                            Field::ValidityPeriod => {
                                if r#validity_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "validityPeriod",
                                    ));
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "initialFill",
                                            "dispenseInterval",
                                            "validityPeriod",
                                            "numberOfRepeatsAllowed",
                                            "quantity",
                                            "expectedSupplyDuration",
                                            "performer",
                                        ],
                                    ));
                                }
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Indicates whether or not substitution can or should be part of the dispense. In some cases, substitution must happen, in other cases substitution must not happen. This block explains the prescriber's intent. If nothing is specified substitution may be done."]
#[derive(Default, Debug, Clone)]
pub struct MedicationRequestSubstitution {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "True if the prescriber allows a different drug to be dispensed from what was prescribed."]
    pub r#allowed: MedicationRequestSubstitutionAllowed,
    #[doc = "Indicates the reason for the substitution, or why substitution must or must not be performed."]
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
                    let some = Ok(some)?;
                    state.serialize_entry("allowedBoolean", &some)?;
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
            Unknown(String),
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
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::AllowedBoolean => {
                                let r#enum = r#allowed.get_or_insert(
                                    MedicationRequestSubstitutionAllowed::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let MedicationRequestSubstitutionAllowed::Boolean(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "allowedBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("allowed[x]"));
                                }
                            }
                            Field::AllowedBooleanPrimitiveElement => {
                                let r#enum = r#allowed.get_or_insert(
                                    MedicationRequestSubstitutionAllowed::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let MedicationRequestSubstitutionAllowed::Boolean(variant) =
                                    r#enum
                                {
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
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "allowedBoolean",
                                            "allowedCodeableConcept",
                                            "reason",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(MedicationRequestSubstitution {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#allowed: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#allowed.unwrap_or(Default::default())
                        } else {
                            r#allowed.ok_or(serde::de::Error::missing_field("allowed[x]"))?
                        },
                        r#reason,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An order or request for both supply of the medication and the instructions for administration of the medication to a patient. The resource is called \"MedicationRequest\" rather than \"MedicationPrescription\" or \"MedicationOrder\" to generalize the use across inpatient and outpatient settings, including care plans, etc., and to harmonize with workflow patterns."]
#[derive(Default, Debug, Clone)]
pub struct MedicationRequest {
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
    #[doc = "Identifiers associated with this medication request that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate. They are business identifiers assigned to this resource by the performer or other systems and remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A code specifying the current state of the order.  Generally, this will be active or completed state."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the MedicationRequest."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether the request is a proposal, plan, or an original order."]
    pub r#intent: super::super::types::Code,
    #[doc = "Indicates the type of medication request (for example, where the medication is expected to be consumed or administered (i.e. inpatient or outpatient))."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates how quickly the Medication Request should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "If true indicates that the provider is asking for the medication request not to occur."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "Indicates if this record was captured as a secondary 'reported' record rather than as an original primary source-of-truth record.  It may also indicate the source of the report."]
    pub r#reported: Option<MedicationRequestReported>,
    #[doc = "Identifies the medication being requested. This is a link to a resource that represents the medication which may be the details of the medication or simply an attribute carrying a code that identifies the medication from a known list of medications."]
    pub r#medication: MedicationRequestMedication,
    #[doc = "A link to a resource representing the person or set of individuals to whom the medication will be given."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter during which this \\[x\\] was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Include additional information (for example, patient height and weight) that supports the ordering of the medication."]
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    #[doc = "The date (and perhaps time) when the prescription was initially written or authored on."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "The individual, organization, or device that initiated the request and has responsibility for its activation."]
    pub r#requester: Option<Box<super::super::types::Reference>>,
    #[doc = "The specified desired performer of the medication treatment (e.g. the performer of the medication administration)."]
    pub r#performer: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates the type of performer of the administration of the medication."]
    pub r#performer_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The person who entered the order on behalf of another individual for example in the case of a verbal or a telephone order."]
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    #[doc = "The reason or the indication for ordering or not ordering the medication."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Condition or observation that supports why the medication was ordered."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "The URL pointing to a protocol, guideline, orderset, or other definition that is adhered to in whole or in part by this MedicationRequest."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, orderset or other definition that is adhered to in whole or in part by this MedicationRequest."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "A plan or request that is fulfilled in whole or in part by this medication request."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "A shared identifier common to all requests that were authorized more or less simultaneously by a single author, representing the identifier of the requisition or prescription."]
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The description of the overall patte3rn of the administration of the medication to the patient."]
    pub r#course_of_therapy_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Insurance plans, coverage extensions, pre-authorizations and/or pre-determinations that may be required for delivering the requested service."]
    pub r#insurance: Vec<Box<super::super::types::Reference>>,
    #[doc = "Extra information about the prescription that could not be conveyed by the other attributes."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Indicates how the medication is to be used by the patient."]
    pub r#dosage_instruction: Vec<Box<super::super::types::Dosage>>,
    #[doc = "Indicates the specific details for the dispense or medication supply part of a medication request (also known as a Medication Prescription or Medication Order).  Note that this information is not always sent with the order.  There may be in some settings (e.g. hospitals) institutional or system support for completing the dispense details in the pharmacy department."]
    pub r#dispense_request: Option<MedicationRequestDispenseRequest>,
    #[doc = "Indicates whether or not substitution can or should be part of the dispense. In some cases, substitution must happen, in other cases substitution must not happen. This block explains the prescriber's intent. If nothing is specified substitution may be done."]
    pub r#substitution: Option<MedicationRequestSubstitution>,
    #[doc = "A link to a resource representing an earlier order related order or prescription."]
    pub r#prior_prescription: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. Drug-drug interaction, duplicate therapy, dosage alert etc."]
    pub r#detected_issue: Vec<Box<super::super::types::Reference>>,
    #[doc = "Links to Provenance records for past versions of this resource or fulfilling request or event resources that identify key state transitions or updates that are likely to be relevant to a user looking at the current version of the resource."]
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
                let some = Ok(some)?;
                state.serialize_entry("implicitRules", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("language", &some)?;
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
            let some = Ok(some)?;
            state.serialize_entry("status", &some)?;
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
            let some = Ok(some)?;
            state.serialize_entry("intent", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("priority", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("doNotPerform", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("reportedBoolean", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("authoredOn", &some)?;
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
            let values = self
                .r#instantiates_canonical
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
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
            let values = self
                .r#instantiates_uri
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
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
            Unknown(String),
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
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
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
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ImplicitRulesPrimitiveElement => {
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
                            }
                            Field::Language => {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "reportedReference",
                                    ));
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "reasonReference",
                                    ));
                                }
                                r#reason_reference = Some(map_access.next_value()?);
                            }
                            Field::InstantiatesCanonical => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
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
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::InstantiatesCanonicalPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
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
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::InstantiatesUri => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesUri",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::InstantiatesUriPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_instantiatesUri",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "groupIdentifier",
                                    ));
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "dosageInstruction",
                                    ));
                                }
                                r#dosage_instruction = Some(map_access.next_value()?);
                            }
                            Field::DispenseRequest => {
                                if r#dispense_request.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dispenseRequest",
                                    ));
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "priorPrescription",
                                    ));
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
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
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
                                            "statusReason",
                                            "intent",
                                            "category",
                                            "priority",
                                            "doNotPerform",
                                            "reportedBoolean",
                                            "reportedReference",
                                            "medicationCodeableConcept",
                                            "medicationReference",
                                            "subject",
                                            "encounter",
                                            "supportingInformation",
                                            "authoredOn",
                                            "requester",
                                            "performer",
                                            "performerType",
                                            "recorder",
                                            "reasonCode",
                                            "reasonReference",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "groupIdentifier",
                                            "courseOfTherapyType",
                                            "insurance",
                                            "note",
                                            "dosageInstruction",
                                            "dispenseRequest",
                                            "substitution",
                                            "priorPrescription",
                                            "detectedIssue",
                                            "eventHistory",
                                        ],
                                    ));
                                }
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
                        r#status: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#status_reason,
                        r#intent: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#intent.unwrap_or(Default::default())
                        } else {
                            r#intent.ok_or(serde::de::Error::missing_field("intent"))?
                        },
                        r#category: r#category.unwrap_or(vec![]),
                        r#priority,
                        r#do_not_perform,
                        r#reported,
                        r#medication: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#medication.unwrap_or(Default::default())
                        } else {
                            r#medication.ok_or(serde::de::Error::missing_field("medication[x]"))?
                        },
                        r#subject: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#subject.unwrap_or(Default::default())
                        } else {
                            r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                        },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
