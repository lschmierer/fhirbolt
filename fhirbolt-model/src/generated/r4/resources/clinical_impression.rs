// Generated on 2023-04-05 by fhirbolt-codegen v0.1.0
#[doc = "The point in time or period over which the subject was assessed."]
#[derive(Debug, Clone, PartialEq)]
pub enum ClinicalImpressionEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ClinicalImpressionEffective {
    fn default() -> ClinicalImpressionEffective {
        ClinicalImpressionEffective::Invalid
    }
}
#[doc = "One or more sets of investigations (signs, symptoms, etc.). The actual grouping of investigations varies greatly depending on the type and context of the assessment. These investigations may include data generated during the assessment process, or data previously generated and recorded that is pertinent to the outcomes."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalImpressionInvestigation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A name/code for the group (\"set\") of investigations. Typically, this will be something like \"signs\", \"symptoms\", \"clinical\", \"diagnostic\", but the list is not constrained, and others such groups such as (exposure|family|travel|nutritional) history may be used."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "A record of a specific investigation that was undertaken."]
    pub r#item: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ClinicalImpressionInvestigation {
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
            state.serialize_entry("code", &self.r#code)?;
            if !self.r#item.is_empty() {
                state.serialize_entry("item", &self.r#item)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalImpressionInvestigation {
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
            #[serde(rename = "item")]
            Item,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalImpressionInvestigation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalImpressionInvestigation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClinicalImpressionInvestigation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#item: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Item => {
                                if _ctx.from_json {
                                    if r#item.is_some() {
                                        return Err(serde::de::Error::duplicate_field("item"));
                                    }
                                    r#item = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#item.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "code", "item"],
                                ));
                            },
                        }
                    }
                    Ok(ClinicalImpressionInvestigation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#item: r#item.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Specific findings or diagnoses that were considered likely or relevant to ongoing treatment."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalImpressionFinding {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Specific text or code for finding or diagnosis, which may include ruled-out or resolved conditions."]
    pub r#item_codeable_concept: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specific reference for finding or diagnosis, which may include ruled-out or resolved conditions."]
    pub r#item_reference: Option<Box<super::super::types::Reference>>,
    #[doc = "Which investigations support finding or diagnosis."]
    pub r#basis: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ClinicalImpressionFinding {
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
            if let Some(some) = self.r#item_codeable_concept.as_ref() {
                state.serialize_entry("itemCodeableConcept", some)?;
            }
            if let Some(some) = self.r#item_reference.as_ref() {
                state.serialize_entry("itemReference", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#basis.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("basis", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_basis", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#basis.as_ref() {
                    state.serialize_entry("basis", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalImpressionFinding {
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
            #[serde(rename = "itemCodeableConcept")]
            ItemCodeableConcept,
            #[serde(rename = "itemReference")]
            ItemReference,
            #[serde(rename = "basis")]
            Basis,
            #[serde(rename = "_basis")]
            BasisPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalImpressionFinding;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalImpressionFinding")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClinicalImpressionFinding, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#item_codeable_concept: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#item_reference: Option<Box<super::super::types::Reference>> = None;
                let mut r#basis: Option<super::super::types::String> = None;
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
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ItemCodeableConcept => {
                                if r#item_codeable_concept.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "itemCodeableConcept",
                                    ));
                                }
                                r#item_codeable_concept = Some(map_access.next_value()?);
                            }
                            Field::ItemReference => {
                                if r#item_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("itemReference"));
                                }
                                r#item_reference = Some(map_access.next_value()?);
                            }
                            Field::Basis => {
                                if _ctx.from_json {
                                    let some = r#basis.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("basis"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#basis.is_some() {
                                        return Err(serde::de::Error::duplicate_field("basis"));
                                    }
                                    r#basis = Some(map_access.next_value()?);
                                }
                            }
                            Field::BasisPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#basis.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_basis"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "basis",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "itemCodeableConcept",
                                            "itemReference",
                                            "basis",
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
                                        "itemCodeableConcept",
                                        "itemReference",
                                        "basis",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ClinicalImpressionFinding {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#item_codeable_concept,
                        r#item_reference,
                        r#basis,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A record of a clinical assessment performed to determine what problem(s) may affect the patient and before planning the treatments or management strategies that are best to manage a patient's condition. Assessments are often 1:1 with a clinical consultation / encounter,  but this varies greatly depending on the clinical workflow. This resource is called \"ClinicalImpression\" rather than \"ClinicalAssessment\" to avoid confusion with the recording of assessment tools such as Apgar score."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalImpression {
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
    #[doc = "Business identifiers assigned to this clinical impression by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Identifies the workflow status of the assessment."]
    pub r#status: super::super::types::Code,
    #[doc = "Captures the reason for the current state of the ClinicalImpression."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Categorizes the type of clinical assessment performed."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A summary of the context and/or cause of the assessment - why / where it was performed, and what patient events/status prompted it."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The patient or group of individuals assessed as part of this record."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter during which this ClinicalImpression was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The point in time or period over which the subject was assessed."]
    pub r#effective: Option<ClinicalImpressionEffective>,
    #[doc = "Indicates when the documentation of the assessment was complete."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The clinician performing the assessment."]
    pub r#assessor: Option<Box<super::super::types::Reference>>,
    #[doc = "A reference to the last assessment that was conducted on this patient. Assessments are often/usually ongoing in nature; a care provider (practitioner or team) will make new assessments on an ongoing basis as new data arises or the patient's conditions changes."]
    pub r#previous: Option<Box<super::super::types::Reference>>,
    #[doc = "A list of the relevant problems/conditions for a patient."]
    pub r#problem: Vec<Box<super::super::types::Reference>>,
    #[doc = "One or more sets of investigations (signs, symptoms, etc.). The actual grouping of investigations varies greatly depending on the type and context of the assessment. These investigations may include data generated during the assessment process, or data previously generated and recorded that is pertinent to the outcomes."]
    pub r#investigation: Vec<ClinicalImpressionInvestigation>,
    #[doc = "Reference to a specific published clinical protocol that was followed during this assessment, and/or that provides evidence in support of the diagnosis."]
    pub r#protocol: Vec<super::super::types::Uri>,
    #[doc = "A text summary of the investigations and the diagnosis."]
    pub r#summary: Option<super::super::types::String>,
    #[doc = "Specific findings or diagnoses that were considered likely or relevant to ongoing treatment."]
    pub r#finding: Vec<ClinicalImpressionFinding>,
    #[doc = "Estimate of likely outcome."]
    pub r#prognosis_codeable_concept: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "RiskAssessment expressing likely outcome."]
    pub r#prognosis_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Information supporting the clinical impression."]
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    #[doc = "Commentary about the impression, typically recorded after the impression itself was made, though supplemental notes by the original author could also appear."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl crate::AnyResource for ClinicalImpression {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for ClinicalImpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ClinicalImpression")?;
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
            if let Some(some) = self.r#status_reason.as_ref() {
                state.serialize_entry("statusReason", some)?;
            }
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
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
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if let Some(some) = self.r#effective.as_ref() {
                match some {
                    ClinicalImpressionEffective::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("effectiveDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_effectiveDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("effectiveDateTime", value)?;
                        }
                    }
                    ClinicalImpressionEffective::Period(ref value) => {
                        state.serialize_entry("effectivePeriod", value)?;
                    }
                    ClinicalImpressionEffective::Invalid => {
                        return Err(serde::ser::Error::custom("effective is invalid"))
                    }
                }
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
            if let Some(some) = self.r#assessor.as_ref() {
                state.serialize_entry("assessor", some)?;
            }
            if let Some(some) = self.r#previous.as_ref() {
                state.serialize_entry("previous", some)?;
            }
            if !self.r#problem.is_empty() {
                state.serialize_entry("problem", &self.r#problem)?;
            }
            if !self.r#investigation.is_empty() {
                state.serialize_entry("investigation", &self.r#investigation)?;
            }
            if _ctx.output_json {
                if !self.r#protocol.is_empty() {
                    let values = self
                        .r#protocol
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("protocol", &values)?;
                    }
                    let requires_elements = self
                        .r#protocol
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#protocol
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
                        state.serialize_entry("_protocol", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#protocol.is_empty() {
                    state.serialize_entry("protocol", &self.r#protocol)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#summary.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("summary", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_summary", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#summary.as_ref() {
                    state.serialize_entry("summary", some)?;
                }
            }
            if !self.r#finding.is_empty() {
                state.serialize_entry("finding", &self.r#finding)?;
            }
            if !self.r#prognosis_codeable_concept.is_empty() {
                state.serialize_entry(
                    "prognosisCodeableConcept",
                    &self.r#prognosis_codeable_concept,
                )?;
            }
            if !self.r#prognosis_reference.is_empty() {
                state.serialize_entry("prognosisReference", &self.r#prognosis_reference)?;
            }
            if !self.r#supporting_info.is_empty() {
                state.serialize_entry("supportingInfo", &self.r#supporting_info)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalImpression {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "effectiveDateTime")]
            EffectiveDateTime,
            #[serde(rename = "_effectiveDateTime")]
            EffectiveDateTimePrimitiveElement,
            #[serde(rename = "effectivePeriod")]
            EffectivePeriod,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "assessor")]
            Assessor,
            #[serde(rename = "previous")]
            Previous,
            #[serde(rename = "problem")]
            Problem,
            #[serde(rename = "investigation")]
            Investigation,
            #[serde(rename = "protocol")]
            Protocol,
            #[serde(rename = "_protocol")]
            ProtocolPrimitiveElement,
            #[serde(rename = "summary")]
            Summary,
            #[serde(rename = "_summary")]
            SummaryPrimitiveElement,
            #[serde(rename = "finding")]
            Finding,
            #[serde(rename = "prognosisCodeableConcept")]
            PrognosisCodeableConcept,
            #[serde(rename = "prognosisReference")]
            PrognosisReference,
            #[serde(rename = "supportingInfo")]
            SupportingInfo,
            #[serde(rename = "note")]
            Note,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalImpression;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalImpression")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClinicalImpression, V::Error>
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
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#effective: Option<ClinicalImpressionEffective> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#assessor: Option<Box<super::super::types::Reference>> = None;
                let mut r#previous: Option<Box<super::super::types::Reference>> = None;
                let mut r#problem: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#investigation: Option<Vec<ClinicalImpressionInvestigation>> = None;
                let mut r#protocol: Option<Vec<super::super::types::Uri>> = None;
                let mut r#summary: Option<super::super::types::String> = None;
                let mut r#finding: Option<Vec<ClinicalImpressionFinding>> = None;
                let mut r#prognosis_codeable_concept: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#prognosis_reference: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#supporting_info: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "ClinicalImpression" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"ClinicalImpression",
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
                                            "statusReason",
                                            "code",
                                            "description",
                                            "subject",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "date",
                                            "assessor",
                                            "previous",
                                            "problem",
                                            "investigation",
                                            "protocol",
                                            "summary",
                                            "finding",
                                            "prognosisCodeableConcept",
                                            "prognosisReference",
                                            "supportingInfo",
                                            "note",
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
                                            "statusReason",
                                            "code",
                                            "description",
                                            "subject",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "date",
                                            "assessor",
                                            "previous",
                                            "problem",
                                            "investigation",
                                            "protocol",
                                            "summary",
                                            "finding",
                                            "prognosisCodeableConcept",
                                            "prognosisReference",
                                            "supportingInfo",
                                            "note",
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
                                if _ctx.from_json {
                                    if r#contained.is_some() {
                                        return Err(serde::de::Error::duplicate_field("contained"));
                                    }
                                    r#contained = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#contained.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Identifier => {
                                if _ctx.from_json {
                                    if r#identifier.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "identifier",
                                        ));
                                    }
                                    r#identifier = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#identifier.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                            "statusReason",
                                            "code",
                                            "description",
                                            "subject",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "date",
                                            "assessor",
                                            "previous",
                                            "problem",
                                            "investigation",
                                            "protocol",
                                            "summary",
                                            "finding",
                                            "prognosisCodeableConcept",
                                            "prognosisReference",
                                            "supportingInfo",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::StatusReason => {
                                if r#status_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusReason"));
                                }
                                r#status_reason = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
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
                                            "status",
                                            "statusReason",
                                            "code",
                                            "description",
                                            "subject",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "date",
                                            "assessor",
                                            "previous",
                                            "problem",
                                            "investigation",
                                            "protocol",
                                            "summary",
                                            "finding",
                                            "prognosisCodeableConcept",
                                            "prognosisReference",
                                            "supportingInfo",
                                            "note",
                                        ],
                                    ));
                                }
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
                            Field::EffectiveDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#effective.get_or_insert(
                                        ClinicalImpressionEffective::DateTime(Default::default()),
                                    );
                                    if let ClinicalImpressionEffective::DateTime(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "effectiveDateTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "effective[x]",
                                        ));
                                    }
                                } else {
                                    if r#effective.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "effectiveDateTime",
                                        ));
                                    }
                                    r#effective = Some(ClinicalImpressionEffective::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::EffectiveDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#effective.get_or_insert(
                                        ClinicalImpressionEffective::DateTime(Default::default()),
                                    );
                                    if let ClinicalImpressionEffective::DateTime(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_effectiveDateTime",
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
                                            "_effective[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "effectiveDateTime",
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
                                            "code",
                                            "description",
                                            "subject",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "date",
                                            "assessor",
                                            "previous",
                                            "problem",
                                            "investigation",
                                            "protocol",
                                            "summary",
                                            "finding",
                                            "prognosisCodeableConcept",
                                            "prognosisReference",
                                            "supportingInfo",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::EffectivePeriod => {
                                if r#effective.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "effectivePeriod",
                                    ));
                                }
                                r#effective = Some(ClinicalImpressionEffective::Period(
                                    map_access.next_value()?,
                                ));
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
                                            "code",
                                            "description",
                                            "subject",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "date",
                                            "assessor",
                                            "previous",
                                            "problem",
                                            "investigation",
                                            "protocol",
                                            "summary",
                                            "finding",
                                            "prognosisCodeableConcept",
                                            "prognosisReference",
                                            "supportingInfo",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Assessor => {
                                if r#assessor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("assessor"));
                                }
                                r#assessor = Some(map_access.next_value()?);
                            }
                            Field::Previous => {
                                if r#previous.is_some() {
                                    return Err(serde::de::Error::duplicate_field("previous"));
                                }
                                r#previous = Some(map_access.next_value()?);
                            }
                            Field::Problem => {
                                if _ctx.from_json {
                                    if r#problem.is_some() {
                                        return Err(serde::de::Error::duplicate_field("problem"));
                                    }
                                    r#problem = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#problem.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Investigation => {
                                if _ctx.from_json {
                                    if r#investigation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "investigation",
                                        ));
                                    }
                                    r#investigation = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#investigation.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Protocol => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#protocol.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("protocol"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    let vec = r#protocol.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ProtocolPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#protocol.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_protocol"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "protocol",
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
                                            "code",
                                            "description",
                                            "subject",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "date",
                                            "assessor",
                                            "previous",
                                            "problem",
                                            "investigation",
                                            "protocol",
                                            "summary",
                                            "finding",
                                            "prognosisCodeableConcept",
                                            "prognosisReference",
                                            "supportingInfo",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Summary => {
                                if _ctx.from_json {
                                    let some = r#summary.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("summary"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#summary.is_some() {
                                        return Err(serde::de::Error::duplicate_field("summary"));
                                    }
                                    r#summary = Some(map_access.next_value()?);
                                }
                            }
                            Field::SummaryPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#summary.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_summary"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "summary",
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
                                            "code",
                                            "description",
                                            "subject",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "date",
                                            "assessor",
                                            "previous",
                                            "problem",
                                            "investigation",
                                            "protocol",
                                            "summary",
                                            "finding",
                                            "prognosisCodeableConcept",
                                            "prognosisReference",
                                            "supportingInfo",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Finding => {
                                if _ctx.from_json {
                                    if r#finding.is_some() {
                                        return Err(serde::de::Error::duplicate_field("finding"));
                                    }
                                    r#finding = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#finding.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::PrognosisCodeableConcept => {
                                if _ctx.from_json {
                                    if r#prognosis_codeable_concept.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "prognosisCodeableConcept",
                                        ));
                                    }
                                    r#prognosis_codeable_concept = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#prognosis_codeable_concept
                                        .get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::PrognosisReference => {
                                if _ctx.from_json {
                                    if r#prognosis_reference.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "prognosisReference",
                                        ));
                                    }
                                    r#prognosis_reference = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#prognosis_reference.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::SupportingInfo => {
                                if _ctx.from_json {
                                    if r#supporting_info.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "supportingInfo",
                                        ));
                                    }
                                    r#supporting_info = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#supporting_info.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Note => {
                                if _ctx.from_json {
                                    if r#note.is_some() {
                                        return Err(serde::de::Error::duplicate_field("note"));
                                    }
                                    r#note = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#note.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                        "statusReason",
                                        "code",
                                        "description",
                                        "subject",
                                        "encounter",
                                        "effectiveDateTime",
                                        "effectivePeriod",
                                        "date",
                                        "assessor",
                                        "previous",
                                        "problem",
                                        "investigation",
                                        "protocol",
                                        "summary",
                                        "finding",
                                        "prognosisCodeableConcept",
                                        "prognosisReference",
                                        "supportingInfo",
                                        "note",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ClinicalImpression {
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
                        r#status_reason,
                        r#code,
                        r#description,
                        r#subject: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#subject.unwrap_or(Default::default())
                        } else {
                            r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                        },
                        r#encounter,
                        r#effective,
                        r#date,
                        r#assessor,
                        r#previous,
                        r#problem: r#problem.unwrap_or(vec![]),
                        r#investigation: r#investigation.unwrap_or(vec![]),
                        r#protocol: r#protocol.unwrap_or(vec![]),
                        r#summary,
                        r#finding: r#finding.unwrap_or(vec![]),
                        r#prognosis_codeable_concept: r#prognosis_codeable_concept
                            .unwrap_or(vec![]),
                        r#prognosis_reference: r#prognosis_reference.unwrap_or(vec![]),
                        r#supporting_info: r#supporting_info.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
