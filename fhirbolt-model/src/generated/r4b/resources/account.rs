// Generated on 2023-04-04 by fhirbolt-codegen v0.1.0
#[doc = "The party(s) that are responsible for covering the payment of this account, and what order should they be applied to the account."]
#[derive(Default, Debug, Clone)]
pub struct AccountCoverage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The party(s) that contribute to payment (or part of) of the charges applied to this account (including self-pay).\n\nA coverage may only be responsible for specific types of charges, and the sequence of the coverages in the account could be important when processing billing."]
    pub r#coverage: Box<super::super::types::Reference>,
    #[doc = "The priority of the coverage in the context of this account."]
    pub r#priority: Option<super::super::types::PositiveInt>,
}
impl serde::ser::Serialize for AccountCoverage {
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
            state.serialize_entry("coverage", &self.r#coverage)?;
            if _ctx.output_json {
                if let Some(some) = self.r#priority.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("priority", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_priority", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#priority.as_ref() {
                    state.serialize_entry("priority", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AccountCoverage {
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
            #[serde(rename = "coverage")]
            Coverage,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "_priority")]
            PriorityPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AccountCoverage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AccountCoverage")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AccountCoverage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#coverage: Option<Box<super::super::types::Reference>> = None;
                let mut r#priority: Option<super::super::types::PositiveInt> = None;
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
                            Field::Coverage => {
                                if r#coverage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("coverage"));
                                }
                                r#coverage = Some(map_access.next_value()?);
                            }
                            Field::Priority => {
                                if _ctx.from_json {
                                    let some = r#priority.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("priority"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#priority.is_some() {
                                        return Err(serde::de::Error::duplicate_field("priority"));
                                    }
                                    r#priority = Some(map_access.next_value()?);
                                }
                            }
                            Field::PriorityPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "priority",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "coverage",
                                            "priority",
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
                                        "coverage",
                                        "priority",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AccountCoverage {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#coverage: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#coverage.unwrap_or(Default::default())
                        } else {
                            r#coverage.ok_or(serde::de::Error::missing_field("coverage"))?
                        },
                        r#priority,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The parties responsible for balancing the account if other payment options fall short."]
#[derive(Default, Debug, Clone)]
pub struct AccountGuarantor {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The entity who is responsible."]
    pub r#party: Box<super::super::types::Reference>,
    #[doc = "A guarantor may be placed on credit hold or otherwise have their role temporarily suspended."]
    pub r#on_hold: Option<super::super::types::Boolean>,
    #[doc = "The timeframe during which the guarantor accepts responsibility for the account."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for AccountGuarantor {
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
            state.serialize_entry("party", &self.r#party)?;
            if _ctx.output_json {
                if let Some(some) = self.r#on_hold.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("onHold", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_onHold", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#on_hold.as_ref() {
                    state.serialize_entry("onHold", some)?;
                }
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AccountGuarantor {
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
            #[serde(rename = "party")]
            Party,
            #[serde(rename = "onHold")]
            OnHold,
            #[serde(rename = "_onHold")]
            OnHoldPrimitiveElement,
            #[serde(rename = "period")]
            Period,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AccountGuarantor;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AccountGuarantor")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AccountGuarantor, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#party: Option<Box<super::super::types::Reference>> = None;
                let mut r#on_hold: Option<super::super::types::Boolean> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
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
                            Field::Party => {
                                if r#party.is_some() {
                                    return Err(serde::de::Error::duplicate_field("party"));
                                }
                                r#party = Some(map_access.next_value()?);
                            }
                            Field::OnHold => {
                                if _ctx.from_json {
                                    let some = r#on_hold.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("onHold"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#on_hold.is_some() {
                                        return Err(serde::de::Error::duplicate_field("onHold"));
                                    }
                                    r#on_hold = Some(map_access.next_value()?);
                                }
                            }
                            Field::OnHoldPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#on_hold.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_onHold"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "onHold",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "party",
                                            "onHold",
                                            "period",
                                        ],
                                    ));
                                }
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
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
                                        "party",
                                        "onHold",
                                        "period",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AccountGuarantor {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#party: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#party.unwrap_or(Default::default())
                        } else {
                            r#party.ok_or(serde::de::Error::missing_field("party"))?
                        },
                        r#on_hold,
                        r#period,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A financial tool for tracking value accrued for a particular purpose.  In the healthcare field, used to track charges for a patient, cost centers, etc."]
