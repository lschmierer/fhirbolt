// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
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
#[derive(Default, Debug, Clone)]
pub struct SubstanceReferenceInformationGene {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#gene_sequence_origin: Option<Box<super::super::types::CodeableConcept>>,
    pub r#gene: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceReferenceInformationGene {
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceReferenceInformationGeneElement {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#element: Option<Box<super::super::types::Identifier>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceReferenceInformationGeneElement {
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceReferenceInformationClassification {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    pub r#classification: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subtype: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceReferenceInformationClassification {
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                                return Err(serde::de::Error::duplicate_field("classification"));
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceReferenceInformationTarget {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#target: Option<Box<super::super::types::Identifier>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#interaction: Option<Box<super::super::types::CodeableConcept>>,
    pub r#organism: Option<Box<super::super::types::CodeableConcept>>,
    pub r#organism_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Option<SubstanceReferenceInformationTargetAmount>,
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SubstanceReferenceInformationTarget {
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
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("amountString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_amountString", &primitive_element)?;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                                return Err(serde::de::Error::duplicate_field("amountQuantity"));
                            }
                            r#amount = Some(SubstanceReferenceInformationTargetAmount::Quantity(
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
                            let r#enum = r#amount.get_or_insert(
                                SubstanceReferenceInformationTargetAmount::String(
                                    Default::default(),
                                ),
                            );
                            if let SubstanceReferenceInformationTargetAmount::String(variant) =
                                r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("amount[x]"));
                            }
                        }
                        Field::AmountStringPrimitiveElement => {
                            let r#enum = r#amount.get_or_insert(
                                SubstanceReferenceInformationTargetAmount::String(
                                    Default::default(),
                                ),
                            );
                            if let SubstanceReferenceInformationTargetAmount::String(variant) =
                                r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_amountString"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_amount[x]"));
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceReferenceInformation {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#comment: Option<super::super::types::String>,
    pub r#gene: Vec<SubstanceReferenceInformationGene>,
    pub r#gene_element: Vec<SubstanceReferenceInformationGeneElement>,
    pub r#classification: Vec<SubstanceReferenceInformationClassification>,
    pub r#target: Vec<SubstanceReferenceInformationTarget>,
}
impl serde::ser::Serialize for SubstanceReferenceInformation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SubstanceReferenceInformation")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
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
                state.serialize_entry("language", some)?;
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
        if let Some(some) = self.r#comment.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("comment", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_comment", &primitive_element)?;
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
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
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
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
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
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
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
                            some.value = Some(map_access.next_value()?);
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Comment => {
                            let some = r#comment.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            some.value = Some(map_access.next_value()?);
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
                                return Err(serde::de::Error::duplicate_field("classification"));
                            }
                            r#classification = Some(map_access.next_value()?);
                        }
                        Field::Target => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            r#target = Some(map_access.next_value()?);
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
