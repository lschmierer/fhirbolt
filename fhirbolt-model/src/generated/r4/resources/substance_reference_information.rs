// Generated on 2022-12-29 by fhirbolt-codegen v0.1.0
#[doc = "Todo."]
#[derive(Debug, Clone)]
pub enum SubstanceReferenceInformationTargetAmount {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SubstanceReferenceInformationTargetAmount {
    fn default() -> SubstanceReferenceInformationTargetAmount {
        SubstanceReferenceInformationTargetAmount::Invalid
    }
}
#[doc = "Todo."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceReferenceInformationGene {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Todo."]
    pub r#gene_sequence_origin: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#gene: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceReferenceInformationGene {
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
            if let Some(some) = self.r#gene_sequence_origin.as_ref() {
                state.serialize_entry("geneSequenceOrigin", some)?;
            }
            if let Some(some) = self.r#gene.as_ref() {
                state.serialize_entry("gene", some)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceReferenceInformationGene {
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
            #[serde(rename = "geneSequenceOrigin")]
            GeneSequenceOrigin,
            #[serde(rename = "gene")]
            Gene,
            #[serde(rename = "source")]
            Source,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceReferenceInformationGene;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceReferenceInformationGene")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceReferenceInformationGene, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#gene_sequence_origin: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#gene: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#source: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                            Field::GeneSequenceOrigin => {
                                if r#gene_sequence_origin.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "geneSequenceOrigin",
                                    ));
                                }
                                r#gene_sequence_origin = Some(map_access.next_value()?);
                            }
                            Field::Gene => {
                                if r#gene.is_some() {
                                    return Err(serde::de::Error::duplicate_field("gene"));
                                }
                                r#gene = Some(map_access.next_value()?);
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
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
                                        "geneSequenceOrigin",
                                        "gene",
                                        "source",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceReferenceInformationGene {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#gene_sequence_origin,
                        r#gene,
                        r#source: r#source.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Todo."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceReferenceInformationGeneElement {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Todo."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#element: Option<Box<super::super::types::Identifier>>,
    #[doc = "Todo."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceReferenceInformationGeneElement {
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
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#element.as_ref() {
                state.serialize_entry("element", some)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceReferenceInformationGeneElement {
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
            #[serde(rename = "element")]
            Element,
            #[serde(rename = "source")]
            Source,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceReferenceInformationGeneElement;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceReferenceInformationGeneElement")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceReferenceInformationGeneElement, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#element: Option<Box<super::super::types::Identifier>> = None;
                let mut r#source: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                            Field::Element => {
                                if r#element.is_some() {
                                    return Err(serde::de::Error::duplicate_field("element"));
                                }
                                r#element = Some(map_access.next_value()?);
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
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
                                        "element",
                                        "source",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceReferenceInformationGeneElement {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#element,
                        r#source: r#source.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Todo."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceReferenceInformationClassification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Todo."]
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#classification: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#subtype: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceReferenceInformationClassification {
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
            if let Some(some) = self.r#domain.as_ref() {
                state.serialize_entry("domain", some)?;
            }
            if let Some(some) = self.r#classification.as_ref() {
                state.serialize_entry("classification", some)?;
            }
            if !self.r#subtype.is_empty() {
                state.serialize_entry("subtype", &self.r#subtype)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceReferenceInformationClassification {
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
            #[serde(rename = "domain")]
            Domain,
            #[serde(rename = "classification")]
            Classification,
            #[serde(rename = "subtype")]
            Subtype,
            #[serde(rename = "source")]
            Source,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceReferenceInformationClassification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceReferenceInformationClassification")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceReferenceInformationClassification, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#domain: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#classification: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subtype: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#source: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                            Field::Domain => {
                                if r#domain.is_some() {
                                    return Err(serde::de::Error::duplicate_field("domain"));
                                }
                                r#domain = Some(map_access.next_value()?);
                            }
                            Field::Classification => {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                r#classification = Some(map_access.next_value()?);
                            }
                            Field::Subtype => {
                                if r#subtype.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subtype"));
                                }
                                r#subtype = Some(map_access.next_value()?);
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
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
                                        "domain",
                                        "classification",
                                        "subtype",
                                        "source",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceReferenceInformationClassification {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#domain,
                        r#classification,
                        r#subtype: r#subtype.unwrap_or(vec![]),
                        r#source: r#source.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Todo."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceReferenceInformationTarget {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Todo."]
    pub r#target: Option<Box<super::super::types::Identifier>>,
    #[doc = "Todo."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#interaction: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#organism: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#organism_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#amount: Option<SubstanceReferenceInformationTargetAmount>,
    #[doc = "Todo."]
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceReferenceInformationTarget {
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
            if let Some(some) = self.r#target.as_ref() {
                state.serialize_entry("target", some)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#interaction.as_ref() {
                state.serialize_entry("interaction", some)?;
            }
            if let Some(some) = self.r#organism.as_ref() {
                state.serialize_entry("organism", some)?;
            }
            if let Some(some) = self.r#organism_type.as_ref() {
                state.serialize_entry("organismType", some)?;
            }
            if let Some(some) = self.r#amount.as_ref() {
                match some {
                    SubstanceReferenceInformationTargetAmount::Quantity(ref value) => {
                        state.serialize_entry("amountQuantity", value)?;
                    }
                    SubstanceReferenceInformationTargetAmount::Range(ref value) => {
                        state.serialize_entry("amountRange", value)?;
                    }
                    SubstanceReferenceInformationTargetAmount::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("amountString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_amountString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("amountString", value)?;
                        }
                    }
                    SubstanceReferenceInformationTargetAmount::Invalid => {
                        return Err(serde::ser::Error::custom("amount is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#amount_type.as_ref() {
                state.serialize_entry("amountType", some)?;
            }
            if !self.r#source.is_empty() {
                state.serialize_entry("source", &self.r#source)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceReferenceInformationTarget {
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
            #[serde(rename = "target")]
            Target,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "interaction")]
            Interaction,
            #[serde(rename = "organism")]
            Organism,
            #[serde(rename = "organismType")]
            OrganismType,
            #[serde(rename = "amountQuantity")]
            AmountQuantity,
            #[serde(rename = "amountRange")]
            AmountRange,
            #[serde(rename = "amountString")]
            AmountString,
            #[serde(rename = "_amountString")]
            AmountStringPrimitiveElement,
            #[serde(rename = "amountType")]
            AmountType,
            #[serde(rename = "source")]
            Source,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceReferenceInformationTarget;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceReferenceInformationTarget")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceReferenceInformationTarget, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#target: Option<Box<super::super::types::Identifier>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#interaction: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#organism: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#organism_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#amount: Option<SubstanceReferenceInformationTargetAmount> = None;
                let mut r#amount_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#source: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                            Field::Target => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field("target"));
                                }
                                r#target = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Interaction => {
                                if r#interaction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("interaction"));
                                }
                                r#interaction = Some(map_access.next_value()?);
                            }
                            Field::Organism => {
                                if r#organism.is_some() {
                                    return Err(serde::de::Error::duplicate_field("organism"));
                                }
                                r#organism = Some(map_access.next_value()?);
                            }
                            Field::OrganismType => {
                                if r#organism_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("organismType"));
                                }
                                r#organism_type = Some(map_access.next_value()?);
                            }
                            Field::AmountQuantity => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "amountQuantity",
                                    ));
                                }
                                r#amount =
                                    Some(SubstanceReferenceInformationTargetAmount::Quantity(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::AmountRange => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountRange"));
                                }
                                r#amount = Some(SubstanceReferenceInformationTargetAmount::Range(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::AmountString => {
                                if _ctx.from_json {
                                    let r#enum = r#amount.get_or_insert(
                                        SubstanceReferenceInformationTargetAmount::String(
                                            Default::default(),
                                        ),
                                    );
                                    if let SubstanceReferenceInformationTargetAmount::String(
                                        variant,
                                    ) = r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "amountString",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("amount[x]"));
                                    }
                                } else {
                                    if r#amount.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "amountString",
                                        ));
                                    }
                                    r#amount =
                                        Some(SubstanceReferenceInformationTargetAmount::String(
                                            map_access.next_value()?,
                                        ));
                                }
                            }
                            Field::AmountStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#amount.get_or_insert(
                                        SubstanceReferenceInformationTargetAmount::String(
                                            Default::default(),
                                        ),
                                    );
                                    if let SubstanceReferenceInformationTargetAmount::String(
                                        variant,
                                    ) = r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_amountString",
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
                                            "_amount[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "amountString",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "target",
                                            "type",
                                            "interaction",
                                            "organism",
                                            "organismType",
                                            "amountQuantity",
                                            "amountRange",
                                            "amountString",
                                            "amountType",
                                            "source",
                                        ],
                                    ));
                                }
                            }
                            Field::AmountType => {
                                if r#amount_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountType"));
                                }
                                r#amount_type = Some(map_access.next_value()?);
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
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
                                        "target",
                                        "type",
                                        "interaction",
                                        "organism",
                                        "organismType",
                                        "amountQuantity",
                                        "amountRange",
                                        "amountString",
                                        "amountType",
                                        "source",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceReferenceInformationTarget {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#target,
                        r#type,
                        r#interaction,
                        r#organism,
                        r#organism_type,
                        r#amount,
                        r#amount_type,
                        r#source: r#source.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Todo."]
#[derive(Default, Debug, Clone)]
pub struct SubstanceReferenceInformation {
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
    #[doc = "Todo."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "Todo."]
    pub r#gene: Vec<SubstanceReferenceInformationGene>,
    #[doc = "Todo."]
    pub r#gene_element: Vec<SubstanceReferenceInformationGeneElement>,
    #[doc = "Todo."]
    pub r#classification: Vec<SubstanceReferenceInformationClassification>,
    #[doc = "Todo."]
    pub r#target: Vec<SubstanceReferenceInformationTarget>,
}
impl crate::AnyResource for SubstanceReferenceInformation {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for SubstanceReferenceInformation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "SubstanceReferenceInformation")?;
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
                if let Some(some) = self.r#comment.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("comment", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_comment", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#comment.as_ref() {
                    state.serialize_entry("comment", some)?;
                }
            }
            if !self.r#gene.is_empty() {
                state.serialize_entry("gene", &self.r#gene)?;
            }
            if !self.r#gene_element.is_empty() {
                state.serialize_entry("geneElement", &self.r#gene_element)?;
            }
            if !self.r#classification.is_empty() {
                state.serialize_entry("classification", &self.r#classification)?;
            }
            if !self.r#target.is_empty() {
                state.serialize_entry("target", &self.r#target)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceReferenceInformation {
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
            #[serde(rename = "comment")]
            Comment,
            #[serde(rename = "_comment")]
            CommentPrimitiveElement,
            #[serde(rename = "gene")]
            Gene,
            #[serde(rename = "geneElement")]
            GeneElement,
            #[serde(rename = "classification")]
            Classification,
            #[serde(rename = "target")]
            Target,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceReferenceInformation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceReferenceInformation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceReferenceInformation, V::Error>
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
                let mut r#comment: Option<super::super::types::String> = None;
                let mut r#gene: Option<Vec<SubstanceReferenceInformationGene>> = None;
                let mut r#gene_element: Option<Vec<SubstanceReferenceInformationGeneElement>> =
                    None;
                let mut r#classification: Option<Vec<SubstanceReferenceInformationClassification>> =
                    None;
                let mut r#target: Option<Vec<SubstanceReferenceInformationTarget>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "SubstanceReferenceInformation" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"SubstanceReferenceInformation",
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
                                            "comment",
                                            "gene",
                                            "geneElement",
                                            "classification",
                                            "target",
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
                                            "comment",
                                            "gene",
                                            "geneElement",
                                            "classification",
                                            "target",
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
                            Field::Comment => {
                                if _ctx.from_json {
                                    let some = r#comment.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#comment.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    r#comment = Some(map_access.next_value()?);
                                }
                            }
                            Field::CommentPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "comment",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "comment",
                                            "gene",
                                            "geneElement",
                                            "classification",
                                            "target",
                                        ],
                                    ));
                                }
                            }
                            Field::Gene => {
                                if r#gene.is_some() {
                                    return Err(serde::de::Error::duplicate_field("gene"));
                                }
                                r#gene = Some(map_access.next_value()?);
                            }
                            Field::GeneElement => {
                                if r#gene_element.is_some() {
                                    return Err(serde::de::Error::duplicate_field("geneElement"));
                                }
                                r#gene_element = Some(map_access.next_value()?);
                            }
                            Field::Classification => {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                r#classification = Some(map_access.next_value()?);
                            }
                            Field::Target => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field("target"));
                                }
                                r#target = Some(map_access.next_value()?);
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
                                        "comment",
                                        "gene",
                                        "geneElement",
                                        "classification",
                                        "target",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubstanceReferenceInformation {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#comment,
                        r#gene: r#gene.unwrap_or(vec![]),
                        r#gene_element: r#gene_element.unwrap_or(vec![]),
                        r#classification: r#classification.unwrap_or(vec![]),
                        r#target: r#target.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
