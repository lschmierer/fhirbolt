// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "Narrows the range of legal concerns to focus on the achievement of specific contractual objectives."]
#[derive(Debug, Clone)]
pub enum ContractTopic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractTopic {
    fn default() -> ContractTopic {
        ContractTopic::Invalid
    }
}
#[doc = "The entity that the term applies to."]
#[derive(Debug, Clone)]
pub enum ContractTermTopic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractTermTopic {
    fn default() -> ContractTermTopic {
        ContractTermTopic::Invalid
    }
}
#[doc = "Response to an offer clause or question text,  which enables selection of values to be agreed to, e.g., the period of participation, the date of occupancy of a rental, warrently duration, or whether biospecimen may be used for further research."]
#[derive(Debug, Clone)]
pub enum ContractTermOfferAnswerValue {
    Boolean(Box<super::super::types::Boolean>),
    Decimal(Box<super::super::types::Decimal>),
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Uri(Box<super::super::types::Uri>),
    Attachment(Box<super::super::types::Attachment>),
    Coding(Box<super::super::types::Coding>),
    Quantity(Box<super::super::types::Quantity>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractTermOfferAnswerValue {
    fn default() -> ContractTermOfferAnswerValue {
        ContractTermOfferAnswerValue::Invalid
    }
}
#[doc = "Specific type of Contract Valued Item that may be priced."]
#[derive(Debug, Clone)]
pub enum ContractTermAssetValuedItemEntity {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractTermAssetValuedItemEntity {
    fn default() -> ContractTermAssetValuedItemEntity {
        ContractTermAssetValuedItemEntity::Invalid
    }
}
#[doc = "When action happens."]
#[derive(Debug, Clone)]
pub enum ContractTermActionOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for ContractTermActionOccurrence {
    fn default() -> ContractTermActionOccurrence {
        ContractTermActionOccurrence::Invalid
    }
}
#[doc = "Human readable rendering of this Contract in a format and representation intended to enhance comprehension and ensure understandability."]
#[derive(Debug, Clone)]
pub enum ContractFriendlyContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractFriendlyContent {
    fn default() -> ContractFriendlyContent {
        ContractFriendlyContent::Invalid
    }
}
#[doc = "Contract legal text in human renderable form."]
#[derive(Debug, Clone)]
pub enum ContractLegalContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractLegalContent {
    fn default() -> ContractLegalContent {
        ContractLegalContent::Invalid
    }
}
#[doc = "Computable Contract conveyed using a policy rule language (e.g. XACML, DKAL, SecPal)."]
#[derive(Debug, Clone)]
pub enum ContractRuleContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractRuleContent {
    fn default() -> ContractRuleContent {
        ContractRuleContent::Invalid
    }
}
#[doc = "Legally binding Contract: This is the signed and legally recognized representation of the Contract, which is considered the \"source of truth\" and which would be the basis for legal action related to enforcement of this Contract."]
#[derive(Debug, Clone)]
pub enum ContractLegallyBinding {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ContractLegallyBinding {
    fn default() -> ContractLegallyBinding {
        ContractLegallyBinding::Invalid
    }
}
#[doc = "Precusory content developed with a focus and intent of supporting the formation a Contract instance, which may be associated with and transformable into a Contract."]
#[derive(Default, Debug, Clone)]
pub struct ContractContentDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Precusory content structure and use, i.e., a boilerplate, template, application for a contract such as an insurance policy or benefits under a program, e.g., workers compensation."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Detailed Precusory content type."]
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The  individual or organization that published the Contract precursor content."]
    pub r#publisher: Option<Box<super::super::types::Reference>>,
    #[doc = "The date (and optionally time) when the contract was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the contract changes."]
    pub r#publication_date: Option<super::super::types::DateTime>,
    #[doc = "amended | appended | cancelled | disputed | entered-in-error | executable | executed | negotiable | offered | policy | rejected | renewed | revoked | resolved | terminated."]
    pub r#publication_status: super::super::types::Code,
    #[doc = "A copyright statement relating to Contract precursor content. Copyright statements are generally legal restrictions on the use and publishing of the Contract precursor content."]
    pub r#copyright: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for ContractContentDefinition {
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
            state.serialize_entry("type", &self.r#type)?;
            if let Some(some) = self.r#sub_type.as_ref() {
                state.serialize_entry("subType", some)?;
            }
            if let Some(some) = self.r#publisher.as_ref() {
                state.serialize_entry("publisher", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#publication_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("publicationDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_publicationDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#publication_date.as_ref() {
                    state.serialize_entry("publicationDate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#publication_status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("publicationStatus", &some)?;
                }
                if self.r#publication_status.id.is_some()
                    || !self.r#publication_status.extension.is_empty()
                {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#publication_status.id.as_ref(),
                        extension: &self.r#publication_status.extension,
                    };
                    state.serialize_entry("_publicationStatus", &primitive_element)?;
                }
            } else {
                state.serialize_entry("publicationStatus", &self.r#publication_status)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#copyright.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("copyright", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_copyright", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#copyright.as_ref() {
                    state.serialize_entry("copyright", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractContentDefinition {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "subType")]
            SubType,
            #[serde(rename = "publisher")]
            Publisher,
            #[serde(rename = "publicationDate")]
            PublicationDate,
            #[serde(rename = "_publicationDate")]
            PublicationDatePrimitiveElement,
            #[serde(rename = "publicationStatus")]
            PublicationStatus,
            #[serde(rename = "_publicationStatus")]
            PublicationStatusPrimitiveElement,
            #[serde(rename = "copyright")]
            Copyright,
            #[serde(rename = "_copyright")]
            CopyrightPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractContentDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractContentDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractContentDefinition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#publisher: Option<Box<super::super::types::Reference>> = None;
                let mut r#publication_date: Option<super::super::types::DateTime> = None;
                let mut r#publication_status: Option<super::super::types::Code> = None;
                let mut r#copyright: Option<super::super::types::Markdown> = None;
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::SubType => {
                                if r#sub_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subType"));
                                }
                                r#sub_type = Some(map_access.next_value()?);
                            }
                            Field::Publisher => {
                                if r#publisher.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publisher"));
                                }
                                r#publisher = Some(map_access.next_value()?);
                            }
                            Field::PublicationDate => {
                                if _ctx.from_json {
                                    let some = r#publication_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "publicationDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#publication_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "publicationDate",
                                        ));
                                    }
                                    r#publication_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::PublicationDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#publication_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_publicationDate",
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
                                        "publicationDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "subType",
                                            "publisher",
                                            "publicationDate",
                                            "publicationStatus",
                                            "copyright",
                                        ],
                                    ));
                                }
                            }
                            Field::PublicationStatus => {
                                if _ctx.from_json {
                                    let some =
                                        r#publication_status.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "publicationStatus",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#publication_status.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "publicationStatus",
                                        ));
                                    }
                                    r#publication_status = Some(map_access.next_value()?);
                                }
                            }
                            Field::PublicationStatusPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#publication_status.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_publicationStatus",
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
                                        "publicationStatus",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "subType",
                                            "publisher",
                                            "publicationDate",
                                            "publicationStatus",
                                            "copyright",
                                        ],
                                    ));
                                }
                            }
                            Field::Copyright => {
                                if _ctx.from_json {
                                    let some = r#copyright.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("copyright"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#copyright.is_some() {
                                        return Err(serde::de::Error::duplicate_field("copyright"));
                                    }
                                    r#copyright = Some(map_access.next_value()?);
                                }
                            }
                            Field::CopyrightPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#copyright.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_copyright",
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
                                        "copyright",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "subType",
                                            "publisher",
                                            "publicationDate",
                                            "publicationStatus",
                                            "copyright",
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
                                        "type",
                                        "subType",
                                        "publisher",
                                        "publicationDate",
                                        "publicationStatus",
                                        "copyright",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractContentDefinition {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#sub_type,
                        r#publisher,
                        r#publication_date,
                        r#publication_status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#publication_status.unwrap_or(Default::default())
                        } else {
                            r#publication_status
                                .ok_or(serde::de::Error::missing_field("publicationStatus"))?
                        },
                        r#copyright,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Security labels that protect the handling of information about the term and its elements, which may be specifically identified.."]
#[derive(Default, Debug, Clone)]
pub struct ContractTermSecurityLabel {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Number used to link this term or term element to the applicable Security Label."]
    pub r#number: Vec<super::super::types::UnsignedInt>,
    #[doc = "Security label privacy tag that species the level of confidentiality protection required for this term and/or term elements."]
    pub r#classification: Box<super::super::types::Coding>,
    #[doc = "Security label privacy tag that species the applicable privacy and security policies governing this term and/or term elements."]
    pub r#category: Vec<Box<super::super::types::Coding>>,
    #[doc = "Security label privacy tag that species the manner in which term and/or term elements are to be protected."]
    pub r#control: Vec<Box<super::super::types::Coding>>,
}
impl serde::ser::Serialize for ContractTermSecurityLabel {
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
                if !self.r#number.is_empty() {
                    let values = self
                        .r#number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("number", &values)?;
                    }
                    let requires_elements = self
                        .r#number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#number
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
                        state.serialize_entry("_number", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#number.is_empty() {
                    state.serialize_entry("number", &self.r#number)?;
                }
            }
            state.serialize_entry("classification", &self.r#classification)?;
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if !self.r#control.is_empty() {
                state.serialize_entry("control", &self.r#control)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermSecurityLabel {
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
            #[serde(rename = "number")]
            Number,
            #[serde(rename = "_number")]
            NumberPrimitiveElement,
            #[serde(rename = "classification")]
            Classification,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "control")]
            Control,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermSecurityLabel;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermSecurityLabel")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermSecurityLabel, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#number: Option<Vec<super::super::types::UnsignedInt>> = None;
                let mut r#classification: Option<Box<super::super::types::Coding>> = None;
                let mut r#category: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#control: Option<Vec<Box<super::super::types::Coding>>> = None;
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
                            Field::Number => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#number.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("number"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#number.is_some() {
                                        return Err(serde::de::Error::duplicate_field("number"));
                                    }
                                    r#number = Some(map_access.next_value()?);
                                }
                            }
                            Field::NumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#number.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_number"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "number",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "number",
                                            "classification",
                                            "category",
                                            "control",
                                        ],
                                    ));
                                }
                            }
                            Field::Classification => {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                r#classification = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Control => {
                                if r#control.is_some() {
                                    return Err(serde::de::Error::duplicate_field("control"));
                                }
                                r#control = Some(map_access.next_value()?);
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
                                        "number",
                                        "classification",
                                        "category",
                                        "control",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractTermSecurityLabel {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#number: r#number.unwrap_or(vec![]),
                        r#classification: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#classification.unwrap_or(Default::default())
                        } else {
                            r#classification
                                .ok_or(serde::de::Error::missing_field("classification"))?
                        },
                        r#category: r#category.unwrap_or(vec![]),
                        r#control: r#control.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Offer Recipient."]
