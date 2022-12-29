// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "The value of this property."]
#[derive(Debug, Clone)]
pub enum CodeSystemConceptPropertyValue {
    Code(Box<super::super::types::Code>),
    Coding(Box<super::super::types::Coding>),
    String(Box<super::super::types::String>),
    Integer(Box<super::super::types::Integer>),
    Boolean(Box<super::super::types::Boolean>),
    DateTime(Box<super::super::types::DateTime>),
    Decimal(Box<super::super::types::Decimal>),
    Invalid,
}
impl Default for CodeSystemConceptPropertyValue {
    fn default() -> CodeSystemConceptPropertyValue {
        CodeSystemConceptPropertyValue::Invalid
    }
}
#[doc = "A filter that can be used in a value set compose statement when selecting concepts using a filter."]
#[derive(Default, Debug, Clone)]
pub struct CodeSystemFilter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The code that identifies this filter when it is used as a filter in [ValueSet](valueset.html#).compose.include.filter."]
    pub r#code: super::super::types::Code,
    #[doc = "A description of how or why the filter is used."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "A list of operators that can be used with the filter."]
    pub r#operator: Vec<super::super::types::Code>,
    #[doc = "A description of what the value for the filter should be."]
    pub r#value: super::super::types::String,
}
impl serde::ser::Serialize for CodeSystemFilter {
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
                if let Some(some) = self.r#code.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("code", &some)?;
                }
                if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#code.id.as_ref(),
                        extension: &self.r#code.extension,
                    };
                    state.serialize_entry("_code", &primitive_element)?;
                }
            } else {
                state.serialize_entry("code", &self.r#code)?;
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
            if _ctx.output_json {
                if !self.r#operator.is_empty() {
                    let values = self
                        .r#operator
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("operator", &values)?;
                    }
                    let requires_elements = self
                        .r#operator
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#operator
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
                        state.serialize_entry("_operator", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#operator.is_empty() {
                    state.serialize_entry("operator", &self.r#operator)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("value", &some)?;
                }
                if self.r#value.id.is_some() || !self.r#value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#value.id.as_ref(),
                        extension: &self.r#value.extension,
                    };
                    state.serialize_entry("_value", &primitive_element)?;
                }
            } else {
                state.serialize_entry("value", &self.r#value)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CodeSystemFilter {
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
            #[serde(rename = "_code")]
            CodePrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "operator")]
            Operator,
            #[serde(rename = "_operator")]
            OperatorPrimitiveElement,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CodeSystemFilter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CodeSystemFilter")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CodeSystemFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<super::super::types::Code> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#operator: Option<Vec<super::super::types::Code>> = None;
                let mut r#value: Option<super::super::types::String> = None;
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
                            Field::Code => {
                                if _ctx.from_json {
                                    let some = r#code.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#code.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    r#code = Some(map_access.next_value()?);
                                }
                            }
                            Field::CodePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#code.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_code"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "code",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "description",
                                            "operator",
                                            "value",
                                        ],
                                    ));
                                }
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
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "description",
                                            "operator",
                                            "value",
                                        ],
                                    ));
                                }
                            }
                            Field::Operator => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#operator.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("operator"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#operator.is_some() {
                                        return Err(serde::de::Error::duplicate_field("operator"));
                                    }
                                    r#operator = Some(map_access.next_value()?);
                                }
                            }
                            Field::OperatorPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#operator.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_operator"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "operator",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "description",
                                            "operator",
                                            "value",
                                        ],
                                    ));
                                }
                            }
                            Field::Value => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    r#value = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValuePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_value"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "value",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "description",
                                            "operator",
                                            "value",
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
                                        "code",
                                        "description",
                                        "operator",
                                        "value",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CodeSystemFilter {
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
                        r#description,
                        r#operator: r#operator.unwrap_or(vec![]),
                        r#value: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A property defines an additional slot through which additional information can be provided about a concept."]
#[derive(Default, Debug, Clone)]
pub struct CodeSystemProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code that is used to identify the property. The code is used internally (in CodeSystem.concept.property.code) and also externally, such as in property filters."]
    pub r#code: super::super::types::Code,
    #[doc = "Reference to the formal meaning of the property. One possible source of meaning is the [Concept Properties](https://hl7.org/FHIR/codesystem-concept-properties.html)) code system."]
    pub r#uri: Option<super::super::types::Uri>,
    #[doc = "A description of the property- why it is defined, and how its value might be used."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The type of the property value. Properties of type \"code\" contain a code defined by the code system (e.g. a reference to another defined concept)."]
    pub r#type: super::super::types::Code,
}
impl serde::ser::Serialize for CodeSystemProperty {
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
                if let Some(some) = self.r#code.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("code", &some)?;
                }
                if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#code.id.as_ref(),
                        extension: &self.r#code.extension,
                    };
                    state.serialize_entry("_code", &primitive_element)?;
                }
            } else {
                state.serialize_entry("code", &self.r#code)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#uri.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("uri", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_uri", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#uri.as_ref() {
                    state.serialize_entry("uri", some)?;
                }
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
            if _ctx.output_json {
                if let Some(some) = self.r#type.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("type", &some)?;
                }
                if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#type.id.as_ref(),
                        extension: &self.r#type.extension,
                    };
                    state.serialize_entry("_type", &primitive_element)?;
                }
            } else {
                state.serialize_entry("type", &self.r#type)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CodeSystemProperty {
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
            #[serde(rename = "_code")]
            CodePrimitiveElement,
            #[serde(rename = "uri")]
            Uri,
            #[serde(rename = "_uri")]
            UriPrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CodeSystemProperty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CodeSystemProperty")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CodeSystemProperty, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<super::super::types::Code> = None;
                let mut r#uri: Option<super::super::types::Uri> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#type: Option<super::super::types::Code> = None;
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
                            Field::Code => {
                                if _ctx.from_json {
                                    let some = r#code.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#code.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    r#code = Some(map_access.next_value()?);
                                }
                            }
                            Field::CodePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#code.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_code"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "code",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "uri",
                                            "description",
                                            "type",
                                        ],
                                    ));
                                }
                            }
                            Field::Uri => {
                                if _ctx.from_json {
                                    let some = r#uri.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("uri"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#uri.is_some() {
                                        return Err(serde::de::Error::duplicate_field("uri"));
                                    }
                                    r#uri = Some(map_access.next_value()?);
                                }
                            }
                            Field::UriPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#uri.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_uri"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "uri",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "uri",
                                            "description",
                                            "type",
                                        ],
                                    ));
                                }
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
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "uri",
                                            "description",
                                            "type",
                                        ],
                                    ));
                                }
                            }
                            Field::Type => {
                                if _ctx.from_json {
                                    let some = r#type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    r#type = Some(map_access.next_value()?);
                                }
                            }
                            Field::TypePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "type",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "uri",
                                            "description",
                                            "type",
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
                                        "code",
                                        "uri",
                                        "description",
                                        "type",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CodeSystemProperty {
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
                        r#uri,
                        r#description,
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Additional representations for the concept - other languages, aliases, specialized purposes, used for particular purposes, etc."]
#[derive(Default, Debug, Clone)]
pub struct CodeSystemConceptDesignation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The language this designation is defined for."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A code that details how this designation would be used."]
    pub r#use: Option<Box<super::super::types::Coding>>,
    #[doc = "The text value for this designation."]
    pub r#value: super::super::types::String,
}
impl serde::ser::Serialize for CodeSystemConceptDesignation {
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
            if let Some(some) = self.r#use.as_ref() {
                state.serialize_entry("use", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("value", &some)?;
                }
                if self.r#value.id.is_some() || !self.r#value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#value.id.as_ref(),
                        extension: &self.r#value.extension,
                    };
                    state.serialize_entry("_value", &primitive_element)?;
                }
            } else {
                state.serialize_entry("value", &self.r#value)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CodeSystemConceptDesignation {
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
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "_language")]
            LanguagePrimitiveElement,
            #[serde(rename = "use")]
            Use,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CodeSystemConceptDesignation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CodeSystemConceptDesignation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CodeSystemConceptDesignation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#use: Option<Box<super::super::types::Coding>> = None;
                let mut r#value: Option<super::super::types::String> = None;
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
                                            "extension",
                                            "modifierExtension",
                                            "language",
                                            "use",
                                            "value",
                                        ],
                                    ));
                                }
                            }
                            Field::Use => {
                                if r#use.is_some() {
                                    return Err(serde::de::Error::duplicate_field("use"));
                                }
                                r#use = Some(map_access.next_value()?);
                            }
                            Field::Value => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    r#value = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValuePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_value"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "value",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "language",
                                            "use",
                                            "value",
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
                                        "language",
                                        "use",
                                        "value",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CodeSystemConceptDesignation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#language,
                        r#use,
                        r#value: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A property value for this concept."]
