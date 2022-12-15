// Generated on 2022-12-15 by fhirbolt-codegen v0.1.0
#[doc = "Information on the possible cause of the event."]
#[derive(Default, Debug, Clone)]
pub struct AdverseEventSuspectEntityCausality {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Assessment of if the entity caused the event."]
    pub r#assessment: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "AdverseEvent.suspectEntity.causalityProductRelatedness."]
    pub r#product_relatedness: Option<super::super::types::String>,
    #[doc = "AdverseEvent.suspectEntity.causalityAuthor."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "ProbabilityScale | Bayesian | Checklist."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
}
impl crate::AnyResource for AdverseEventSuspectEntityCausality {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for AdverseEventSuspectEntityCausality {
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
            if let Some(some) = self.r#assessment.as_ref() {
                state.serialize_entry("assessment", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#product_relatedness.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("productRelatedness", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_productRelatedness", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#product_relatedness.as_ref() {
                    state.serialize_entry("productRelatedness", some)?;
                }
            }
            if let Some(some) = self.r#author.as_ref() {
                state.serialize_entry("author", some)?;
            }
            if let Some(some) = self.r#method.as_ref() {
                state.serialize_entry("method", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AdverseEventSuspectEntityCausality {
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
            #[serde(rename = "assessment")]
            Assessment,
            #[serde(rename = "productRelatedness")]
            ProductRelatedness,
            #[serde(rename = "_productRelatedness")]
            ProductRelatednessPrimitiveElement,
            #[serde(rename = "author")]
            Author,
            #[serde(rename = "method")]
            Method,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdverseEventSuspectEntityCausality;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AdverseEventSuspectEntityCausality")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<AdverseEventSuspectEntityCausality, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#assessment: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#product_relatedness: Option<super::super::types::String> = None;
                let mut r#author: Option<Box<super::super::types::Reference>> = None;
                let mut r#method: Option<Box<super::super::types::CodeableConcept>> = None;
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
                            Field::Assessment => {
                                if r#assessment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("assessment"));
                                }
                                r#assessment = Some(map_access.next_value()?);
                            }
                            Field::ProductRelatedness => {
                                let some = r#product_relatedness.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productRelatedness",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ProductRelatednessPrimitiveElement => {
                                let some = r#product_relatedness.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_productRelatedness",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Author => {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field("author"));
                                }
                                r#author = Some(map_access.next_value()?);
                            }
                            Field::Method => {
                                if r#method.is_some() {
                                    return Err(serde::de::Error::duplicate_field("method"));
                                }
                                r#method = Some(map_access.next_value()?);
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
                                        "assessment",
                                        "productRelatedness",
                                        "author",
                                        "method",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AdverseEventSuspectEntityCausality {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#assessment,
                        r#product_relatedness,
                        r#author,
                        r#method,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Describes the entity that is suspected to have caused the adverse event."]
#[derive(Default, Debug, Clone)]
pub struct AdverseEventSuspectEntity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies the actual instance of what caused the adverse event.  May be a substance, medication, medication administration, medication statement or a device."]
    pub r#instance: Box<super::super::types::Reference>,
    #[doc = "Information on the possible cause of the event."]
    pub r#causality: Vec<AdverseEventSuspectEntityCausality>,
}
impl serde::ser::Serialize for AdverseEventSuspectEntity {
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
            state.serialize_entry("instance", &self.r#instance)?;
            if !self.r#causality.is_empty() {
                state.serialize_entry("causality", &self.r#causality)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AdverseEventSuspectEntity {
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
            #[serde(rename = "instance")]
            Instance,
            #[serde(rename = "causality")]
            Causality,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdverseEventSuspectEntity;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AdverseEventSuspectEntity")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AdverseEventSuspectEntity, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#instance: Option<Box<super::super::types::Reference>> = None;
                let mut r#causality: Option<Vec<AdverseEventSuspectEntityCausality>> = None;
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
                            Field::Instance => {
                                if r#instance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("instance"));
                                }
                                r#instance = Some(map_access.next_value()?);
                            }
                            Field::Causality => {
                                if r#causality.is_some() {
                                    return Err(serde::de::Error::duplicate_field("causality"));
                                }
                                r#causality = Some(map_access.next_value()?);
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
                                        "instance",
                                        "causality",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AdverseEventSuspectEntity {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#instance: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#instance.unwrap_or(Default::default())
                        } else {
                            r#instance.ok_or(serde::de::Error::missing_field("instance"))?
                        },
                        r#causality: r#causality.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Actual or  potential/avoided event causing unintended physical injury resulting from or contributed to by medical care, a research study or other healthcare setting factors that requires additional monitoring, treatment, or hospitalization, or that results in death."]
#[derive(Default, Debug, Clone)]
pub struct AdverseEvent {
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
    #[doc = "Business identifiers assigned to this adverse event by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Whether the event actually happened, or just had the potential to. Note that this is independent of whether anyone was affected or harmed or how severely."]
    pub r#actuality: super::super::types::Code,
    #[doc = "The overall type of event, intended for search and filtering purposes."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "This element defines the specific type of event that occurred or that was prevented from occurring."]
    pub r#event: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This subject or group impacted by the event."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter during which AdverseEvent was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date (and perhaps time) when the adverse event occurred."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Estimated or actual date the AdverseEvent began, in the opinion of the reporter."]
    pub r#detected: Option<super::super::types::DateTime>,
    #[doc = "The date on which the existence of the AdverseEvent was first recorded."]
    pub r#recorded_date: Option<super::super::types::DateTime>,
    #[doc = "Includes information about the reaction that occurred as a result of exposure to a substance (for example, a drug or a chemical)."]
    pub r#resulting_condition: Vec<Box<super::super::types::Reference>>,
    #[doc = "The information about where the adverse event occurred."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Assessment whether this event was of real importance."]
    pub r#seriousness: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Describes the severity of the adverse event, in relation to the subject. Contrast to AdverseEvent.seriousness - a severe rash might not be serious, but a mild heart problem is."]
    pub r#severity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Describes the type of outcome from the adverse event."]
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Information on who recorded the adverse event.  May be the patient or a practitioner."]
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    #[doc = "Parties that may or should contribute or have contributed information to the adverse event, which can consist of one or more activities.  Such information includes information leading to the decision to perform the activity and how to perform the activity (e.g. consultant), information that the activity itself seeks to reveal (e.g. informant of clinical history), or information about what activity was performed (e.g. informant witness)."]
    pub r#contributor: Vec<Box<super::super::types::Reference>>,
    #[doc = "Describes the entity that is suspected to have caused the adverse event."]
    pub r#suspect_entity: Vec<AdverseEventSuspectEntity>,
    #[doc = "AdverseEvent.subjectMedicalHistory."]
    pub r#subject_medical_history: Vec<Box<super::super::types::Reference>>,
    #[doc = "AdverseEvent.referenceDocument."]
    pub r#reference_document: Vec<Box<super::super::types::Reference>>,
    #[doc = "AdverseEvent.study."]
    pub r#study: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for AdverseEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "AdverseEvent")?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#actuality.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("actuality", &some)?;
                }
                if self.r#actuality.id.is_some() || !self.r#actuality.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#actuality.id.as_ref(),
                        extension: &self.r#actuality.extension,
                    };
                    state.serialize_entry("_actuality", &primitive_element)?;
                }
            } else {
                state.serialize_entry("actuality", &self.r#actuality)?;
            }
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if let Some(some) = self.r#event.as_ref() {
                state.serialize_entry("event", some)?;
            }
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#detected.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("detected", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_detected", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#detected.as_ref() {
                    state.serialize_entry("detected", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#recorded_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("recordedDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_recordedDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#recorded_date.as_ref() {
                    state.serialize_entry("recordedDate", some)?;
                }
            }
            if !self.r#resulting_condition.is_empty() {
                state.serialize_entry("resultingCondition", &self.r#resulting_condition)?;
            }
            if let Some(some) = self.r#location.as_ref() {
                state.serialize_entry("location", some)?;
            }
            if let Some(some) = self.r#seriousness.as_ref() {
                state.serialize_entry("seriousness", some)?;
            }
            if let Some(some) = self.r#severity.as_ref() {
                state.serialize_entry("severity", some)?;
            }
            if let Some(some) = self.r#outcome.as_ref() {
                state.serialize_entry("outcome", some)?;
            }
            if let Some(some) = self.r#recorder.as_ref() {
                state.serialize_entry("recorder", some)?;
            }
            if !self.r#contributor.is_empty() {
                state.serialize_entry("contributor", &self.r#contributor)?;
            }
            if !self.r#suspect_entity.is_empty() {
                state.serialize_entry("suspectEntity", &self.r#suspect_entity)?;
            }
            if !self.r#subject_medical_history.is_empty() {
                state.serialize_entry("subjectMedicalHistory", &self.r#subject_medical_history)?;
            }
            if !self.r#reference_document.is_empty() {
                state.serialize_entry("referenceDocument", &self.r#reference_document)?;
            }
            if !self.r#study.is_empty() {
                state.serialize_entry("study", &self.r#study)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AdverseEvent {
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
            #[serde(rename = "actuality")]
            Actuality,
            #[serde(rename = "_actuality")]
            ActualityPrimitiveElement,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "event")]
            Event,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "detected")]
            Detected,
            #[serde(rename = "_detected")]
            DetectedPrimitiveElement,
            #[serde(rename = "recordedDate")]
            RecordedDate,
            #[serde(rename = "_recordedDate")]
            RecordedDatePrimitiveElement,
            #[serde(rename = "resultingCondition")]
            ResultingCondition,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "seriousness")]
            Seriousness,
            #[serde(rename = "severity")]
            Severity,
            #[serde(rename = "outcome")]
            Outcome,
            #[serde(rename = "recorder")]
            Recorder,
            #[serde(rename = "contributor")]
            Contributor,
            #[serde(rename = "suspectEntity")]
            SuspectEntity,
            #[serde(rename = "subjectMedicalHistory")]
            SubjectMedicalHistory,
            #[serde(rename = "referenceDocument")]
            ReferenceDocument,
            #[serde(rename = "study")]
            Study,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdverseEvent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AdverseEvent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AdverseEvent, V::Error>
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
                let mut r#actuality: Option<super::super::types::Code> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#event: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#detected: Option<super::super::types::DateTime> = None;
                let mut r#recorded_date: Option<super::super::types::DateTime> = None;
                let mut r#resulting_condition: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#seriousness: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#severity: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#outcome: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#recorder: Option<Box<super::super::types::Reference>> = None;
                let mut r#contributor: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#suspect_entity: Option<Vec<AdverseEventSuspectEntity>> = None;
                let mut r#subject_medical_history: Option<
                    Vec<Box<super::super::types::Reference>>,
                > = None;
                let mut r#reference_document: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#study: Option<Vec<Box<super::super::types::Reference>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "AdverseEvent" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"AdverseEvent",
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
                            Field::Actuality => {
                                let some = r#actuality.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("actuality"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ActualityPrimitiveElement => {
                                let some = r#actuality.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_actuality"));
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
                            Field::Event => {
                                if r#event.is_some() {
                                    return Err(serde::de::Error::duplicate_field("event"));
                                }
                                r#event = Some(map_access.next_value()?);
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
                            Field::Date => {
                                let some = r#date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Detected => {
                                let some = r#detected.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detected"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DetectedPrimitiveElement => {
                                let some = r#detected.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_detected"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::RecordedDate => {
                                let some = r#recorded_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recordedDate"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::RecordedDatePrimitiveElement => {
                                let some = r#recorded_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_recordedDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::ResultingCondition => {
                                if r#resulting_condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "resultingCondition",
                                    ));
                                }
                                r#resulting_condition = Some(map_access.next_value()?);
                            }
                            Field::Location => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                r#location = Some(map_access.next_value()?);
                            }
                            Field::Seriousness => {
                                if r#seriousness.is_some() {
                                    return Err(serde::de::Error::duplicate_field("seriousness"));
                                }
                                r#seriousness = Some(map_access.next_value()?);
                            }
                            Field::Severity => {
                                if r#severity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("severity"));
                                }
                                r#severity = Some(map_access.next_value()?);
                            }
                            Field::Outcome => {
                                if r#outcome.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcome"));
                                }
                                r#outcome = Some(map_access.next_value()?);
                            }
                            Field::Recorder => {
                                if r#recorder.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recorder"));
                                }
                                r#recorder = Some(map_access.next_value()?);
                            }
                            Field::Contributor => {
                                if r#contributor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contributor"));
                                }
                                r#contributor = Some(map_access.next_value()?);
                            }
                            Field::SuspectEntity => {
                                if r#suspect_entity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("suspectEntity"));
                                }
                                r#suspect_entity = Some(map_access.next_value()?);
                            }
                            Field::SubjectMedicalHistory => {
                                if r#subject_medical_history.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subjectMedicalHistory",
                                    ));
                                }
                                r#subject_medical_history = Some(map_access.next_value()?);
                            }
                            Field::ReferenceDocument => {
                                if r#reference_document.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceDocument",
                                    ));
                                }
                                r#reference_document = Some(map_access.next_value()?);
                            }
                            Field::Study => {
                                if r#study.is_some() {
                                    return Err(serde::de::Error::duplicate_field("study"));
                                }
                                r#study = Some(map_access.next_value()?);
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
                                        "actuality",
                                        "category",
                                        "event",
                                        "subject",
                                        "encounter",
                                        "date",
                                        "detected",
                                        "recordedDate",
                                        "resultingCondition",
                                        "location",
                                        "seriousness",
                                        "severity",
                                        "outcome",
                                        "recorder",
                                        "contributor",
                                        "suspectEntity",
                                        "subjectMedicalHistory",
                                        "referenceDocument",
                                        "study",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AdverseEvent {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier,
                        r#actuality: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#actuality.unwrap_or(Default::default())
                        } else {
                            r#actuality.ok_or(serde::de::Error::missing_field("actuality"))?
                        },
                        r#category: r#category.unwrap_or(vec![]),
                        r#event,
                        r#subject: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#subject.unwrap_or(Default::default())
                        } else {
                            r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                        },
                        r#encounter,
                        r#date,
                        r#detected,
                        r#recorded_date,
                        r#resulting_condition: r#resulting_condition.unwrap_or(vec![]),
                        r#location,
                        r#seriousness,
                        r#severity,
                        r#outcome,
                        r#recorder,
                        r#contributor: r#contributor.unwrap_or(vec![]),
                        r#suspect_entity: r#suspect_entity.unwrap_or(vec![]),
                        r#subject_medical_history: r#subject_medical_history.unwrap_or(vec![]),
                        r#reference_document: r#reference_document.unwrap_or(vec![]),
                        r#study: r#study.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