#[derive(Default, Debug, Clone)]
pub struct ContractTermOfferParty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Participant in the offer."]
    pub r#reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "How the party participates in the offer."]
    pub r#role: Box<super::super::types::CodeableConcept>,
}
impl serde::ser::Serialize for ContractTermOfferParty {
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
            if !self.r#reference.is_empty() {
                state.serialize_entry("reference", &self.r#reference)?;
            }
            state.serialize_entry("role", &self.r#role)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermOfferParty {
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
            #[serde(rename = "reference")]
            Reference,
            #[serde(rename = "role")]
            Role,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermOfferParty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermOfferParty")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermOfferParty, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
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
                            Field::Reference => {
                                if r#reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                r#reference = Some(map_access.next_value()?);
                            }
                            Field::Role => {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                r#role = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "reference", "role"],
                                ));
                            },
                        }
                    }
                    Ok(ContractTermOfferParty {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#reference: r#reference.unwrap_or(vec![]),
                        r#role: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#role.unwrap_or(Default::default())
                        } else {
                            r#role.ok_or(serde::de::Error::missing_field("role"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Response to offer text."]
#[derive(Default, Debug, Clone)]
pub struct ContractTermOfferAnswer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Response to an offer clause or question text,  which enables selection of values to be agreed to, e.g., the period of participation, the date of occupancy of a rental, warrently duration, or whether biospecimen may be used for further research."]
    pub r#value: ContractTermOfferAnswerValue,
}
impl serde::ser::Serialize for ContractTermOfferAnswer {
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
            match self.r#value {
                ContractTermOfferAnswerValue::Boolean(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueBoolean", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueBoolean", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueBoolean", value)?;
                    }
                }
                ContractTermOfferAnswerValue::Decimal(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = some.parse::<serde_json::Number>().map_err(|_| {
                                serde::ser::Error::custom("error serializing decimal")
                            })?;
                            state.serialize_entry("valueDecimal", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueDecimal", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueDecimal", value)?;
                    }
                }
                ContractTermOfferAnswerValue::Integer(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueInteger", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueInteger", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueInteger", value)?;
                    }
                }
                ContractTermOfferAnswerValue::Date(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueDate", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueDate", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueDate", value)?;
                    }
                }
                ContractTermOfferAnswerValue::DateTime(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueDateTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueDateTime", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueDateTime", value)?;
                    }
                }
                ContractTermOfferAnswerValue::Time(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueTime", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueTime", value)?;
                    }
                }
                ContractTermOfferAnswerValue::String(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueString", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueString", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueString", value)?;
                    }
                }
                ContractTermOfferAnswerValue::Uri(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueUri", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueUri", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueUri", value)?;
                    }
                }
                ContractTermOfferAnswerValue::Attachment(ref value) => {
                    state.serialize_entry("valueAttachment", value)?;
                }
                ContractTermOfferAnswerValue::Coding(ref value) => {
                    state.serialize_entry("valueCoding", value)?;
                }
                ContractTermOfferAnswerValue::Quantity(ref value) => {
                    state.serialize_entry("valueQuantity", value)?;
                }
                ContractTermOfferAnswerValue::Reference(ref value) => {
                    state.serialize_entry("valueReference", value)?;
                }
                ContractTermOfferAnswerValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermOfferAnswer {
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
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueDecimal")]
            ValueDecimal,
            #[serde(rename = "_valueDecimal")]
            ValueDecimalPrimitiveElement,
            #[serde(rename = "valueInteger")]
            ValueInteger,
            #[serde(rename = "_valueInteger")]
            ValueIntegerPrimitiveElement,
            #[serde(rename = "valueDate")]
            ValueDate,
            #[serde(rename = "_valueDate")]
            ValueDatePrimitiveElement,
            #[serde(rename = "valueDateTime")]
            ValueDateTime,
            #[serde(rename = "_valueDateTime")]
            ValueDateTimePrimitiveElement,
            #[serde(rename = "valueTime")]
            ValueTime,
            #[serde(rename = "_valueTime")]
            ValueTimePrimitiveElement,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueUri")]
            ValueUri,
            #[serde(rename = "_valueUri")]
            ValueUriPrimitiveElement,
            #[serde(rename = "valueAttachment")]
            ValueAttachment,
            #[serde(rename = "valueCoding")]
            ValueCoding,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueReference")]
            ValueReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermOfferAnswer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermOfferAnswer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermOfferAnswer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#value: Option<ContractTermOfferAnswerValue> = None;
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
                            Field::ValueBoolean => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::Boolean(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::Boolean(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueBoolean",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("value[x]"));
                                    }
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    r#value = Some(ContractTermOfferAnswerValue::Boolean(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueBooleanPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::Boolean(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::Boolean(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueBoolean",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_value[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueBoolean",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "valueBoolean",
                                            "valueDecimal",
                                            "valueInteger",
                                            "valueDate",
                                            "valueDateTime",
                                            "valueTime",
                                            "valueString",
                                            "valueUri",
                                            "valueAttachment",
                                            "valueCoding",
                                            "valueQuantity",
                                            "valueReference",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueDecimal => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::Decimal(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::Decimal(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueDecimal",
                                            ));
                                        }
                                        let value: serde_json::Number = map_access.next_value()?;
                                        variant.value = Some(format!("{}", value));
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("value[x]"));
                                    }
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDecimal",
                                        ));
                                    }
                                    r#value = Some(ContractTermOfferAnswerValue::Decimal(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueDecimalPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::Decimal(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::Decimal(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueDecimal",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_value[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueDecimal",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "valueBoolean",
                                            "valueDecimal",
                                            "valueInteger",
                                            "valueDate",
                                            "valueDateTime",
                                            "valueTime",
                                            "valueString",
                                            "valueUri",
                                            "valueAttachment",
                                            "valueCoding",
                                            "valueQuantity",
                                            "valueReference",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueInteger => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::Integer(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::Integer(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueInteger",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("value[x]"));
                                    }
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInteger",
                                        ));
                                    }
                                    r#value = Some(ContractTermOfferAnswerValue::Integer(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueIntegerPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::Integer(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::Integer(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueInteger",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_value[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueInteger",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "valueBoolean",
                                            "valueDecimal",
                                            "valueInteger",
                                            "valueDate",
                                            "valueDateTime",
                                            "valueTime",
                                            "valueString",
                                            "valueUri",
                                            "valueAttachment",
                                            "valueCoding",
                                            "valueQuantity",
                                            "valueReference",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueDate => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::Date(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::Date(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueDate",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("value[x]"));
                                    }
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueDate"));
                                    }
                                    r#value = Some(ContractTermOfferAnswerValue::Date(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::Date(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::Date(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueDate",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_value[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "valueBoolean",
                                            "valueDecimal",
                                            "valueInteger",
                                            "valueDate",
                                            "valueDateTime",
                                            "valueTime",
                                            "valueString",
                                            "valueUri",
                                            "valueAttachment",
                                            "valueCoding",
                                            "valueQuantity",
                                            "valueReference",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::DateTime(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::DateTime(variant) = r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueDateTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("value[x]"));
                                    }
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDateTime",
                                        ));
                                    }
                                    r#value = Some(ContractTermOfferAnswerValue::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::DateTime(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::DateTime(variant) = r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueDateTime",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_value[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueDateTime",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "valueBoolean",
                                            "valueDecimal",
                                            "valueInteger",
                                            "valueDate",
                                            "valueDateTime",
                                            "valueTime",
                                            "valueString",
                                            "valueUri",
                                            "valueAttachment",
                                            "valueCoding",
                                            "valueQuantity",
                                            "valueReference",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueTime => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::Time(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::Time(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("value[x]"));
                                    }
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueTime"));
                                    }
                                    r#value = Some(ContractTermOfferAnswerValue::Time(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::Time(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::Time(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueTime",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_value[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueTime",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "valueBoolean",
                                            "valueDecimal",
                                            "valueInteger",
                                            "valueDate",
                                            "valueDateTime",
                                            "valueTime",
                                            "valueString",
                                            "valueUri",
                                            "valueAttachment",
                                            "valueCoding",
                                            "valueQuantity",
                                            "valueReference",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueString => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::String(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::String(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueString",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("value[x]"));
                                    }
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    r#value = Some(ContractTermOfferAnswerValue::String(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::String(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::String(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueString",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_value[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueString",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "valueBoolean",
                                            "valueDecimal",
                                            "valueInteger",
                                            "valueDate",
                                            "valueDateTime",
                                            "valueTime",
                                            "valueString",
                                            "valueUri",
                                            "valueAttachment",
                                            "valueCoding",
                                            "valueQuantity",
                                            "valueReference",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueUri => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::Uri(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::Uri(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueUri",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("value[x]"));
                                    }
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUri"));
                                    }
                                    r#value = Some(ContractTermOfferAnswerValue::Uri(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueUriPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ContractTermOfferAnswerValue::Uri(Default::default()),
                                    );
                                    if let ContractTermOfferAnswerValue::Uri(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueUri",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_value[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueUri",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "valueBoolean",
                                            "valueDecimal",
                                            "valueInteger",
                                            "valueDate",
                                            "valueDateTime",
                                            "valueTime",
                                            "valueString",
                                            "valueUri",
                                            "valueAttachment",
                                            "valueCoding",
                                            "valueQuantity",
                                            "valueReference",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueAttachment => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAttachment",
                                    ));
                                }
                                r#value = Some(ContractTermOfferAnswerValue::Attachment(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueCoding => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCoding"));
                                }
                                r#value = Some(ContractTermOfferAnswerValue::Coding(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value = Some(ContractTermOfferAnswerValue::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueReference => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueReference",
                                    ));
                                }
                                r#value = Some(ContractTermOfferAnswerValue::Reference(
                                    map_access.next_value()?,
                                ));
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
                                        "valueBoolean",
                                        "valueDecimal",
                                        "valueInteger",
                                        "valueDate",
                                        "valueDateTime",
                                        "valueTime",
                                        "valueString",
                                        "valueUri",
                                        "valueAttachment",
                                        "valueCoding",
                                        "valueQuantity",
                                        "valueReference",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractTermOfferAnswer {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#value: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The matter of concern in the context of this provision of the agrement."]
#[derive(Default, Debug, Clone)]
pub struct ContractTermOffer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Unique identifier for this particular Contract Provision."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Offer Recipient."]
    pub r#party: Vec<ContractTermOfferParty>,
    #[doc = "The owner of an asset has the residual control rights over the asset: the right to decide all usages of the asset in any way not inconsistent with a prior contract, custom, or law (Hart, 1995, p. 30)."]
    pub r#topic: Option<Box<super::super::types::Reference>>,
    #[doc = "Type of Contract Provision such as specific requirements, purposes for actions, obligations, prohibitions, e.g. life time maximum benefit."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Type of choice made by accepting party with respect to an offer made by an offeror/ grantee."]
    pub r#decision: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "How the decision about a Contract was conveyed."]
    pub r#decision_mode: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Response to offer text."]
    pub r#answer: Vec<ContractTermOfferAnswer>,
    #[doc = "Human readable form of this Contract Offer."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "The id of the clause or question text of the offer in the referenced questionnaire/response."]
    pub r#link_id: Vec<super::super::types::String>,
    #[doc = "Security labels that protects the offer."]
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
}
impl serde::ser::Serialize for ContractTermOffer {
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
            if !self.r#party.is_empty() {
                state.serialize_entry("party", &self.r#party)?;
            }
            if let Some(some) = self.r#topic.as_ref() {
                state.serialize_entry("topic", some)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#decision.as_ref() {
                state.serialize_entry("decision", some)?;
            }
            if !self.r#decision_mode.is_empty() {
                state.serialize_entry("decisionMode", &self.r#decision_mode)?;
            }
            if !self.r#answer.is_empty() {
                state.serialize_entry("answer", &self.r#answer)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#link_id.is_empty() {
                    let values = self
                        .r#link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("linkId", &values)?;
                    }
                    let requires_elements = self
                        .r#link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#link_id
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
                        state.serialize_entry("_linkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#link_id.is_empty() {
                    state.serialize_entry("linkId", &self.r#link_id)?;
                }
            }
            if _ctx.output_json {
                if !self.r#security_label_number.is_empty() {
                    let values = self
                        .r#security_label_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("securityLabelNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#security_label_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#security_label_number
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
                        state.serialize_entry("_securityLabelNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#security_label_number.is_empty() {
                    state.serialize_entry("securityLabelNumber", &self.r#security_label_number)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermOffer {
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
            #[serde(rename = "party")]
            Party,
            #[serde(rename = "topic")]
            Topic,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "decision")]
            Decision,
            #[serde(rename = "decisionMode")]
            DecisionMode,
            #[serde(rename = "answer")]
            Answer,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            #[serde(rename = "linkId")]
            LinkId,
            #[serde(rename = "_linkId")]
            LinkIdPrimitiveElement,
            #[serde(rename = "securityLabelNumber")]
            SecurityLabelNumber,
            #[serde(rename = "_securityLabelNumber")]
            SecurityLabelNumberPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermOffer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermOffer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermOffer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#party: Option<Vec<ContractTermOfferParty>> = None;
                let mut r#topic: Option<Box<super::super::types::Reference>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#decision: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#decision_mode: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#answer: Option<Vec<ContractTermOfferAnswer>> = None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#security_label_number: Option<Vec<super::super::types::UnsignedInt>> =
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
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Party => {
                                if r#party.is_some() {
                                    return Err(serde::de::Error::duplicate_field("party"));
                                }
                                r#party = Some(map_access.next_value()?);
                            }
                            Field::Topic => {
                                if r#topic.is_some() {
                                    return Err(serde::de::Error::duplicate_field("topic"));
                                }
                                r#topic = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Decision => {
                                if r#decision.is_some() {
                                    return Err(serde::de::Error::duplicate_field("decision"));
                                }
                                r#decision = Some(map_access.next_value()?);
                            }
                            Field::DecisionMode => {
                                if r#decision_mode.is_some() {
                                    return Err(serde::de::Error::duplicate_field("decisionMode"));
                                }
                                r#decision_mode = Some(map_access.next_value()?);
                            }
                            Field::Answer => {
                                if r#answer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("answer"));
                                }
                                r#answer = Some(map_access.next_value()?);
                            }
                            Field::Text => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    r#text = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_text"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "text",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "party",
                                            "topic",
                                            "type",
                                            "decision",
                                            "decisionMode",
                                            "answer",
                                            "text",
                                            "linkId",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::LinkId => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#link_id.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("linkId"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#link_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field("linkId"));
                                    }
                                    r#link_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::LinkIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#link_id.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_linkId"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "linkId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "party",
                                            "topic",
                                            "type",
                                            "decision",
                                            "decisionMode",
                                            "answer",
                                            "text",
                                            "linkId",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::SecurityLabelNumber => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#security_label_number.get_or_insert(
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
                                            "securityLabelNumber",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#security_label_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "securityLabelNumber",
                                        ));
                                    }
                                    r#security_label_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::SecurityLabelNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#security_label_number.get_or_insert(
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
                                            "_securityLabelNumber",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "securityLabelNumber",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "party",
                                            "topic",
                                            "type",
                                            "decision",
                                            "decisionMode",
                                            "answer",
                                            "text",
                                            "linkId",
                                            "securityLabelNumber",
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
                                        "identifier",
                                        "party",
                                        "topic",
                                        "type",
                                        "decision",
                                        "decisionMode",
                                        "answer",
                                        "text",
                                        "linkId",
                                        "securityLabelNumber",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractTermOffer {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#party: r#party.unwrap_or(vec![]),
                        r#topic,
                        r#type,
                        r#decision,
                        r#decision_mode: r#decision_mode.unwrap_or(vec![]),
                        r#answer: r#answer.unwrap_or(vec![]),
                        r#text,
                        r#link_id: r#link_id.unwrap_or(vec![]),
                        r#security_label_number: r#security_label_number.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Circumstance of the asset."]
#[derive(Default, Debug, Clone)]
pub struct ContractTermAssetContext {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Asset context reference may include the creator, custodian, or owning Person or Organization (e.g., bank, repository),  location held, e.g., building,  jurisdiction."]
    pub r#reference: Option<Box<super::super::types::Reference>>,
    #[doc = "Coded representation of the context generally or of the Referenced entity, such as the asset holder type or location."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Context description."]
    pub r#text: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ContractTermAssetContext {
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
            if let Some(some) = self.r#reference.as_ref() {
                state.serialize_entry("reference", some)?;
            }
            if !self.r#code.is_empty() {
                state.serialize_entry("code", &self.r#code)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermAssetContext {
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
            #[serde(rename = "reference")]
            Reference,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermAssetContext;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermAssetContext")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermAssetContext, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#reference: Option<Box<super::super::types::Reference>> = None;
                let mut r#code: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#text: Option<super::super::types::String> = None;
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
                            Field::Reference => {
                                if r#reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                r#reference = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Text => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    r#text = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_text"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "text",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "reference",
                                            "code",
                                            "text",
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
                                        "reference",
                                        "code",
                                        "text",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractTermAssetContext {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#reference,
                        r#code: r#code.unwrap_or(vec![]),
                        r#text,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Contract Valued Item List."]
#[derive(Default, Debug, Clone)]
pub struct ContractTermAssetValuedItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Specific type of Contract Valued Item that may be priced."]
    pub r#entity: Option<ContractTermAssetValuedItemEntity>,
    #[doc = "Identifies a Contract Valued Item instance."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Indicates the time during which this Contract ValuedItem information is effective."]
    pub r#effective_time: Option<super::super::types::DateTime>,
    #[doc = "Specifies the units by which the Contract Valued Item is measured or counted, and quantifies the countable or measurable Contract Valued Item instances."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "A Contract Valued Item unit valuation measure."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of the Contract Valued Item delivered. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "An amount that expresses the weighting (based on difficulty, cost and/or resource intensiveness) associated with the Contract Valued Item delivered. The concept of Points allows for assignment of point values for a Contract Valued Item, such that a monetary amount can be assigned to each point."]
    pub r#points: Option<super::super::types::Decimal>,
    #[doc = "Expresses the product of the Contract Valued Item unitQuantity and the unitPriceAmt. For example, the formula: unit Quantity * unit Price (Cost per Point) * factor Number  * points = net Amount. Quantity, factor and points are assumed to be 1 if not supplied."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Terms of valuation."]
    pub r#payment: Option<super::super::types::String>,
    #[doc = "When payment is due."]
    pub r#payment_date: Option<super::super::types::DateTime>,
    #[doc = "Who will make payment."]
    pub r#responsible: Option<Box<super::super::types::Reference>>,
    #[doc = "Who will receive payment."]
    pub r#recipient: Option<Box<super::super::types::Reference>>,
    #[doc = "Id  of the clause or question text related to the context of this valuedItem in the referenced form or QuestionnaireResponse."]
    pub r#link_id: Vec<super::super::types::String>,
    #[doc = "A set of security labels that define which terms are controlled by this condition."]
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
}
impl serde::ser::Serialize for ContractTermAssetValuedItem {
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
            if let Some(some) = self.r#entity.as_ref() {
                match some {
                    ContractTermAssetValuedItemEntity::CodeableConcept(ref value) => {
                        state.serialize_entry("entityCodeableConcept", value)?;
                    }
                    ContractTermAssetValuedItemEntity::Reference(ref value) => {
                        state.serialize_entry("entityReference", value)?;
                    }
                    ContractTermAssetValuedItemEntity::Invalid => {
                        return Err(serde::ser::Error::custom("entity is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#effective_time.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("effectiveTime", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_effectiveTime", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#effective_time.as_ref() {
                    state.serialize_entry("effectiveTime", some)?;
                }
            }
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            if let Some(some) = self.r#unit_price.as_ref() {
                state.serialize_entry("unitPrice", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#factor.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("factor", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_factor", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#factor.as_ref() {
                    state.serialize_entry("factor", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#points.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("points", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_points", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#points.as_ref() {
                    state.serialize_entry("points", some)?;
                }
            }
            if let Some(some) = self.r#net.as_ref() {
                state.serialize_entry("net", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#payment.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("payment", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_payment", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#payment.as_ref() {
                    state.serialize_entry("payment", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#payment_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("paymentDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_paymentDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#payment_date.as_ref() {
                    state.serialize_entry("paymentDate", some)?;
                }
            }
            if let Some(some) = self.r#responsible.as_ref() {
                state.serialize_entry("responsible", some)?;
            }
            if let Some(some) = self.r#recipient.as_ref() {
                state.serialize_entry("recipient", some)?;
            }
            if _ctx.output_json {
                if !self.r#link_id.is_empty() {
                    let values = self
                        .r#link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("linkId", &values)?;
                    }
                    let requires_elements = self
                        .r#link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#link_id
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
                        state.serialize_entry("_linkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#link_id.is_empty() {
                    state.serialize_entry("linkId", &self.r#link_id)?;
                }
            }
            if _ctx.output_json {
                if !self.r#security_label_number.is_empty() {
                    let values = self
                        .r#security_label_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("securityLabelNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#security_label_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#security_label_number
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
                        state.serialize_entry("_securityLabelNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#security_label_number.is_empty() {
                    state.serialize_entry("securityLabelNumber", &self.r#security_label_number)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermAssetValuedItem {
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
            #[serde(rename = "entityCodeableConcept")]
            EntityCodeableConcept,
            #[serde(rename = "entityReference")]
            EntityReference,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "effectiveTime")]
            EffectiveTime,
            #[serde(rename = "_effectiveTime")]
            EffectiveTimePrimitiveElement,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "points")]
            Points,
            #[serde(rename = "_points")]
            PointsPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "payment")]
            Payment,
            #[serde(rename = "_payment")]
            PaymentPrimitiveElement,
            #[serde(rename = "paymentDate")]
            PaymentDate,
            #[serde(rename = "_paymentDate")]
            PaymentDatePrimitiveElement,
            #[serde(rename = "responsible")]
            Responsible,
            #[serde(rename = "recipient")]
            Recipient,
            #[serde(rename = "linkId")]
            LinkId,
            #[serde(rename = "_linkId")]
            LinkIdPrimitiveElement,
            #[serde(rename = "securityLabelNumber")]
            SecurityLabelNumber,
            #[serde(rename = "_securityLabelNumber")]
            SecurityLabelNumberPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermAssetValuedItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermAssetValuedItem")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ContractTermAssetValuedItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#entity: Option<ContractTermAssetValuedItemEntity> = None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#effective_time: Option<super::super::types::DateTime> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#points: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#payment: Option<super::super::types::String> = None;
                let mut r#payment_date: Option<super::super::types::DateTime> = None;
                let mut r#responsible: Option<Box<super::super::types::Reference>> = None;
                let mut r#recipient: Option<Box<super::super::types::Reference>> = None;
                let mut r#link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#security_label_number: Option<Vec<super::super::types::UnsignedInt>> =
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
                            Field::EntityCodeableConcept => {
                                if r#entity.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "entityCodeableConcept",
                                    ));
                                }
                                r#entity =
                                    Some(ContractTermAssetValuedItemEntity::CodeableConcept(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::EntityReference => {
                                if r#entity.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "entityReference",
                                    ));
                                }
                                r#entity = Some(ContractTermAssetValuedItemEntity::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::EffectiveTime => {
                                if _ctx.from_json {
                                    let some = r#effective_time.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "effectiveTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#effective_time.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "effectiveTime",
                                        ));
                                    }
                                    r#effective_time = Some(map_access.next_value()?);
                                }
                            }
                            Field::EffectiveTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#effective_time.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_effectiveTime",
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
                                        "effectiveTime",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "entityCodeableConcept",
                                            "entityReference",
                                            "identifier",
                                            "effectiveTime",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "points",
                                            "net",
                                            "payment",
                                            "paymentDate",
                                            "responsible",
                                            "recipient",
                                            "linkId",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::UnitPrice => {
                                if r#unit_price.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unitPrice"));
                                }
                                r#unit_price = Some(map_access.next_value()?);
                            }
                            Field::Factor => {
                                if _ctx.from_json {
                                    let some = r#factor.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#factor.is_some() {
                                        return Err(serde::de::Error::duplicate_field("factor"));
                                    }
                                    r#factor = Some(map_access.next_value()?);
                                }
                            }
                            Field::FactorPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#factor.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_factor"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "factor",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "entityCodeableConcept",
                                            "entityReference",
                                            "identifier",
                                            "effectiveTime",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "points",
                                            "net",
                                            "payment",
                                            "paymentDate",
                                            "responsible",
                                            "recipient",
                                            "linkId",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::Points => {
                                if _ctx.from_json {
                                    let some = r#points.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("points"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#points.is_some() {
                                        return Err(serde::de::Error::duplicate_field("points"));
                                    }
                                    r#points = Some(map_access.next_value()?);
                                }
                            }
                            Field::PointsPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#points.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_points"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "points",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "entityCodeableConcept",
                                            "entityReference",
                                            "identifier",
                                            "effectiveTime",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "points",
                                            "net",
                                            "payment",
                                            "paymentDate",
                                            "responsible",
                                            "recipient",
                                            "linkId",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::Net => {
                                if r#net.is_some() {
                                    return Err(serde::de::Error::duplicate_field("net"));
                                }
                                r#net = Some(map_access.next_value()?);
                            }
                            Field::Payment => {
                                if _ctx.from_json {
                                    let some = r#payment.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("payment"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#payment.is_some() {
                                        return Err(serde::de::Error::duplicate_field("payment"));
                                    }
                                    r#payment = Some(map_access.next_value()?);
                                }
                            }
                            Field::PaymentPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#payment.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_payment"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "payment",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "entityCodeableConcept",
                                            "entityReference",
                                            "identifier",
                                            "effectiveTime",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "points",
                                            "net",
                                            "payment",
                                            "paymentDate",
                                            "responsible",
                                            "recipient",
                                            "linkId",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::PaymentDate => {
                                if _ctx.from_json {
                                    let some = r#payment_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "paymentDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#payment_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "paymentDate",
                                        ));
                                    }
                                    r#payment_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::PaymentDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#payment_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_paymentDate",
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
                                        "paymentDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "entityCodeableConcept",
                                            "entityReference",
                                            "identifier",
                                            "effectiveTime",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "points",
                                            "net",
                                            "payment",
                                            "paymentDate",
                                            "responsible",
                                            "recipient",
                                            "linkId",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::Responsible => {
                                if r#responsible.is_some() {
                                    return Err(serde::de::Error::duplicate_field("responsible"));
                                }
                                r#responsible = Some(map_access.next_value()?);
                            }
                            Field::Recipient => {
                                if r#recipient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recipient"));
                                }
                                r#recipient = Some(map_access.next_value()?);
                            }
                            Field::LinkId => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#link_id.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("linkId"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#link_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field("linkId"));
                                    }
                                    r#link_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::LinkIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#link_id.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_linkId"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "linkId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "entityCodeableConcept",
                                            "entityReference",
                                            "identifier",
                                            "effectiveTime",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "points",
                                            "net",
                                            "payment",
                                            "paymentDate",
                                            "responsible",
                                            "recipient",
                                            "linkId",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::SecurityLabelNumber => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#security_label_number.get_or_insert(
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
                                            "securityLabelNumber",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#security_label_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "securityLabelNumber",
                                        ));
                                    }
                                    r#security_label_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::SecurityLabelNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#security_label_number.get_or_insert(
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
                                            "_securityLabelNumber",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "securityLabelNumber",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "entityCodeableConcept",
                                            "entityReference",
                                            "identifier",
                                            "effectiveTime",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "points",
                                            "net",
                                            "payment",
                                            "paymentDate",
                                            "responsible",
                                            "recipient",
                                            "linkId",
                                            "securityLabelNumber",
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
                                        "entityCodeableConcept",
                                        "entityReference",
                                        "identifier",
                                        "effectiveTime",
                                        "quantity",
                                        "unitPrice",
                                        "factor",
                                        "points",
                                        "net",
                                        "payment",
                                        "paymentDate",
                                        "responsible",
                                        "recipient",
                                        "linkId",
                                        "securityLabelNumber",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractTermAssetValuedItem {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#entity,
                        r#identifier,
                        r#effective_time,
                        r#quantity,
                        r#unit_price,
                        r#factor,
                        r#points,
                        r#net,
                        r#payment,
                        r#payment_date,
                        r#responsible,
                        r#recipient,
                        r#link_id: r#link_id.unwrap_or(vec![]),
                        r#security_label_number: r#security_label_number.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Contract Term Asset List."]
#[derive(Default, Debug, Clone)]
pub struct ContractTermAsset {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Differentiates the kind of the asset ."]
    pub r#scope: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Target entity type about which the term may be concerned."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Associated entities."]
    pub r#type_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "May be a subtype or part of an offered asset."]
    pub r#subtype: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specifies the applicability of the term to an asset resource instance, and instances it refers to orinstances that refer to it, and/or are owned by the offeree."]
    pub r#relationship: Option<Box<super::super::types::Coding>>,
    #[doc = "Circumstance of the asset."]
    pub r#context: Vec<ContractTermAssetContext>,
    #[doc = "Description of the quality and completeness of the asset that imay be a factor in its valuation."]
    pub r#condition: Option<super::super::types::String>,
    #[doc = "Type of Asset availability for use or ownership."]
    pub r#period_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Asset relevant contractual time period."]
    pub r#period: Vec<Box<super::super::types::Period>>,
    #[doc = "Time period of asset use."]
    pub r#use_period: Vec<Box<super::super::types::Period>>,
    #[doc = "Clause or question text (Prose Object) concerning the asset in a linked form, such as a QuestionnaireResponse used in the formation of the contract."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "Id [identifier??] of the clause or question text about the asset in the referenced form or QuestionnaireResponse."]
    pub r#link_id: Vec<super::super::types::String>,
    #[doc = "Response to assets."]
    pub r#answer: Vec<ContractTermOfferAnswer>,
    #[doc = "Security labels that protects the asset."]
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
    #[doc = "Contract Valued Item List."]
    pub r#valued_item: Vec<ContractTermAssetValuedItem>,
}
impl serde::ser::Serialize for ContractTermAsset {
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
            if let Some(some) = self.r#scope.as_ref() {
                state.serialize_entry("scope", some)?;
            }
            if !self.r#type.is_empty() {
                state.serialize_entry("type", &self.r#type)?;
            }
            if !self.r#type_reference.is_empty() {
                state.serialize_entry("typeReference", &self.r#type_reference)?;
            }
            if !self.r#subtype.is_empty() {
                state.serialize_entry("subtype", &self.r#subtype)?;
            }
            if let Some(some) = self.r#relationship.as_ref() {
                state.serialize_entry("relationship", some)?;
            }
            if !self.r#context.is_empty() {
                state.serialize_entry("context", &self.r#context)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#condition.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("condition", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_condition", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#condition.as_ref() {
                    state.serialize_entry("condition", some)?;
                }
            }
            if !self.r#period_type.is_empty() {
                state.serialize_entry("periodType", &self.r#period_type)?;
            }
            if !self.r#period.is_empty() {
                state.serialize_entry("period", &self.r#period)?;
            }
            if !self.r#use_period.is_empty() {
                state.serialize_entry("usePeriod", &self.r#use_period)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#link_id.is_empty() {
                    let values = self
                        .r#link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("linkId", &values)?;
                    }
                    let requires_elements = self
                        .r#link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#link_id
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
                        state.serialize_entry("_linkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#link_id.is_empty() {
                    state.serialize_entry("linkId", &self.r#link_id)?;
                }
            }
            if !self.r#answer.is_empty() {
                state.serialize_entry("answer", &self.r#answer)?;
            }
            if _ctx.output_json {
                if !self.r#security_label_number.is_empty() {
                    let values = self
                        .r#security_label_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("securityLabelNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#security_label_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#security_label_number
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
                        state.serialize_entry("_securityLabelNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#security_label_number.is_empty() {
                    state.serialize_entry("securityLabelNumber", &self.r#security_label_number)?;
                }
            }
            if !self.r#valued_item.is_empty() {
                state.serialize_entry("valuedItem", &self.r#valued_item)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermAsset {
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
            #[serde(rename = "scope")]
            Scope,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "typeReference")]
            TypeReference,
            #[serde(rename = "subtype")]
            Subtype,
            #[serde(rename = "relationship")]
            Relationship,
            #[serde(rename = "context")]
            Context,
            #[serde(rename = "condition")]
            Condition,
            #[serde(rename = "_condition")]
            ConditionPrimitiveElement,
            #[serde(rename = "periodType")]
            PeriodType,
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "usePeriod")]
            UsePeriod,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            #[serde(rename = "linkId")]
            LinkId,
            #[serde(rename = "_linkId")]
            LinkIdPrimitiveElement,
            #[serde(rename = "answer")]
            Answer,
            #[serde(rename = "securityLabelNumber")]
            SecurityLabelNumber,
            #[serde(rename = "_securityLabelNumber")]
            SecurityLabelNumberPrimitiveElement,
            #[serde(rename = "valuedItem")]
            ValuedItem,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermAsset;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermAsset")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermAsset, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#scope: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#type_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#subtype: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#relationship: Option<Box<super::super::types::Coding>> = None;
                let mut r#context: Option<Vec<ContractTermAssetContext>> = None;
                let mut r#condition: Option<super::super::types::String> = None;
                let mut r#period_type: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#period: Option<Vec<Box<super::super::types::Period>>> = None;
                let mut r#use_period: Option<Vec<Box<super::super::types::Period>>> = None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#answer: Option<Vec<ContractTermOfferAnswer>> = None;
                let mut r#security_label_number: Option<Vec<super::super::types::UnsignedInt>> =
                    None;
                let mut r#valued_item: Option<Vec<ContractTermAssetValuedItem>> = None;
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
                            Field::Scope => {
                                if r#scope.is_some() {
                                    return Err(serde::de::Error::duplicate_field("scope"));
                                }
                                r#scope = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::TypeReference => {
                                if r#type_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("typeReference"));
                                }
                                r#type_reference = Some(map_access.next_value()?);
                            }
                            Field::Subtype => {
                                if r#subtype.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subtype"));
                                }
                                r#subtype = Some(map_access.next_value()?);
                            }
                            Field::Relationship => {
                                if r#relationship.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relationship"));
                                }
                                r#relationship = Some(map_access.next_value()?);
                            }
                            Field::Context => {
                                if r#context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("context"));
                                }
                                r#context = Some(map_access.next_value()?);
                            }
                            Field::Condition => {
                                if _ctx.from_json {
                                    let some = r#condition.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("condition"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#condition.is_some() {
                                        return Err(serde::de::Error::duplicate_field("condition"));
                                    }
                                    r#condition = Some(map_access.next_value()?);
                                }
                            }
                            Field::ConditionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#condition.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_condition",
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
                                        "condition",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "scope",
                                            "type",
                                            "typeReference",
                                            "subtype",
                                            "relationship",
                                            "context",
                                            "condition",
                                            "periodType",
                                            "period",
                                            "usePeriod",
                                            "text",
                                            "linkId",
                                            "answer",
                                            "securityLabelNumber",
                                            "valuedItem",
                                        ],
                                    ));
                                }
                            }
                            Field::PeriodType => {
                                if r#period_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("periodType"));
                                }
                                r#period_type = Some(map_access.next_value()?);
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::UsePeriod => {
                                if r#use_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("usePeriod"));
                                }
                                r#use_period = Some(map_access.next_value()?);
                            }
                            Field::Text => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    r#text = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_text"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "text",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "scope",
                                            "type",
                                            "typeReference",
                                            "subtype",
                                            "relationship",
                                            "context",
                                            "condition",
                                            "periodType",
                                            "period",
                                            "usePeriod",
                                            "text",
                                            "linkId",
                                            "answer",
                                            "securityLabelNumber",
                                            "valuedItem",
                                        ],
                                    ));
                                }
                            }
                            Field::LinkId => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#link_id.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("linkId"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#link_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field("linkId"));
                                    }
                                    r#link_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::LinkIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#link_id.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_linkId"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "linkId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "scope",
                                            "type",
                                            "typeReference",
                                            "subtype",
                                            "relationship",
                                            "context",
                                            "condition",
                                            "periodType",
                                            "period",
                                            "usePeriod",
                                            "text",
                                            "linkId",
                                            "answer",
                                            "securityLabelNumber",
                                            "valuedItem",
                                        ],
                                    ));
                                }
                            }
                            Field::Answer => {
                                if r#answer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("answer"));
                                }
                                r#answer = Some(map_access.next_value()?);
                            }
                            Field::SecurityLabelNumber => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#security_label_number.get_or_insert(
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
                                            "securityLabelNumber",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#security_label_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "securityLabelNumber",
                                        ));
                                    }
                                    r#security_label_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::SecurityLabelNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#security_label_number.get_or_insert(
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
                                            "_securityLabelNumber",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "securityLabelNumber",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "scope",
                                            "type",
                                            "typeReference",
                                            "subtype",
                                            "relationship",
                                            "context",
                                            "condition",
                                            "periodType",
                                            "period",
                                            "usePeriod",
                                            "text",
                                            "linkId",
                                            "answer",
                                            "securityLabelNumber",
                                            "valuedItem",
                                        ],
                                    ));
                                }
                            }
                            Field::ValuedItem => {
                                if r#valued_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valuedItem"));
                                }
                                r#valued_item = Some(map_access.next_value()?);
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
                                        "scope",
                                        "type",
                                        "typeReference",
                                        "subtype",
                                        "relationship",
                                        "context",
                                        "condition",
                                        "periodType",
                                        "period",
                                        "usePeriod",
                                        "text",
                                        "linkId",
                                        "answer",
                                        "securityLabelNumber",
                                        "valuedItem",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractTermAsset {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#scope,
                        r#type: r#type.unwrap_or(vec![]),
                        r#type_reference: r#type_reference.unwrap_or(vec![]),
                        r#subtype: r#subtype.unwrap_or(vec![]),
                        r#relationship,
                        r#context: r#context.unwrap_or(vec![]),
                        r#condition,
                        r#period_type: r#period_type.unwrap_or(vec![]),
                        r#period: r#period.unwrap_or(vec![]),
                        r#use_period: r#use_period.unwrap_or(vec![]),
                        r#text,
                        r#link_id: r#link_id.unwrap_or(vec![]),
                        r#answer: r#answer.unwrap_or(vec![]),
                        r#security_label_number: r#security_label_number.unwrap_or(vec![]),
                        r#valued_item: r#valued_item.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Entity of the action."]
#[derive(Default, Debug, Clone)]
pub struct ContractTermActionSubject {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The entity the action is performed or not performed on or for."]
    pub r#reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Role type of agent assigned roles in this Contract."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ContractTermActionSubject {
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
            if !self.r#reference.is_empty() {
                state.serialize_entry("reference", &self.r#reference)?;
            }
            if let Some(some) = self.r#role.as_ref() {
                state.serialize_entry("role", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermActionSubject {
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
            #[serde(rename = "reference")]
            Reference,
            #[serde(rename = "role")]
            Role,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermActionSubject;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermActionSubject")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermActionSubject, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
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
                            Field::Reference => {
                                if r#reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                r#reference = Some(map_access.next_value()?);
                            }
                            Field::Role => {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                r#role = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "reference", "role"],
                                ));
                            },
                        }
                    }
                    Ok(ContractTermActionSubject {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#reference: r#reference.unwrap_or(vec![]),
                        r#role,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An actor taking a role in an activity for which it can be assigned some degree of responsibility for the activity taking place."]
#[derive(Default, Debug, Clone)]
pub struct ContractTermAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "True if the term prohibits the  action."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "Activity or service obligation to be done or not done, performed or not performed, effectuated or not by this Contract term."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Entity of the action."]
    pub r#subject: Vec<ContractTermActionSubject>,
    #[doc = "Reason or purpose for the action stipulated by this Contract Provision."]
    pub r#intent: Box<super::super::types::CodeableConcept>,
    #[doc = "Id [identifier??] of the clause or question text related to this action in the referenced form or QuestionnaireResponse."]
    pub r#link_id: Vec<super::super::types::String>,
    #[doc = "Current state of the term action."]
    pub r#status: Box<super::super::types::CodeableConcept>,
    #[doc = "Encounter or Episode with primary association to specified term activity."]
    pub r#context: Option<Box<super::super::types::Reference>>,
    #[doc = "Id [identifier??] of the clause or question text related to the requester of this action in the referenced form or QuestionnaireResponse."]
    pub r#context_link_id: Vec<super::super::types::String>,
    #[doc = "When action happens."]
    pub r#occurrence: Option<ContractTermActionOccurrence>,
    #[doc = "Who or what initiated the action and has responsibility for its activation."]
    pub r#requester: Vec<Box<super::super::types::Reference>>,
    #[doc = "Id [identifier??] of the clause or question text related to the requester of this action in the referenced form or QuestionnaireResponse."]
    pub r#requester_link_id: Vec<super::super::types::String>,
    #[doc = "The type of individual that is desired or required to perform or not perform the action."]
    pub r#performer_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of role or competency of an individual desired or required to perform or not perform the action."]
    pub r#performer_role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates who or what is being asked to perform (or not perform) the ction."]
    pub r#performer: Option<Box<super::super::types::Reference>>,
    #[doc = "Id [identifier??] of the clause or question text related to the reason type or reference of this  action in the referenced form or QuestionnaireResponse."]
    pub r#performer_link_id: Vec<super::super::types::String>,
    #[doc = "Rationale for the action to be performed or not performed. Describes why the action is permitted or prohibited."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates another resource whose existence justifies permitting or not permitting this action."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Describes why the action is to be performed or not performed in textual form."]
    pub r#reason: Vec<super::super::types::String>,
    #[doc = "Id [identifier??] of the clause or question text related to the reason type or reference of this  action in the referenced form or QuestionnaireResponse."]
    pub r#reason_link_id: Vec<super::super::types::String>,
    #[doc = "Comments made about the term action made by the requester, performer, subject or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Security labels that protects the action."]
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
}
impl serde::ser::Serialize for ContractTermAction {
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
                if let Some(some) = self.r#do_not_perform.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("doNotPerform", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_doNotPerform", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#do_not_perform.as_ref() {
                    state.serialize_entry("doNotPerform", some)?;
                }
            }
            state.serialize_entry("type", &self.r#type)?;
            if !self.r#subject.is_empty() {
                state.serialize_entry("subject", &self.r#subject)?;
            }
            state.serialize_entry("intent", &self.r#intent)?;
            if _ctx.output_json {
                if !self.r#link_id.is_empty() {
                    let values = self
                        .r#link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("linkId", &values)?;
                    }
                    let requires_elements = self
                        .r#link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#link_id
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
                        state.serialize_entry("_linkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#link_id.is_empty() {
                    state.serialize_entry("linkId", &self.r#link_id)?;
                }
            }
            state.serialize_entry("status", &self.r#status)?;
            if let Some(some) = self.r#context.as_ref() {
                state.serialize_entry("context", some)?;
            }
            if _ctx.output_json {
                if !self.r#context_link_id.is_empty() {
                    let values = self
                        .r#context_link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("contextLinkId", &values)?;
                    }
                    let requires_elements = self
                        .r#context_link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#context_link_id
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
                        state.serialize_entry("_contextLinkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#context_link_id.is_empty() {
                    state.serialize_entry("contextLinkId", &self.r#context_link_id)?;
                }
            }
            if let Some(some) = self.r#occurrence.as_ref() {
                match some {
                    ContractTermActionOccurrence::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("occurrenceDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_occurrenceDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("occurrenceDateTime", value)?;
                        }
                    }
                    ContractTermActionOccurrence::Period(ref value) => {
                        state.serialize_entry("occurrencePeriod", value)?;
                    }
                    ContractTermActionOccurrence::Timing(ref value) => {
                        state.serialize_entry("occurrenceTiming", value)?;
                    }
                    ContractTermActionOccurrence::Invalid => {
                        return Err(serde::ser::Error::custom("occurrence is invalid"))
                    }
                }
            }
            if !self.r#requester.is_empty() {
                state.serialize_entry("requester", &self.r#requester)?;
            }
            if _ctx.output_json {
                if !self.r#requester_link_id.is_empty() {
                    let values = self
                        .r#requester_link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("requesterLinkId", &values)?;
                    }
                    let requires_elements = self
                        .r#requester_link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#requester_link_id
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
                        state.serialize_entry("_requesterLinkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#requester_link_id.is_empty() {
                    state.serialize_entry("requesterLinkId", &self.r#requester_link_id)?;
                }
            }
            if !self.r#performer_type.is_empty() {
                state.serialize_entry("performerType", &self.r#performer_type)?;
            }
            if let Some(some) = self.r#performer_role.as_ref() {
                state.serialize_entry("performerRole", some)?;
            }
            if let Some(some) = self.r#performer.as_ref() {
                state.serialize_entry("performer", some)?;
            }
            if _ctx.output_json {
                if !self.r#performer_link_id.is_empty() {
                    let values = self
                        .r#performer_link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("performerLinkId", &values)?;
                    }
                    let requires_elements = self
                        .r#performer_link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#performer_link_id
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
                        state.serialize_entry("_performerLinkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#performer_link_id.is_empty() {
                    state.serialize_entry("performerLinkId", &self.r#performer_link_id)?;
                }
            }
            if !self.r#reason_code.is_empty() {
                state.serialize_entry("reasonCode", &self.r#reason_code)?;
            }
            if !self.r#reason_reference.is_empty() {
                state.serialize_entry("reasonReference", &self.r#reason_reference)?;
            }
            if _ctx.output_json {
                if !self.r#reason.is_empty() {
                    let values = self
                        .r#reason
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("reason", &values)?;
                    }
                    let requires_elements = self
                        .r#reason
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#reason
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
                        state.serialize_entry("_reason", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#reason.is_empty() {
                    state.serialize_entry("reason", &self.r#reason)?;
                }
            }
            if _ctx.output_json {
                if !self.r#reason_link_id.is_empty() {
                    let values = self
                        .r#reason_link_id
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("reasonLinkId", &values)?;
                    }
                    let requires_elements = self
                        .r#reason_link_id
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#reason_link_id
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
                        state.serialize_entry("_reasonLinkId", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#reason_link_id.is_empty() {
                    state.serialize_entry("reasonLinkId", &self.r#reason_link_id)?;
                }
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if _ctx.output_json {
                if !self.r#security_label_number.is_empty() {
                    let values = self
                        .r#security_label_number
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("securityLabelNumber", &values)?;
                    }
                    let requires_elements = self
                        .r#security_label_number
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#security_label_number
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
                        state.serialize_entry("_securityLabelNumber", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#security_label_number.is_empty() {
                    state.serialize_entry("securityLabelNumber", &self.r#security_label_number)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermAction {
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
            #[serde(rename = "doNotPerform")]
            DoNotPerform,
            #[serde(rename = "_doNotPerform")]
            DoNotPerformPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "intent")]
            Intent,
            #[serde(rename = "linkId")]
            LinkId,
            #[serde(rename = "_linkId")]
            LinkIdPrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "context")]
            Context,
            #[serde(rename = "contextLinkId")]
            ContextLinkId,
            #[serde(rename = "_contextLinkId")]
            ContextLinkIdPrimitiveElement,
            #[serde(rename = "occurrenceDateTime")]
            OccurrenceDateTime,
            #[serde(rename = "_occurrenceDateTime")]
            OccurrenceDateTimePrimitiveElement,
            #[serde(rename = "occurrencePeriod")]
            OccurrencePeriod,
            #[serde(rename = "occurrenceTiming")]
            OccurrenceTiming,
            #[serde(rename = "requester")]
            Requester,
            #[serde(rename = "requesterLinkId")]
            RequesterLinkId,
            #[serde(rename = "_requesterLinkId")]
            RequesterLinkIdPrimitiveElement,
            #[serde(rename = "performerType")]
            PerformerType,
            #[serde(rename = "performerRole")]
            PerformerRole,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "performerLinkId")]
            PerformerLinkId,
            #[serde(rename = "_performerLinkId")]
            PerformerLinkIdPrimitiveElement,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "reason")]
            Reason,
            #[serde(rename = "_reason")]
            ReasonPrimitiveElement,
            #[serde(rename = "reasonLinkId")]
            ReasonLinkId,
            #[serde(rename = "_reasonLinkId")]
            ReasonLinkIdPrimitiveElement,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "securityLabelNumber")]
            SecurityLabelNumber,
            #[serde(rename = "_securityLabelNumber")]
            SecurityLabelNumberPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermAction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermAction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#do_not_perform: Option<super::super::types::Boolean> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Vec<ContractTermActionSubject>> = None;
                let mut r#intent: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#context: Option<Box<super::super::types::Reference>> = None;
                let mut r#context_link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#occurrence: Option<ContractTermActionOccurrence> = None;
                let mut r#requester: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#requester_link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#performer_type: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#performer_role: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#performer: Option<Box<super::super::types::Reference>> = None;
                let mut r#performer_link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#reason: Option<Vec<super::super::types::String>> = None;
                let mut r#reason_link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#security_label_number: Option<Vec<super::super::types::UnsignedInt>> =
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
                            Field::DoNotPerform => {
                                if _ctx.from_json {
                                    let some = r#do_not_perform.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "doNotPerform",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#do_not_perform.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "doNotPerform",
                                        ));
                                    }
                                    r#do_not_perform = Some(map_access.next_value()?);
                                }
                            }
                            Field::DoNotPerformPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#do_not_perform.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_doNotPerform",
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
                                        "doNotPerform",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "doNotPerform",
                                            "type",
                                            "subject",
                                            "intent",
                                            "linkId",
                                            "status",
                                            "context",
                                            "contextLinkId",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "requester",
                                            "requesterLinkId",
                                            "performerType",
                                            "performerRole",
                                            "performer",
                                            "performerLinkId",
                                            "reasonCode",
                                            "reasonReference",
                                            "reason",
                                            "reasonLinkId",
                                            "note",
                                            "securityLabelNumber",
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
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Intent => {
                                if r#intent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intent"));
                                }
                                r#intent = Some(map_access.next_value()?);
                            }
                            Field::LinkId => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#link_id.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("linkId"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#link_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field("linkId"));
                                    }
                                    r#link_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::LinkIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#link_id.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_linkId"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "linkId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "doNotPerform",
                                            "type",
                                            "subject",
                                            "intent",
                                            "linkId",
                                            "status",
                                            "context",
                                            "contextLinkId",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "requester",
                                            "requesterLinkId",
                                            "performerType",
                                            "performerRole",
                                            "performer",
                                            "performerLinkId",
                                            "reasonCode",
                                            "reasonReference",
                                            "reason",
                                            "reasonLinkId",
                                            "note",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::Status => {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value()?);
                            }
                            Field::Context => {
                                if r#context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("context"));
                                }
                                r#context = Some(map_access.next_value()?);
                            }
                            Field::ContextLinkId => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#context_link_id.get_or_insert(
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
                                            "contextLinkId",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#context_link_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "contextLinkId",
                                        ));
                                    }
                                    r#context_link_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::ContextLinkIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#context_link_id.get_or_insert(
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
                                            "_contextLinkId",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "contextLinkId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "doNotPerform",
                                            "type",
                                            "subject",
                                            "intent",
                                            "linkId",
                                            "status",
                                            "context",
                                            "contextLinkId",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "requester",
                                            "requesterLinkId",
                                            "performerType",
                                            "performerRole",
                                            "performer",
                                            "performerLinkId",
                                            "reasonCode",
                                            "reasonReference",
                                            "reason",
                                            "reasonLinkId",
                                            "note",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::OccurrenceDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#occurrence.get_or_insert(
                                        ContractTermActionOccurrence::DateTime(Default::default()),
                                    );
                                    if let ContractTermActionOccurrence::DateTime(variant) = r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "occurrenceDateTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "occurrence[x]",
                                        ));
                                    }
                                } else {
                                    if r#occurrence.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "occurrenceDateTime",
                                        ));
                                    }
                                    r#occurrence = Some(ContractTermActionOccurrence::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::OccurrenceDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#occurrence.get_or_insert(
                                        ContractTermActionOccurrence::DateTime(Default::default()),
                                    );
                                    if let ContractTermActionOccurrence::DateTime(variant) = r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_occurrenceDateTime",
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
                                            "_occurrence[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "occurrenceDateTime",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "doNotPerform",
                                            "type",
                                            "subject",
                                            "intent",
                                            "linkId",
                                            "status",
                                            "context",
                                            "contextLinkId",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "requester",
                                            "requesterLinkId",
                                            "performerType",
                                            "performerRole",
                                            "performer",
                                            "performerLinkId",
                                            "reasonCode",
                                            "reasonReference",
                                            "reason",
                                            "reasonLinkId",
                                            "note",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::OccurrencePeriod => {
                                if r#occurrence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrencePeriod",
                                    ));
                                }
                                r#occurrence = Some(ContractTermActionOccurrence::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::OccurrenceTiming => {
                                if r#occurrence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceTiming",
                                    ));
                                }
                                r#occurrence = Some(ContractTermActionOccurrence::Timing(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Requester => {
                                if r#requester.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requester"));
                                }
                                r#requester = Some(map_access.next_value()?);
                            }
                            Field::RequesterLinkId => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#requester_link_id.get_or_insert(
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
                                            "requesterLinkId",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#requester_link_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "requesterLinkId",
                                        ));
                                    }
                                    r#requester_link_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::RequesterLinkIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#requester_link_id.get_or_insert(
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
                                            "_requesterLinkId",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "requesterLinkId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "doNotPerform",
                                            "type",
                                            "subject",
                                            "intent",
                                            "linkId",
                                            "status",
                                            "context",
                                            "contextLinkId",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "requester",
                                            "requesterLinkId",
                                            "performerType",
                                            "performerRole",
                                            "performer",
                                            "performerLinkId",
                                            "reasonCode",
                                            "reasonReference",
                                            "reason",
                                            "reasonLinkId",
                                            "note",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::PerformerType => {
                                if r#performer_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performerType"));
                                }
                                r#performer_type = Some(map_access.next_value()?);
                            }
                            Field::PerformerRole => {
                                if r#performer_role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performerRole"));
                                }
                                r#performer_role = Some(map_access.next_value()?);
                            }
                            Field::Performer => {
                                if r#performer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performer"));
                                }
                                r#performer = Some(map_access.next_value()?);
                            }
                            Field::PerformerLinkId => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#performer_link_id.get_or_insert(
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
                                            "performerLinkId",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#performer_link_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "performerLinkId",
                                        ));
                                    }
                                    r#performer_link_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::PerformerLinkIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#performer_link_id.get_or_insert(
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
                                            "_performerLinkId",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "performerLinkId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "doNotPerform",
                                            "type",
                                            "subject",
                                            "intent",
                                            "linkId",
                                            "status",
                                            "context",
                                            "contextLinkId",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "requester",
                                            "requesterLinkId",
                                            "performerType",
                                            "performerRole",
                                            "performer",
                                            "performerLinkId",
                                            "reasonCode",
                                            "reasonReference",
                                            "reason",
                                            "reasonLinkId",
                                            "note",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
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
                            Field::Reason => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#reason.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("reason"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#reason.is_some() {
                                        return Err(serde::de::Error::duplicate_field("reason"));
                                    }
                                    r#reason = Some(map_access.next_value()?);
                                }
                            }
                            Field::ReasonPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#reason.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_reason"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "reason",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "doNotPerform",
                                            "type",
                                            "subject",
                                            "intent",
                                            "linkId",
                                            "status",
                                            "context",
                                            "contextLinkId",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "requester",
                                            "requesterLinkId",
                                            "performerType",
                                            "performerRole",
                                            "performer",
                                            "performerLinkId",
                                            "reasonCode",
                                            "reasonReference",
                                            "reason",
                                            "reasonLinkId",
                                            "note",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::ReasonLinkId => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#reason_link_id.get_or_insert(
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
                                            "reasonLinkId",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#reason_link_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "reasonLinkId",
                                        ));
                                    }
                                    r#reason_link_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::ReasonLinkIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#reason_link_id.get_or_insert(
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
                                            "_reasonLinkId",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "reasonLinkId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "doNotPerform",
                                            "type",
                                            "subject",
                                            "intent",
                                            "linkId",
                                            "status",
                                            "context",
                                            "contextLinkId",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "requester",
                                            "requesterLinkId",
                                            "performerType",
                                            "performerRole",
                                            "performer",
                                            "performerLinkId",
                                            "reasonCode",
                                            "reasonReference",
                                            "reason",
                                            "reasonLinkId",
                                            "note",
                                            "securityLabelNumber",
                                        ],
                                    ));
                                }
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
                            }
                            Field::SecurityLabelNumber => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#security_label_number.get_or_insert(
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
                                            "securityLabelNumber",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#security_label_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "securityLabelNumber",
                                        ));
                                    }
                                    r#security_label_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::SecurityLabelNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#security_label_number.get_or_insert(
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
                                            "_securityLabelNumber",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "securityLabelNumber",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "doNotPerform",
                                            "type",
                                            "subject",
                                            "intent",
                                            "linkId",
                                            "status",
                                            "context",
                                            "contextLinkId",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "requester",
                                            "requesterLinkId",
                                            "performerType",
                                            "performerRole",
                                            "performer",
                                            "performerLinkId",
                                            "reasonCode",
                                            "reasonReference",
                                            "reason",
                                            "reasonLinkId",
                                            "note",
                                            "securityLabelNumber",
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
                                        "doNotPerform",
                                        "type",
                                        "subject",
                                        "intent",
                                        "linkId",
                                        "status",
                                        "context",
                                        "contextLinkId",
                                        "occurrenceDateTime",
                                        "occurrencePeriod",
                                        "occurrenceTiming",
                                        "requester",
                                        "requesterLinkId",
                                        "performerType",
                                        "performerRole",
                                        "performer",
                                        "performerLinkId",
                                        "reasonCode",
                                        "reasonReference",
                                        "reason",
                                        "reasonLinkId",
                                        "note",
                                        "securityLabelNumber",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractTermAction {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#do_not_perform,
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#subject: r#subject.unwrap_or(vec![]),
                        r#intent: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#intent.unwrap_or(Default::default())
                        } else {
                            r#intent.ok_or(serde::de::Error::missing_field("intent"))?
                        },
                        r#link_id: r#link_id.unwrap_or(vec![]),
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#context,
                        r#context_link_id: r#context_link_id.unwrap_or(vec![]),
                        r#occurrence,
                        r#requester: r#requester.unwrap_or(vec![]),
                        r#requester_link_id: r#requester_link_id.unwrap_or(vec![]),
                        r#performer_type: r#performer_type.unwrap_or(vec![]),
                        r#performer_role,
                        r#performer,
                        r#performer_link_id: r#performer_link_id.unwrap_or(vec![]),
                        r#reason_code: r#reason_code.unwrap_or(vec![]),
                        r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                        r#reason: r#reason.unwrap_or(vec![]),
                        r#reason_link_id: r#reason_link_id.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                        r#security_label_number: r#security_label_number.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "One or more Contract Provisions, which may be related and conveyed as a group, and may contain nested groups."]
