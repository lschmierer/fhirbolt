// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicationDispenseStatusReason {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicationDispenseStatusReason {
    fn default() -> MedicationDispenseStatusReason {
        MedicationDispenseStatusReason::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum MedicationDispenseMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicationDispenseMedication {
    fn default() -> MedicationDispenseMedication {
        MedicationDispenseMedication::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationDispensePerformer {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#actor: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for MedicationDispensePerformer {
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
        if let Some(some) = self.r#function.as_ref() {
            state.serialize_entry("function", some)?;
        }
        state.serialize_entry("actor", &self.r#actor)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationDispensePerformer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationDispensePerformer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationDispensePerformer")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationDispensePerformer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#function: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#actor: Option<Box<super::super::types::Reference>> = None;
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
                        "function" => {
                            if r#function.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            r#function = Some(map_access.next_value()?);
                        }
                        "actor" => {
                            if r#actor.is_some() {
                                return Err(serde::de::Error::duplicate_field("actor"));
                            }
                            r#actor = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifierExtension", "function", "actor"],
                            ))
                        }
                    }
                }
                Ok(MedicationDispensePerformer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#function,
                    r#actor: r#actor.ok_or(serde::de::Error::missing_field("actor"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationDispenseSubstitution {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#was_substituted: super::super::types::Boolean,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#responsible_party: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MedicationDispenseSubstitution {
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
        if let Some(some) = self.r#was_substituted.value.as_ref() {
            state.serialize_entry("wasSubstituted", some)?;
        }
        if self.r#was_substituted.id.is_some() || !self.r#was_substituted.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#was_substituted.id,
                extension: &self.r#was_substituted.extension,
            };
            state.serialize_entry("_wasSubstituted", &primitive_element)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if !self.r#reason.is_empty() {
            state.serialize_entry("reason", &self.r#reason)?;
        }
        if !self.r#responsible_party.is_empty() {
            state.serialize_entry("responsibleParty", &self.r#responsible_party)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationDispenseSubstitution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationDispenseSubstitution;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationDispenseSubstitution")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationDispenseSubstitution, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#was_substituted: Option<super::super::types::Boolean> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#reason: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#responsible_party: Option<Vec<Box<super::super::types::Reference>>> =
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
                        "wasSubstituted" => {
                            let some = r#was_substituted.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("wasSubstituted"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_wasSubstituted" => {
                            let some = r#was_substituted.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_wasSubstituted"));
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
                        "reason" => {
                            if r#reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            r#reason = Some(map_access.next_value()?);
                        }
                        "responsibleParty" => {
                            if r#responsible_party.is_some() {
                                return Err(serde::de::Error::duplicate_field("responsibleParty"));
                            }
                            r#responsible_party = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifierExtension",
                                    "wasSubstituted",
                                    "type",
                                    "reason",
                                    "responsibleParty",
                                ],
                            ))
                        }
                    }
                }
                Ok(MedicationDispenseSubstitution {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#was_substituted: r#was_substituted
                        .ok_or(serde::de::Error::missing_field("was_substituted"))?,
                    r#type,
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#responsible_party: r#responsible_party.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationDispense {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#status_reason: Option<MedicationDispenseStatusReason>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#medication: MedicationDispenseMedication,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#context: Option<Box<super::super::types::Reference>>,
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    pub r#performer: Vec<MedicationDispensePerformer>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#authorizing_prescription: Vec<Box<super::super::types::Reference>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#days_supply: Option<Box<super::super::types::Quantity>>,
    pub r#when_prepared: Option<super::super::types::DateTime>,
    pub r#when_handed_over: Option<super::super::types::DateTime>,
    pub r#destination: Option<Box<super::super::types::Reference>>,
    pub r#receiver: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#dosage_instruction: Vec<Box<super::super::types::Dosage>>,
    pub r#substitution: Option<MedicationDispenseSubstitution>,
    pub r#detected_issue: Vec<Box<super::super::types::Reference>>,
    pub r#event_history: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MedicationDispense {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicationDispense")?;
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
        if !self.r#part_of.is_empty() {
            state.serialize_entry("partOf", &self.r#part_of)?;
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
            match some {
                MedicationDispenseStatusReason::CodeableConcept(ref value) => {
                    state.serialize_entry("statusReasonCodeableConcept", value)?;
                }
                MedicationDispenseStatusReason::Reference(ref value) => {
                    state.serialize_entry("statusReasonReference", value)?;
                }
                MedicationDispenseStatusReason::Invalid => {
                    return Err(serde::ser::Error::custom("status_reason is invalid"))
                }
            }
        }
        if let Some(some) = self.r#category.as_ref() {
            state.serialize_entry("category", some)?;
        }
        match self.r#medication {
            MedicationDispenseMedication::CodeableConcept(ref value) => {
                state.serialize_entry("medicationCodeableConcept", value)?;
            }
            MedicationDispenseMedication::Reference(ref value) => {
                state.serialize_entry("medicationReference", value)?;
            }
            MedicationDispenseMedication::Invalid => {
                return Err(serde::ser::Error::custom("medication is a required field"))
            }
        }
        if let Some(some) = self.r#subject.as_ref() {
            state.serialize_entry("subject", some)?;
        }
        if let Some(some) = self.r#context.as_ref() {
            state.serialize_entry("context", some)?;
        }
        if !self.r#supporting_information.is_empty() {
            state.serialize_entry("supportingInformation", &self.r#supporting_information)?;
        }
        if !self.r#performer.is_empty() {
            state.serialize_entry("performer", &self.r#performer)?;
        }
        if let Some(some) = self.r#location.as_ref() {
            state.serialize_entry("location", some)?;
        }
        if !self.r#authorizing_prescription.is_empty() {
            state.serialize_entry("authorizingPrescription", &self.r#authorizing_prescription)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#days_supply.as_ref() {
            state.serialize_entry("daysSupply", some)?;
        }
        if let Some(some) = self.r#when_prepared.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("whenPrepared", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_whenPrepared", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#when_handed_over.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("whenHandedOver", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_whenHandedOver", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#destination.as_ref() {
            state.serialize_entry("destination", some)?;
        }
        if !self.r#receiver.is_empty() {
            state.serialize_entry("receiver", &self.r#receiver)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if !self.r#dosage_instruction.is_empty() {
            state.serialize_entry("dosageInstruction", &self.r#dosage_instruction)?;
        }
        if let Some(some) = self.r#substitution.as_ref() {
            state.serialize_entry("substitution", some)?;
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
impl<'de> serde::de::Deserialize<'de> for MedicationDispense {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationDispense;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationDispense")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicationDispense, V::Error>
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
                let mut r#part_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#status_reason: Option<MedicationDispenseStatusReason> = None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#medication: Option<MedicationDispenseMedication> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#context: Option<Box<super::super::types::Reference>> = None;
                let mut r#supporting_information: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#performer: Option<Vec<MedicationDispensePerformer>> = None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#authorizing_prescription: Option<
                    Vec<Box<super::super::types::Reference>>,
                > = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#days_supply: Option<Box<super::super::types::Quantity>> = None;
                let mut r#when_prepared: Option<super::super::types::DateTime> = None;
                let mut r#when_handed_over: Option<super::super::types::DateTime> = None;
                let mut r#destination: Option<Box<super::super::types::Reference>> = None;
                let mut r#receiver: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#dosage_instruction: Option<Vec<Box<super::super::types::Dosage>>> = None;
                let mut r#substitution: Option<MedicationDispenseSubstitution> = None;
                let mut r#detected_issue: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#event_history: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                        "partOf" => {
                            if r#part_of.is_some() {
                                return Err(serde::de::Error::duplicate_field("partOf"));
                            }
                            r#part_of = Some(map_access.next_value()?);
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
                        "statusReasonCodeableConcept" => {
                            if r#status_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "statusReasonCodeableConcept",
                                ));
                            }
                            r#status_reason =
                                Some(MedicationDispenseStatusReason::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                        }
                        "statusReasonReference" => {
                            if r#status_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "statusReasonReference",
                                ));
                            }
                            r#status_reason = Some(MedicationDispenseStatusReason::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        "category" => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "medicationCodeableConcept" => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "medicationCodeableConcept",
                                ));
                            }
                            r#medication = Some(MedicationDispenseMedication::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "medicationReference" => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "medicationReference",
                                ));
                            }
                            r#medication = Some(MedicationDispenseMedication::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        "subject" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        "context" => {
                            if r#context.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            r#context = Some(map_access.next_value()?);
                        }
                        "supportingInformation" => {
                            if r#supporting_information.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "supportingInformation",
                                ));
                            }
                            r#supporting_information = Some(map_access.next_value()?);
                        }
                        "performer" => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(map_access.next_value()?);
                        }
                        "location" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some(map_access.next_value()?);
                        }
                        "authorizingPrescription" => {
                            if r#authorizing_prescription.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "authorizingPrescription",
                                ));
                            }
                            r#authorizing_prescription = Some(map_access.next_value()?);
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "daysSupply" => {
                            if r#days_supply.is_some() {
                                return Err(serde::de::Error::duplicate_field("daysSupply"));
                            }
                            r#days_supply = Some(map_access.next_value()?);
                        }
                        "whenPrepared" => {
                            let some = r#when_prepared.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("whenPrepared"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_whenPrepared" => {
                            let some = r#when_prepared.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_whenPrepared"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "whenHandedOver" => {
                            let some = r#when_handed_over.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("whenHandedOver"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_whenHandedOver" => {
                            let some = r#when_handed_over.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_whenHandedOver"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "destination" => {
                            if r#destination.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            r#destination = Some(map_access.next_value()?);
                        }
                        "receiver" => {
                            if r#receiver.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiver"));
                            }
                            r#receiver = Some(map_access.next_value()?);
                        }
                        "note" => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        "dosageInstruction" => {
                            if r#dosage_instruction.is_some() {
                                return Err(serde::de::Error::duplicate_field("dosageInstruction"));
                            }
                            r#dosage_instruction = Some(map_access.next_value()?);
                        }
                        "substitution" => {
                            if r#substitution.is_some() {
                                return Err(serde::de::Error::duplicate_field("substitution"));
                            }
                            r#substitution = Some(map_access.next_value()?);
                        }
                        "detectedIssue" => {
                            if r#detected_issue.is_some() {
                                return Err(serde::de::Error::duplicate_field("detectedIssue"));
                            }
                            r#detected_issue = Some(map_access.next_value()?);
                        }
                        "eventHistory" => {
                            if r#event_history.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventHistory"));
                            }
                            r#event_history = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
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
                                    "partOf",
                                    "status",
                                    "statusReasonCodeableConcept",
                                    "statusReasonReference",
                                    "category",
                                    "medicationCodeableConcept",
                                    "medicationReference",
                                    "subject",
                                    "context",
                                    "supportingInformation",
                                    "performer",
                                    "location",
                                    "authorizingPrescription",
                                    "type",
                                    "quantity",
                                    "daysSupply",
                                    "whenPrepared",
                                    "whenHandedOver",
                                    "destination",
                                    "receiver",
                                    "note",
                                    "dosageInstruction",
                                    "substitution",
                                    "detectedIssue",
                                    "eventHistory",
                                ],
                            ))
                        }
                    }
                }
                Ok(MedicationDispense {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#part_of: r#part_of.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#status_reason,
                    r#category,
                    r#medication: r#medication
                        .ok_or(serde::de::Error::missing_field("medication"))?,
                    r#subject,
                    r#context,
                    r#supporting_information: r#supporting_information.unwrap_or(vec![]),
                    r#performer: r#performer.unwrap_or(vec![]),
                    r#location,
                    r#authorizing_prescription: r#authorizing_prescription.unwrap_or(vec![]),
                    r#type,
                    r#quantity,
                    r#days_supply,
                    r#when_prepared,
                    r#when_handed_over,
                    r#destination,
                    r#receiver: r#receiver.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#dosage_instruction: r#dosage_instruction.unwrap_or(vec![]),
                    r#substitution,
                    r#detected_issue: r#detected_issue.unwrap_or(vec![]),
                    r#event_history: r#event_history.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
