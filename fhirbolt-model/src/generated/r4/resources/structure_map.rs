// Generated on 2022-12-16 by fhirbolt-codegen v0.1.0
#[doc = "A value to use if there is no existing value in the source object."]
#[derive(Debug, Clone)]
pub enum StructureMapGroupRuleSourceDefaultValue {
    Base64Binary(Box<super::super::types::Base64Binary>),
    Boolean(Box<super::super::types::Boolean>),
    Canonical(Box<super::super::types::Canonical>),
    Code(Box<super::super::types::Code>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Decimal(Box<super::super::types::Decimal>),
    Id(Box<super::super::types::Id>),
    Instant(Box<super::super::types::Instant>),
    Integer(Box<super::super::types::Integer>),
    Markdown(Box<super::super::types::Markdown>),
    Oid(Box<super::super::types::Oid>),
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    Time(Box<super::super::types::Time>),
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    Uri(Box<super::super::types::Uri>),
    Url(Box<super::super::types::Url>),
    Uuid(Box<super::super::types::Uuid>),
    Address(Box<super::super::types::Address>),
    Age(Box<super::super::types::Age>),
    Annotation(Box<super::super::types::Annotation>),
    Attachment(Box<super::super::types::Attachment>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Coding(Box<super::super::types::Coding>),
    ContactPoint(Box<super::super::types::ContactPoint>),
    Count(Box<super::super::types::Count>),
    Distance(Box<super::super::types::Distance>),
    Duration(Box<super::super::types::Duration>),
    HumanName(Box<super::super::types::HumanName>),
    Identifier(Box<super::super::types::Identifier>),
    Money(Box<super::super::types::Money>),
    Period(Box<super::super::types::Period>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    Reference(Box<super::super::types::Reference>),
    SampledData(Box<super::super::types::SampledData>),
    Signature(Box<super::super::types::Signature>),
    Timing(Box<super::super::types::Timing>),
    ContactDetail(Box<super::super::types::ContactDetail>),
    Contributor(Box<super::super::types::Contributor>),
    DataRequirement(Box<super::super::types::DataRequirement>),
    Expression(Box<super::super::types::Expression>),
    ParameterDefinition(Box<super::super::types::ParameterDefinition>),
    RelatedArtifact(Box<super::super::types::RelatedArtifact>),
    TriggerDefinition(Box<super::super::types::TriggerDefinition>),
    UsageContext(Box<super::super::types::UsageContext>),
    Dosage(Box<super::super::types::Dosage>),
    Meta(Box<super::super::types::Meta>),
    Invalid,
}
impl Default for StructureMapGroupRuleSourceDefaultValue {
    fn default() -> StructureMapGroupRuleSourceDefaultValue {
        StructureMapGroupRuleSourceDefaultValue::Invalid
    }
}
#[doc = "Parameter value - variable or literal."]
#[derive(Debug, Clone)]
pub enum StructureMapGroupRuleTargetParameterValue {
    Id(Box<super::super::types::Id>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Decimal(Box<super::super::types::Decimal>),
    Invalid,
}
impl Default for StructureMapGroupRuleTargetParameterValue {
    fn default() -> StructureMapGroupRuleTargetParameterValue {
        StructureMapGroupRuleTargetParameterValue::Invalid
    }
}
#[doc = "A structure definition used by this map. The structure definition may describe instances that are converted, or the instances that are produced."]
#[derive(Default, Debug, Clone)]
pub struct StructureMapStructure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The canonical reference to the structure."]
    pub r#url: super::super::types::Canonical,
    #[doc = "How the referenced structure is used in this mapping."]
    pub r#mode: super::super::types::Code,
    #[doc = "The name used for this type in the map."]
    pub r#alias: Option<super::super::types::String>,
    #[doc = "Documentation that describes how the structure is used in the mapping."]
    pub r#documentation: Option<super::super::types::String>,
}
impl serde::ser::Serialize for StructureMapStructure {
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
                if let Some(some) = self.r#url.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("url", &some)?;
                }
                if self.r#url.id.is_some() || !self.r#url.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#url.id.as_ref(),
                        extension: &self.r#url.extension,
                    };
                    state.serialize_entry("_url", &primitive_element)?;
                }
            } else {
                state.serialize_entry("url", &self.r#url)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#mode.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("mode", &some)?;
                }
                if self.r#mode.id.is_some() || !self.r#mode.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#mode.id.as_ref(),
                        extension: &self.r#mode.extension,
                    };
                    state.serialize_entry("_mode", &primitive_element)?;
                }
            } else {
                state.serialize_entry("mode", &self.r#mode)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#alias.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("alias", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_alias", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#alias.as_ref() {
                    state.serialize_entry("alias", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for StructureMapStructure {
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
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            #[serde(rename = "mode")]
            Mode,
            #[serde(rename = "_mode")]
            ModePrimitiveElement,
            #[serde(rename = "alias")]
            Alias,
            #[serde(rename = "_alias")]
            AliasPrimitiveElement,
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "_documentation")]
            DocumentationPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StructureMapStructure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapStructure")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<StructureMapStructure, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#url: Option<super::super::types::Canonical> = None;
                let mut r#mode: Option<super::super::types::Code> = None;
                let mut r#alias: Option<super::super::types::String> = None;
                let mut r#documentation: Option<super::super::types::String> = None;
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
                            Field::Url => {
                                let some = r#url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::UrlPrimitiveElement => {
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
                            }
                            Field::Mode => {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mode"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ModePrimitiveElement => {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_mode"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Alias => {
                                let some = r#alias.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("alias"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::AliasPrimitiveElement => {
                                let some = r#alias.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_alias"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Documentation => {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DocumentationPrimitiveElement => {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_documentation",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
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
                                        "url",
                                        "mode",
                                        "alias",
                                        "documentation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(StructureMapStructure {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#url: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#url.unwrap_or(Default::default())
                        } else {
                            r#url.ok_or(serde::de::Error::missing_field("url"))?
                        },
                        r#mode: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#mode.unwrap_or(Default::default())
                        } else {
                            r#mode.ok_or(serde::de::Error::missing_field("mode"))?
                        },
                        r#alias,
                        r#documentation,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A name assigned to an instance of data. The instance must be provided when the mapping is invoked."]
#[derive(Default, Debug, Clone)]
pub struct StructureMapGroupInput {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Name for this instance of data."]
    pub r#name: super::super::types::Id,
    #[doc = "Type for this instance of data."]
    pub r#type: Option<super::super::types::String>,
    #[doc = "Mode for this instance of data."]
    pub r#mode: super::super::types::Code,
    #[doc = "Documentation for this instance of data."]
    pub r#documentation: Option<super::super::types::String>,
}
impl serde::ser::Serialize for StructureMapGroupInput {
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
                if let Some(some) = self.r#name.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#name.id.as_ref(),
                        extension: &self.r#name.extension,
                    };
                    state.serialize_entry("_name", &primitive_element)?;
                }
            } else {
                state.serialize_entry("name", &self.r#name)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("type", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_type", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#type.as_ref() {
                    state.serialize_entry("type", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#mode.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("mode", &some)?;
                }
                if self.r#mode.id.is_some() || !self.r#mode.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#mode.id.as_ref(),
                        extension: &self.r#mode.extension,
                    };
                    state.serialize_entry("_mode", &primitive_element)?;
                }
            } else {
                state.serialize_entry("mode", &self.r#mode)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for StructureMapGroupInput {
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
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "mode")]
            Mode,
            #[serde(rename = "_mode")]
            ModePrimitiveElement,
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "_documentation")]
            DocumentationPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StructureMapGroupInput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupInput")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<StructureMapGroupInput, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::Id> = None;
                let mut r#type: Option<super::super::types::String> = None;
                let mut r#mode: Option<super::super::types::Code> = None;
                let mut r#documentation: Option<super::super::types::String> = None;
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
                            Field::Name => {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::NamePrimitiveElement => {
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
                            }
                            Field::Type => {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TypePrimitiveElement => {
                                let some = r#type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_type"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Mode => {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mode"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ModePrimitiveElement => {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_mode"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Documentation => {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DocumentationPrimitiveElement => {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_documentation",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
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
                                        "name",
                                        "type",
                                        "mode",
                                        "documentation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(StructureMapGroupInput {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#type,
                        r#mode: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#mode.unwrap_or(Default::default())
                        } else {
                            r#mode.ok_or(serde::de::Error::missing_field("mode"))?
                        },
                        r#documentation,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Source inputs to the mapping."]
#[derive(Default, Debug, Clone)]
pub struct StructureMapGroupRuleSource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type or variable this rule applies to."]
    pub r#context: super::super::types::Id,
    #[doc = "Specified minimum cardinality for the element. This is optional; if present, it acts an implicit check on the input content."]
    pub r#min: Option<super::super::types::Integer>,
    #[doc = "Specified maximum cardinality for the element - a number or a \"*\". This is optional; if present, it acts an implicit check on the input content (* just serves as documentation; it's the default value)."]
    pub r#max: Option<super::super::types::String>,
    #[doc = "Specified type for the element. This works as a condition on the mapping - use for polymorphic elements."]
    pub r#type: Option<super::super::types::String>,
    #[doc = "A value to use if there is no existing value in the source object."]
    pub r#default_value: Option<StructureMapGroupRuleSourceDefaultValue>,
    #[doc = "Optional field for this source."]
    pub r#element: Option<super::super::types::String>,
    #[doc = "How to handle the list mode for this element."]
    pub r#list_mode: Option<super::super::types::Code>,
    #[doc = "Named context for field, if a field is specified."]
    pub r#variable: Option<super::super::types::Id>,
    #[doc = "FHIRPath expression  - must be true or the rule does not apply."]
    pub r#condition: Option<super::super::types::String>,
    #[doc = "FHIRPath expression  - must be true or the mapping engine throws an error instead of completing."]
    pub r#check: Option<super::super::types::String>,
    #[doc = "A FHIRPath expression which specifies a message to put in the transform log when content matching the source rule is found."]
    pub r#log_message: Option<super::super::types::String>,
}
impl serde::ser::Serialize for StructureMapGroupRuleSource {
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
                if let Some(some) = self.r#context.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("context", &some)?;
                }
                if self.r#context.id.is_some() || !self.r#context.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#context.id.as_ref(),
                        extension: &self.r#context.extension,
                    };
                    state.serialize_entry("_context", &primitive_element)?;
                }
            } else {
                state.serialize_entry("context", &self.r#context)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#min.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("min", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_min", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#min.as_ref() {
                    state.serialize_entry("min", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#max.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("max", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_max", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#max.as_ref() {
                    state.serialize_entry("max", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("type", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_type", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#type.as_ref() {
                    state.serialize_entry("type", some)?;
                }
            }
            if let Some(some) = self.r#default_value.as_ref() {
                match some {
                    StructureMapGroupRuleSourceDefaultValue::Base64Binary(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueBase64Binary", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry(
                                    "_defaultValueBase64Binary",
                                    &primitive_element,
                                )?;
                            }
                        } else {
                            state.serialize_entry("defaultValueBase64Binary", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_defaultValueBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueBoolean", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Canonical(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueCanonical", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry(
                                    "_defaultValueCanonical",
                                    &primitive_element,
                                )?;
                            }
                        } else {
                            state.serialize_entry("defaultValueCanonical", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Code(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueCode", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_defaultValueCode", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueCode", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_defaultValueDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueDate", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_defaultValueDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueDateTime", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Decimal(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = some.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })?;
                                state.serialize_entry("defaultValueDecimal", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_defaultValueDecimal", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueDecimal", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Id(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueId", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_defaultValueId", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueId", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Instant(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueInstant", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_defaultValueInstant", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueInstant", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Integer(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueInteger", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_defaultValueInteger", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueInteger", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Markdown(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueMarkdown", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_defaultValueMarkdown", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueMarkdown", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Oid(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueOid", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_defaultValueOid", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueOid", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::PositiveInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValuePositiveInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry(
                                    "_defaultValuePositiveInt",
                                    &primitive_element,
                                )?;
                            }
                        } else {
                            state.serialize_entry("defaultValuePositiveInt", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_defaultValueString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueString", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Time(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_defaultValueTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueTime", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::UnsignedInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueUnsignedInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry(
                                    "_defaultValueUnsignedInt",
                                    &primitive_element,
                                )?;
                            }
                        } else {
                            state.serialize_entry("defaultValueUnsignedInt", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Uri(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueUri", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_defaultValueUri", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueUri", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Url(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueUrl", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_defaultValueUrl", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueUrl", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Uuid(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("defaultValueUuid", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_defaultValueUuid", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("defaultValueUuid", value)?;
                        }
                    }
                    StructureMapGroupRuleSourceDefaultValue::Address(ref value) => {
                        state.serialize_entry("defaultValueAddress", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Age(ref value) => {
                        state.serialize_entry("defaultValueAge", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Annotation(ref value) => {
                        state.serialize_entry("defaultValueAnnotation", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Attachment(ref value) => {
                        state.serialize_entry("defaultValueAttachment", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::CodeableConcept(ref value) => {
                        state.serialize_entry("defaultValueCodeableConcept", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Coding(ref value) => {
                        state.serialize_entry("defaultValueCoding", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::ContactPoint(ref value) => {
                        state.serialize_entry("defaultValueContactPoint", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Count(ref value) => {
                        state.serialize_entry("defaultValueCount", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Distance(ref value) => {
                        state.serialize_entry("defaultValueDistance", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Duration(ref value) => {
                        state.serialize_entry("defaultValueDuration", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::HumanName(ref value) => {
                        state.serialize_entry("defaultValueHumanName", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Identifier(ref value) => {
                        state.serialize_entry("defaultValueIdentifier", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Money(ref value) => {
                        state.serialize_entry("defaultValueMoney", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Period(ref value) => {
                        state.serialize_entry("defaultValuePeriod", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Quantity(ref value) => {
                        state.serialize_entry("defaultValueQuantity", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Range(ref value) => {
                        state.serialize_entry("defaultValueRange", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Ratio(ref value) => {
                        state.serialize_entry("defaultValueRatio", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Reference(ref value) => {
                        state.serialize_entry("defaultValueReference", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::SampledData(ref value) => {
                        state.serialize_entry("defaultValueSampledData", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Signature(ref value) => {
                        state.serialize_entry("defaultValueSignature", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Timing(ref value) => {
                        state.serialize_entry("defaultValueTiming", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::ContactDetail(ref value) => {
                        state.serialize_entry("defaultValueContactDetail", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Contributor(ref value) => {
                        state.serialize_entry("defaultValueContributor", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::DataRequirement(ref value) => {
                        state.serialize_entry("defaultValueDataRequirement", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Expression(ref value) => {
                        state.serialize_entry("defaultValueExpression", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::ParameterDefinition(ref value) => {
                        state.serialize_entry("defaultValueParameterDefinition", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::RelatedArtifact(ref value) => {
                        state.serialize_entry("defaultValueRelatedArtifact", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::TriggerDefinition(ref value) => {
                        state.serialize_entry("defaultValueTriggerDefinition", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::UsageContext(ref value) => {
                        state.serialize_entry("defaultValueUsageContext", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Dosage(ref value) => {
                        state.serialize_entry("defaultValueDosage", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Meta(ref value) => {
                        state.serialize_entry("defaultValueMeta", value)?;
                    }
                    StructureMapGroupRuleSourceDefaultValue::Invalid => {
                        return Err(serde::ser::Error::custom("default_value is invalid"))
                    }
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#element.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("element", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_element", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#element.as_ref() {
                    state.serialize_entry("element", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#list_mode.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("listMode", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_listMode", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#list_mode.as_ref() {
                    state.serialize_entry("listMode", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#variable.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("variable", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_variable", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#variable.as_ref() {
                    state.serialize_entry("variable", some)?;
                }
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
            if _ctx.output_json {
                if let Some(some) = self.r#check.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("check", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_check", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#check.as_ref() {
                    state.serialize_entry("check", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#log_message.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("logMessage", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_logMessage", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#log_message.as_ref() {
                    state.serialize_entry("logMessage", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for StructureMapGroupRuleSource {
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
            #[serde(rename = "context")]
            Context,
            #[serde(rename = "_context")]
            ContextPrimitiveElement,
            #[serde(rename = "min")]
            Min,
            #[serde(rename = "_min")]
            MinPrimitiveElement,
            #[serde(rename = "max")]
            Max,
            #[serde(rename = "_max")]
            MaxPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "defaultValueBase64Binary")]
            DefaultValueBase64Binary,
            #[serde(rename = "_defaultValueBase64Binary")]
            DefaultValueBase64BinaryPrimitiveElement,
            #[serde(rename = "defaultValueBoolean")]
            DefaultValueBoolean,
            #[serde(rename = "_defaultValueBoolean")]
            DefaultValueBooleanPrimitiveElement,
            #[serde(rename = "defaultValueCanonical")]
            DefaultValueCanonical,
            #[serde(rename = "_defaultValueCanonical")]
            DefaultValueCanonicalPrimitiveElement,
            #[serde(rename = "defaultValueCode")]
            DefaultValueCode,
            #[serde(rename = "_defaultValueCode")]
            DefaultValueCodePrimitiveElement,
            #[serde(rename = "defaultValueDate")]
            DefaultValueDate,
            #[serde(rename = "_defaultValueDate")]
            DefaultValueDatePrimitiveElement,
            #[serde(rename = "defaultValueDateTime")]
            DefaultValueDateTime,
            #[serde(rename = "_defaultValueDateTime")]
            DefaultValueDateTimePrimitiveElement,
            #[serde(rename = "defaultValueDecimal")]
            DefaultValueDecimal,
            #[serde(rename = "_defaultValueDecimal")]
            DefaultValueDecimalPrimitiveElement,
            #[serde(rename = "defaultValueId")]
            DefaultValueId,
            #[serde(rename = "_defaultValueId")]
            DefaultValueIdPrimitiveElement,
            #[serde(rename = "defaultValueInstant")]
            DefaultValueInstant,
            #[serde(rename = "_defaultValueInstant")]
            DefaultValueInstantPrimitiveElement,
            #[serde(rename = "defaultValueInteger")]
            DefaultValueInteger,
            #[serde(rename = "_defaultValueInteger")]
            DefaultValueIntegerPrimitiveElement,
            #[serde(rename = "defaultValueMarkdown")]
            DefaultValueMarkdown,
            #[serde(rename = "_defaultValueMarkdown")]
            DefaultValueMarkdownPrimitiveElement,
            #[serde(rename = "defaultValueOid")]
            DefaultValueOid,
            #[serde(rename = "_defaultValueOid")]
            DefaultValueOidPrimitiveElement,
            #[serde(rename = "defaultValuePositiveInt")]
            DefaultValuePositiveInt,
            #[serde(rename = "_defaultValuePositiveInt")]
            DefaultValuePositiveIntPrimitiveElement,
            #[serde(rename = "defaultValueString")]
            DefaultValueString,
            #[serde(rename = "_defaultValueString")]
            DefaultValueStringPrimitiveElement,
            #[serde(rename = "defaultValueTime")]
            DefaultValueTime,
            #[serde(rename = "_defaultValueTime")]
            DefaultValueTimePrimitiveElement,
            #[serde(rename = "defaultValueUnsignedInt")]
            DefaultValueUnsignedInt,
            #[serde(rename = "_defaultValueUnsignedInt")]
            DefaultValueUnsignedIntPrimitiveElement,
            #[serde(rename = "defaultValueUri")]
            DefaultValueUri,
            #[serde(rename = "_defaultValueUri")]
            DefaultValueUriPrimitiveElement,
            #[serde(rename = "defaultValueUrl")]
            DefaultValueUrl,
            #[serde(rename = "_defaultValueUrl")]
            DefaultValueUrlPrimitiveElement,
            #[serde(rename = "defaultValueUuid")]
            DefaultValueUuid,
            #[serde(rename = "_defaultValueUuid")]
            DefaultValueUuidPrimitiveElement,
            #[serde(rename = "defaultValueAddress")]
            DefaultValueAddress,
            #[serde(rename = "defaultValueAge")]
            DefaultValueAge,
            #[serde(rename = "defaultValueAnnotation")]
            DefaultValueAnnotation,
            #[serde(rename = "defaultValueAttachment")]
            DefaultValueAttachment,
            #[serde(rename = "defaultValueCodeableConcept")]
            DefaultValueCodeableConcept,
            #[serde(rename = "defaultValueCoding")]
            DefaultValueCoding,
            #[serde(rename = "defaultValueContactPoint")]
            DefaultValueContactPoint,
            #[serde(rename = "defaultValueCount")]
            DefaultValueCount,
            #[serde(rename = "defaultValueDistance")]
            DefaultValueDistance,
            #[serde(rename = "defaultValueDuration")]
            DefaultValueDuration,
            #[serde(rename = "defaultValueHumanName")]
            DefaultValueHumanName,
            #[serde(rename = "defaultValueIdentifier")]
            DefaultValueIdentifier,
            #[serde(rename = "defaultValueMoney")]
            DefaultValueMoney,
            #[serde(rename = "defaultValuePeriod")]
            DefaultValuePeriod,
            #[serde(rename = "defaultValueQuantity")]
            DefaultValueQuantity,
            #[serde(rename = "defaultValueRange")]
            DefaultValueRange,
            #[serde(rename = "defaultValueRatio")]
            DefaultValueRatio,
            #[serde(rename = "defaultValueReference")]
            DefaultValueReference,
            #[serde(rename = "defaultValueSampledData")]
            DefaultValueSampledData,
            #[serde(rename = "defaultValueSignature")]
            DefaultValueSignature,
            #[serde(rename = "defaultValueTiming")]
            DefaultValueTiming,
            #[serde(rename = "defaultValueContactDetail")]
            DefaultValueContactDetail,
            #[serde(rename = "defaultValueContributor")]
            DefaultValueContributor,
            #[serde(rename = "defaultValueDataRequirement")]
            DefaultValueDataRequirement,
            #[serde(rename = "defaultValueExpression")]
            DefaultValueExpression,
            #[serde(rename = "defaultValueParameterDefinition")]
            DefaultValueParameterDefinition,
            #[serde(rename = "defaultValueRelatedArtifact")]
            DefaultValueRelatedArtifact,
            #[serde(rename = "defaultValueTriggerDefinition")]
            DefaultValueTriggerDefinition,
            #[serde(rename = "defaultValueUsageContext")]
            DefaultValueUsageContext,
            #[serde(rename = "defaultValueDosage")]
            DefaultValueDosage,
            #[serde(rename = "defaultValueMeta")]
            DefaultValueMeta,
            #[serde(rename = "element")]
            Element,
            #[serde(rename = "_element")]
            ElementPrimitiveElement,
            #[serde(rename = "listMode")]
            ListMode,
            #[serde(rename = "_listMode")]
            ListModePrimitiveElement,
            #[serde(rename = "variable")]
            Variable,
            #[serde(rename = "_variable")]
            VariablePrimitiveElement,
            #[serde(rename = "condition")]
            Condition,
            #[serde(rename = "_condition")]
            ConditionPrimitiveElement,
            #[serde(rename = "check")]
            Check,
            #[serde(rename = "_check")]
            CheckPrimitiveElement,
            #[serde(rename = "logMessage")]
            LogMessage,
            #[serde(rename = "_logMessage")]
            LogMessagePrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StructureMapGroupRuleSource;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRuleSource")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<StructureMapGroupRuleSource, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#context: Option<super::super::types::Id> = None;
                let mut r#min: Option<super::super::types::Integer> = None;
                let mut r#max: Option<super::super::types::String> = None;
                let mut r#type: Option<super::super::types::String> = None;
                let mut r#default_value: Option<StructureMapGroupRuleSourceDefaultValue> = None;
                let mut r#element: Option<super::super::types::String> = None;
                let mut r#list_mode: Option<super::super::types::Code> = None;
                let mut r#variable: Option<super::super::types::Id> = None;
                let mut r#condition: Option<super::super::types::String> = None;
                let mut r#check: Option<super::super::types::String> = None;
                let mut r#log_message: Option<super::super::types::String> = None;
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
                            Field::Context => {
                                let some = r#context.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("context"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ContextPrimitiveElement => {
                                let some = r#context.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_context"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Min => {
                                let some = r#min.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("min"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::MinPrimitiveElement => {
                                let some = r#min.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_min"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Max => {
                                let some = r#max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("max"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::MaxPrimitiveElement => {
                                let some = r#max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_max"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Type => {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TypePrimitiveElement => {
                                let some = r#type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_type"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::DefaultValueBase64Binary => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Base64Binary(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Base64Binary(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueBase64Binary",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueBase64BinaryPrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Base64Binary(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Base64Binary(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueBase64Binary",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueBoolean => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Boolean(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueBooleanPrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Boolean(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueBoolean",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueCanonical => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Canonical(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Canonical(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueCanonical",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueCanonicalPrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Canonical(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Canonical(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueCanonical",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueCode => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Code(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Code(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueCode",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueCodePrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Code(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Code(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueCode",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueDate => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Date(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueDatePrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Date(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueDate",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueDateTime => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::DateTime(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueDateTimePrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::DateTime(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueDateTime",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueDecimal => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Decimal(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueDecimal",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    variant.value = Some(format!("{}", value));
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueDecimalPrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Decimal(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueDecimal",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueId => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Id(Default::default()),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Id(variant) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueId",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueIdPrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Id(Default::default()),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Id(variant) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueId",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueInstant => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Instant(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueInstant",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueInstantPrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Instant(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueInstant",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueInteger => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Integer(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueInteger",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueIntegerPrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Integer(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueInteger",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueMarkdown => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Markdown(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueMarkdown",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueMarkdownPrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Markdown(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueMarkdown",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueOid => {
                                let r#enum =
                                    r#default_value.get_or_insert(
                                        StructureMapGroupRuleSourceDefaultValue::Oid(
                                            Default::default(),
                                        ),
                                    );
                                if let StructureMapGroupRuleSourceDefaultValue::Oid(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueOid",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueOidPrimitiveElement => {
                                let r#enum =
                                    r#default_value.get_or_insert(
                                        StructureMapGroupRuleSourceDefaultValue::Oid(
                                            Default::default(),
                                        ),
                                    );
                                if let StructureMapGroupRuleSourceDefaultValue::Oid(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueOid",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValuePositiveInt => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::PositiveInt(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValuePositiveInt",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValuePositiveIntPrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::PositiveInt(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValuePositiveInt",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueString => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::String(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueStringPrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::String(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueString",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueTime => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Time(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueTimePrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Time(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueTime",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueUnsignedInt => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::UnsignedInt(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::UnsignedInt(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueUnsignedInt",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueUnsignedIntPrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::UnsignedInt(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::UnsignedInt(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueUnsignedInt",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueUri => {
                                let r#enum =
                                    r#default_value.get_or_insert(
                                        StructureMapGroupRuleSourceDefaultValue::Uri(
                                            Default::default(),
                                        ),
                                    );
                                if let StructureMapGroupRuleSourceDefaultValue::Uri(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueUri",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueUriPrimitiveElement => {
                                let r#enum =
                                    r#default_value.get_or_insert(
                                        StructureMapGroupRuleSourceDefaultValue::Uri(
                                            Default::default(),
                                        ),
                                    );
                                if let StructureMapGroupRuleSourceDefaultValue::Uri(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueUri",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueUrl => {
                                let r#enum =
                                    r#default_value.get_or_insert(
                                        StructureMapGroupRuleSourceDefaultValue::Url(
                                            Default::default(),
                                        ),
                                    );
                                if let StructureMapGroupRuleSourceDefaultValue::Url(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueUrl",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueUrlPrimitiveElement => {
                                let r#enum =
                                    r#default_value.get_or_insert(
                                        StructureMapGroupRuleSourceDefaultValue::Url(
                                            Default::default(),
                                        ),
                                    );
                                if let StructureMapGroupRuleSourceDefaultValue::Url(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueUrl",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueUuid => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Uuid(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueUuid",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueUuidPrimitiveElement => {
                                let r#enum = r#default_value.get_or_insert(
                                    StructureMapGroupRuleSourceDefaultValue::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleSourceDefaultValue::Uuid(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueUuid",
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
                                        "_defaultValue[x]",
                                    ));
                                }
                            }
                            Field::DefaultValueAddress => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueAddress",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Address(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueAge => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueAge",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Age(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueAnnotation => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueAnnotation",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Annotation(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueAttachment => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueAttachment",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Attachment(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueCodeableConcept => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCodeableConcept",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::CodeableConcept(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueCoding => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCoding",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Coding(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueContactPoint => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueContactPoint",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::ContactPoint(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueCount => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCount",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Count(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueDistance => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDistance",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Distance(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueDuration => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDuration",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Duration(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueHumanName => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueHumanName",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::HumanName(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueIdentifier => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueIdentifier",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Identifier(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueMoney => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueMoney",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Money(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValuePeriod => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValuePeriod",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Period(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueQuantity => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueQuantity",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Quantity(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueRange => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueRange",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Range(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueRatio => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueRatio",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Ratio(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueReference => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueReference",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Reference(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueSampledData => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueSampledData",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::SampledData(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueSignature => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueSignature",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Signature(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueTiming => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueTiming",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Timing(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueContactDetail => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueContactDetail",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::ContactDetail(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueContributor => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueContributor",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Contributor(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueDataRequirement => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDataRequirement",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::DataRequirement(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueExpression => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueExpression",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Expression(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueParameterDefinition => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueParameterDefinition",
                                    ));
                                }
                                r#default_value = Some(
                                    StructureMapGroupRuleSourceDefaultValue::ParameterDefinition(
                                        map_access.next_value()?,
                                    ),
                                );
                            }
                            Field::DefaultValueRelatedArtifact => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueRelatedArtifact",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::RelatedArtifact(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueTriggerDefinition => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueTriggerDefinition",
                                    ));
                                }
                                r#default_value = Some(
                                    StructureMapGroupRuleSourceDefaultValue::TriggerDefinition(
                                        map_access.next_value()?,
                                    ),
                                );
                            }
                            Field::DefaultValueUsageContext => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUsageContext",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::UsageContext(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueDosage => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDosage",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Dosage(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::DefaultValueMeta => {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueMeta",
                                    ));
                                }
                                r#default_value =
                                    Some(StructureMapGroupRuleSourceDefaultValue::Meta(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::Element => {
                                let some = r#element.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("element"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ElementPrimitiveElement => {
                                let some = r#element.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_element"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::ListMode => {
                                let some = r#list_mode.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("listMode"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ListModePrimitiveElement => {
                                let some = r#list_mode.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_listMode"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Variable => {
                                let some = r#variable.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("variable"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::VariablePrimitiveElement => {
                                let some = r#variable.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_variable"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Condition => {
                                let some = r#condition.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ConditionPrimitiveElement => {
                                let some = r#condition.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_condition"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Check => {
                                let some = r#check.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("check"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::CheckPrimitiveElement => {
                                let some = r#check.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_check"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::LogMessage => {
                                let some = r#log_message.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("logMessage"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::LogMessagePrimitiveElement => {
                                let some = r#log_message.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_logMessage"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
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
                                        "context",
                                        "min",
                                        "max",
                                        "type",
                                        "defaultValueBase64Binary",
                                        "defaultValueBoolean",
                                        "defaultValueCanonical",
                                        "defaultValueCode",
                                        "defaultValueDate",
                                        "defaultValueDateTime",
                                        "defaultValueDecimal",
                                        "defaultValueId",
                                        "defaultValueInstant",
                                        "defaultValueInteger",
                                        "defaultValueMarkdown",
                                        "defaultValueOid",
                                        "defaultValuePositiveInt",
                                        "defaultValueString",
                                        "defaultValueTime",
                                        "defaultValueUnsignedInt",
                                        "defaultValueUri",
                                        "defaultValueUrl",
                                        "defaultValueUuid",
                                        "defaultValueAddress",
                                        "defaultValueAge",
                                        "defaultValueAnnotation",
                                        "defaultValueAttachment",
                                        "defaultValueCodeableConcept",
                                        "defaultValueCoding",
                                        "defaultValueContactPoint",
                                        "defaultValueCount",
                                        "defaultValueDistance",
                                        "defaultValueDuration",
                                        "defaultValueHumanName",
                                        "defaultValueIdentifier",
                                        "defaultValueMoney",
                                        "defaultValuePeriod",
                                        "defaultValueQuantity",
                                        "defaultValueRange",
                                        "defaultValueRatio",
                                        "defaultValueReference",
                                        "defaultValueSampledData",
                                        "defaultValueSignature",
                                        "defaultValueTiming",
                                        "defaultValueContactDetail",
                                        "defaultValueContributor",
                                        "defaultValueDataRequirement",
                                        "defaultValueExpression",
                                        "defaultValueParameterDefinition",
                                        "defaultValueRelatedArtifact",
                                        "defaultValueTriggerDefinition",
                                        "defaultValueUsageContext",
                                        "defaultValueDosage",
                                        "defaultValueMeta",
                                        "element",
                                        "listMode",
                                        "variable",
                                        "condition",
                                        "check",
                                        "logMessage",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(StructureMapGroupRuleSource {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#context: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#context.unwrap_or(Default::default())
                        } else {
                            r#context.ok_or(serde::de::Error::missing_field("context"))?
                        },
                        r#min,
                        r#max,
                        r#type,
                        r#default_value,
                        r#element,
                        r#list_mode,
                        r#variable,
                        r#condition,
                        r#check,
                        r#log_message,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Parameters to the transform."]
#[derive(Default, Debug, Clone)]
pub struct StructureMapGroupRuleTargetParameter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Parameter value - variable or literal."]
    pub r#value: StructureMapGroupRuleTargetParameterValue,
}
impl serde::ser::Serialize for StructureMapGroupRuleTargetParameter {
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
                StructureMapGroupRuleTargetParameterValue::Id(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueId", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueId", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueId", value)?;
                    }
                }
                StructureMapGroupRuleTargetParameterValue::String(ref value) => {
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
                StructureMapGroupRuleTargetParameterValue::Boolean(ref value) => {
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
                StructureMapGroupRuleTargetParameterValue::Integer(ref value) => {
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
                StructureMapGroupRuleTargetParameterValue::Decimal(ref value) => {
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
                StructureMapGroupRuleTargetParameterValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for StructureMapGroupRuleTargetParameter {
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
            #[serde(rename = "valueId")]
            ValueId,
            #[serde(rename = "_valueId")]
            ValueIdPrimitiveElement,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueInteger")]
            ValueInteger,
            #[serde(rename = "_valueInteger")]
            ValueIntegerPrimitiveElement,
            #[serde(rename = "valueDecimal")]
            ValueDecimal,
            #[serde(rename = "_valueDecimal")]
            ValueDecimalPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StructureMapGroupRuleTargetParameter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRuleTargetParameter")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<StructureMapGroupRuleTargetParameter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#value: Option<StructureMapGroupRuleTargetParameterValue> = None;
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
                            Field::ValueId => {
                                let r#enum = r#value.get_or_insert(
                                    StructureMapGroupRuleTargetParameterValue::Id(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleTargetParameterValue::Id(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueIdPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    StructureMapGroupRuleTargetParameterValue::Id(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleTargetParameterValue::Id(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueId"));
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
                            }
                            Field::ValueString => {
                                let r#enum = r#value.get_or_insert(
                                    StructureMapGroupRuleTargetParameterValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleTargetParameterValue::String(variant) =
                                    r#enum
                                {
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
                            }
                            Field::ValueStringPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    StructureMapGroupRuleTargetParameterValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleTargetParameterValue::String(variant) =
                                    r#enum
                                {
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
                            }
                            Field::ValueBoolean => {
                                let r#enum = r#value.get_or_insert(
                                    StructureMapGroupRuleTargetParameterValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleTargetParameterValue::Boolean(variant) =
                                    r#enum
                                {
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
                            }
                            Field::ValueBooleanPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    StructureMapGroupRuleTargetParameterValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleTargetParameterValue::Boolean(variant) =
                                    r#enum
                                {
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
                            }
                            Field::ValueInteger => {
                                let r#enum = r#value.get_or_insert(
                                    StructureMapGroupRuleTargetParameterValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleTargetParameterValue::Integer(variant) =
                                    r#enum
                                {
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
                            }
                            Field::ValueIntegerPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    StructureMapGroupRuleTargetParameterValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleTargetParameterValue::Integer(variant) =
                                    r#enum
                                {
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
                            }
                            Field::ValueDecimal => {
                                let r#enum = r#value.get_or_insert(
                                    StructureMapGroupRuleTargetParameterValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleTargetParameterValue::Decimal(variant) =
                                    r#enum
                                {
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
                            }
                            Field::ValueDecimalPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    StructureMapGroupRuleTargetParameterValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let StructureMapGroupRuleTargetParameterValue::Decimal(variant) =
                                    r#enum
                                {
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
                                        "valueId",
                                        "valueString",
                                        "valueBoolean",
                                        "valueInteger",
                                        "valueDecimal",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(StructureMapGroupRuleTargetParameter {
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
#[doc = "Content to create because of this mapping rule."]
#[derive(Default, Debug, Clone)]
pub struct StructureMapGroupRuleTarget {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type or variable this rule applies to."]
    pub r#context: Option<super::super::types::Id>,
    #[doc = "How to interpret the context."]
    pub r#context_type: Option<super::super::types::Code>,
    #[doc = "Field to create in the context."]
    pub r#element: Option<super::super::types::String>,
    #[doc = "Named context for field, if desired, and a field is specified."]
    pub r#variable: Option<super::super::types::Id>,
    #[doc = "If field is a list, how to manage the list."]
    pub r#list_mode: Vec<super::super::types::Code>,
    #[doc = "Internal rule reference for shared list items."]
    pub r#list_rule_id: Option<super::super::types::Id>,
    #[doc = "How the data is copied / created."]
    pub r#transform: Option<super::super::types::Code>,
    #[doc = "Parameters to the transform."]
    pub r#parameter: Vec<StructureMapGroupRuleTargetParameter>,
}
impl serde::ser::Serialize for StructureMapGroupRuleTarget {
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
                if let Some(some) = self.r#context.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("context", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_context", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#context.as_ref() {
                    state.serialize_entry("context", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#context_type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("contextType", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_contextType", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#context_type.as_ref() {
                    state.serialize_entry("contextType", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#element.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("element", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_element", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#element.as_ref() {
                    state.serialize_entry("element", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#variable.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("variable", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_variable", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#variable.as_ref() {
                    state.serialize_entry("variable", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#list_mode.is_empty() {
                    let values = self
                        .r#list_mode
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("listMode", &values)?;
                    }
                    let requires_elements = self
                        .r#list_mode
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#list_mode
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
                        state.serialize_entry("_listMode", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#list_mode.is_empty() {
                    state.serialize_entry("listMode", &self.r#list_mode)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#list_rule_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("listRuleId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_listRuleId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#list_rule_id.as_ref() {
                    state.serialize_entry("listRuleId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#transform.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("transform", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_transform", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#transform.as_ref() {
                    state.serialize_entry("transform", some)?;
                }
            }
            if !self.r#parameter.is_empty() {
                state.serialize_entry("parameter", &self.r#parameter)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for StructureMapGroupRuleTarget {
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
            #[serde(rename = "context")]
            Context,
            #[serde(rename = "_context")]
            ContextPrimitiveElement,
            #[serde(rename = "contextType")]
            ContextType,
            #[serde(rename = "_contextType")]
            ContextTypePrimitiveElement,
            #[serde(rename = "element")]
            Element,
            #[serde(rename = "_element")]
            ElementPrimitiveElement,
            #[serde(rename = "variable")]
            Variable,
            #[serde(rename = "_variable")]
            VariablePrimitiveElement,
            #[serde(rename = "listMode")]
            ListMode,
            #[serde(rename = "_listMode")]
            ListModePrimitiveElement,
            #[serde(rename = "listRuleId")]
            ListRuleId,
            #[serde(rename = "_listRuleId")]
            ListRuleIdPrimitiveElement,
            #[serde(rename = "transform")]
            Transform,
            #[serde(rename = "_transform")]
            TransformPrimitiveElement,
            #[serde(rename = "parameter")]
            Parameter,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StructureMapGroupRuleTarget;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRuleTarget")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<StructureMapGroupRuleTarget, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#context: Option<super::super::types::Id> = None;
                let mut r#context_type: Option<super::super::types::Code> = None;
                let mut r#element: Option<super::super::types::String> = None;
                let mut r#variable: Option<super::super::types::Id> = None;
                let mut r#list_mode: Option<Vec<super::super::types::Code>> = None;
                let mut r#list_rule_id: Option<super::super::types::Id> = None;
                let mut r#transform: Option<super::super::types::Code> = None;
                let mut r#parameter: Option<Vec<StructureMapGroupRuleTargetParameter>> = None;
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
                            Field::Context => {
                                let some = r#context.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("context"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ContextPrimitiveElement => {
                                let some = r#context.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_context"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::ContextType => {
                                let some = r#context_type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contextType"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ContextTypePrimitiveElement => {
                                let some = r#context_type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_contextType"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Element => {
                                let some = r#element.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("element"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ElementPrimitiveElement => {
                                let some = r#element.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_element"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Variable => {
                                let some = r#variable.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("variable"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::VariablePrimitiveElement => {
                                let some = r#variable.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_variable"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::ListMode => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#list_mode.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("listMode"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::ListModePrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#list_mode.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_listMode"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::ListRuleId => {
                                let some = r#list_rule_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("listRuleId"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ListRuleIdPrimitiveElement => {
                                let some = r#list_rule_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_listRuleId"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Transform => {
                                let some = r#transform.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("transform"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TransformPrimitiveElement => {
                                let some = r#transform.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_transform"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Parameter => {
                                if r#parameter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("parameter"));
                                }
                                r#parameter = Some(map_access.next_value()?);
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
                                        "context",
                                        "contextType",
                                        "element",
                                        "variable",
                                        "listMode",
                                        "listRuleId",
                                        "transform",
                                        "parameter",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(StructureMapGroupRuleTarget {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#context,
                        r#context_type,
                        r#element,
                        r#variable,
                        r#list_mode: r#list_mode.unwrap_or(vec![]),
                        r#list_rule_id,
                        r#transform,
                        r#parameter: r#parameter.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Which other rules to apply in the context of this rule."]
#[derive(Default, Debug, Clone)]
pub struct StructureMapGroupRuleDependent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Name of a rule or group to apply."]
    pub r#name: super::super::types::Id,
    #[doc = "Variable to pass to the rule or group."]
    pub r#variable: Vec<super::super::types::String>,
}
impl serde::ser::Serialize for StructureMapGroupRuleDependent {
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
                if let Some(some) = self.r#name.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#name.id.as_ref(),
                        extension: &self.r#name.extension,
                    };
                    state.serialize_entry("_name", &primitive_element)?;
                }
            } else {
                state.serialize_entry("name", &self.r#name)?;
            }
            if _ctx.output_json {
                if !self.r#variable.is_empty() {
                    let values = self
                        .r#variable
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("variable", &values)?;
                    }
                    let requires_elements = self
                        .r#variable
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#variable
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
                        state.serialize_entry("_variable", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#variable.is_empty() {
                    state.serialize_entry("variable", &self.r#variable)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for StructureMapGroupRuleDependent {
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
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "variable")]
            Variable,
            #[serde(rename = "_variable")]
            VariablePrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StructureMapGroupRuleDependent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRuleDependent")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<StructureMapGroupRuleDependent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::Id> = None;
                let mut r#variable: Option<Vec<super::super::types::String>> = None;
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
                            Field::Name => {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::NamePrimitiveElement => {
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
                            }
                            Field::Variable => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#variable.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("variable"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::VariablePrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#variable.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_variable"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "name", "variable"],
                                ));
                            },
                        }
                    }
                    Ok(StructureMapGroupRuleDependent {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#variable: r#variable.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Transform Rule from source to target."]
#[derive(Default, Debug, Clone)]
pub struct StructureMapGroupRule {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Name of the rule for internal references."]
    pub r#name: super::super::types::Id,
    #[doc = "Source inputs to the mapping."]
    pub r#source: Vec<StructureMapGroupRuleSource>,
    #[doc = "Content to create because of this mapping rule."]
    pub r#target: Vec<StructureMapGroupRuleTarget>,
    #[doc = "Rules contained in this rule."]
    pub r#rule: Vec<StructureMapGroupRule>,
    #[doc = "Which other rules to apply in the context of this rule."]
    pub r#dependent: Vec<StructureMapGroupRuleDependent>,
    #[doc = "Documentation for this instance of data."]
    pub r#documentation: Option<super::super::types::String>,
}
impl serde::ser::Serialize for StructureMapGroupRule {
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
                if let Some(some) = self.r#name.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#name.id.as_ref(),
                        extension: &self.r#name.extension,
                    };
                    state.serialize_entry("_name", &primitive_element)?;
                }
            } else {
                state.serialize_entry("name", &self.r#name)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            if !self.r#target.is_empty() {
                state.serialize_entry("target", &self.r#target)?;
            }
            if !self.r#rule.is_empty() {
                state.serialize_entry("rule", &self.r#rule)?;
            }
            if !self.r#dependent.is_empty() {
                state.serialize_entry("dependent", &self.r#dependent)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for StructureMapGroupRule {
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
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "source")]
            Source,
            #[serde(rename = "target")]
            Target,
            #[serde(rename = "rule")]
            Rule,
            #[serde(rename = "dependent")]
            Dependent,
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "_documentation")]
            DocumentationPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StructureMapGroupRule;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRule")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<StructureMapGroupRule, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::Id> = None;
                let mut r#source: Option<Vec<StructureMapGroupRuleSource>> = None;
                let mut r#target: Option<Vec<StructureMapGroupRuleTarget>> = None;
                let mut r#rule: Option<Vec<StructureMapGroupRule>> = None;
                let mut r#dependent: Option<Vec<StructureMapGroupRuleDependent>> = None;
                let mut r#documentation: Option<super::super::types::String> = None;
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
                            Field::Name => {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::NamePrimitiveElement => {
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
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
                            }
                            Field::Target => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field("target"));
                                }
                                r#target = Some(map_access.next_value()?);
                            }
                            Field::Rule => {
                                if r#rule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rule"));
                                }
                                r#rule = Some(map_access.next_value()?);
                            }
                            Field::Dependent => {
                                if r#dependent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dependent"));
                                }
                                r#dependent = Some(map_access.next_value()?);
                            }
                            Field::Documentation => {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DocumentationPrimitiveElement => {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_documentation",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
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
                                        "name",
                                        "source",
                                        "target",
                                        "rule",
                                        "dependent",
                                        "documentation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(StructureMapGroupRule {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#source: r#source.unwrap_or(vec![]),
                        r#target: r#target.unwrap_or(vec![]),
                        r#rule: r#rule.unwrap_or(vec![]),
                        r#dependent: r#dependent.unwrap_or(vec![]),
                        r#documentation,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Organizes the mapping into manageable chunks for human review/ease of maintenance."]
#[derive(Default, Debug, Clone)]
pub struct StructureMapGroup {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A unique name for the group for the convenience of human readers."]
    pub r#name: super::super::types::Id,
    #[doc = "Another group that this group adds rules to."]
    pub r#extends: Option<super::super::types::Id>,
    #[doc = "If this is the default rule set to apply for the source type or this combination of types."]
    pub r#type_mode: super::super::types::Code,
    #[doc = "Additional supporting documentation that explains the purpose of the group and the types of mappings within it."]
    pub r#documentation: Option<super::super::types::String>,
    #[doc = "A name assigned to an instance of data. The instance must be provided when the mapping is invoked."]
    pub r#input: Vec<StructureMapGroupInput>,
    #[doc = "Transform Rule from source to target."]
    pub r#rule: Vec<StructureMapGroupRule>,
}
impl serde::ser::Serialize for StructureMapGroup {
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
                if let Some(some) = self.r#name.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#name.id.as_ref(),
                        extension: &self.r#name.extension,
                    };
                    state.serialize_entry("_name", &primitive_element)?;
                }
            } else {
                state.serialize_entry("name", &self.r#name)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#extends.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("extends", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_extends", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#extends.as_ref() {
                    state.serialize_entry("extends", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#type_mode.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("typeMode", &some)?;
                }
                if self.r#type_mode.id.is_some() || !self.r#type_mode.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#type_mode.id.as_ref(),
                        extension: &self.r#type_mode.extension,
                    };
                    state.serialize_entry("_typeMode", &primitive_element)?;
                }
            } else {
                state.serialize_entry("typeMode", &self.r#type_mode)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#documentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("documentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_documentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#documentation.as_ref() {
                    state.serialize_entry("documentation", some)?;
                }
            }
            if !self.r#input.is_empty() {
                state.serialize_entry("input", &self.r#input)?;
            }
            if !self.r#rule.is_empty() {
                state.serialize_entry("rule", &self.r#rule)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for StructureMapGroup {
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
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "extends")]
            Extends,
            #[serde(rename = "_extends")]
            ExtendsPrimitiveElement,
            #[serde(rename = "typeMode")]
            TypeMode,
            #[serde(rename = "_typeMode")]
            TypeModePrimitiveElement,
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "_documentation")]
            DocumentationPrimitiveElement,
            #[serde(rename = "input")]
            Input,
            #[serde(rename = "rule")]
            Rule,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StructureMapGroup;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroup")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<StructureMapGroup, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::Id> = None;
                let mut r#extends: Option<super::super::types::Id> = None;
                let mut r#type_mode: Option<super::super::types::Code> = None;
                let mut r#documentation: Option<super::super::types::String> = None;
                let mut r#input: Option<Vec<StructureMapGroupInput>> = None;
                let mut r#rule: Option<Vec<StructureMapGroupRule>> = None;
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
                            Field::Name => {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::NamePrimitiveElement => {
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
                            }
                            Field::Extends => {
                                let some = r#extends.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extends"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ExtendsPrimitiveElement => {
                                let some = r#extends.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_extends"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::TypeMode => {
                                let some = r#type_mode.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("typeMode"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TypeModePrimitiveElement => {
                                let some = r#type_mode.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_typeMode"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Documentation => {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DocumentationPrimitiveElement => {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_documentation",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Input => {
                                if r#input.is_some() {
                                    return Err(serde::de::Error::duplicate_field("input"));
                                }
                                r#input = Some(map_access.next_value()?);
                            }
                            Field::Rule => {
                                if r#rule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rule"));
                                }
                                r#rule = Some(map_access.next_value()?);
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
                                        "name",
                                        "extends",
                                        "typeMode",
                                        "documentation",
                                        "input",
                                        "rule",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(StructureMapGroup {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#extends,
                        r#type_mode: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type_mode.unwrap_or(Default::default())
                        } else {
                            r#type_mode.ok_or(serde::de::Error::missing_field("typeMode"))?
                        },
                        r#documentation,
                        r#input: r#input.unwrap_or(vec![]),
                        r#rule: r#rule.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A Map of relationships between 2 structures that can be used to transform data."]
#[derive(Default, Debug, Clone)]
pub struct StructureMap {
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
    #[doc = "An absolute URI that is used to identify this structure map when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this structure map is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the structure map is stored on different servers."]
    pub r#url: super::super::types::Uri,
    #[doc = "A formal identifier that is used to identify this structure map when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the structure map when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the structure map author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the structure map. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: super::super::types::String,
    #[doc = "A short, descriptive, user-friendly title for the structure map."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this structure map. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this structure map is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the structure map was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the structure map changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the structure map."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the structure map from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate structure map instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the structure map is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this structure map is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the structure map and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the structure map."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A structure definition used by this map. The structure definition may describe instances that are converted, or the instances that are produced."]
    pub r#structure: Vec<StructureMapStructure>,
    #[doc = "Other maps used by this map (canonical URLs)."]
    pub r#import: Vec<super::super::types::Canonical>,
    #[doc = "Organizes the mapping into manageable chunks for human review/ease of maintenance."]
    pub r#group: Vec<StructureMapGroup>,
}
impl crate::AnyResource for StructureMap {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for StructureMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "StructureMap")?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#url.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("url", &some)?;
                }
                if self.r#url.id.is_some() || !self.r#url.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#url.id.as_ref(),
                        extension: &self.r#url.extension,
                    };
                    state.serialize_entry("_url", &primitive_element)?;
                }
            } else {
                state.serialize_entry("url", &self.r#url)?;
            }
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
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
                if let Some(some) = self.r#name.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#name.id.as_ref(),
                        extension: &self.r#name.extension,
                    };
                    state.serialize_entry("_name", &primitive_element)?;
                }
            } else {
                state.serialize_entry("name", &self.r#name)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#experimental.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("experimental", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_experimental", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#experimental.as_ref() {
                    state.serialize_entry("experimental", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#publisher.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("publisher", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_publisher", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#publisher.as_ref() {
                    state.serialize_entry("publisher", some)?;
                }
            }
            if !self.r#contact.is_empty() {
                state.serialize_entry("contact", &self.r#contact)?;
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
            if !self.r#use_context.is_empty() {
                state.serialize_entry("useContext", &self.r#use_context)?;
            }
            if !self.r#jurisdiction.is_empty() {
                state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#purpose.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("purpose", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_purpose", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#purpose.as_ref() {
                    state.serialize_entry("purpose", some)?;
                }
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
            if !self.r#structure.is_empty() {
                state.serialize_entry("structure", &self.r#structure)?;
            }
            if _ctx.output_json {
                if !self.r#import.is_empty() {
                    let values = self
                        .r#import
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("import", &values)?;
                    }
                    let requires_elements = self
                        .r#import
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#import
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
                        state.serialize_entry("_import", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#import.is_empty() {
                    state.serialize_entry("import", &self.r#import)?;
                }
            }
            if !self.r#group.is_empty() {
                state.serialize_entry("group", &self.r#group)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for StructureMap {
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
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "experimental")]
            Experimental,
            #[serde(rename = "_experimental")]
            ExperimentalPrimitiveElement,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "publisher")]
            Publisher,
            #[serde(rename = "_publisher")]
            PublisherPrimitiveElement,
            #[serde(rename = "contact")]
            Contact,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "useContext")]
            UseContext,
            #[serde(rename = "jurisdiction")]
            Jurisdiction,
            #[serde(rename = "purpose")]
            Purpose,
            #[serde(rename = "_purpose")]
            PurposePrimitiveElement,
            #[serde(rename = "copyright")]
            Copyright,
            #[serde(rename = "_copyright")]
            CopyrightPrimitiveElement,
            #[serde(rename = "structure")]
            Structure,
            #[serde(rename = "import")]
            Import,
            #[serde(rename = "_import")]
            ImportPrimitiveElement,
            #[serde(rename = "group")]
            Group,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StructureMap;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMap")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<StructureMap, V::Error>
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
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#version: Option<super::super::types::String> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#experimental: Option<super::super::types::Boolean> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#publisher: Option<super::super::types::String> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#use_context: Option<Vec<Box<super::super::types::UsageContext>>> = None;
                let mut r#jurisdiction: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#purpose: Option<super::super::types::Markdown> = None;
                let mut r#copyright: Option<super::super::types::Markdown> = None;
                let mut r#structure: Option<Vec<StructureMapStructure>> = None;
                let mut r#import: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#group: Option<Vec<StructureMapGroup>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "StructureMap" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"StructureMap",
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
                            Field::Url => {
                                let some = r#url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::UrlPrimitiveElement => {
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
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Version => {
                                let some = r#version.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::VersionPrimitiveElement => {
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
                            }
                            Field::Name => {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::NamePrimitiveElement => {
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
                            }
                            Field::Title => {
                                let some = r#title.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TitlePrimitiveElement => {
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
                            Field::Experimental => {
                                let some = r#experimental.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("experimental"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ExperimentalPrimitiveElement => {
                                let some = r#experimental.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_experimental"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
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
                            Field::Publisher => {
                                let some = r#publisher.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publisher"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PublisherPrimitiveElement => {
                                let some = r#publisher.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_publisher"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Contact => {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact = Some(map_access.next_value()?);
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
                            Field::UseContext => {
                                if r#use_context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("useContext"));
                                }
                                r#use_context = Some(map_access.next_value()?);
                            }
                            Field::Jurisdiction => {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                r#jurisdiction = Some(map_access.next_value()?);
                            }
                            Field::Purpose => {
                                let some = r#purpose.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("purpose"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PurposePrimitiveElement => {
                                let some = r#purpose.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_purpose"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Copyright => {
                                let some = r#copyright.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("copyright"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::CopyrightPrimitiveElement => {
                                let some = r#copyright.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_copyright"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Structure => {
                                if r#structure.is_some() {
                                    return Err(serde::de::Error::duplicate_field("structure"));
                                }
                                r#structure = Some(map_access.next_value()?);
                            }
                            Field::Import => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#import.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("import"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::ImportPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#import.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_import"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
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
                                        "meta",
                                        "implicitRules",
                                        "language",
                                        "text",
                                        "contained",
                                        "extension",
                                        "modifierExtension",
                                        "url",
                                        "identifier",
                                        "version",
                                        "name",
                                        "title",
                                        "status",
                                        "experimental",
                                        "date",
                                        "publisher",
                                        "contact",
                                        "description",
                                        "useContext",
                                        "jurisdiction",
                                        "purpose",
                                        "copyright",
                                        "structure",
                                        "import",
                                        "group",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(StructureMap {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#url: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#url.unwrap_or(Default::default())
                        } else {
                            r#url.ok_or(serde::de::Error::missing_field("url"))?
                        },
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#version,
                        r#name: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#title,
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#experimental,
                        r#date,
                        r#publisher,
                        r#contact: r#contact.unwrap_or(vec![]),
                        r#description,
                        r#use_context: r#use_context.unwrap_or(vec![]),
                        r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                        r#purpose,
                        r#copyright,
                        r#structure: r#structure.unwrap_or(vec![]),
                        r#import: r#import.unwrap_or(vec![]),
                        r#group: r#group.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
