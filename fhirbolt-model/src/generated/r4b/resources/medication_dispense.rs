// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "Indicates the reason why a dispense was not performed."]
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
#[doc = "Identifies the medication being administered. This is either a link to a resource representing the details of the medication or a simple attribute carrying a code that identifies the medication from a known list of medications."]
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
#[doc = "Indicates who or what performed the event."]
#[derive(Default, Debug, Clone)]
pub struct MedicationDispensePerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Distinguishes the type of performer in the dispense.  For example, date enterer, packager, final checker."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The device, practitioner, etc. who performed the action.  It should be assumed that the actor is the dispenser of the medication."]
    pub r#actor: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for MedicationDispensePerformer {
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
            if let Some(some) = self.r#function.as_ref() {
                state.serialize_entry("function", some)?;
            }
            state.serialize_entry("actor", &self.r#actor)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationDispensePerformer {
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
            #[serde(rename = "function")]
            Function,
            #[serde(rename = "actor")]
            Actor,
            Unknown(String),
        }
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
                            Field::Function => {
                                if r#function.is_some() {
                                    return Err(serde::de::Error::duplicate_field("function"));
                                }
                                r#function = Some(map_access.next_value()?);
                            }
                            Field::Actor => {
                                if r#actor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("actor"));
                                }
                                r#actor = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "function", "actor"],
                                ));
                            },
                        }
                    }
                    Ok(MedicationDispensePerformer {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#function,
                        r#actor: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#actor.unwrap_or(Default::default())
                        } else {
                            r#actor.ok_or(serde::de::Error::missing_field("actor"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Indicates whether or not substitution was made as part of the dispense.  In some cases, substitution will be expected but does not happen, in other cases substitution is not expected but does happen.  This block explains what substitution did or did not happen and why.  If nothing is specified, substitution was not done."]
#[derive(Default, Debug, Clone)]
pub struct MedicationDispenseSubstitution {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "True if the dispenser dispensed a different drug or product from what was prescribed."]
    pub r#was_substituted: super::super::types::Boolean,
    #[doc = "A code signifying whether a different drug was dispensed from what was prescribed."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the reason for the substitution (or lack of substitution) from what was prescribed."]
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The person or organization that has primary responsibility for the substitution."]
    pub r#responsible_party: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MedicationDispenseSubstitution {
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
                if let Some(some) = self.r#was_substituted.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("wasSubstituted", &some)?;
                }
                if self.r#was_substituted.id.is_some()
                    || !self.r#was_substituted.extension.is_empty()
                {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#was_substituted.id.as_ref(),
                        extension: &self.r#was_substituted.extension,
                    };
                    state.serialize_entry("_wasSubstituted", &primitive_element)?;
                }
            } else {
                state.serialize_entry("wasSubstituted", &self.r#was_substituted)?;
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
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationDispenseSubstitution {
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
            #[serde(rename = "wasSubstituted")]
            WasSubstituted,
            #[serde(rename = "_wasSubstituted")]
            WasSubstitutedPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "reason")]
            Reason,
            #[serde(rename = "responsibleParty")]
            ResponsibleParty,
            Unknown(String),
        }
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
                            Field::WasSubstituted => {
                                if _ctx.from_json {
                                    let some = r#was_substituted.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "wasSubstituted",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#was_substituted.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "wasSubstituted",
                                        ));
                                    }
                                    r#was_substituted = Some(map_access.next_value()?);
                                }
                            }
                            Field::WasSubstitutedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#was_substituted.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_wasSubstituted",
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
                                        "wasSubstituted",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "wasSubstituted",
                                            "type",
                                            "reason",
                                            "responsibleParty",
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
                            Field::Reason => {
                                if r#reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                r#reason = Some(map_access.next_value()?);
                            }
                            Field::ResponsibleParty => {
                                if r#responsible_party.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "responsibleParty",
                                    ));
                                }
                                r#responsible_party = Some(map_access.next_value()?);
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
                                        "wasSubstituted",
                                        "type",
                                        "reason",
                                        "responsibleParty",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MedicationDispenseSubstitution {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#was_substituted: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#was_substituted.unwrap_or(Default::default())
                        } else {
                            r#was_substituted
                                .ok_or(serde::de::Error::missing_field("wasSubstituted"))?
                        },
                        r#type,
                        r#reason: r#reason.unwrap_or(vec![]),
                        r#responsible_party: r#responsible_party.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Indicates that a medication product is to be or has been dispensed for a named person/patient.  This includes a description of the medication product (supply) provided and the instructions for administering the medication.  The medication dispense is the result of a pharmacy system responding to a medication order."]
