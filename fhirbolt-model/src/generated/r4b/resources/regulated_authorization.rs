// Generated on 2022-12-07 by fhirbolt-codegen v0.1.0
#[doc = "Relevant date for this case."]
#[derive(Debug, Clone)]
pub enum RegulatedAuthorizationCaseDate {
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
    Invalid,
}
impl Default for RegulatedAuthorizationCaseDate {
    fn default() -> RegulatedAuthorizationCaseDate {
        RegulatedAuthorizationCaseDate::Invalid
    }
}
#[doc = "The case or regulatory procedure for granting or amending a regulated authorization. An authorization is granted in response to submissions/applications by those seeking authorization. A case is the administrative process that deals with the application(s) that relate to this and assesses them. Note: This area is subject to ongoing review and the workgroup is seeking implementer feedback on its use (see link at bottom of page)."]
#[derive(Default, Debug, Clone)]
pub struct RegulatedAuthorizationCase {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifier by which this case can be referenced."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The defining type of case."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status associated with the case."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Relevant date for this case."]
    pub r#date: Option<RegulatedAuthorizationCaseDate>,
    #[doc = "A regulatory submission from an organization to a regulator, as part of an assessing case. Multiple applications may occur over time, with more or different information to support or modify the submission or the authorization. The applications can be considered as steps within the longer running case or procedure for this authorization process."]
    pub r#application: Vec<RegulatedAuthorizationCase>,
}
impl crate::AnyResource for RegulatedAuthorizationCase {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for RegulatedAuthorizationCase {
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
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#status.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if let Some(some) = self.r#date.as_ref() {
            match some {
                RegulatedAuthorizationCaseDate::Period(ref value) => {
                    state.serialize_entry("datePeriod", value)?;
                }
                RegulatedAuthorizationCaseDate::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("dateDateTime", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_dateDateTime", &primitive_element)?;
                    }
                }
                RegulatedAuthorizationCaseDate::Invalid => {
                    return Err(serde::ser::Error::custom("date is invalid"))
                }
            }
        }
        if !self.r#application.is_empty() {
            state.serialize_entry("application", &self.r#application)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for RegulatedAuthorizationCase {
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "datePeriod")]
            DatePeriod,
            #[serde(rename = "dateDateTime")]
            DateDateTime,
            #[serde(rename = "_dateDateTime")]
            DateDateTimePrimitiveElement,
            #[serde(rename = "application")]
            Application,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RegulatedAuthorizationCase;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RegulatedAuthorizationCase")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<RegulatedAuthorizationCase, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#date: Option<RegulatedAuthorizationCaseDate> = None;
                let mut r#application: Option<Vec<RegulatedAuthorizationCase>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value()?);
                            }
                            Field::DatePeriod => {
                                if r#date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("datePeriod"));
                                }
                                r#date = Some(RegulatedAuthorizationCaseDate::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::DateDateTime => {
                                let r#enum = r#date.get_or_insert(
                                    RegulatedAuthorizationCaseDate::DateTime(Default::default()),
                                );
                                if let RegulatedAuthorizationCaseDate::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "dateDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("date[x]"));
                                }
                            }
                            Field::DateDateTimePrimitiveElement => {
                                let r#enum = r#date.get_or_insert(
                                    RegulatedAuthorizationCaseDate::DateTime(Default::default()),
                                );
                                if let RegulatedAuthorizationCaseDate::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_dateDateTime",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_date[x]"));
                                }
                            }
                            Field::Application => {
                                if r#application.is_some() {
                                    return Err(serde::de::Error::duplicate_field("application"));
                                }
                                r#application = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "identifier",
                                        "type",
                                        "status",
                                        "datePeriod",
                                        "dateDateTime",
                                        "application",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(RegulatedAuthorizationCase {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier,
                        r#type,
                        r#status,
                        r#date,
                        r#application: r#application.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Regulatory approval, clearance or licencing related to a regulated product, treatment, facility or activity that is cited in a guidance, regulation, rule or legislative act. An example is Market Authorization relating to a Medicinal Product."]
#[derive(Default, Debug, Clone)]
pub struct RegulatedAuthorization {
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
    #[doc = "Business identifier for the authorization, typically assigned by the authorizing body."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The product type, treatment, facility or activity that is being authorized."]
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    #[doc = "Overall type of this authorization, for example drug marketing approval, orphan drug designation."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "General textual supporting information."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The territory (e.g., country, jurisdiction etc.) in which the authorization has been granted."]
    pub r#region: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status that is authorised e.g. approved. Intermediate states and actions can be tracked with cases and applications."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date at which the current status was assigned."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "The time period in which the regulatory approval, clearance or licencing is in effect. As an example, a Marketing Authorization includes the date of authorization and/or an expiration date."]
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    #[doc = "Condition for which the use of the regulated product applies."]
    pub r#indication: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The intended use of the product, e.g. prevention, treatment, diagnosis."]
    pub r#intended_use: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The legal or regulatory framework against which this authorization is granted, or other reasons for it."]
    pub r#basis: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The organization that has been granted this authorization, by some authoritative body (the 'regulator')."]
    pub r#holder: Option<Box<super::super::types::Reference>>,
    #[doc = "The regulatory authority or authorizing body granting the authorization. For example, European Medicines Agency (EMA), Food and Drug Administration (FDA), Health Canada (HC), etc."]
    pub r#regulator: Option<Box<super::super::types::Reference>>,
    #[doc = "The case or regulatory procedure for granting or amending a regulated authorization. An authorization is granted in response to submissions/applications by those seeking authorization. A case is the administrative process that deals with the application(s) that relate to this and assesses them. Note: This area is subject to ongoing review and the workgroup is seeking implementer feedback on its use (see link at bottom of page)."]
    pub r#case: Option<RegulatedAuthorizationCase>,
}
impl serde::ser::Serialize for RegulatedAuthorization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "RegulatedAuthorization")?;
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
        if !self.r#subject.is_empty() {
            state.serialize_entry("subject", &self.r#subject)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("description", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if !self.r#region.is_empty() {
            state.serialize_entry("region", &self.r#region)?;
        }
        if let Some(some) = self.r#status.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if let Some(some) = self.r#status_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("statusDate", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_statusDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#validity_period.as_ref() {
            state.serialize_entry("validityPeriod", some)?;
        }
        if let Some(some) = self.r#indication.as_ref() {
            state.serialize_entry("indication", some)?;
        }
        if let Some(some) = self.r#intended_use.as_ref() {
            state.serialize_entry("intendedUse", some)?;
        }
        if !self.r#basis.is_empty() {
            state.serialize_entry("basis", &self.r#basis)?;
        }
        if let Some(some) = self.r#holder.as_ref() {
            state.serialize_entry("holder", some)?;
        }
        if let Some(some) = self.r#regulator.as_ref() {
            state.serialize_entry("regulator", some)?;
        }
        if let Some(some) = self.r#case.as_ref() {
            state.serialize_entry("case", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for RegulatedAuthorization {
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
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "region")]
            Region,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "statusDate")]
            StatusDate,
            #[serde(rename = "_statusDate")]
            StatusDatePrimitiveElement,
            #[serde(rename = "validityPeriod")]
            ValidityPeriod,
            #[serde(rename = "indication")]
            Indication,
            #[serde(rename = "intendedUse")]
            IntendedUse,
            #[serde(rename = "basis")]
            Basis,
            #[serde(rename = "holder")]
            Holder,
            #[serde(rename = "regulator")]
            Regulator,
            #[serde(rename = "case")]
            Case,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RegulatedAuthorization;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RegulatedAuthorization")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<RegulatedAuthorization, V::Error>
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
                let mut r#subject: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#region: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status_date: Option<super::super::types::DateTime> = None;
                let mut r#validity_period: Option<Box<super::super::types::Period>> = None;
                let mut r#indication: Option<Box<super::super::types::CodeableReference>> = None;
                let mut r#intended_use: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#basis: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#holder: Option<Box<super::super::types::Reference>> = None;
                let mut r#regulator: Option<Box<super::super::types::Reference>> = None;
                let mut r#case: Option<RegulatedAuthorizationCase> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "RegulatedAuthorization" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"RegulatedAuthorization",
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
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Description => {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DescriptionPrimitiveElement => {
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
                            Field::Region => {
                                if r#region.is_some() {
                                    return Err(serde::de::Error::duplicate_field("region"));
                                }
                                r#region = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value()?);
                            }
                            Field::StatusDate => {
                                let some = r#status_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::StatusDatePrimitiveElement => {
                                let some = r#status_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_statusDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::ValidityPeriod => {
                                if r#validity_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "validityPeriod",
                                    ));
                                }
                                r#validity_period = Some(map_access.next_value()?);
                            }
                            Field::Indication => {
                                if r#indication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("indication"));
                                }
                                r#indication = Some(map_access.next_value()?);
                            }
                            Field::IntendedUse => {
                                if r#intended_use.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intendedUse"));
                                }
                                r#intended_use = Some(map_access.next_value()?);
                            }
                            Field::Basis => {
                                if r#basis.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basis"));
                                }
                                r#basis = Some(map_access.next_value()?);
                            }
                            Field::Holder => {
                                if r#holder.is_some() {
                                    return Err(serde::de::Error::duplicate_field("holder"));
                                }
                                r#holder = Some(map_access.next_value()?);
                            }
                            Field::Regulator => {
                                if r#regulator.is_some() {
                                    return Err(serde::de::Error::duplicate_field("regulator"));
                                }
                                r#regulator = Some(map_access.next_value()?);
                            }
                            Field::Case => {
                                if r#case.is_some() {
                                    return Err(serde::de::Error::duplicate_field("case"));
                                }
                                r#case = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
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
                                        "subject",
                                        "type",
                                        "description",
                                        "region",
                                        "status",
                                        "statusDate",
                                        "validityPeriod",
                                        "indication",
                                        "intendedUse",
                                        "basis",
                                        "holder",
                                        "regulator",
                                        "case",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(RegulatedAuthorization {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#subject: r#subject.unwrap_or(vec![]),
                        r#type,
                        r#description,
                        r#region: r#region.unwrap_or(vec![]),
                        r#status,
                        r#status_date,
                        r#validity_period,
                        r#indication,
                        r#intended_use,
                        r#basis: r#basis.unwrap_or(vec![]),
                        r#holder,
                        r#regulator,
                        r#case,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