#[derive(Default, Debug, Clone)]
pub struct ContractTerm {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Unique identifier for this particular Contract Provision."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "When this Contract Provision was issued."]
    pub r#issued: Option<super::super::types::DateTime>,
    #[doc = "Relevant time or time-period when this Contract Provision is applicable."]
    pub r#applies: Option<Box<super::super::types::Period>>,
    #[doc = "The entity that the term applies to."]
    pub r#topic: Option<ContractTermTopic>,
    #[doc = "A legal clause or condition contained within a contract that requires one or both parties to perform a particular requirement by some specified time or prevents one or both parties from performing a particular requirement by some specified time."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A specialized legal clause or condition based on overarching contract type."]
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Statement of a provision in a policy or a contract."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "Security labels that protect the handling of information about the term and its elements, which may be specifically identified.."]
    pub r#security_label: Vec<ContractTermSecurityLabel>,
    #[doc = "The matter of concern in the context of this provision of the agrement."]
    pub r#offer: ContractTermOffer,
    #[doc = "Contract Term Asset List."]
    pub r#asset: Vec<ContractTermAsset>,
    #[doc = "An actor taking a role in an activity for which it can be assigned some degree of responsibility for the activity taking place."]
    pub r#action: Vec<ContractTermAction>,
    #[doc = "Nested group of Contract Provisions."]
    pub r#group: Vec<ContractTerm>,
}
impl serde::ser::Serialize for ContractTerm {
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
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#issued.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("issued", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_issued", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#issued.as_ref() {
                    state.serialize_entry("issued", some)?;
                }
            }
            if let Some(some) = self.r#applies.as_ref() {
                state.serialize_entry("applies", some)?;
            }
            if let Some(some) = self.r#topic.as_ref() {
                match some {
                    ContractTermTopic::CodeableConcept(ref value) => {
                        state.serialize_entry("topicCodeableConcept", value)?;
                    }
                    ContractTermTopic::Reference(ref value) => {
                        state.serialize_entry("topicReference", value)?;
                    }
                    ContractTermTopic::Invalid => {
                        return Err(serde::ser::Error::custom("topic is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#sub_type.as_ref() {
                state.serialize_entry("subType", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            if !self.r#security_label.is_empty() {
                state.serialize_entry("securityLabel", &self.r#security_label)?;
            }
            state.serialize_entry("offer", &self.r#offer)?;
            if !self.r#asset.is_empty() {
                state.serialize_entry("asset", &self.r#asset)?;
            }
            if !self.r#action.is_empty() {
                state.serialize_entry("action", &self.r#action)?;
            }
            if !self.r#group.is_empty() {
                state.serialize_entry("group", &self.r#group)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTerm {
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
            #[serde(rename = "issued")]
            Issued,
            #[serde(rename = "_issued")]
            IssuedPrimitiveElement,
            #[serde(rename = "applies")]
            Applies,
            #[serde(rename = "topicCodeableConcept")]
            TopicCodeableConcept,
            #[serde(rename = "topicReference")]
            TopicReference,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "subType")]
            SubType,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            #[serde(rename = "securityLabel")]
            SecurityLabel,
            #[serde(rename = "offer")]
            Offer,
            #[serde(rename = "asset")]
            Asset,
            #[serde(rename = "action")]
            Action,
            #[serde(rename = "group")]
            Group,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTerm;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTerm")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTerm, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#issued: Option<super::super::types::DateTime> = None;
                let mut r#applies: Option<Box<super::super::types::Period>> = None;
                let mut r#topic: Option<ContractTermTopic> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#security_label: Option<Vec<ContractTermSecurityLabel>> = None;
                let mut r#offer: Option<ContractTermOffer> = None;
                let mut r#asset: Option<Vec<ContractTermAsset>> = None;
                let mut r#action: Option<Vec<ContractTermAction>> = None;
                let mut r#group: Option<Vec<ContractTerm>> = None;
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
                            Field::Issued => {
                                if _ctx.from_json {
                                    let some = r#issued.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("issued"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#issued.is_some() {
                                        return Err(serde::de::Error::duplicate_field("issued"));
                                    }
                                    r#issued = Some(map_access.next_value()?);
                                }
                            }
                            Field::IssuedPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "issued",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "issued",
                                            "applies",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "text",
                                            "securityLabel",
                                            "offer",
                                            "asset",
                                            "action",
                                            "group",
                                        ],
                                    ));
                                }
                            }
                            Field::Applies => {
                                if r#applies.is_some() {
                                    return Err(serde::de::Error::duplicate_field("applies"));
                                }
                                r#applies = Some(map_access.next_value()?);
                            }
                            Field::TopicCodeableConcept => {
                                if r#topic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "topicCodeableConcept",
                                    ));
                                }
                                r#topic = Some(ContractTermTopic::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::TopicReference => {
                                if r#topic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "topicReference",
                                    ));
                                }
                                r#topic =
                                    Some(ContractTermTopic::Reference(map_access.next_value()?));
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::SubType => {
                                if r#sub_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subType"));
                                }
                                r#sub_type = Some(map_access.next_value()?);
                            }
                            Field::Text => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    r#text = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_text"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "text",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "issued",
                                            "applies",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "text",
                                            "securityLabel",
                                            "offer",
                                            "asset",
                                            "action",
                                            "group",
                                        ],
                                    ));
                                }
                            }
                            Field::SecurityLabel => {
                                if r#security_label.is_some() {
                                    return Err(serde::de::Error::duplicate_field("securityLabel"));
                                }
                                r#security_label = Some(map_access.next_value()?);
                            }
                            Field::Offer => {
                                if r#offer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("offer"));
                                }
                                r#offer = Some(map_access.next_value()?);
                            }
                            Field::Asset => {
                                if r#asset.is_some() {
                                    return Err(serde::de::Error::duplicate_field("asset"));
                                }
                                r#asset = Some(map_access.next_value()?);
                            }
                            Field::Action => {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                r#action = Some(map_access.next_value()?);
                            }
                            Field::Group => {
                                if r#group.is_some() {
                                    return Err(serde::de::Error::duplicate_field("group"));
                                }
                                r#group = Some(map_access.next_value()?);
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
                                        "issued",
                                        "applies",
                                        "topicCodeableConcept",
                                        "topicReference",
                                        "type",
                                        "subType",
                                        "text",
                                        "securityLabel",
                                        "offer",
                                        "asset",
                                        "action",
                                        "group",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractTerm {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier,
                        r#issued,
                        r#applies,
                        r#topic,
                        r#type,
                        r#sub_type,
                        r#text,
                        r#security_label: r#security_label.unwrap_or(vec![]),
                        r#offer: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#offer.unwrap_or(Default::default())
                        } else {
                            r#offer.ok_or(serde::de::Error::missing_field("offer"))?
                        },
                        r#asset: r#asset.unwrap_or(vec![]),
                        r#action: r#action.unwrap_or(vec![]),
                        r#group: r#group.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Parties with legal standing in the Contract, including the principal parties, the grantor(s) and grantee(s), which are any person or organization bound by the contract, and any ancillary parties, which facilitate the execution of the contract such as a notary or witness."]