#[derive(Default, Debug, Clone)]
pub struct MedicationDispense {
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
    #[doc = "Identifiers associated with this Medication Dispense that are defined by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate. They are business identifiers assigned to this resource by the performer or other systems and remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The procedure that trigger the dispense."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "A code specifying the state of the set of dispense events."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the reason why a dispense was not performed."]
    pub r#status_reason: Option<MedicationDispenseStatusReason>,
    #[doc = "Indicates the type of medication dispense (for example, where the medication is expected to be consumed or administered (i.e. inpatient or outpatient))."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the medication being administered. This is either a link to a resource representing the details of the medication or a simple attribute carrying a code that identifies the medication from a known list of medications."]
    pub r#medication: MedicationDispenseMedication,
    #[doc = "A link to a resource representing the person or the group to whom the medication will be given."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The encounter or episode of care that establishes the context for this event."]
    pub r#context: Option<Box<super::super::types::Reference>>,
    #[doc = "Additional information that supports the medication being dispensed."]
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    #[doc = "Indicates who or what performed the event."]
    pub r#performer: Vec<MedicationDispensePerformer>,
    #[doc = "The principal physical location where the dispense was performed."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates the medication order that is being dispensed against."]
    pub r#authorizing_prescription: Vec<Box<super::super::types::Reference>>,
    #[doc = "Indicates the type of dispensing event that is performed. For example, Trial Fill, Completion of Trial, Partial Fill, Emergency Fill, Samples, etc."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The amount of medication that has been dispensed. Includes unit of measure."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The amount of medication expressed as a timing amount."]
    pub r#days_supply: Option<Box<super::super::types::Quantity>>,
    #[doc = "The time when the dispensed product was packaged and reviewed."]
    pub r#when_prepared: Option<super::super::types::DateTime>,
    #[doc = "The time the dispensed product was provided to the patient or their representative."]
    pub r#when_handed_over: Option<super::super::types::DateTime>,
    #[doc = "Identification of the facility/location where the medication was shipped to, as part of the dispense event."]
    pub r#destination: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies the person who picked up the medication.  This will usually be a patient or their caregiver, but some cases exist where it can be a healthcare professional."]
    pub r#receiver: Vec<Box<super::super::types::Reference>>,
    #[doc = "Extra information about the dispense that could not be conveyed in the other attributes."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Indicates how the medication is to be used by the patient."]
    pub r#dosage_instruction: Vec<Box<super::super::types::Dosage>>,
    #[doc = "Indicates whether or not substitution was made as part of the dispense.  In some cases, substitution will be expected but does not happen, in other cases substitution is not expected but does happen.  This block explains what substitution did or did not happen and why.  If nothing is specified, substitution was not done."]
    pub r#substitution: Option<MedicationDispenseSubstitution>,
    #[doc = "Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. drug-drug interaction, duplicate therapy, dosage alert etc."]
    pub r#detected_issue: Vec<Box<super::super::types::Reference>>,
    #[doc = "A summary of the events of interest that have occurred, such as when the dispense was verified."]
    pub r#event_history: Vec<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for MedicationDispense {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for MedicationDispense {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "MedicationDispense")?;
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
            if !self.r#part_of.is_empty() {
                state.serialize_entry("partOf", &self.r#part_of)?;
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
                state
                    .serialize_entry("authorizingPrescription", &self.r#authorizing_prescription)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#when_prepared.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("whenPrepared", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_whenPrepared", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#when_prepared.as_ref() {
                    state.serialize_entry("whenPrepared", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#when_handed_over.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("whenHandedOver", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_whenHandedOver", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#when_handed_over.as_ref() {
                    state.serialize_entry("whenHandedOver", some)?;
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
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationDispense {
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
            #[serde(rename = "partOf")]
            PartOf,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "statusReasonCodeableConcept")]
            StatusReasonCodeableConcept,
            #[serde(rename = "statusReasonReference")]
            StatusReasonReference,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "medicationCodeableConcept")]
            MedicationCodeableConcept,
            #[serde(rename = "medicationReference")]
            MedicationReference,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "context")]
            Context,
            #[serde(rename = "supportingInformation")]
            SupportingInformation,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "authorizingPrescription")]
            AuthorizingPrescription,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "daysSupply")]
            DaysSupply,
            #[serde(rename = "whenPrepared")]
            WhenPrepared,
            #[serde(rename = "_whenPrepared")]
            WhenPreparedPrimitiveElement,
            #[serde(rename = "whenHandedOver")]
            WhenHandedOver,
            #[serde(rename = "_whenHandedOver")]
            WhenHandedOverPrimitiveElement,
            #[serde(rename = "destination")]
            Destination,
            #[serde(rename = "receiver")]
            Receiver,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "dosageInstruction")]
            DosageInstruction,
            #[serde(rename = "substitution")]
            Substitution,
            #[serde(rename = "detectedIssue")]
            DetectedIssue,
            #[serde(rename = "eventHistory")]
            EventHistory,
            Unknown(String),
        }
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
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "MedicationDispense" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"MedicationDispense",
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
                            Field::PartOf => {
                                if r#part_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partOf"));
                                }
                                r#part_of = Some(map_access.next_value()?);
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
                                    ));
                                }
                            }
                            Field::StatusReasonCodeableConcept => {
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
                            Field::StatusReasonReference => {
                                if r#status_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "statusReasonReference",
                                    ));
                                }
                                r#status_reason = Some(MedicationDispenseStatusReason::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::MedicationCodeableConcept => {
                                if r#medication.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "medicationCodeableConcept",
                                    ));
                                }
                                r#medication = Some(MedicationDispenseMedication::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::MedicationReference => {
                                if r#medication.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "medicationReference",
                                    ));
                                }
                                r#medication = Some(MedicationDispenseMedication::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Context => {
                                if r#context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("context"));
                                }
                                r#context = Some(map_access.next_value()?);
                            }
                            Field::SupportingInformation => {
                                if r#supporting_information.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "supportingInformation",
                                    ));
                                }
                                r#supporting_information = Some(map_access.next_value()?);
                            }
                            Field::Performer => {
                                if r#performer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performer"));
                                }
                                r#performer = Some(map_access.next_value()?);
                            }
                            Field::Location => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                r#location = Some(map_access.next_value()?);
                            }
                            Field::AuthorizingPrescription => {
                                if r#authorizing_prescription.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizingPrescription",
                                    ));
                                }
                                r#authorizing_prescription = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::DaysSupply => {
                                if r#days_supply.is_some() {
                                    return Err(serde::de::Error::duplicate_field("daysSupply"));
                                }
                                r#days_supply = Some(map_access.next_value()?);
                            }
                            Field::WhenPrepared => {
                                if _ctx.from_json {
                                    let some = r#when_prepared.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "whenPrepared",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#when_prepared.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "whenPrepared",
                                        ));
                                    }
                                    r#when_prepared = Some(map_access.next_value()?);
                                }
                            }
                            Field::WhenPreparedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#when_prepared.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_whenPrepared",
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
                                        "whenPrepared",
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
                                    ));
                                }
                            }
                            Field::WhenHandedOver => {
                                if _ctx.from_json {
                                    let some = r#when_handed_over.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "whenHandedOver",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#when_handed_over.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "whenHandedOver",
                                        ));
                                    }
                                    r#when_handed_over = Some(map_access.next_value()?);
                                }
                            }
                            Field::WhenHandedOverPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#when_handed_over.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_whenHandedOver",
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
                                        "whenHandedOver",
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
                                    ));
                                }
                            }
                            Field::Destination => {
                                if r#destination.is_some() {
                                    return Err(serde::de::Error::duplicate_field("destination"));
                                }
                                r#destination = Some(map_access.next_value()?);
                            }
                            Field::Receiver => {
                                if r#receiver.is_some() {
                                    return Err(serde::de::Error::duplicate_field("receiver"));
                                }
                                r#receiver = Some(map_access.next_value()?);
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
                            Field::Substitution => {
                                if r#substitution.is_some() {
                                    return Err(serde::de::Error::duplicate_field("substitution"));
                                }
                                r#substitution = Some(map_access.next_value()?);
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
                                ));
                            },
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
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#status_reason,
                        r#category,
                        r#medication: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#medication.unwrap_or(Default::default())
                        } else {
                            r#medication.ok_or(serde::de::Error::missing_field("medication[x]"))?
                        },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
