// Generated on 2022-10-14 by fhirbolt-codegen v0.1.0
#[doc = "Date of procedure."]
#[derive(Debug, Clone)]
pub enum MedicinalProductAuthorizationProcedureDate {
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
    Invalid,
}
impl Default for MedicinalProductAuthorizationProcedureDate {
    fn default() -> MedicinalProductAuthorizationProcedureDate {
        MedicinalProductAuthorizationProcedureDate::Invalid
    }
}
#[doc = "Authorization in areas within a country."]
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductAuthorizationJurisdictionalAuthorization {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The assigned number for the marketing authorization."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Country of authorization."]
    pub r#country: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Jurisdiction within a country."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The legal status of supply in a jurisdiction or region."]
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The start and expected end date of the authorization."]
    pub r#validity_period: Option<Box<super::super::types::Period>>,
}
impl crate::AnyResource for MedicinalProductAuthorizationJurisdictionalAuthorization {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for MedicinalProductAuthorizationJurisdictionalAuthorization {
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
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if let Some(some) = self.r#country.as_ref() {
            state.serialize_entry("country", some)?;
        }
        if !self.r#jurisdiction.is_empty() {
            state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
        }
        if let Some(some) = self.r#legal_status_of_supply.as_ref() {
            state.serialize_entry("legalStatusOfSupply", some)?;
        }
        if let Some(some) = self.r#validity_period.as_ref() {
            state.serialize_entry("validityPeriod", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductAuthorizationJurisdictionalAuthorization {
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
            #[serde(rename = "country")]
            Country,
            #[serde(rename = "jurisdiction")]
            Jurisdiction,
            #[serde(rename = "legalStatusOfSupply")]
            LegalStatusOfSupply,
            #[serde(rename = "validityPeriod")]
            ValidityPeriod,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductAuthorizationJurisdictionalAuthorization;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductAuthorizationJurisdictionalAuthorization")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductAuthorizationJurisdictionalAuthorization, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#country: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#jurisdiction: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#legal_status_of_supply: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#validity_period: Option<Box<super::super::types::Period>> = None;
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
                            Field::Country => {
                                if r#country.is_some() {
                                    return Err(serde::de::Error::duplicate_field("country"));
                                }
                                r#country = Some(map_access.next_value()?);
                            }
                            Field::Jurisdiction => {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                r#jurisdiction = Some(map_access.next_value()?);
                            }
                            Field::LegalStatusOfSupply => {
                                if r#legal_status_of_supply.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "legalStatusOfSupply",
                                    ));
                                }
                                r#legal_status_of_supply = Some(map_access.next_value()?);
                            }
                            Field::ValidityPeriod => {
                                if r#validity_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "validityPeriod",
                                    ));
                                }
                                r#validity_period = Some(map_access.next_value()?);
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
                                        "country",
                                        "jurisdiction",
                                        "legalStatusOfSupply",
                                        "validityPeriod",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MedicinalProductAuthorizationJurisdictionalAuthorization {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#country,
                        r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                        r#legal_status_of_supply,
                        r#validity_period,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The regulatory procedure for granting or amending a marketing authorization."]
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductAuthorizationProcedure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifier for this procedure."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Type of procedure."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Date of procedure."]
    pub r#date: Option<MedicinalProductAuthorizationProcedureDate>,
    #[doc = "Applcations submitted to obtain a marketing authorization."]
    pub r#application: Vec<MedicinalProductAuthorizationProcedure>,
}
impl serde::ser::Serialize for MedicinalProductAuthorizationProcedure {
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
        state.serialize_entry("type", &self.r#type)?;
        if let Some(some) = self.r#date.as_ref() {
            match some {
                MedicinalProductAuthorizationProcedureDate::Period(ref value) => {
                    state.serialize_entry("datePeriod", value)?;
                }
                MedicinalProductAuthorizationProcedureDate::DateTime(ref value) => {
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
                MedicinalProductAuthorizationProcedureDate::Invalid => {
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
impl<'de> serde::de::Deserialize<'de> for MedicinalProductAuthorizationProcedure {
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
            type Value = MedicinalProductAuthorizationProcedure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductAuthorizationProcedure")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductAuthorizationProcedure, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#date: Option<MedicinalProductAuthorizationProcedureDate> = None;
                let mut r#application: Option<Vec<MedicinalProductAuthorizationProcedure>> = None;
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
                            Field::DatePeriod => {
                                if r#date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("datePeriod"));
                                }
                                r#date = Some(MedicinalProductAuthorizationProcedureDate::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::DateDateTime => {
                                let r#enum = r#date.get_or_insert(
                                    MedicinalProductAuthorizationProcedureDate::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let MedicinalProductAuthorizationProcedureDate::DateTime(
                                    variant,
                                ) = r#enum
                                {
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
                                    MedicinalProductAuthorizationProcedureDate::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let MedicinalProductAuthorizationProcedureDate::DateTime(
                                    variant,
                                ) = r#enum
                                {
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
                                        "datePeriod",
                                        "dateDateTime",
                                        "application",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MedicinalProductAuthorizationProcedure {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier,
                        r#type: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#date,
                        r#application: r#application.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The regulatory authorization of a medicinal product."]
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductAuthorization {
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
    #[doc = "Business identifier for the marketing authorization, as assigned by a regulator."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The medicinal product that is being authorized."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The country in which the marketing authorization has been granted."]
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Jurisdiction within a country."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the marketing authorization."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date at which the given status has become applicable."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "The date when a suspended the marketing or the marketing authorization of the product is anticipated to be restored."]
    pub r#restore_date: Option<super::super::types::DateTime>,
    #[doc = "The beginning of the time period in which the marketing authorization is in the specific status shall be specified A complete date consisting of day, month and year shall be specified using the ISO 8601 date format."]
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    #[doc = "A period of time after authorization before generic product applicatiosn can be submitted."]
    pub r#data_exclusivity_period: Option<Box<super::super::types::Period>>,
    #[doc = "The date when the first authorization was granted by a Medicines Regulatory Agency."]
    pub r#date_of_first_authorization: Option<super::super::types::DateTime>,
    #[doc = "Date of first marketing authorization for a company's new medicinal product in any country in the World."]
    pub r#international_birth_date: Option<super::super::types::DateTime>,
    #[doc = "The legal framework against which this authorization is granted."]
    pub r#legal_basis: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Authorization in areas within a country."]
    pub r#jurisdictional_authorization:
        Vec<MedicinalProductAuthorizationJurisdictionalAuthorization>,
    #[doc = "Marketing Authorization Holder."]
    pub r#holder: Option<Box<super::super::types::Reference>>,
    #[doc = "Medicines Regulatory Agency."]
    pub r#regulator: Option<Box<super::super::types::Reference>>,
    #[doc = "The regulatory procedure for granting or amending a marketing authorization."]
    pub r#procedure: Option<MedicinalProductAuthorizationProcedure>,
}
impl serde::ser::Serialize for MedicinalProductAuthorization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProductAuthorization")?;
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
        if let Some(some) = self.r#subject.as_ref() {
            state.serialize_entry("subject", some)?;
        }
        if !self.r#country.is_empty() {
            state.serialize_entry("country", &self.r#country)?;
        }
        if !self.r#jurisdiction.is_empty() {
            state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
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
        if let Some(some) = self.r#restore_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("restoreDate", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_restoreDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#validity_period.as_ref() {
            state.serialize_entry("validityPeriod", some)?;
        }
        if let Some(some) = self.r#data_exclusivity_period.as_ref() {
            state.serialize_entry("dataExclusivityPeriod", some)?;
        }
        if let Some(some) = self.r#date_of_first_authorization.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("dateOfFirstAuthorization", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_dateOfFirstAuthorization", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#international_birth_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("internationalBirthDate", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_internationalBirthDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#legal_basis.as_ref() {
            state.serialize_entry("legalBasis", some)?;
        }
        if !self.r#jurisdictional_authorization.is_empty() {
            state.serialize_entry(
                "jurisdictionalAuthorization",
                &self.r#jurisdictional_authorization,
            )?;
        }
        if let Some(some) = self.r#holder.as_ref() {
            state.serialize_entry("holder", some)?;
        }
        if let Some(some) = self.r#regulator.as_ref() {
            state.serialize_entry("regulator", some)?;
        }
        if let Some(some) = self.r#procedure.as_ref() {
            state.serialize_entry("procedure", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductAuthorization {
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
            #[serde(rename = "country")]
            Country,
            #[serde(rename = "jurisdiction")]
            Jurisdiction,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "statusDate")]
            StatusDate,
            #[serde(rename = "_statusDate")]
            StatusDatePrimitiveElement,
            #[serde(rename = "restoreDate")]
            RestoreDate,
            #[serde(rename = "_restoreDate")]
            RestoreDatePrimitiveElement,
            #[serde(rename = "validityPeriod")]
            ValidityPeriod,
            #[serde(rename = "dataExclusivityPeriod")]
            DataExclusivityPeriod,
            #[serde(rename = "dateOfFirstAuthorization")]
            DateOfFirstAuthorization,
            #[serde(rename = "_dateOfFirstAuthorization")]
            DateOfFirstAuthorizationPrimitiveElement,
            #[serde(rename = "internationalBirthDate")]
            InternationalBirthDate,
            #[serde(rename = "_internationalBirthDate")]
            InternationalBirthDatePrimitiveElement,
            #[serde(rename = "legalBasis")]
            LegalBasis,
            #[serde(rename = "jurisdictionalAuthorization")]
            JurisdictionalAuthorization,
            #[serde(rename = "holder")]
            Holder,
            #[serde(rename = "regulator")]
            Regulator,
            #[serde(rename = "procedure")]
            Procedure,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductAuthorization;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductAuthorization")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductAuthorization, V::Error>
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
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#country: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#jurisdiction: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status_date: Option<super::super::types::DateTime> = None;
                let mut r#restore_date: Option<super::super::types::DateTime> = None;
                let mut r#validity_period: Option<Box<super::super::types::Period>> = None;
                let mut r#data_exclusivity_period: Option<Box<super::super::types::Period>> = None;
                let mut r#date_of_first_authorization: Option<super::super::types::DateTime> = None;
                let mut r#international_birth_date: Option<super::super::types::DateTime> = None;
                let mut r#legal_basis: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#jurisdictional_authorization: Option<
                    Vec<MedicinalProductAuthorizationJurisdictionalAuthorization>,
                > = None;
                let mut r#holder: Option<Box<super::super::types::Reference>> = None;
                let mut r#regulator: Option<Box<super::super::types::Reference>> = None;
                let mut r#procedure: Option<MedicinalProductAuthorizationProcedure> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "MedicinalProductAuthorization" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"MedicinalProductAuthorization",
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
                            Field::Country => {
                                if r#country.is_some() {
                                    return Err(serde::de::Error::duplicate_field("country"));
                                }
                                r#country = Some(map_access.next_value()?);
                            }
                            Field::Jurisdiction => {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                r#jurisdiction = Some(map_access.next_value()?);
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
                            Field::RestoreDate => {
                                let some = r#restore_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("restoreDate"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::RestoreDatePrimitiveElement => {
                                let some = r#restore_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_restoreDate"));
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
                            Field::DataExclusivityPeriod => {
                                if r#data_exclusivity_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dataExclusivityPeriod",
                                    ));
                                }
                                r#data_exclusivity_period = Some(map_access.next_value()?);
                            }
                            Field::DateOfFirstAuthorization => {
                                let some =
                                    r#date_of_first_authorization.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dateOfFirstAuthorization",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DateOfFirstAuthorizationPrimitiveElement => {
                                let some =
                                    r#date_of_first_authorization.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_dateOfFirstAuthorization",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::InternationalBirthDate => {
                                let some =
                                    r#international_birth_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "internationalBirthDate",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::InternationalBirthDatePrimitiveElement => {
                                let some =
                                    r#international_birth_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_internationalBirthDate",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::LegalBasis => {
                                if r#legal_basis.is_some() {
                                    return Err(serde::de::Error::duplicate_field("legalBasis"));
                                }
                                r#legal_basis = Some(map_access.next_value()?);
                            }
                            Field::JurisdictionalAuthorization => {
                                if r#jurisdictional_authorization.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "jurisdictionalAuthorization",
                                    ));
                                }
                                r#jurisdictional_authorization = Some(map_access.next_value()?);
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
                            Field::Procedure => {
                                if r#procedure.is_some() {
                                    return Err(serde::de::Error::duplicate_field("procedure"));
                                }
                                r#procedure = Some(map_access.next_value()?);
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
                                        "country",
                                        "jurisdiction",
                                        "status",
                                        "statusDate",
                                        "restoreDate",
                                        "validityPeriod",
                                        "dataExclusivityPeriod",
                                        "dateOfFirstAuthorization",
                                        "internationalBirthDate",
                                        "legalBasis",
                                        "jurisdictionalAuthorization",
                                        "holder",
                                        "regulator",
                                        "procedure",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MedicinalProductAuthorization {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#subject,
                        r#country: r#country.unwrap_or(vec![]),
                        r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                        r#status,
                        r#status_date,
                        r#restore_date,
                        r#validity_period,
                        r#data_exclusivity_period,
                        r#date_of_first_authorization,
                        r#international_birth_date,
                        r#legal_basis,
                        r#jurisdictional_authorization: r#jurisdictional_authorization
                            .unwrap_or(vec![]),
                        r#holder,
                        r#regulator,
                        r#procedure,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
