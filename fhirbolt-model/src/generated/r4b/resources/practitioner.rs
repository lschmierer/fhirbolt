// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "The official certifications, training, and licenses that authorize or otherwise pertain to the provision of care by the practitioner.  For example, a medical license issued by a medical board authorizing the practitioner to practice medicine within a certian locality."]
#[derive(Default, Debug, Clone)]
pub struct PractitionerQualification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An identifier that applies to this person's qualification in this role."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Coded representation of the qualification."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Period during which the qualification is valid."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Organization that regulates and issues the qualification."]
    pub r#issuer: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for PractitionerQualification {
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
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            state.serialize_entry("code", &self.r#code)?;
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            if let Some(some) = self.r#issuer.as_ref() {
                state.serialize_entry("issuer", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for PractitionerQualification {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "issuer")]
            Issuer,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PractitionerQualification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PractitionerQualification")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PractitionerQualification, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                let mut r#issuer: Option<Box<super::super::types::Reference>> = None;
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
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Issuer => {
                                if r#issuer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issuer"));
                                }
                                r#issuer = Some(map_access.next_value()?);
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
                                        "identifier",
                                        "code",
                                        "period",
                                        "issuer",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(PractitionerQualification {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#code: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#period,
                        r#issuer,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A person who is directly or indirectly involved in the provisioning of healthcare.\n\nNeed to track doctors, staff, locums etc. for both healthcare practitioners, funders, etc."]
#[derive(Default, Debug, Clone)]
pub struct Practitioner {
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
    #[doc = "An identifier that applies to this person in this role."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Whether this practitioner's record is in active use."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "The name(s) associated with the practitioner."]
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    #[doc = "A contact detail for the practitioner, e.g. a telephone number or an email address."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "Address(es) of the practitioner that are not role specific (typically home address). \rWork addresses are not typically entered in this property as they are usually role dependent."]
    pub r#address: Vec<Box<super::super::types::Address>>,
    #[doc = "Administrative Gender - the gender that the person is considered to have for administration and record keeping purposes."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "The date of birth for the practitioner."]
    pub r#birth_date: Option<super::super::types::Date>,
    #[doc = "Image of the person."]
    pub r#photo: Vec<Box<super::super::types::Attachment>>,
    #[doc = "The official certifications, training, and licenses that authorize or otherwise pertain to the provision of care by the practitioner.  For example, a medical license issued by a medical board authorizing the practitioner to practice medicine within a certian locality."]
    pub r#qualification: Vec<PractitionerQualification>,
    #[doc = "A language the practitioner can use in patient communication."]
    pub r#communication: Vec<Box<super::super::types::CodeableConcept>>,
}
impl crate::AnyResource for Practitioner {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for Practitioner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Practitioner")?;
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
                if let Some(some) = self.r#active.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("active", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_active", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#active.as_ref() {
                    state.serialize_entry("active", some)?;
                }
            }
            if !self.r#name.is_empty() {
                state.serialize_entry("name", &self.r#name)?;
            }
            if !self.r#telecom.is_empty() {
                state.serialize_entry("telecom", &self.r#telecom)?;
            }
            if !self.r#address.is_empty() {
                state.serialize_entry("address", &self.r#address)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#gender.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("gender", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_gender", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#gender.as_ref() {
                    state.serialize_entry("gender", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#birth_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("birthDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_birthDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#birth_date.as_ref() {
                    state.serialize_entry("birthDate", some)?;
                }
            }
            if !self.r#photo.is_empty() {
                state.serialize_entry("photo", &self.r#photo)?;
            }
            if !self.r#qualification.is_empty() {
                state.serialize_entry("qualification", &self.r#qualification)?;
            }
            if !self.r#communication.is_empty() {
                state.serialize_entry("communication", &self.r#communication)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Practitioner {
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
            #[serde(rename = "active")]
            Active,
            #[serde(rename = "_active")]
            ActivePrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "telecom")]
            Telecom,
            #[serde(rename = "address")]
            Address,
            #[serde(rename = "gender")]
            Gender,
            #[serde(rename = "_gender")]
            GenderPrimitiveElement,
            #[serde(rename = "birthDate")]
            BirthDate,
            #[serde(rename = "_birthDate")]
            BirthDatePrimitiveElement,
            #[serde(rename = "photo")]
            Photo,
            #[serde(rename = "qualification")]
            Qualification,
            #[serde(rename = "communication")]
            Communication,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Practitioner;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Practitioner")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Practitioner, V::Error>
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
                let mut r#active: Option<super::super::types::Boolean> = None;
                let mut r#name: Option<Vec<Box<super::super::types::HumanName>>> = None;
                let mut r#telecom: Option<Vec<Box<super::super::types::ContactPoint>>> = None;
                let mut r#address: Option<Vec<Box<super::super::types::Address>>> = None;
                let mut r#gender: Option<super::super::types::Code> = None;
                let mut r#birth_date: Option<super::super::types::Date> = None;
                let mut r#photo: Option<Vec<Box<super::super::types::Attachment>>> = None;
                let mut r#qualification: Option<Vec<PractitionerQualification>> = None;
                let mut r#communication: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Practitioner" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Practitioner",
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
                                            "active",
                                            "name",
                                            "telecom",
                                            "address",
                                            "gender",
                                            "birthDate",
                                            "photo",
                                            "qualification",
                                            "communication",
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
                                            "active",
                                            "name",
                                            "telecom",
                                            "address",
                                            "gender",
                                            "birthDate",
                                            "photo",
                                            "qualification",
                                            "communication",
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
                            Field::Active => {
                                if _ctx.from_json {
                                    let some = r#active.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("active"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#active.is_some() {
                                        return Err(serde::de::Error::duplicate_field("active"));
                                    }
                                    r#active = Some(map_access.next_value()?);
                                }
                            }
                            Field::ActivePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#active.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_active"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "active",
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
                                            "active",
                                            "name",
                                            "telecom",
                                            "address",
                                            "gender",
                                            "birthDate",
                                            "photo",
                                            "qualification",
                                            "communication",
                                        ],
                                    ));
                                }
                            }
                            Field::Name => {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                r#name = Some(map_access.next_value()?);
                            }
                            Field::Telecom => {
                                if r#telecom.is_some() {
                                    return Err(serde::de::Error::duplicate_field("telecom"));
                                }
                                r#telecom = Some(map_access.next_value()?);
                            }
                            Field::Address => {
                                if r#address.is_some() {
                                    return Err(serde::de::Error::duplicate_field("address"));
                                }
                                r#address = Some(map_access.next_value()?);
                            }
                            Field::Gender => {
                                if _ctx.from_json {
                                    let some = r#gender.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("gender"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#gender.is_some() {
                                        return Err(serde::de::Error::duplicate_field("gender"));
                                    }
                                    r#gender = Some(map_access.next_value()?);
                                }
                            }
                            Field::GenderPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#gender.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_gender"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "gender",
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
                                            "active",
                                            "name",
                                            "telecom",
                                            "address",
                                            "gender",
                                            "birthDate",
                                            "photo",
                                            "qualification",
                                            "communication",
                                        ],
                                    ));
                                }
                            }
                            Field::BirthDate => {
                                if _ctx.from_json {
                                    let some = r#birth_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("birthDate"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#birth_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field("birthDate"));
                                    }
                                    r#birth_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::BirthDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#birth_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_birthDate",
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
                                        "birthDate",
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
                                            "active",
                                            "name",
                                            "telecom",
                                            "address",
                                            "gender",
                                            "birthDate",
                                            "photo",
                                            "qualification",
                                            "communication",
                                        ],
                                    ));
                                }
                            }
                            Field::Photo => {
                                if r#photo.is_some() {
                                    return Err(serde::de::Error::duplicate_field("photo"));
                                }
                                r#photo = Some(map_access.next_value()?);
                            }
                            Field::Qualification => {
                                if r#qualification.is_some() {
                                    return Err(serde::de::Error::duplicate_field("qualification"));
                                }
                                r#qualification = Some(map_access.next_value()?);
                            }
                            Field::Communication => {
                                if r#communication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("communication"));
                                }
                                r#communication = Some(map_access.next_value()?);
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
                                        "active",
                                        "name",
                                        "telecom",
                                        "address",
                                        "gender",
                                        "birthDate",
                                        "photo",
                                        "qualification",
                                        "communication",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Practitioner {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#active,
                        r#name: r#name.unwrap_or(vec![]),
                        r#telecom: r#telecom.unwrap_or(vec![]),
                        r#address: r#address.unwrap_or(vec![]),
                        r#gender,
                        r#birth_date,
                        r#photo: r#photo.unwrap_or(vec![]),
                        r#qualification: r#qualification.unwrap_or(vec![]),
                        r#communication: r#communication.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