#[derive(Default, Debug, Clone)]
pub struct Account {
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
    #[doc = "Unique identifier used to reference the account.  Might or might not be intended for human use (e.g. credit card number)."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Indicates whether the account is presently used/usable or not."]
    pub r#status: super::super::types::Code,
    #[doc = "Categorizes the account for reporting and searching purposes."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Name used for the account when displaying it to humans in reports, etc."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Identifies the entity which incurs the expenses. While the immediate recipients of services or goods might be entities related to the subject, the expenses were ultimately incurred by the subject of the Account."]
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    #[doc = "The date range of services associated with this account."]
    pub r#service_period: Option<Box<super::super::types::Period>>,
    #[doc = "The party(s) that are responsible for covering the payment of this account, and what order should they be applied to the account."]
    pub r#coverage: Vec<AccountCoverage>,
    #[doc = "Indicates the service area, hospital, department, etc. with responsibility for managing the Account."]
    pub r#owner: Option<Box<super::super::types::Reference>>,
    #[doc = "Provides additional information about what the account tracks and how it is used."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The parties responsible for balancing the account if other payment options fall short."]
    pub r#guarantor: Vec<AccountGuarantor>,
    #[doc = "Reference to a parent Account."]
    pub r#part_of: Option<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for Account {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4B;
}
impl serde::ser::Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Account")?;
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
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("name", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_name", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#name.as_ref() {
                    state.serialize_entry("name", some)?;
                }
            }
            if !self.r#subject.is_empty() {
                state.serialize_entry("subject", &self.r#subject)?;
            }
            if let Some(some) = self.r#service_period.as_ref() {
                state.serialize_entry("servicePeriod", some)?;
            }
            if !self.r#coverage.is_empty() {
                state.serialize_entry("coverage", &self.r#coverage)?;
            }
            if let Some(some) = self.r#owner.as_ref() {
                state.serialize_entry("owner", some)?;
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
            if !self.r#guarantor.is_empty() {
                state.serialize_entry("guarantor", &self.r#guarantor)?;
            }
            if let Some(some) = self.r#part_of.as_ref() {
                state.serialize_entry("partOf", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Account {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "servicePeriod")]
            ServicePeriod,
            #[serde(rename = "coverage")]
            Coverage,
            #[serde(rename = "owner")]
            Owner,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "guarantor")]
            Guarantor,
            #[serde(rename = "partOf")]
            PartOf,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Account;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Account")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Account, V::Error>
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
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#subject: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#service_period: Option<Box<super::super::types::Period>> = None;
                let mut r#coverage: Option<Vec<AccountCoverage>> = None;
                let mut r#owner: Option<Box<super::super::types::Reference>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#guarantor: Option<Vec<AccountGuarantor>> = None;
                let mut r#part_of: Option<Box<super::super::types::Reference>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Account" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Account",
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
                                            "type",
                                            "name",
                                            "subject",
                                            "servicePeriod",
                                            "coverage",
                                            "owner",
                                            "description",
                                            "guarantor",
                                            "partOf",
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
                                            "type",
                                            "name",
                                            "subject",
                                            "servicePeriod",
                                            "coverage",
                                            "owner",
                                            "description",
                                            "guarantor",
                                            "partOf",
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
                                            "type",
                                            "name",
                                            "subject",
                                            "servicePeriod",
                                            "coverage",
                                            "owner",
                                            "description",
                                            "guarantor",
                                            "partOf",
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
                            Field::Name => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                }
                            }
                            Field::NamePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_name"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "name",
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
                                            "type",
                                            "name",
                                            "subject",
                                            "servicePeriod",
                                            "coverage",
                                            "owner",
                                            "description",
                                            "guarantor",
                                            "partOf",
                                        ],
                                    ));
                                }
                            }
                            Field::Subject => {
                                if _ctx.from_json {
                                    if r#subject.is_some() {
                                        return Err(serde::de::Error::duplicate_field("subject"));
                                    }
                                    r#subject = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#subject.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ServicePeriod => {
                                if r#service_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("servicePeriod"));
                                }
                                r#service_period = Some(map_access.next_value()?);
                            }
                            Field::Coverage => {
                                if _ctx.from_json {
                                    if r#coverage.is_some() {
                                        return Err(serde::de::Error::duplicate_field("coverage"));
                                    }
                                    r#coverage = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#coverage.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Owner => {
                                if r#owner.is_some() {
                                    return Err(serde::de::Error::duplicate_field("owner"));
                                }
                                r#owner = Some(map_access.next_value()?);
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
                                            "type",
                                            "name",
                                            "subject",
                                            "servicePeriod",
                                            "coverage",
                                            "owner",
                                            "description",
                                            "guarantor",
                                            "partOf",
                                        ],
                                    ));
                                }
                            }
                            Field::Guarantor => {
                                if _ctx.from_json {
                                    if r#guarantor.is_some() {
                                        return Err(serde::de::Error::duplicate_field("guarantor"));
                                    }
                                    r#guarantor = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#guarantor.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::PartOf => {
                                if r#part_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partOf"));
                                }
                                r#part_of = Some(map_access.next_value()?);
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
                                        "type",
                                        "name",
                                        "subject",
                                        "servicePeriod",
                                        "coverage",
                                        "owner",
                                        "description",
                                        "guarantor",
                                        "partOf",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Account {
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
                        r#type,
                        r#name,
                        r#subject: r#subject.unwrap_or(vec![]),
                        r#service_period,
                        r#coverage: r#coverage.unwrap_or(vec![]),
                        r#owner,
                        r#description,
                        r#guarantor: r#guarantor.unwrap_or(vec![]),
                        r#part_of,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