#[derive(Default, Debug, Clone)]
pub struct ContractSigner {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Role of this Contract signer, e.g. notary, grantee."]
    pub r#type: Box<super::super::types::Coding>,
    #[doc = "Party which is a signator to this Contract."]
    pub r#party: Box<super::super::types::Reference>,
    #[doc = "Legally binding Contract DSIG signature contents in Base64."]
    pub r#signature: Vec<Box<super::super::types::Signature>>,
}
impl serde::ser::Serialize for ContractSigner {
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
            state.serialize_entry("type", &self.r#type)?;
            state.serialize_entry("party", &self.r#party)?;
            if !self.r#signature.is_empty() {
                state.serialize_entry("signature", &self.r#signature)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractSigner {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "party")]
            Party,
            #[serde(rename = "signature")]
            Signature,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractSigner;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractSigner")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractSigner, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::Coding>> = None;
                let mut r#party: Option<Box<super::super::types::Reference>> = None;
                let mut r#signature: Option<Vec<Box<super::super::types::Signature>>> = None;
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Party => {
                                if r#party.is_some() {
                                    return Err(serde::de::Error::duplicate_field("party"));
                                }
                                r#party = Some(map_access.next_value()?);
                            }
                            Field::Signature => {
                                if r#signature.is_some() {
                                    return Err(serde::de::Error::duplicate_field("signature"));
                                }
                                r#signature = Some(map_access.next_value()?);
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
                                        "type",
                                        "party",
                                        "signature",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractSigner {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#party: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#party.unwrap_or(Default::default())
                        } else {
                            r#party.ok_or(serde::de::Error::missing_field("party"))?
                        },
                        r#signature: r#signature.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The \"patient friendly language\" versionof the Contract in whole or in parts. \"Patient friendly language\" means the representation of the Contract and Contract Provisions in a manner that is readily accessible and understandable by a layperson in accordance with best practices for communication styles that ensure that those agreeing to or signing the Contract understand the roles, actions, obligations, responsibilities, and implication of the agreement."]
#[derive(Default, Debug, Clone)]
pub struct ContractFriendly {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Human readable rendering of this Contract in a format and representation intended to enhance comprehension and ensure understandability."]
    pub r#content: ContractFriendlyContent,
}
impl serde::ser::Serialize for ContractFriendly {
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
            match self.r#content {
                ContractFriendlyContent::Attachment(ref value) => {
                    state.serialize_entry("contentAttachment", value)?;
                }
                ContractFriendlyContent::Reference(ref value) => {
                    state.serialize_entry("contentReference", value)?;
                }
                ContractFriendlyContent::Invalid => {
                    return Err(serde::ser::Error::custom("content is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractFriendly {
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
            #[serde(rename = "contentAttachment")]
            ContentAttachment,
            #[serde(rename = "contentReference")]
            ContentReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractFriendly;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractFriendly")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractFriendly, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#content: Option<ContractFriendlyContent> = None;
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
                            Field::ContentAttachment => {
                                if r#content.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentAttachment",
                                    ));
                                }
                                r#content = Some(ContractFriendlyContent::Attachment(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ContentReference => {
                                if r#content.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentReference",
                                    ));
                                }
                                r#content = Some(ContractFriendlyContent::Reference(
                                    map_access.next_value()?,
                                ));
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
                                        "contentAttachment",
                                        "contentReference",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractFriendly {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#content: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#content.unwrap_or(Default::default())
                        } else {
                            r#content.ok_or(serde::de::Error::missing_field("content[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "List of Legal expressions or representations of this Contract."]
#[derive(Default, Debug, Clone)]
pub struct ContractLegal {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Contract legal text in human renderable form."]
    pub r#content: ContractLegalContent,
}
impl serde::ser::Serialize for ContractLegal {
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
            match self.r#content {
                ContractLegalContent::Attachment(ref value) => {
                    state.serialize_entry("contentAttachment", value)?;
                }
                ContractLegalContent::Reference(ref value) => {
                    state.serialize_entry("contentReference", value)?;
                }
                ContractLegalContent::Invalid => {
                    return Err(serde::ser::Error::custom("content is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractLegal {
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
            #[serde(rename = "contentAttachment")]
            ContentAttachment,
            #[serde(rename = "contentReference")]
            ContentReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractLegal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractLegal")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractLegal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#content: Option<ContractLegalContent> = None;
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
                            Field::ContentAttachment => {
                                if r#content.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentAttachment",
                                    ));
                                }
                                r#content = Some(ContractLegalContent::Attachment(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ContentReference => {
                                if r#content.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentReference",
                                    ));
                                }
                                r#content =
                                    Some(ContractLegalContent::Reference(map_access.next_value()?));
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
                                        "contentAttachment",
                                        "contentReference",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractLegal {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#content: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#content.unwrap_or(Default::default())
                        } else {
                            r#content.ok_or(serde::de::Error::missing_field("content[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "List of Computable Policy Rule Language Representations of this Contract."]
#[derive(Default, Debug, Clone)]
pub struct ContractRule {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Computable Contract conveyed using a policy rule language (e.g. XACML, DKAL, SecPal)."]
    pub r#content: ContractRuleContent,
}
impl serde::ser::Serialize for ContractRule {
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
            match self.r#content {
                ContractRuleContent::Attachment(ref value) => {
                    state.serialize_entry("contentAttachment", value)?;
                }
                ContractRuleContent::Reference(ref value) => {
                    state.serialize_entry("contentReference", value)?;
                }
                ContractRuleContent::Invalid => {
                    return Err(serde::ser::Error::custom("content is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractRule {
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
            #[serde(rename = "contentAttachment")]
            ContentAttachment,
            #[serde(rename = "contentReference")]
            ContentReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractRule;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractRule")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractRule, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#content: Option<ContractRuleContent> = None;
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
                            Field::ContentAttachment => {
                                if r#content.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentAttachment",
                                    ));
                                }
                                r#content =
                                    Some(ContractRuleContent::Attachment(map_access.next_value()?));
                            }
                            Field::ContentReference => {
                                if r#content.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentReference",
                                    ));
                                }
                                r#content =
                                    Some(ContractRuleContent::Reference(map_access.next_value()?));
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
                                        "contentAttachment",
                                        "contentReference",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ContractRule {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#content: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#content.unwrap_or(Default::default())
                        } else {
                            r#content.ok_or(serde::de::Error::missing_field("content[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Legally enforceable, formally recorded unilateral or bilateral directive i.e., a policy or agreement."]
#[derive(Default, Debug, Clone)]
pub struct Contract {
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
    #[doc = "Unique identifier for this Contract or a derivative that references a Source Contract."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Canonical identifier for this contract, represented as a URI (globally unique)."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "An edition identifier used for business purposes to label business significant variants."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "The status of the resource instance."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Legal states of the formation of a legal instrument, which is a formally executed written document that can be formally attributed to its author, records and formally expresses a legally enforceable act, process, or contractual duty, obligation, or right, and therefore evidences that act, process, or agreement."]
    pub r#legal_state: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The URL pointing to a FHIR-defined Contract Definition that is adhered to in whole or part by this Contract."]
    pub r#instantiates_canonical: Option<Box<super::super::types::Reference>>,
    #[doc = "The URL pointing to an externally maintained definition that is adhered to in whole or in part by this Contract."]
    pub r#instantiates_uri: Option<super::super::types::Uri>,
    #[doc = "The minimal content derived from the basal information source at a specific stage in its lifecycle."]
    pub r#content_derivative: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When this  Contract was issued."]
    pub r#issued: Option<super::super::types::DateTime>,
    #[doc = "Relevant time or time-period when this Contract is applicable."]
    pub r#applies: Option<Box<super::super::types::Period>>,
    #[doc = "Event resulting in discontinuation or termination of this Contract instance by one or more parties to the contract."]
    pub r#expiration_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The target entity impacted by or of interest to parties to the agreement."]
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    #[doc = "A formally or informally recognized grouping of people, principals, organizations, or jurisdictions formed for the purpose of achieving some form of collective action such as the promulgation, administration and enforcement of contracts and policies."]
    pub r#authority: Vec<Box<super::super::types::Reference>>,
    #[doc = "Recognized governance framework or system operating with a circumscribed scope in accordance with specified principles, policies, processes or procedures for managing rights, actions, or behaviors of parties or principals relative to resources."]
    pub r#domain: Vec<Box<super::super::types::Reference>>,
    #[doc = "Sites in which the contract is complied with,  exercised, or in force."]
    pub r#site: Vec<Box<super::super::types::Reference>>,
    #[doc = "A natural language name identifying this Contract definition, derivative, or instance in any legal state. Provides additional information about its content. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for this Contract definition, derivative, or instance in any legal state.t giving additional information about its content."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "An explanatory or alternate user-friendly title for this Contract definition, derivative, or instance in any legal state.t giving additional information about its content."]
    pub r#subtitle: Option<super::super::types::String>,
    #[doc = "Alternative representation of the title for this Contract definition, derivative, or instance in any legal state., e.g., a domain specific contract number related to legislation."]
    pub r#alias: Vec<super::super::types::String>,
    #[doc = "The individual or organization that authored the Contract definition, derivative, or instance in any legal state."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "A selector of legal concerns for this Contract definition, derivative, or instance in any legal state."]
    pub r#scope: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Narrows the range of legal concerns to focus on the achievement of specific contractual objectives."]
    pub r#topic: Option<ContractTopic>,
    #[doc = "A high-level category for the legal instrument, whether constructed as a Contract definition, derivative, or instance in any legal state.  Provides additional information about its content within the context of the Contract's scope to distinguish the kinds of systems that would be interested in the contract."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Sub-category for the Contract that distinguishes the kinds of systems that would be interested in the Contract within the context of the Contract's scope."]
    pub r#sub_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Precusory content developed with a focus and intent of supporting the formation a Contract instance, which may be associated with and transformable into a Contract."]
    pub r#content_definition: Option<ContractContentDefinition>,
    #[doc = "One or more Contract Provisions, which may be related and conveyed as a group, and may contain nested groups."]
    pub r#term: Vec<ContractTerm>,
    #[doc = "Information that may be needed by/relevant to the performer in their execution of this term action."]
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    #[doc = "Links to Provenance records for past versions of this Contract definition, derivative, or instance, which identify key state transitions or updates that are likely to be relevant to a user looking at the current version of the Contract.  The Provence.entity indicates the target that was changed in the update. <http://build.fhir.org/provenance>-definitions.html#Provenance.entity."]
    pub r#relevant_history: Vec<Box<super::super::types::Reference>>,
    #[doc = "Parties with legal standing in the Contract, including the principal parties, the grantor(s) and grantee(s), which are any person or organization bound by the contract, and any ancillary parties, which facilitate the execution of the contract such as a notary or witness."]
    pub r#signer: Vec<ContractSigner>,
    #[doc = "The \"patient friendly language\" versionof the Contract in whole or in parts. \"Patient friendly language\" means the representation of the Contract and Contract Provisions in a manner that is readily accessible and understandable by a layperson in accordance with best practices for communication styles that ensure that those agreeing to or signing the Contract understand the roles, actions, obligations, responsibilities, and implication of the agreement."]
    pub r#friendly: Vec<ContractFriendly>,
    #[doc = "List of Legal expressions or representations of this Contract."]
    pub r#legal: Vec<ContractLegal>,
    #[doc = "List of Computable Policy Rule Language Representations of this Contract."]
    pub r#rule: Vec<ContractRule>,
    #[doc = "Legally binding Contract: This is the signed and legally recognized representation of the Contract, which is considered the \"source of truth\" and which would be the basis for legal action related to enforcement of this Contract."]
    pub r#legally_binding: Option<ContractLegallyBinding>,
}
impl crate::AnyResource for Contract {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for Contract {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Contract")?;
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
                if let Some(some) = self.r#url.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("url", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_url", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#url.as_ref() {
                    state.serialize_entry("url", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#version.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("version", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_version", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#version.as_ref() {
                    state.serialize_entry("version", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("status", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_status", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status.as_ref() {
                    state.serialize_entry("status", some)?;
                }
            }
            if let Some(some) = self.r#legal_state.as_ref() {
                state.serialize_entry("legalState", some)?;
            }
            if let Some(some) = self.r#instantiates_canonical.as_ref() {
                state.serialize_entry("instantiatesCanonical", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#instantiates_uri.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("instantiatesUri", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_instantiatesUri", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#instantiates_uri.as_ref() {
                    state.serialize_entry("instantiatesUri", some)?;
                }
            }
            if let Some(some) = self.r#content_derivative.as_ref() {
                state.serialize_entry("contentDerivative", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#issued.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("issued", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_issued", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#issued.as_ref() {
                    state.serialize_entry("issued", some)?;
                }
            }
            if let Some(some) = self.r#applies.as_ref() {
                state.serialize_entry("applies", some)?;
            }
            if let Some(some) = self.r#expiration_type.as_ref() {
                state.serialize_entry("expirationType", some)?;
            }
            if !self.r#subject.is_empty() {
                state.serialize_entry("subject", &self.r#subject)?;
            }
            if !self.r#authority.is_empty() {
                state.serialize_entry("authority", &self.r#authority)?;
            }
            if !self.r#domain.is_empty() {
                state.serialize_entry("domain", &self.r#domain)?;
            }
            if !self.r#site.is_empty() {
                state.serialize_entry("site", &self.r#site)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("title", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_title", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#title.as_ref() {
                    state.serialize_entry("title", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#subtitle.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("subtitle", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_subtitle", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#subtitle.as_ref() {
                    state.serialize_entry("subtitle", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#alias.is_empty() {
                    let values = self
                        .r#alias
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("alias", &values)?;
                    }
                    let requires_elements = self
                        .r#alias
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#alias
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
                        state.serialize_entry("_alias", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#alias.is_empty() {
                    state.serialize_entry("alias", &self.r#alias)?;
                }
            }
            if let Some(some) = self.r#author.as_ref() {
                state.serialize_entry("author", some)?;
            }
            if let Some(some) = self.r#scope.as_ref() {
                state.serialize_entry("scope", some)?;
            }
            if let Some(some) = self.r#topic.as_ref() {
                match some {
                    ContractTopic::CodeableConcept(ref value) => {
                        state.serialize_entry("topicCodeableConcept", value)?;
                    }
                    ContractTopic::Reference(ref value) => {
                        state.serialize_entry("topicReference", value)?;
                    }
                    ContractTopic::Invalid => {
                        return Err(serde::ser::Error::custom("topic is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if !self.r#sub_type.is_empty() {
                state.serialize_entry("subType", &self.r#sub_type)?;
            }
            if let Some(some) = self.r#content_definition.as_ref() {
                state.serialize_entry("contentDefinition", some)?;
            }
            if !self.r#term.is_empty() {
                state.serialize_entry("term", &self.r#term)?;
            }
            if !self.r#supporting_info.is_empty() {
                state.serialize_entry("supportingInfo", &self.r#supporting_info)?;
            }
            if !self.r#relevant_history.is_empty() {
                state.serialize_entry("relevantHistory", &self.r#relevant_history)?;
            }
            if !self.r#signer.is_empty() {
                state.serialize_entry("signer", &self.r#signer)?;
            }
            if !self.r#friendly.is_empty() {
                state.serialize_entry("friendly", &self.r#friendly)?;
            }
            if !self.r#legal.is_empty() {
                state.serialize_entry("legal", &self.r#legal)?;
            }
            if !self.r#rule.is_empty() {
                state.serialize_entry("rule", &self.r#rule)?;
            }
            if let Some(some) = self.r#legally_binding.as_ref() {
                match some {
                    ContractLegallyBinding::Attachment(ref value) => {
                        state.serialize_entry("legallyBindingAttachment", value)?;
                    }
                    ContractLegallyBinding::Reference(ref value) => {
                        state.serialize_entry("legallyBindingReference", value)?;
                    }
                    ContractLegallyBinding::Invalid => {
                        return Err(serde::ser::Error::custom("legally_binding is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Contract {
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
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "legalState")]
            LegalState,
            #[serde(rename = "instantiatesCanonical")]
            InstantiatesCanonical,
            #[serde(rename = "instantiatesUri")]
            InstantiatesUri,
            #[serde(rename = "_instantiatesUri")]
            InstantiatesUriPrimitiveElement,
            #[serde(rename = "contentDerivative")]
            ContentDerivative,
            #[serde(rename = "issued")]
            Issued,
            #[serde(rename = "_issued")]
            IssuedPrimitiveElement,
            #[serde(rename = "applies")]
            Applies,
            #[serde(rename = "expirationType")]
            ExpirationType,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "authority")]
            Authority,
            #[serde(rename = "domain")]
            Domain,
            #[serde(rename = "site")]
            Site,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            #[serde(rename = "subtitle")]
            Subtitle,
            #[serde(rename = "_subtitle")]
            SubtitlePrimitiveElement,
            #[serde(rename = "alias")]
            Alias,
            #[serde(rename = "_alias")]
            AliasPrimitiveElement,
            #[serde(rename = "author")]
            Author,
            #[serde(rename = "scope")]
            Scope,
            #[serde(rename = "topicCodeableConcept")]
            TopicCodeableConcept,
            #[serde(rename = "topicReference")]
            TopicReference,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "subType")]
            SubType,
            #[serde(rename = "contentDefinition")]
            ContentDefinition,
            #[serde(rename = "term")]
            Term,
            #[serde(rename = "supportingInfo")]
            SupportingInfo,
            #[serde(rename = "relevantHistory")]
            RelevantHistory,
            #[serde(rename = "signer")]
            Signer,
            #[serde(rename = "friendly")]
            Friendly,
            #[serde(rename = "legal")]
            Legal,
            #[serde(rename = "rule")]
            Rule,
            #[serde(rename = "legallyBindingAttachment")]
            LegallyBindingAttachment,
            #[serde(rename = "legallyBindingReference")]
            LegallyBindingReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Contract;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Contract")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Contract, V::Error>
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
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#version: Option<super::super::types::String> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#legal_state: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#instantiates_canonical: Option<Box<super::super::types::Reference>> =
                    None;
                let mut r#instantiates_uri: Option<super::super::types::Uri> = None;
                let mut r#content_derivative: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#issued: Option<super::super::types::DateTime> = None;
                let mut r#applies: Option<Box<super::super::types::Period>> = None;
                let mut r#expiration_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#authority: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#domain: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#site: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#subtitle: Option<super::super::types::String> = None;
                let mut r#alias: Option<Vec<super::super::types::String>> = None;
                let mut r#author: Option<Box<super::super::types::Reference>> = None;
                let mut r#scope: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#topic: Option<ContractTopic> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#content_definition: Option<ContractContentDefinition> = None;
                let mut r#term: Option<Vec<ContractTerm>> = None;
                let mut r#supporting_info: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#relevant_history: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#signer: Option<Vec<ContractSigner>> = None;
                let mut r#friendly: Option<Vec<ContractFriendly>> = None;
                let mut r#legal: Option<Vec<ContractLegal>> = None;
                let mut r#rule: Option<Vec<ContractRule>> = None;
                let mut r#legally_binding: Option<ContractLegallyBinding> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Contract" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Contract",
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
                                            "url",
                                            "version",
                                            "status",
                                            "legalState",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "contentDerivative",
                                            "issued",
                                            "applies",
                                            "expirationType",
                                            "subject",
                                            "authority",
                                            "domain",
                                            "site",
                                            "name",
                                            "title",
                                            "subtitle",
                                            "alias",
                                            "author",
                                            "scope",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "contentDefinition",
                                            "term",
                                            "supportingInfo",
                                            "relevantHistory",
                                            "signer",
                                            "friendly",
                                            "legal",
                                            "rule",
                                            "legallyBindingAttachment",
                                            "legallyBindingReference",
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
                                            "url",
                                            "version",
                                            "status",
                                            "legalState",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "contentDerivative",
                                            "issued",
                                            "applies",
                                            "expirationType",
                                            "subject",
                                            "authority",
                                            "domain",
                                            "site",
                                            "name",
                                            "title",
                                            "subtitle",
                                            "alias",
                                            "author",
                                            "scope",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "contentDefinition",
                                            "term",
                                            "supportingInfo",
                                            "relevantHistory",
                                            "signer",
                                            "friendly",
                                            "legal",
                                            "rule",
                                            "legallyBindingAttachment",
                                            "legallyBindingReference",
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
                            Field::Url => {
                                if _ctx.from_json {
                                    let some = r#url.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#url.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    r#url = Some(map_access.next_value()?);
                                }
                            }
                            Field::UrlPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#url.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_url"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "url",
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
                                            "url",
                                            "version",
                                            "status",
                                            "legalState",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "contentDerivative",
                                            "issued",
                                            "applies",
                                            "expirationType",
                                            "subject",
                                            "authority",
                                            "domain",
                                            "site",
                                            "name",
                                            "title",
                                            "subtitle",
                                            "alias",
                                            "author",
                                            "scope",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "contentDefinition",
                                            "term",
                                            "supportingInfo",
                                            "relevantHistory",
                                            "signer",
                                            "friendly",
                                            "legal",
                                            "rule",
                                            "legallyBindingAttachment",
                                            "legallyBindingReference",
                                        ],
                                    ));
                                }
                            }
                            Field::Version => {
                                if _ctx.from_json {
                                    let some = r#version.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#version.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    r#version = Some(map_access.next_value()?);
                                }
                            }
                            Field::VersionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#version.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_version"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "version",
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
                                            "url",
                                            "version",
                                            "status",
                                            "legalState",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "contentDerivative",
                                            "issued",
                                            "applies",
                                            "expirationType",
                                            "subject",
                                            "authority",
                                            "domain",
                                            "site",
                                            "name",
                                            "title",
                                            "subtitle",
                                            "alias",
                                            "author",
                                            "scope",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "contentDefinition",
                                            "term",
                                            "supportingInfo",
                                            "relevantHistory",
                                            "signer",
                                            "friendly",
                                            "legal",
                                            "rule",
                                            "legallyBindingAttachment",
                                            "legallyBindingReference",
                                        ],
                                    ));
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
                                            "url",
                                            "version",
                                            "status",
                                            "legalState",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "contentDerivative",
                                            "issued",
                                            "applies",
                                            "expirationType",
                                            "subject",
                                            "authority",
                                            "domain",
                                            "site",
                                            "name",
                                            "title",
                                            "subtitle",
                                            "alias",
                                            "author",
                                            "scope",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "contentDefinition",
                                            "term",
                                            "supportingInfo",
                                            "relevantHistory",
                                            "signer",
                                            "friendly",
                                            "legal",
                                            "rule",
                                            "legallyBindingAttachment",
                                            "legallyBindingReference",
                                        ],
                                    ));
                                }
                            }
                            Field::LegalState => {
                                if r#legal_state.is_some() {
                                    return Err(serde::de::Error::duplicate_field("legalState"));
                                }
                                r#legal_state = Some(map_access.next_value()?);
                            }
                            Field::InstantiatesCanonical => {
                                if r#instantiates_canonical.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesCanonical",
                                    ));
                                }
                                r#instantiates_canonical = Some(map_access.next_value()?);
                            }
                            Field::InstantiatesUri => {
                                if _ctx.from_json {
                                    let some = r#instantiates_uri.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "instantiatesUri",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#instantiates_uri.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "instantiatesUri",
                                        ));
                                    }
                                    r#instantiates_uri = Some(map_access.next_value()?);
                                }
                            }
                            Field::InstantiatesUriPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#instantiates_uri.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_instantiatesUri",
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
                                        "instantiatesUri",
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
                                            "url",
                                            "version",
                                            "status",
                                            "legalState",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "contentDerivative",
                                            "issued",
                                            "applies",
                                            "expirationType",
                                            "subject",
                                            "authority",
                                            "domain",
                                            "site",
                                            "name",
                                            "title",
                                            "subtitle",
                                            "alias",
                                            "author",
                                            "scope",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "contentDefinition",
                                            "term",
                                            "supportingInfo",
                                            "relevantHistory",
                                            "signer",
                                            "friendly",
                                            "legal",
                                            "rule",
                                            "legallyBindingAttachment",
                                            "legallyBindingReference",
                                        ],
                                    ));
                                }
                            }
                            Field::ContentDerivative => {
                                if r#content_derivative.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentDerivative",
                                    ));
                                }
                                r#content_derivative = Some(map_access.next_value()?);
                            }
                            Field::Issued => {
                                if _ctx.from_json {
                                    let some = r#issued.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("issued"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#issued.is_some() {
                                        return Err(serde::de::Error::duplicate_field("issued"));
                                    }
                                    r#issued = Some(map_access.next_value()?);
                                }
                            }
                            Field::IssuedPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "issued",
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
                                            "url",
                                            "version",
                                            "status",
                                            "legalState",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "contentDerivative",
                                            "issued",
                                            "applies",
                                            "expirationType",
                                            "subject",
                                            "authority",
                                            "domain",
                                            "site",
                                            "name",
                                            "title",
                                            "subtitle",
                                            "alias",
                                            "author",
                                            "scope",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "contentDefinition",
                                            "term",
                                            "supportingInfo",
                                            "relevantHistory",
                                            "signer",
                                            "friendly",
                                            "legal",
                                            "rule",
                                            "legallyBindingAttachment",
                                            "legallyBindingReference",
                                        ],
                                    ));
                                }
                            }
                            Field::Applies => {
                                if r#applies.is_some() {
                                    return Err(serde::de::Error::duplicate_field("applies"));
                                }
                                r#applies = Some(map_access.next_value()?);
                            }
                            Field::ExpirationType => {
                                if r#expiration_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "expirationType",
                                    ));
                                }
                                r#expiration_type = Some(map_access.next_value()?);
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Authority => {
                                if r#authority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authority"));
                                }
                                r#authority = Some(map_access.next_value()?);
                            }
                            Field::Domain => {
                                if r#domain.is_some() {
                                    return Err(serde::de::Error::duplicate_field("domain"));
                                }
                                r#domain = Some(map_access.next_value()?);
                            }
                            Field::Site => {
                                if r#site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("site"));
                                }
                                r#site = Some(map_access.next_value()?);
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
                                            "url",
                                            "version",
                                            "status",
                                            "legalState",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "contentDerivative",
                                            "issued",
                                            "applies",
                                            "expirationType",
                                            "subject",
                                            "authority",
                                            "domain",
                                            "site",
                                            "name",
                                            "title",
                                            "subtitle",
                                            "alias",
                                            "author",
                                            "scope",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "contentDefinition",
                                            "term",
                                            "supportingInfo",
                                            "relevantHistory",
                                            "signer",
                                            "friendly",
                                            "legal",
                                            "rule",
                                            "legallyBindingAttachment",
                                            "legallyBindingReference",
                                        ],
                                    ));
                                }
                            }
                            Field::Title => {
                                if _ctx.from_json {
                                    let some = r#title.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("title"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#title.is_some() {
                                        return Err(serde::de::Error::duplicate_field("title"));
                                    }
                                    r#title = Some(map_access.next_value()?);
                                }
                            }
                            Field::TitlePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#title.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_title"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "title",
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
                                            "url",
                                            "version",
                                            "status",
                                            "legalState",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "contentDerivative",
                                            "issued",
                                            "applies",
                                            "expirationType",
                                            "subject",
                                            "authority",
                                            "domain",
                                            "site",
                                            "name",
                                            "title",
                                            "subtitle",
                                            "alias",
                                            "author",
                                            "scope",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "contentDefinition",
                                            "term",
                                            "supportingInfo",
                                            "relevantHistory",
                                            "signer",
                                            "friendly",
                                            "legal",
                                            "rule",
                                            "legallyBindingAttachment",
                                            "legallyBindingReference",
                                        ],
                                    ));
                                }
                            }
                            Field::Subtitle => {
                                if _ctx.from_json {
                                    let some = r#subtitle.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("subtitle"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#subtitle.is_some() {
                                        return Err(serde::de::Error::duplicate_field("subtitle"));
                                    }
                                    r#subtitle = Some(map_access.next_value()?);
                                }
                            }
                            Field::SubtitlePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#subtitle.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_subtitle"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "subtitle",
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
                                            "url",
                                            "version",
                                            "status",
                                            "legalState",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "contentDerivative",
                                            "issued",
                                            "applies",
                                            "expirationType",
                                            "subject",
                                            "authority",
                                            "domain",
                                            "site",
                                            "name",
                                            "title",
                                            "subtitle",
                                            "alias",
                                            "author",
                                            "scope",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "contentDefinition",
                                            "term",
                                            "supportingInfo",
                                            "relevantHistory",
                                            "signer",
                                            "friendly",
                                            "legal",
                                            "rule",
                                            "legallyBindingAttachment",
                                            "legallyBindingReference",
                                        ],
                                    ));
                                }
                            }
                            Field::Alias => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#alias.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("alias"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#alias.is_some() {
                                        return Err(serde::de::Error::duplicate_field("alias"));
                                    }
                                    r#alias = Some(map_access.next_value()?);
                                }
                            }
                            Field::AliasPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#alias.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_alias"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "alias",
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
                                            "url",
                                            "version",
                                            "status",
                                            "legalState",
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "contentDerivative",
                                            "issued",
                                            "applies",
                                            "expirationType",
                                            "subject",
                                            "authority",
                                            "domain",
                                            "site",
                                            "name",
                                            "title",
                                            "subtitle",
                                            "alias",
                                            "author",
                                            "scope",
                                            "topicCodeableConcept",
                                            "topicReference",
                                            "type",
                                            "subType",
                                            "contentDefinition",
                                            "term",
                                            "supportingInfo",
                                            "relevantHistory",
                                            "signer",
                                            "friendly",
                                            "legal",
                                            "rule",
                                            "legallyBindingAttachment",
                                            "legallyBindingReference",
                                        ],
                                    ));
                                }
                            }
                            Field::Author => {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field("author"));
                                }
                                r#author = Some(map_access.next_value()?);
                            }
                            Field::Scope => {
                                if r#scope.is_some() {
                                    return Err(serde::de::Error::duplicate_field("scope"));
                                }
                                r#scope = Some(map_access.next_value()?);
                            }
                            Field::TopicCodeableConcept => {
                                if r#topic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "topicCodeableConcept",
                                    ));
                                }
                                r#topic =
                                    Some(ContractTopic::CodeableConcept(map_access.next_value()?));
                            }
                            Field::TopicReference => {
                                if r#topic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "topicReference",
                                    ));
                                }
                                r#topic = Some(ContractTopic::Reference(map_access.next_value()?));
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::SubType => {
                                if r#sub_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subType"));
                                }
                                r#sub_type = Some(map_access.next_value()?);
                            }
                            Field::ContentDefinition => {
                                if r#content_definition.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentDefinition",
                                    ));
                                }
                                r#content_definition = Some(map_access.next_value()?);
                            }
                            Field::Term => {
                                if r#term.is_some() {
                                    return Err(serde::de::Error::duplicate_field("term"));
                                }
                                r#term = Some(map_access.next_value()?);
                            }
                            Field::SupportingInfo => {
                                if r#supporting_info.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "supportingInfo",
                                    ));
                                }
                                r#supporting_info = Some(map_access.next_value()?);
                            }
                            Field::RelevantHistory => {
                                if r#relevant_history.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relevantHistory",
                                    ));
                                }
                                r#relevant_history = Some(map_access.next_value()?);
                            }
                            Field::Signer => {
                                if r#signer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("signer"));
                                }
                                r#signer = Some(map_access.next_value()?);
                            }
                            Field::Friendly => {
                                if r#friendly.is_some() {
                                    return Err(serde::de::Error::duplicate_field("friendly"));
                                }
                                r#friendly = Some(map_access.next_value()?);
                            }
                            Field::Legal => {
                                if r#legal.is_some() {
                                    return Err(serde::de::Error::duplicate_field("legal"));
                                }
                                r#legal = Some(map_access.next_value()?);
                            }
                            Field::Rule => {
                                if r#rule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rule"));
                                }
                                r#rule = Some(map_access.next_value()?);
                            }
                            Field::LegallyBindingAttachment => {
                                if r#legally_binding.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "legallyBindingAttachment",
                                    ));
                                }
                                r#legally_binding = Some(ContractLegallyBinding::Attachment(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::LegallyBindingReference => {
                                if r#legally_binding.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "legallyBindingReference",
                                    ));
                                }
                                r#legally_binding = Some(ContractLegallyBinding::Reference(
                                    map_access.next_value()?,
                                ));
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
                                        "url",
                                        "version",
                                        "status",
                                        "legalState",
                                        "instantiatesCanonical",
                                        "instantiatesUri",
                                        "contentDerivative",
                                        "issued",
                                        "applies",
                                        "expirationType",
                                        "subject",
                                        "authority",
                                        "domain",
                                        "site",
                                        "name",
                                        "title",
                                        "subtitle",
                                        "alias",
                                        "author",
                                        "scope",
                                        "topicCodeableConcept",
                                        "topicReference",
                                        "type",
                                        "subType",
                                        "contentDefinition",
                                        "term",
                                        "supportingInfo",
                                        "relevantHistory",
                                        "signer",
                                        "friendly",
                                        "legal",
                                        "rule",
                                        "legallyBindingAttachment",
                                        "legallyBindingReference",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Contract {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#url,
                        r#version,
                        r#status,
                        r#legal_state,
                        r#instantiates_canonical,
                        r#instantiates_uri,
                        r#content_derivative,
                        r#issued,
                        r#applies,
                        r#expiration_type,
                        r#subject: r#subject.unwrap_or(vec![]),
                        r#authority: r#authority.unwrap_or(vec![]),
                        r#domain: r#domain.unwrap_or(vec![]),
                        r#site: r#site.unwrap_or(vec![]),
                        r#name,
                        r#title,
                        r#subtitle,
                        r#alias: r#alias.unwrap_or(vec![]),
                        r#author,
                        r#scope,
                        r#topic,
                        r#type,
                        r#sub_type: r#sub_type.unwrap_or(vec![]),
                        r#content_definition,
                        r#term: r#term.unwrap_or(vec![]),
                        r#supporting_info: r#supporting_info.unwrap_or(vec![]),
                        r#relevant_history: r#relevant_history.unwrap_or(vec![]),
                        r#signer: r#signer.unwrap_or(vec![]),
                        r#friendly: r#friendly.unwrap_or(vec![]),
                        r#legal: r#legal.unwrap_or(vec![]),
                        r#rule: r#rule.unwrap_or(vec![]),
                        r#legally_binding,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