#[derive(Default, Debug, Clone)]
pub struct CodeSystemConceptProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code that is a reference to CodeSystem.property.code."]
    pub r#code: super::super::types::Code,
    #[doc = "The value of this property."]
    pub r#value: CodeSystemConceptPropertyValue,
}
impl serde::ser::Serialize for CodeSystemConceptProperty {
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
                if let Some(some) = self.r#code.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("code", &some)?;
                }
                if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#code.id.as_ref(),
                        extension: &self.r#code.extension,
                    };
                    state.serialize_entry("_code", &primitive_element)?;
                }
            } else {
                state.serialize_entry("code", &self.r#code)?;
            }
            match self.r#value {
                CodeSystemConceptPropertyValue::Code(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueCode", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueCode", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueCode", value)?;
                    }
                }
                CodeSystemConceptPropertyValue::Coding(ref value) => {
                    state.serialize_entry("valueCoding", value)?;
                }
                CodeSystemConceptPropertyValue::String(ref value) => {
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
                CodeSystemConceptPropertyValue::Integer(ref value) => {
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
                CodeSystemConceptPropertyValue::Boolean(ref value) => {
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
                CodeSystemConceptPropertyValue::DateTime(ref value) => {
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
                CodeSystemConceptPropertyValue::Decimal(ref value) => {
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
                CodeSystemConceptPropertyValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CodeSystemConceptProperty {
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
            #[serde(rename = "_code")]
            CodePrimitiveElement,
            #[serde(rename = "valueCode")]
            ValueCode,
            #[serde(rename = "_valueCode")]
            ValueCodePrimitiveElement,
            #[serde(rename = "valueCoding")]
            ValueCoding,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueInteger")]
            ValueInteger,
            #[serde(rename = "_valueInteger")]
            ValueIntegerPrimitiveElement,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueDateTime")]
            ValueDateTime,
            #[serde(rename = "_valueDateTime")]
            ValueDateTimePrimitiveElement,
            #[serde(rename = "valueDecimal")]
            ValueDecimal,
            #[serde(rename = "_valueDecimal")]
            ValueDecimalPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CodeSystemConceptProperty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CodeSystemConceptProperty")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CodeSystemConceptProperty, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<super::super::types::Code> = None;
                let mut r#value: Option<CodeSystemConceptPropertyValue> = None;
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
                            Field::Code => {
                                if _ctx.from_json {
                                    let some = r#code.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#code.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    r#code = Some(map_access.next_value()?);
                                }
                            }
                            Field::CodePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#code.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_code"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "code",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "valueCode",
                                            "valueCoding",
                                            "valueString",
                                            "valueInteger",
                                            "valueBoolean",
                                            "valueDateTime",
                                            "valueDecimal",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueCode => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        CodeSystemConceptPropertyValue::Code(Default::default()),
                                    );
                                    if let CodeSystemConceptPropertyValue::Code(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueCode",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("value[x]"));
                                    }
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueCode"));
                                    }
                                    r#value = Some(CodeSystemConceptPropertyValue::Code(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueCodePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        CodeSystemConceptPropertyValue::Code(Default::default()),
                                    );
                                    if let CodeSystemConceptPropertyValue::Code(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueCode",
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
                                        "valueCode",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "valueCode",
                                            "valueCoding",
                                            "valueString",
                                            "valueInteger",
                                            "valueBoolean",
                                            "valueDateTime",
                                            "valueDecimal",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueCoding => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCoding"));
                                }
                                r#value = Some(CodeSystemConceptPropertyValue::Coding(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueString => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        CodeSystemConceptPropertyValue::String(Default::default()),
                                    );
                                    if let CodeSystemConceptPropertyValue::String(variant) = r#enum
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
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    r#value = Some(CodeSystemConceptPropertyValue::String(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        CodeSystemConceptPropertyValue::String(Default::default()),
                                    );
                                    if let CodeSystemConceptPropertyValue::String(variant) = r#enum
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueString",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "valueCode",
                                            "valueCoding",
                                            "valueString",
                                            "valueInteger",
                                            "valueBoolean",
                                            "valueDateTime",
                                            "valueDecimal",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueInteger => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        CodeSystemConceptPropertyValue::Integer(Default::default()),
                                    );
                                    if let CodeSystemConceptPropertyValue::Integer(variant) = r#enum
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
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInteger",
                                        ));
                                    }
                                    r#value = Some(CodeSystemConceptPropertyValue::Integer(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueIntegerPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        CodeSystemConceptPropertyValue::Integer(Default::default()),
                                    );
                                    if let CodeSystemConceptPropertyValue::Integer(variant) = r#enum
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueInteger",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "valueCode",
                                            "valueCoding",
                                            "valueString",
                                            "valueInteger",
                                            "valueBoolean",
                                            "valueDateTime",
                                            "valueDecimal",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueBoolean => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        CodeSystemConceptPropertyValue::Boolean(Default::default()),
                                    );
                                    if let CodeSystemConceptPropertyValue::Boolean(variant) = r#enum
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
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    r#value = Some(CodeSystemConceptPropertyValue::Boolean(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueBooleanPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        CodeSystemConceptPropertyValue::Boolean(Default::default()),
                                    );
                                    if let CodeSystemConceptPropertyValue::Boolean(variant) = r#enum
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueBoolean",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "valueCode",
                                            "valueCoding",
                                            "valueString",
                                            "valueInteger",
                                            "valueBoolean",
                                            "valueDateTime",
                                            "valueDecimal",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueDateTime => {
                                if _ctx.from_json {
                                    let r#enum =
                                        r#value.get_or_insert(
                                            CodeSystemConceptPropertyValue::DateTime(
                                                Default::default(),
                                            ),
                                        );
                                    if let CodeSystemConceptPropertyValue::DateTime(variant) =
                                        r#enum
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
                                    r#value = Some(CodeSystemConceptPropertyValue::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum =
                                        r#value.get_or_insert(
                                            CodeSystemConceptPropertyValue::DateTime(
                                                Default::default(),
                                            ),
                                        );
                                    if let CodeSystemConceptPropertyValue::DateTime(variant) =
                                        r#enum
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
                                            "code",
                                            "valueCode",
                                            "valueCoding",
                                            "valueString",
                                            "valueInteger",
                                            "valueBoolean",
                                            "valueDateTime",
                                            "valueDecimal",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueDecimal => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        CodeSystemConceptPropertyValue::Decimal(Default::default()),
                                    );
                                    if let CodeSystemConceptPropertyValue::Decimal(variant) = r#enum
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
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDecimal",
                                        ));
                                    }
                                    r#value = Some(CodeSystemConceptPropertyValue::Decimal(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueDecimalPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        CodeSystemConceptPropertyValue::Decimal(Default::default()),
                                    );
                                    if let CodeSystemConceptPropertyValue::Decimal(variant) = r#enum
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueDecimal",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "valueCode",
                                            "valueCoding",
                                            "valueString",
                                            "valueInteger",
                                            "valueBoolean",
                                            "valueDateTime",
                                            "valueDecimal",
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
                                        "code",
                                        "valueCode",
                                        "valueCoding",
                                        "valueString",
                                        "valueInteger",
                                        "valueBoolean",
                                        "valueDateTime",
                                        "valueDecimal",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CodeSystemConceptProperty {
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
#[doc = "Concepts that are in the code system. The concept definitions are inherently hierarchical, but the definitions must be consulted to determine what the meanings of the hierarchical relationships are."]
#[derive(Default, Debug, Clone)]
pub struct CodeSystemConcept {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code - a text symbol - that uniquely identifies the concept within the code system."]
    pub r#code: super::super::types::Code,
    #[doc = "A human readable string that is the recommended default way to present this concept to a user."]
    pub r#display: Option<super::super::types::String>,
    #[doc = "The formal definition of the concept. The code system resource does not make formal definitions required, because of the prevalence of legacy systems. However, they are highly recommended, as without them there is no formal meaning associated with the concept."]
    pub r#definition: Option<super::super::types::String>,
    #[doc = "Additional representations for the concept - other languages, aliases, specialized purposes, used for particular purposes, etc."]
    pub r#designation: Vec<CodeSystemConceptDesignation>,
    #[doc = "A property value for this concept."]
    pub r#property: Vec<CodeSystemConceptProperty>,
    #[doc = "Defines children of a concept to produce a hierarchy of concepts. The nature of the relationships is variable (is-a/contains/categorizes) - see hierarchyMeaning."]
    pub r#concept: Vec<CodeSystemConcept>,
}
impl serde::ser::Serialize for CodeSystemConcept {
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
                if let Some(some) = self.r#code.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("code", &some)?;
                }
                if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#code.id.as_ref(),
                        extension: &self.r#code.extension,
                    };
                    state.serialize_entry("_code", &primitive_element)?;
                }
            } else {
                state.serialize_entry("code", &self.r#code)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#display.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("display", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_display", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#display.as_ref() {
                    state.serialize_entry("display", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#definition.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("definition", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_definition", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#definition.as_ref() {
                    state.serialize_entry("definition", some)?;
                }
            }
            if !self.r#designation.is_empty() {
                state.serialize_entry("designation", &self.r#designation)?;
            }
            if !self.r#property.is_empty() {
                state.serialize_entry("property", &self.r#property)?;
            }
            if !self.r#concept.is_empty() {
                state.serialize_entry("concept", &self.r#concept)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CodeSystemConcept {
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
            #[serde(rename = "_code")]
            CodePrimitiveElement,
            #[serde(rename = "display")]
            Display,
            #[serde(rename = "_display")]
            DisplayPrimitiveElement,
            #[serde(rename = "definition")]
            Definition,
            #[serde(rename = "_definition")]
            DefinitionPrimitiveElement,
            #[serde(rename = "designation")]
            Designation,
            #[serde(rename = "property")]
            Property,
            #[serde(rename = "concept")]
            Concept,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CodeSystemConcept;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CodeSystemConcept")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CodeSystemConcept, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<super::super::types::Code> = None;
                let mut r#display: Option<super::super::types::String> = None;
                let mut r#definition: Option<super::super::types::String> = None;
                let mut r#designation: Option<Vec<CodeSystemConceptDesignation>> = None;
                let mut r#property: Option<Vec<CodeSystemConceptProperty>> = None;
                let mut r#concept: Option<Vec<CodeSystemConcept>> = None;
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
                            Field::Code => {
                                if _ctx.from_json {
                                    let some = r#code.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#code.is_some() {
                                        return Err(serde::de::Error::duplicate_field("code"));
                                    }
                                    r#code = Some(map_access.next_value()?);
                                }
                            }
                            Field::CodePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#code.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_code"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "code",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "display",
                                            "definition",
                                            "designation",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::Display => {
                                if _ctx.from_json {
                                    let some = r#display.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("display"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#display.is_some() {
                                        return Err(serde::de::Error::duplicate_field("display"));
                                    }
                                    r#display = Some(map_access.next_value()?);
                                }
                            }
                            Field::DisplayPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#display.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_display"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "display",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "display",
                                            "definition",
                                            "designation",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::Definition => {
                                if _ctx.from_json {
                                    let some = r#definition.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "definition",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#definition.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "definition",
                                        ));
                                    }
                                    r#definition = Some(map_access.next_value()?);
                                }
                            }
                            Field::DefinitionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#definition.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_definition",
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
                                        "definition",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "code",
                                            "display",
                                            "definition",
                                            "designation",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::Designation => {
                                if r#designation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("designation"));
                                }
                                r#designation = Some(map_access.next_value()?);
                            }
                            Field::Property => {
                                if r#property.is_some() {
                                    return Err(serde::de::Error::duplicate_field("property"));
                                }
                                r#property = Some(map_access.next_value()?);
                            }
                            Field::Concept => {
                                if r#concept.is_some() {
                                    return Err(serde::de::Error::duplicate_field("concept"));
                                }
                                r#concept = Some(map_access.next_value()?);
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
                                        "code",
                                        "display",
                                        "definition",
                                        "designation",
                                        "property",
                                        "concept",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CodeSystemConcept {
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
                        r#display,
                        r#definition,
                        r#designation: r#designation.unwrap_or(vec![]),
                        r#property: r#property.unwrap_or(vec![]),
                        r#concept: r#concept.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The CodeSystem resource is used to declare the existence of and describe a code system or code system supplement and its key properties, and optionally define a part or all of its content."]
#[derive(Default, Debug, Clone)]
pub struct CodeSystem {
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
    #[doc = "An absolute URI that is used to identify this code system when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this code system is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the code system is stored on different servers. This is used in [Coding](datatypes.html#Coding).system."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this code system when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the code system when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the code system author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence. This is used in [Coding](datatypes.html#Coding).version."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the code system. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the code system."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The date (and optionally time) when the code system resource was created or revised."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this code system is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the code system was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the code system changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the code system."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the code system from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate code system instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the code system is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this code system is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the code system and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the code system."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "If code comparison is case sensitive when codes within this system are compared to each other."]
    pub r#case_sensitive: Option<super::super::types::Boolean>,
    #[doc = "Canonical reference to the value set that contains the entire code system."]
    pub r#value_set: Option<super::super::types::Canonical>,
    #[doc = "The meaning of the hierarchy of concepts as represented in this resource."]
    pub r#hierarchy_meaning: Option<super::super::types::Code>,
    #[doc = "The code system defines a compositional (post-coordination) grammar."]
    pub r#compositional: Option<super::super::types::Boolean>,
    #[doc = "This flag is used to signify that the code system does not commit to concept permanence across versions. If true, a version must be specified when referencing this code system."]
    pub r#version_needed: Option<super::super::types::Boolean>,
    #[doc = "The extent of the content of the code system (the concepts and codes it defines) are represented in this resource instance."]
    pub r#content: super::super::types::Code,
    #[doc = "The canonical URL of the code system that this code system supplement is adding designations and properties to."]
    pub r#supplements: Option<super::super::types::Canonical>,
    #[doc = "The total number of concepts defined by the code system. Where the code system has a compositional grammar, the basis of this count is defined by the system steward."]
    pub r#count: Option<super::super::types::UnsignedInt>,
    #[doc = "A filter that can be used in a value set compose statement when selecting concepts using a filter."]
    pub r#filter: Vec<CodeSystemFilter>,
    #[doc = "A property defines an additional slot through which additional information can be provided about a concept."]
    pub r#property: Vec<CodeSystemProperty>,
    #[doc = "Concepts that are in the code system. The concept definitions are inherently hierarchical, but the definitions must be consulted to determine what the meanings of the hierarchical relationships are."]
    pub r#concept: Vec<CodeSystemConcept>,
}
impl crate::AnyResource for CodeSystem {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for CodeSystem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "CodeSystem")?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#case_sensitive.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("caseSensitive", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_caseSensitive", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#case_sensitive.as_ref() {
                    state.serialize_entry("caseSensitive", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value_set.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueSet", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_valueSet", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#value_set.as_ref() {
                    state.serialize_entry("valueSet", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#hierarchy_meaning.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("hierarchyMeaning", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_hierarchyMeaning", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#hierarchy_meaning.as_ref() {
                    state.serialize_entry("hierarchyMeaning", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#compositional.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("compositional", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_compositional", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#compositional.as_ref() {
                    state.serialize_entry("compositional", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#version_needed.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("versionNeeded", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_versionNeeded", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#version_needed.as_ref() {
                    state.serialize_entry("versionNeeded", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#content.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("content", &some)?;
                }
                if self.r#content.id.is_some() || !self.r#content.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#content.id.as_ref(),
                        extension: &self.r#content.extension,
                    };
                    state.serialize_entry("_content", &primitive_element)?;
                }
            } else {
                state.serialize_entry("content", &self.r#content)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#supplements.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("supplements", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_supplements", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#supplements.as_ref() {
                    state.serialize_entry("supplements", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#count.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("count", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_count", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#count.as_ref() {
                    state.serialize_entry("count", some)?;
                }
            }
            if !self.r#filter.is_empty() {
                state.serialize_entry("filter", &self.r#filter)?;
            }
            if !self.r#property.is_empty() {
                state.serialize_entry("property", &self.r#property)?;
            }
            if !self.r#concept.is_empty() {
                state.serialize_entry("concept", &self.r#concept)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CodeSystem {
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
            #[serde(rename = "caseSensitive")]
            CaseSensitive,
            #[serde(rename = "_caseSensitive")]
            CaseSensitivePrimitiveElement,
            #[serde(rename = "valueSet")]
            ValueSet,
            #[serde(rename = "_valueSet")]
            ValueSetPrimitiveElement,
            #[serde(rename = "hierarchyMeaning")]
            HierarchyMeaning,
            #[serde(rename = "_hierarchyMeaning")]
            HierarchyMeaningPrimitiveElement,
            #[serde(rename = "compositional")]
            Compositional,
            #[serde(rename = "_compositional")]
            CompositionalPrimitiveElement,
            #[serde(rename = "versionNeeded")]
            VersionNeeded,
            #[serde(rename = "_versionNeeded")]
            VersionNeededPrimitiveElement,
            #[serde(rename = "content")]
            Content,
            #[serde(rename = "_content")]
            ContentPrimitiveElement,
            #[serde(rename = "supplements")]
            Supplements,
            #[serde(rename = "_supplements")]
            SupplementsPrimitiveElement,
            #[serde(rename = "count")]
            Count,
            #[serde(rename = "_count")]
            CountPrimitiveElement,
            #[serde(rename = "filter")]
            Filter,
            #[serde(rename = "property")]
            Property,
            #[serde(rename = "concept")]
            Concept,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CodeSystem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CodeSystem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CodeSystem, V::Error>
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
                let mut r#case_sensitive: Option<super::super::types::Boolean> = None;
                let mut r#value_set: Option<super::super::types::Canonical> = None;
                let mut r#hierarchy_meaning: Option<super::super::types::Code> = None;
                let mut r#compositional: Option<super::super::types::Boolean> = None;
                let mut r#version_needed: Option<super::super::types::Boolean> = None;
                let mut r#content: Option<super::super::types::Code> = None;
                let mut r#supplements: Option<super::super::types::Canonical> = None;
                let mut r#count: Option<super::super::types::UnsignedInt> = None;
                let mut r#filter: Option<Vec<CodeSystemFilter>> = None;
                let mut r#property: Option<Vec<CodeSystemProperty>> = None;
                let mut r#concept: Option<Vec<CodeSystemConcept>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "CodeSystem" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"CodeSystem",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::Experimental => {
                                if _ctx.from_json {
                                    let some = r#experimental.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "experimental",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#experimental.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "experimental",
                                        ));
                                    }
                                    r#experimental = Some(map_access.next_value()?);
                                }
                            }
                            Field::ExperimentalPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#experimental.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_experimental",
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
                                        "experimental",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::Publisher => {
                                if _ctx.from_json {
                                    let some = r#publisher.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("publisher"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#publisher.is_some() {
                                        return Err(serde::de::Error::duplicate_field("publisher"));
                                    }
                                    r#publisher = Some(map_access.next_value()?);
                                }
                            }
                            Field::PublisherPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#publisher.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_publisher",
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
                                        "publisher",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::Contact => {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact = Some(map_access.next_value()?);
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
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
                                if _ctx.from_json {
                                    let some = r#purpose.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("purpose"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#purpose.is_some() {
                                        return Err(serde::de::Error::duplicate_field("purpose"));
                                    }
                                    r#purpose = Some(map_access.next_value()?);
                                }
                            }
                            Field::PurposePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "purpose",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::CaseSensitive => {
                                if _ctx.from_json {
                                    let some = r#case_sensitive.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "caseSensitive",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#case_sensitive.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "caseSensitive",
                                        ));
                                    }
                                    r#case_sensitive = Some(map_access.next_value()?);
                                }
                            }
                            Field::CaseSensitivePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#case_sensitive.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_caseSensitive",
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
                                        "caseSensitive",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueSet => {
                                if _ctx.from_json {
                                    let some = r#value_set.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueSet"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#value_set.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueSet"));
                                    }
                                    r#value_set = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValueSetPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#value_set.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueSet"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueSet",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::HierarchyMeaning => {
                                if _ctx.from_json {
                                    let some =
                                        r#hierarchy_meaning.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "hierarchyMeaning",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#hierarchy_meaning.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "hierarchyMeaning",
                                        ));
                                    }
                                    r#hierarchy_meaning = Some(map_access.next_value()?);
                                }
                            }
                            Field::HierarchyMeaningPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#hierarchy_meaning.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_hierarchyMeaning",
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
                                        "hierarchyMeaning",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::Compositional => {
                                if _ctx.from_json {
                                    let some = r#compositional.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "compositional",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#compositional.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "compositional",
                                        ));
                                    }
                                    r#compositional = Some(map_access.next_value()?);
                                }
                            }
                            Field::CompositionalPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#compositional.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_compositional",
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
                                        "compositional",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::VersionNeeded => {
                                if _ctx.from_json {
                                    let some = r#version_needed.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "versionNeeded",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#version_needed.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "versionNeeded",
                                        ));
                                    }
                                    r#version_needed = Some(map_access.next_value()?);
                                }
                            }
                            Field::VersionNeededPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#version_needed.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_versionNeeded",
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
                                        "versionNeeded",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::Content => {
                                if _ctx.from_json {
                                    let some = r#content.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("content"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#content.is_some() {
                                        return Err(serde::de::Error::duplicate_field("content"));
                                    }
                                    r#content = Some(map_access.next_value()?);
                                }
                            }
                            Field::ContentPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#content.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_content"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "content",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::Supplements => {
                                if _ctx.from_json {
                                    let some = r#supplements.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "supplements",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#supplements.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "supplements",
                                        ));
                                    }
                                    r#supplements = Some(map_access.next_value()?);
                                }
                            }
                            Field::SupplementsPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#supplements.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_supplements",
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
                                        "supplements",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::Count => {
                                if _ctx.from_json {
                                    let some = r#count.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("count"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#count.is_some() {
                                        return Err(serde::de::Error::duplicate_field("count"));
                                    }
                                    r#count = Some(map_access.next_value()?);
                                }
                            }
                            Field::CountPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#count.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_count"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "count",
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
                                            "caseSensitive",
                                            "valueSet",
                                            "hierarchyMeaning",
                                            "compositional",
                                            "versionNeeded",
                                            "content",
                                            "supplements",
                                            "count",
                                            "filter",
                                            "property",
                                            "concept",
                                        ],
                                    ));
                                }
                            }
                            Field::Filter => {
                                if r#filter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("filter"));
                                }
                                r#filter = Some(map_access.next_value()?);
                            }
                            Field::Property => {
                                if r#property.is_some() {
                                    return Err(serde::de::Error::duplicate_field("property"));
                                }
                                r#property = Some(map_access.next_value()?);
                            }
                            Field::Concept => {
                                if r#concept.is_some() {
                                    return Err(serde::de::Error::duplicate_field("concept"));
                                }
                                r#concept = Some(map_access.next_value()?);
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
                                        "caseSensitive",
                                        "valueSet",
                                        "hierarchyMeaning",
                                        "compositional",
                                        "versionNeeded",
                                        "content",
                                        "supplements",
                                        "count",
                                        "filter",
                                        "property",
                                        "concept",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CodeSystem {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#url,
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#version,
                        r#name,
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
                        r#case_sensitive,
                        r#value_set,
                        r#hierarchy_meaning,
                        r#compositional,
                        r#version_needed,
                        r#content: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#content.unwrap_or(Default::default())
                        } else {
                            r#content.ok_or(serde::de::Error::missing_field("content"))?
                        },
                        r#supplements,
                        r#count,
                        r#filter: r#filter.unwrap_or(vec![]),
                        r#property: r#property.unwrap_or(vec![]),
                        r#concept: r#concept.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
