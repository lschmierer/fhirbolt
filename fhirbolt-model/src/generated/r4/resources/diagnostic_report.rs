// Generated on 2022-10-13 by fhirbolt-codegen v0.1.0
#[doc = "The time or time-period the observed values are related to. When the subject of the report is a patient, this is usually either the time of the procedure or of specimen collection(s), but very often the source of the date/time is not known, only the date/time itself."]
#[derive(Debug, Clone)]
pub enum DiagnosticReportEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for DiagnosticReportEffective {
    fn default() -> DiagnosticReportEffective {
        DiagnosticReportEffective::Invalid
    }
}
#[doc = "A list of key images associated with this report. The images are generally created during the diagnostic process, and may be directly of the patient, or of treated specimens (i.e. slides of interest)."]
#[derive(Default, Debug, Clone)]
pub struct DiagnosticReportMedia {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A comment about the image. Typically, this is used to provide an explanation for why the image is included, or to draw the viewer's attention to important features."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "Reference to the image source."]
    pub r#link: Box<super::super::types::Reference>,
}
impl crate::AnyResource for DiagnosticReportMedia {}
impl serde::ser::Serialize for DiagnosticReportMedia {
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
        if let Some(some) = self.r#comment.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("comment", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_comment", &primitive_element)?;
            }
        }
        state.serialize_entry("link", &self.r#link)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DiagnosticReportMedia {
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
            #[serde(rename = "comment")]
            Comment,
            #[serde(rename = "_comment")]
            CommentPrimitiveElement,
            #[serde(rename = "link")]
            Link,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DiagnosticReportMedia;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DiagnosticReportMedia")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DiagnosticReportMedia, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#comment: Option<super::super::types::String> = None;
                let mut r#link: Option<Box<super::super::types::Reference>> = None;
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
                            Field::Comment => {
                                let some = r#comment.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comment"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Link => {
                                if r#link.is_some() {
                                    return Err(serde::de::Error::duplicate_field("link"));
                                }
                                r#link = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "comment",
                                            "link",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(DiagnosticReportMedia {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#comment,
                        r#link: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#link.unwrap_or(Default::default())
                        } else {
                            r#link.ok_or(serde::de::Error::missing_field("link"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The findings and interpretation of diagnostic  tests performed on patients, groups of patients, devices, and locations, and/or specimens derived from these. The report includes clinical context such as requesting and provider information, and some mix of atomic results, images, textual and coded interpretations, and formatted representation of diagnostic reports.\n\nTo support reporting for any diagnostic report into a clinical data repository."]
#[derive(Default, Debug, Clone)]
pub struct DiagnosticReport {
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
    #[doc = "Identifiers assigned to this report by the performer or other systems."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Details concerning a service requested."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "The status of the diagnostic report."]
    pub r#status: super::super::types::Code,
    #[doc = "A code that classifies the clinical discipline, department or diagnostic service that created the report (e.g. cardiology, biochemistry, hematology, MRI). This is used for searching, sorting and display purposes."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code or name that describes this diagnostic report."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The subject of the report. Usually, but not always, this is a patient. However, diagnostic services also perform analyses on specimens collected from a variety of other sources."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The healthcare event  (e.g. a patient and healthcare provider interaction) which this DiagnosticReport is about."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The time or time-period the observed values are related to. When the subject of the report is a patient, this is usually either the time of the procedure or of specimen collection(s), but very often the source of the date/time is not known, only the date/time itself."]
    pub r#effective: Option<DiagnosticReportEffective>,
    #[doc = "The date and time that this version of the report was made available to providers, typically after the report was reviewed and verified."]
    pub r#issued: Option<super::super::types::Instant>,
    #[doc = "The diagnostic service that is responsible for issuing the report."]
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    #[doc = "The practitioner or organization that is responsible for the report's conclusions and interpretations."]
    pub r#results_interpreter: Vec<Box<super::super::types::Reference>>,
    #[doc = "Details about the specimens on which this diagnostic report is based."]
    pub r#specimen: Vec<Box<super::super::types::Reference>>,
    #[doc = "[Observations](https://hl7.org/FHIR/observation.html))  that are part of this diagnostic report."]
    pub r#result: Vec<Box<super::super::types::Reference>>,
    #[doc = "One or more links to full details of any imaging performed during the diagnostic investigation. Typically, this is imaging performed by DICOM enabled modalities, but this is not required. A fully enabled PACS viewer can use this information to provide views of the source images."]
    pub r#imaging_study: Vec<Box<super::super::types::Reference>>,
    #[doc = "A list of key images associated with this report. The images are generally created during the diagnostic process, and may be directly of the patient, or of treated specimens (i.e. slides of interest)."]
    pub r#media: Vec<DiagnosticReportMedia>,
    #[doc = "Concise and clinically contextualized summary conclusion (interpretation/impression) of the diagnostic report."]
    pub r#conclusion: Option<super::super::types::String>,
    #[doc = "One or more codes that represent the summary conclusion (interpretation/impression) of the diagnostic report."]
    pub r#conclusion_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Rich text representation of the entire result as issued by the diagnostic service. Multiple formats are allowed but they SHALL be semantically equivalent."]
    pub r#presented_form: Vec<Box<super::super::types::Attachment>>,
}
impl serde::ser::Serialize for DiagnosticReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "DiagnosticReport")?;
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
        if !self.r#based_on.is_empty() {
            state.serialize_entry("basedOn", &self.r#based_on)?;
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
        if !self.r#category.is_empty() {
            state.serialize_entry("category", &self.r#category)?;
        }
        state.serialize_entry("code", &self.r#code)?;
        if let Some(some) = self.r#subject.as_ref() {
            state.serialize_entry("subject", some)?;
        }
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#effective.as_ref() {
            match some {
                DiagnosticReportEffective::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("effectiveDateTime", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_effectiveDateTime", &primitive_element)?;
                    }
                }
                DiagnosticReportEffective::Period(ref value) => {
                    state.serialize_entry("effectivePeriod", value)?;
                }
                DiagnosticReportEffective::Invalid => {
                    return Err(serde::ser::Error::custom("effective is invalid"))
                }
            }
        }
        if let Some(some) = self.r#issued.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("issued", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_issued", &primitive_element)?;
            }
        }
        if !self.r#performer.is_empty() {
            state.serialize_entry("performer", &self.r#performer)?;
        }
        if !self.r#results_interpreter.is_empty() {
            state.serialize_entry("resultsInterpreter", &self.r#results_interpreter)?;
        }
        if !self.r#specimen.is_empty() {
            state.serialize_entry("specimen", &self.r#specimen)?;
        }
        if !self.r#result.is_empty() {
            state.serialize_entry("result", &self.r#result)?;
        }
        if !self.r#imaging_study.is_empty() {
            state.serialize_entry("imagingStudy", &self.r#imaging_study)?;
        }
        if !self.r#media.is_empty() {
            state.serialize_entry("media", &self.r#media)?;
        }
        if let Some(some) = self.r#conclusion.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("conclusion", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_conclusion", &primitive_element)?;
            }
        }
        if !self.r#conclusion_code.is_empty() {
            state.serialize_entry("conclusionCode", &self.r#conclusion_code)?;
        }
        if !self.r#presented_form.is_empty() {
            state.serialize_entry("presentedForm", &self.r#presented_form)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DiagnosticReport {
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
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "code")]
            Code,
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
            #[serde(rename = "issued")]
            Issued,
            #[serde(rename = "_issued")]
            IssuedPrimitiveElement,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "resultsInterpreter")]
            ResultsInterpreter,
            #[serde(rename = "specimen")]
            Specimen,
            #[serde(rename = "result")]
            Result,
            #[serde(rename = "imagingStudy")]
            ImagingStudy,
            #[serde(rename = "media")]
            Media,
            #[serde(rename = "conclusion")]
            Conclusion,
            #[serde(rename = "_conclusion")]
            ConclusionPrimitiveElement,
            #[serde(rename = "conclusionCode")]
            ConclusionCode,
            #[serde(rename = "presentedForm")]
            PresentedForm,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DiagnosticReport;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DiagnosticReport")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DiagnosticReport, V::Error>
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
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#effective: Option<DiagnosticReportEffective> = None;
                let mut r#issued: Option<super::super::types::Instant> = None;
                let mut r#performer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#results_interpreter: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#specimen: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#result: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#imaging_study: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#media: Option<Vec<DiagnosticReportMedia>> = None;
                let mut r#conclusion: Option<super::super::types::String> = None;
                let mut r#conclusion_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#presented_form: Option<Vec<Box<super::super::types::Attachment>>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "DiagnosticReport" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"DiagnosticReport",
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
                            Field::BasedOn => {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some(map_access.next_value()?);
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
                                let r#enum = r#effective.get_or_insert(
                                    DiagnosticReportEffective::DateTime(Default::default()),
                                );
                                if let DiagnosticReportEffective::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "effectiveDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("effective[x]"));
                                }
                            }
                            Field::EffectiveDateTimePrimitiveElement => {
                                let r#enum = r#effective.get_or_insert(
                                    DiagnosticReportEffective::DateTime(Default::default()),
                                );
                                if let DiagnosticReportEffective::DateTime(variant) = r#enum {
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
                                    return Err(serde::de::Error::duplicate_field("_effective[x]"));
                                }
                            }
                            Field::EffectivePeriod => {
                                if r#effective.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "effectivePeriod",
                                    ));
                                }
                                r#effective = Some(DiagnosticReportEffective::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Issued => {
                                let some = r#issued.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issued"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::IssuedPrimitiveElement => {
                                let some = r#issued.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_issued"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Performer => {
                                if r#performer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performer"));
                                }
                                r#performer = Some(map_access.next_value()?);
                            }
                            Field::ResultsInterpreter => {
                                if r#results_interpreter.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "resultsInterpreter",
                                    ));
                                }
                                r#results_interpreter = Some(map_access.next_value()?);
                            }
                            Field::Specimen => {
                                if r#specimen.is_some() {
                                    return Err(serde::de::Error::duplicate_field("specimen"));
                                }
                                r#specimen = Some(map_access.next_value()?);
                            }
                            Field::Result => {
                                if r#result.is_some() {
                                    return Err(serde::de::Error::duplicate_field("result"));
                                }
                                r#result = Some(map_access.next_value()?);
                            }
                            Field::ImagingStudy => {
                                if r#imaging_study.is_some() {
                                    return Err(serde::de::Error::duplicate_field("imagingStudy"));
                                }
                                r#imaging_study = Some(map_access.next_value()?);
                            }
                            Field::Media => {
                                if r#media.is_some() {
                                    return Err(serde::de::Error::duplicate_field("media"));
                                }
                                r#media = Some(map_access.next_value()?);
                            }
                            Field::Conclusion => {
                                let some = r#conclusion.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("conclusion"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ConclusionPrimitiveElement => {
                                let some = r#conclusion.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_conclusion"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::ConclusionCode => {
                                if r#conclusion_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "conclusionCode",
                                    ));
                                }
                                r#conclusion_code = Some(map_access.next_value()?);
                            }
                            Field::PresentedForm => {
                                if r#presented_form.is_some() {
                                    return Err(serde::de::Error::duplicate_field("presentedForm"));
                                }
                                r#presented_form = Some(map_access.next_value()?);
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
                                            "basedOn",
                                            "status",
                                            "category",
                                            "code",
                                            "subject",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "issued",
                                            "performer",
                                            "resultsInterpreter",
                                            "specimen",
                                            "result",
                                            "imagingStudy",
                                            "media",
                                            "conclusion",
                                            "conclusionCode",
                                            "presentedForm",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(DiagnosticReport {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#based_on: r#based_on.unwrap_or(vec![]),
                        r#status: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#category: r#category.unwrap_or(vec![]),
                        r#code: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#subject,
                        r#encounter,
                        r#effective,
                        r#issued,
                        r#performer: r#performer.unwrap_or(vec![]),
                        r#results_interpreter: r#results_interpreter.unwrap_or(vec![]),
                        r#specimen: r#specimen.unwrap_or(vec![]),
                        r#result: r#result.unwrap_or(vec![]),
                        r#imaging_study: r#imaging_study.unwrap_or(vec![]),
                        r#media: r#media.unwrap_or(vec![]),
                        r#conclusion,
                        r#conclusion_code: r#conclusion_code.unwrap_or(vec![]),
                        r#presented_form: r#presented_form.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
