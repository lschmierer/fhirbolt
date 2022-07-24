// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ElementDefinitionDefaultValue {
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
impl Default for ElementDefinitionDefaultValue {
    fn default() -> ElementDefinitionDefaultValue {
        ElementDefinitionDefaultValue::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ElementDefinitionFixed {
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
impl Default for ElementDefinitionFixed {
    fn default() -> ElementDefinitionFixed {
        ElementDefinitionFixed::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ElementDefinitionPattern {
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
impl Default for ElementDefinitionPattern {
    fn default() -> ElementDefinitionPattern {
        ElementDefinitionPattern::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ElementDefinitionExampleValue {
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
impl Default for ElementDefinitionExampleValue {
    fn default() -> ElementDefinitionExampleValue {
        ElementDefinitionExampleValue::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ElementDefinitionMinValue {
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Instant(Box<super::super::types::Instant>),
    Time(Box<super::super::types::Time>),
    Decimal(Box<super::super::types::Decimal>),
    Integer(Box<super::super::types::Integer>),
    PositiveInt(Box<super::super::types::PositiveInt>),
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for ElementDefinitionMinValue {
    fn default() -> ElementDefinitionMinValue {
        ElementDefinitionMinValue::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ElementDefinitionMaxValue {
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Instant(Box<super::super::types::Instant>),
    Time(Box<super::super::types::Time>),
    Decimal(Box<super::super::types::Decimal>),
    Integer(Box<super::super::types::Integer>),
    PositiveInt(Box<super::super::types::PositiveInt>),
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for ElementDefinitionMaxValue {
    fn default() -> ElementDefinitionMaxValue {
        ElementDefinitionMaxValue::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct ElementDefinitionSlicingDiscriminator {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#path: super::super::types::String,
}
impl serde::ser::Serialize for ElementDefinitionSlicingDiscriminator {
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
        if let Some(some) = self.r#type.value.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        if let Some(some) = self.r#path.value.as_ref() {
            state.serialize_entry("path", some)?;
        }
        if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#path.id,
                extension: &self.r#path.extension,
            };
            state.serialize_entry("_path", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ElementDefinitionSlicingDiscriminator {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "path")]
            Path,
            #[serde(rename = "_path")]
            PathPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ElementDefinitionSlicingDiscriminator;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionSlicingDiscriminator")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ElementDefinitionSlicingDiscriminator, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#path: Option<super::super::types::String> = None;
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
                        Field::Type => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
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
                        Field::Path => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PathPrimitiveElement => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_path"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ElementDefinitionSlicingDiscriminator {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#path: r#path.ok_or(serde::de::Error::missing_field("path"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ElementDefinitionSlicing {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#discriminator: Vec<ElementDefinitionSlicingDiscriminator>,
    pub r#description: Option<super::super::types::String>,
    pub r#ordered: Option<super::super::types::Boolean>,
    pub r#rules: super::super::types::Code,
}
impl serde::ser::Serialize for ElementDefinitionSlicing {
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
        if !self.r#discriminator.is_empty() {
            state.serialize_entry("discriminator", &self.r#discriminator)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#ordered.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("ordered", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_ordered", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#rules.value.as_ref() {
            state.serialize_entry("rules", some)?;
        }
        if self.r#rules.id.is_some() || !self.r#rules.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#rules.id,
                extension: &self.r#rules.extension,
            };
            state.serialize_entry("_rules", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ElementDefinitionSlicing {
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
            #[serde(rename = "discriminator")]
            Discriminator,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "ordered")]
            Ordered,
            #[serde(rename = "_ordered")]
            OrderedPrimitiveElement,
            #[serde(rename = "rules")]
            Rules,
            #[serde(rename = "_rules")]
            RulesPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ElementDefinitionSlicing;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionSlicing")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinitionSlicing, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#discriminator: Option<Vec<ElementDefinitionSlicingDiscriminator>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#ordered: Option<super::super::types::Boolean> = None;
                let mut r#rules: Option<super::super::types::Code> = None;
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
                        Field::Discriminator => {
                            if r#discriminator.is_some() {
                                return Err(serde::de::Error::duplicate_field("discriminator"));
                            }
                            r#discriminator = Some(map_access.next_value()?);
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
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
                        Field::Ordered => {
                            let some = r#ordered.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("ordered"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::OrderedPrimitiveElement => {
                            let some = r#ordered.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_ordered"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Rules => {
                            let some = r#rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RulesPrimitiveElement => {
                            let some = r#rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_rules"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ElementDefinitionSlicing {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#discriminator: r#discriminator.unwrap_or(vec![]),
                    r#description,
                    r#ordered,
                    r#rules: r#rules.ok_or(serde::de::Error::missing_field("rules"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ElementDefinitionBase {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: super::super::types::String,
    pub r#min: super::super::types::UnsignedInt,
    pub r#max: super::super::types::String,
}
impl serde::ser::Serialize for ElementDefinitionBase {
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
        if let Some(some) = self.r#path.value.as_ref() {
            state.serialize_entry("path", some)?;
        }
        if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#path.id,
                extension: &self.r#path.extension,
            };
            state.serialize_entry("_path", &primitive_element)?;
        }
        if let Some(some) = self.r#min.value.as_ref() {
            state.serialize_entry("min", some)?;
        }
        if self.r#min.id.is_some() || !self.r#min.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#min.id,
                extension: &self.r#min.extension,
            };
            state.serialize_entry("_min", &primitive_element)?;
        }
        if let Some(some) = self.r#max.value.as_ref() {
            state.serialize_entry("max", some)?;
        }
        if self.r#max.id.is_some() || !self.r#max.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#max.id,
                extension: &self.r#max.extension,
            };
            state.serialize_entry("_max", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ElementDefinitionBase {
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
            #[serde(rename = "path")]
            Path,
            #[serde(rename = "_path")]
            PathPrimitiveElement,
            #[serde(rename = "min")]
            Min,
            #[serde(rename = "_min")]
            MinPrimitiveElement,
            #[serde(rename = "max")]
            Max,
            #[serde(rename = "_max")]
            MaxPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ElementDefinitionBase;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionBase")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinitionBase, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#path: Option<super::super::types::String> = None;
                let mut r#min: Option<super::super::types::UnsignedInt> = None;
                let mut r#max: Option<super::super::types::String> = None;
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
                        Field::Path => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PathPrimitiveElement => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_path"));
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
                            some.value = Some(map_access.next_value()?);
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
                            some.value = Some(map_access.next_value()?);
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
                    }
                }
                Ok(ElementDefinitionBase {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#path: r#path.ok_or(serde::de::Error::missing_field("path"))?,
                    r#min: r#min.ok_or(serde::de::Error::missing_field("min"))?,
                    r#max: r#max.ok_or(serde::de::Error::missing_field("max"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ElementDefinitionType {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: super::super::types::Uri,
    pub r#profile: Vec<super::super::types::Canonical>,
    pub r#target_profile: Vec<super::super::types::Canonical>,
    pub r#aggregation: Vec<super::super::types::Code>,
    pub r#versioning: Option<super::super::types::Code>,
}
impl serde::ser::Serialize for ElementDefinitionType {
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
        if let Some(some) = self.r#code.value.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#code.id,
                extension: &self.r#code.extension,
            };
            state.serialize_entry("_code", &primitive_element)?;
        }
        if !self.r#profile.is_empty() {
            let values: Vec<_> = self.r#profile.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("profile", &values)?;
            }
            let requires_elements = self
                .r#profile
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#profile
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_profile", &primitive_elements)?;
            }
        }
        if !self.r#target_profile.is_empty() {
            let values: Vec<_> = self.r#target_profile.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("targetProfile", &values)?;
            }
            let requires_elements = self
                .r#target_profile
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#target_profile
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_targetProfile", &primitive_elements)?;
            }
        }
        if !self.r#aggregation.is_empty() {
            let values: Vec<_> = self.r#aggregation.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("aggregation", &values)?;
            }
            let requires_elements = self
                .r#aggregation
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#aggregation
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_aggregation", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#versioning.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("versioning", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_versioning", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ElementDefinitionType {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "_code")]
            CodePrimitiveElement,
            #[serde(rename = "profile")]
            Profile,
            #[serde(rename = "_profile")]
            ProfilePrimitiveElement,
            #[serde(rename = "targetProfile")]
            TargetProfile,
            #[serde(rename = "_targetProfile")]
            TargetProfilePrimitiveElement,
            #[serde(rename = "aggregation")]
            Aggregation,
            #[serde(rename = "_aggregation")]
            AggregationPrimitiveElement,
            #[serde(rename = "versioning")]
            Versioning,
            #[serde(rename = "_versioning")]
            VersioningPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ElementDefinitionType;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionType")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinitionType, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#code: Option<super::super::types::Uri> = None;
                let mut r#profile: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#target_profile: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#aggregation: Option<Vec<super::super::types::Code>> = None;
                let mut r#versioning: Option<super::super::types::Code> = None;
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
                        Field::Code => {
                            let some = r#code.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::CodePrimitiveElement => {
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
                        }
                        Field::Profile => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#profile.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::ProfilePrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
                            let vec = r#profile.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("_profile"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                if let Some(element) = element {
                                    vec[i].id = element.id;
                                    vec[i].extension = element.extension;
                                }
                            }
                        }
                        Field::TargetProfile => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#target_profile.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("targetProfile"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::TargetProfilePrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
                            let vec = r#target_profile.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("_targetProfile"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                if let Some(element) = element {
                                    vec[i].id = element.id;
                                    vec[i].extension = element.extension;
                                }
                            }
                        }
                        Field::Aggregation => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#aggregation.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("aggregation"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::AggregationPrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
                            let vec = r#aggregation.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("_aggregation"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                if let Some(element) = element {
                                    vec[i].id = element.id;
                                    vec[i].extension = element.extension;
                                }
                            }
                        }
                        Field::Versioning => {
                            let some = r#versioning.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("versioning"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::VersioningPrimitiveElement => {
                            let some = r#versioning.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_versioning"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ElementDefinitionType {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#profile: r#profile.unwrap_or(vec![]),
                    r#target_profile: r#target_profile.unwrap_or(vec![]),
                    r#aggregation: r#aggregation.unwrap_or(vec![]),
                    r#versioning,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ElementDefinitionExample {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#label: super::super::types::String,
    pub r#value: ElementDefinitionExampleValue,
}
impl serde::ser::Serialize for ElementDefinitionExample {
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
        if let Some(some) = self.r#label.value.as_ref() {
            state.serialize_entry("label", some)?;
        }
        if self.r#label.id.is_some() || !self.r#label.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#label.id,
                extension: &self.r#label.extension,
            };
            state.serialize_entry("_label", &primitive_element)?;
        }
        match self.r#value {
            ElementDefinitionExampleValue::Base64Binary(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueBase64Binary", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueBase64Binary", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Boolean(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueBoolean", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueBoolean", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Canonical(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueCanonical", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueCanonical", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Code(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueCode", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueCode", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Date(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueDate", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueDate", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::DateTime(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueDateTime", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueDateTime", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Decimal(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueDecimal", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueDecimal", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Id(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueId", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueId", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Instant(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueInstant", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueInstant", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Integer(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueInteger", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueInteger", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Markdown(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueMarkdown", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueMarkdown", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Oid(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueOid", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueOid", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::PositiveInt(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valuePositiveInt", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valuePositiveInt", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::String(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueString", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueString", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Time(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueTime", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueTime", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::UnsignedInt(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueUnsignedInt", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueUnsignedInt", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Uri(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueUri", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueUri", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Url(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueUrl", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueUrl", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Uuid(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueUuid", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueUuid", &primitive_element)?;
                }
            }
            ElementDefinitionExampleValue::Address(ref value) => {
                state.serialize_entry("valueAddress", value)?;
            }
            ElementDefinitionExampleValue::Age(ref value) => {
                state.serialize_entry("valueAge", value)?;
            }
            ElementDefinitionExampleValue::Annotation(ref value) => {
                state.serialize_entry("valueAnnotation", value)?;
            }
            ElementDefinitionExampleValue::Attachment(ref value) => {
                state.serialize_entry("valueAttachment", value)?;
            }
            ElementDefinitionExampleValue::CodeableConcept(ref value) => {
                state.serialize_entry("valueCodeableConcept", value)?;
            }
            ElementDefinitionExampleValue::Coding(ref value) => {
                state.serialize_entry("valueCoding", value)?;
            }
            ElementDefinitionExampleValue::ContactPoint(ref value) => {
                state.serialize_entry("valueContactPoint", value)?;
            }
            ElementDefinitionExampleValue::Count(ref value) => {
                state.serialize_entry("valueCount", value)?;
            }
            ElementDefinitionExampleValue::Distance(ref value) => {
                state.serialize_entry("valueDistance", value)?;
            }
            ElementDefinitionExampleValue::Duration(ref value) => {
                state.serialize_entry("valueDuration", value)?;
            }
            ElementDefinitionExampleValue::HumanName(ref value) => {
                state.serialize_entry("valueHumanName", value)?;
            }
            ElementDefinitionExampleValue::Identifier(ref value) => {
                state.serialize_entry("valueIdentifier", value)?;
            }
            ElementDefinitionExampleValue::Money(ref value) => {
                state.serialize_entry("valueMoney", value)?;
            }
            ElementDefinitionExampleValue::Period(ref value) => {
                state.serialize_entry("valuePeriod", value)?;
            }
            ElementDefinitionExampleValue::Quantity(ref value) => {
                state.serialize_entry("valueQuantity", value)?;
            }
            ElementDefinitionExampleValue::Range(ref value) => {
                state.serialize_entry("valueRange", value)?;
            }
            ElementDefinitionExampleValue::Ratio(ref value) => {
                state.serialize_entry("valueRatio", value)?;
            }
            ElementDefinitionExampleValue::Reference(ref value) => {
                state.serialize_entry("valueReference", value)?;
            }
            ElementDefinitionExampleValue::SampledData(ref value) => {
                state.serialize_entry("valueSampledData", value)?;
            }
            ElementDefinitionExampleValue::Signature(ref value) => {
                state.serialize_entry("valueSignature", value)?;
            }
            ElementDefinitionExampleValue::Timing(ref value) => {
                state.serialize_entry("valueTiming", value)?;
            }
            ElementDefinitionExampleValue::ContactDetail(ref value) => {
                state.serialize_entry("valueContactDetail", value)?;
            }
            ElementDefinitionExampleValue::Contributor(ref value) => {
                state.serialize_entry("valueContributor", value)?;
            }
            ElementDefinitionExampleValue::DataRequirement(ref value) => {
                state.serialize_entry("valueDataRequirement", value)?;
            }
            ElementDefinitionExampleValue::Expression(ref value) => {
                state.serialize_entry("valueExpression", value)?;
            }
            ElementDefinitionExampleValue::ParameterDefinition(ref value) => {
                state.serialize_entry("valueParameterDefinition", value)?;
            }
            ElementDefinitionExampleValue::RelatedArtifact(ref value) => {
                state.serialize_entry("valueRelatedArtifact", value)?;
            }
            ElementDefinitionExampleValue::TriggerDefinition(ref value) => {
                state.serialize_entry("valueTriggerDefinition", value)?;
            }
            ElementDefinitionExampleValue::UsageContext(ref value) => {
                state.serialize_entry("valueUsageContext", value)?;
            }
            ElementDefinitionExampleValue::Dosage(ref value) => {
                state.serialize_entry("valueDosage", value)?;
            }
            ElementDefinitionExampleValue::Meta(ref value) => {
                state.serialize_entry("valueMeta", value)?;
            }
            ElementDefinitionExampleValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ElementDefinitionExample {
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
            #[serde(rename = "label")]
            Label,
            #[serde(rename = "_label")]
            LabelPrimitiveElement,
            #[serde(rename = "valueBase64Binary")]
            ValueBase64Binary,
            #[serde(rename = "_valueBase64Binary")]
            ValueBase64BinaryPrimitiveElement,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueCanonical")]
            ValueCanonical,
            #[serde(rename = "_valueCanonical")]
            ValueCanonicalPrimitiveElement,
            #[serde(rename = "valueCode")]
            ValueCode,
            #[serde(rename = "_valueCode")]
            ValueCodePrimitiveElement,
            #[serde(rename = "valueDate")]
            ValueDate,
            #[serde(rename = "_valueDate")]
            ValueDatePrimitiveElement,
            #[serde(rename = "valueDateTime")]
            ValueDateTime,
            #[serde(rename = "_valueDateTime")]
            ValueDateTimePrimitiveElement,
            #[serde(rename = "valueDecimal")]
            ValueDecimal,
            #[serde(rename = "_valueDecimal")]
            ValueDecimalPrimitiveElement,
            #[serde(rename = "valueId")]
            ValueId,
            #[serde(rename = "_valueId")]
            ValueIdPrimitiveElement,
            #[serde(rename = "valueInstant")]
            ValueInstant,
            #[serde(rename = "_valueInstant")]
            ValueInstantPrimitiveElement,
            #[serde(rename = "valueInteger")]
            ValueInteger,
            #[serde(rename = "_valueInteger")]
            ValueIntegerPrimitiveElement,
            #[serde(rename = "valueMarkdown")]
            ValueMarkdown,
            #[serde(rename = "_valueMarkdown")]
            ValueMarkdownPrimitiveElement,
            #[serde(rename = "valueOid")]
            ValueOid,
            #[serde(rename = "_valueOid")]
            ValueOidPrimitiveElement,
            #[serde(rename = "valuePositiveInt")]
            ValuePositiveInt,
            #[serde(rename = "_valuePositiveInt")]
            ValuePositiveIntPrimitiveElement,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueTime")]
            ValueTime,
            #[serde(rename = "_valueTime")]
            ValueTimePrimitiveElement,
            #[serde(rename = "valueUnsignedInt")]
            ValueUnsignedInt,
            #[serde(rename = "_valueUnsignedInt")]
            ValueUnsignedIntPrimitiveElement,
            #[serde(rename = "valueUri")]
            ValueUri,
            #[serde(rename = "_valueUri")]
            ValueUriPrimitiveElement,
            #[serde(rename = "valueUrl")]
            ValueUrl,
            #[serde(rename = "_valueUrl")]
            ValueUrlPrimitiveElement,
            #[serde(rename = "valueUuid")]
            ValueUuid,
            #[serde(rename = "_valueUuid")]
            ValueUuidPrimitiveElement,
            #[serde(rename = "valueAddress")]
            ValueAddress,
            #[serde(rename = "valueAge")]
            ValueAge,
            #[serde(rename = "valueAnnotation")]
            ValueAnnotation,
            #[serde(rename = "valueAttachment")]
            ValueAttachment,
            #[serde(rename = "valueCodeableConcept")]
            ValueCodeableConcept,
            #[serde(rename = "valueCoding")]
            ValueCoding,
            #[serde(rename = "valueContactPoint")]
            ValueContactPoint,
            #[serde(rename = "valueCount")]
            ValueCount,
            #[serde(rename = "valueDistance")]
            ValueDistance,
            #[serde(rename = "valueDuration")]
            ValueDuration,
            #[serde(rename = "valueHumanName")]
            ValueHumanName,
            #[serde(rename = "valueIdentifier")]
            ValueIdentifier,
            #[serde(rename = "valueMoney")]
            ValueMoney,
            #[serde(rename = "valuePeriod")]
            ValuePeriod,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueRange")]
            ValueRange,
            #[serde(rename = "valueRatio")]
            ValueRatio,
            #[serde(rename = "valueReference")]
            ValueReference,
            #[serde(rename = "valueSampledData")]
            ValueSampledData,
            #[serde(rename = "valueSignature")]
            ValueSignature,
            #[serde(rename = "valueTiming")]
            ValueTiming,
            #[serde(rename = "valueContactDetail")]
            ValueContactDetail,
            #[serde(rename = "valueContributor")]
            ValueContributor,
            #[serde(rename = "valueDataRequirement")]
            ValueDataRequirement,
            #[serde(rename = "valueExpression")]
            ValueExpression,
            #[serde(rename = "valueParameterDefinition")]
            ValueParameterDefinition,
            #[serde(rename = "valueRelatedArtifact")]
            ValueRelatedArtifact,
            #[serde(rename = "valueTriggerDefinition")]
            ValueTriggerDefinition,
            #[serde(rename = "valueUsageContext")]
            ValueUsageContext,
            #[serde(rename = "valueDosage")]
            ValueDosage,
            #[serde(rename = "valueMeta")]
            ValueMeta,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ElementDefinitionExample;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionExample")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinitionExample, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#label: Option<super::super::types::String> = None;
                let mut r#value: Option<ElementDefinitionExampleValue> = None;
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
                        Field::Label => {
                            let some = r#label.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LabelPrimitiveElement => {
                            let some = r#label.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_label"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ValueBase64Binary => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Base64Binary(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Base64Binary(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueBase64BinaryPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Base64Binary(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Base64Binary(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueBase64Binary",
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
                                ElementDefinitionExampleValue::Boolean(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Boolean(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueBoolean"));
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
                        Field::ValueCanonical => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Canonical(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Canonical(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCanonical",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueCanonicalPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Canonical(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Canonical(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueCanonical",
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
                        Field::ValueCode => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Code(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Code(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCode"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueCodePrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Code(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Code(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueCode"));
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
                        Field::ValueDate => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Date(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueDatePrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Date(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueDate"));
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
                        Field::ValueDateTime => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::DateTime(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueDateTimePrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::DateTime(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::DateTime(variant) = r#enum {
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
                        }
                        Field::ValueDecimal => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Decimal(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Decimal(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDecimal"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueDecimalPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Decimal(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Decimal(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueDecimal"));
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
                        Field::ValueId => {
                            let r#enum = r#value.get_or_insert(ElementDefinitionExampleValue::Id(
                                Default::default(),
                            ));
                            if let ElementDefinitionExampleValue::Id(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueId"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueIdPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(ElementDefinitionExampleValue::Id(
                                Default::default(),
                            ));
                            if let ElementDefinitionExampleValue::Id(variant) = r#enum {
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
                        Field::ValueInstant => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Instant(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Instant(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInstant"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueInstantPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Instant(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Instant(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueInstant"));
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
                                ElementDefinitionExampleValue::Integer(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Integer(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Integer(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Integer(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueInteger"));
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
                        Field::ValueMarkdown => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Markdown(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Markdown(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMarkdown"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueMarkdownPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Markdown(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Markdown(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueMarkdown",
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
                        Field::ValueOid => {
                            let r#enum = r#value.get_or_insert(ElementDefinitionExampleValue::Oid(
                                Default::default(),
                            ));
                            if let ElementDefinitionExampleValue::Oid(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueOid"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueOidPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(ElementDefinitionExampleValue::Oid(
                                Default::default(),
                            ));
                            if let ElementDefinitionExampleValue::Oid(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueOid"));
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
                        Field::ValuePositiveInt => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::PositiveInt(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::PositiveInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valuePositiveInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValuePositiveIntPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::PositiveInt(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::PositiveInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valuePositiveInt",
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
                        Field::ValueString => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::String(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::String(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueString"));
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
                        Field::ValueTime => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Time(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Time(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueTimePrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Time(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Time(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueTime"));
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
                        Field::ValueUnsignedInt => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::UnsignedInt(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::UnsignedInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueUnsignedInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueUnsignedIntPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::UnsignedInt(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::UnsignedInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueUnsignedInt",
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
                        Field::ValueUri => {
                            let r#enum = r#value.get_or_insert(ElementDefinitionExampleValue::Uri(
                                Default::default(),
                            ));
                            if let ElementDefinitionExampleValue::Uri(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUri"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueUriPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(ElementDefinitionExampleValue::Uri(
                                Default::default(),
                            ));
                            if let ElementDefinitionExampleValue::Uri(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueUri"));
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
                        Field::ValueUrl => {
                            let r#enum = r#value.get_or_insert(ElementDefinitionExampleValue::Url(
                                Default::default(),
                            ));
                            if let ElementDefinitionExampleValue::Url(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUrl"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueUrlPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(ElementDefinitionExampleValue::Url(
                                Default::default(),
                            ));
                            if let ElementDefinitionExampleValue::Url(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueUrl"));
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
                        Field::ValueUuid => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Uuid(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Uuid(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUuid"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueUuidPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                ElementDefinitionExampleValue::Uuid(Default::default()),
                            );
                            if let ElementDefinitionExampleValue::Uuid(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueUuid"));
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
                        Field::ValueAddress => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAddress"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Address(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueAge => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAge"));
                            }
                            r#value =
                                Some(ElementDefinitionExampleValue::Age(map_access.next_value()?));
                        }
                        Field::ValueAnnotation => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAnnotation"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Annotation(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueAttachment => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Attachment(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some(ElementDefinitionExampleValue::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueCoding => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCoding"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Coding(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueContactPoint => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContactPoint"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::ContactPoint(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueCount => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCount"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Count(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueDistance => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDistance"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Distance(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueDuration => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDuration"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Duration(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueHumanName => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueHumanName"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::HumanName(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueIdentifier => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueIdentifier"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Identifier(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueMoney => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMoney"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Money(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValuePeriod => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Period(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueRange => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Range(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueRatio => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatio"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Ratio(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueReference => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueSampledData => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSampledData"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::SampledData(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueSignature => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSignature"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Signature(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueTiming => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueTiming"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Timing(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueContactDetail => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueContactDetail",
                                ));
                            }
                            r#value = Some(ElementDefinitionExampleValue::ContactDetail(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueContributor => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContributor"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Contributor(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueDataRequirement => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueDataRequirement",
                                ));
                            }
                            r#value = Some(ElementDefinitionExampleValue::DataRequirement(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueExpression => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueExpression"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Expression(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueParameterDefinition => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueParameterDefinition",
                                ));
                            }
                            r#value = Some(ElementDefinitionExampleValue::ParameterDefinition(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueRelatedArtifact => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueRelatedArtifact",
                                ));
                            }
                            r#value = Some(ElementDefinitionExampleValue::RelatedArtifact(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueTriggerDefinition => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueTriggerDefinition",
                                ));
                            }
                            r#value = Some(ElementDefinitionExampleValue::TriggerDefinition(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueUsageContext => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueUsageContext"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::UsageContext(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueDosage => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDosage"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Dosage(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueMeta => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMeta"));
                            }
                            r#value = Some(ElementDefinitionExampleValue::Meta(
                                map_access.next_value()?,
                            ));
                        }
                    }
                }
                Ok(ElementDefinitionExample {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#label: r#label.ok_or(serde::de::Error::missing_field("label"))?,
                    r#value: r#value.ok_or(serde::de::Error::missing_field("value[x]"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ElementDefinitionConstraint {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#key: super::super::types::Id,
    pub r#requirements: Option<super::super::types::String>,
    pub r#severity: super::super::types::Code,
    pub r#human: super::super::types::String,
    pub r#expression: Option<super::super::types::String>,
    pub r#xpath: Option<super::super::types::String>,
    pub r#source: Option<super::super::types::Canonical>,
}
impl serde::ser::Serialize for ElementDefinitionConstraint {
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
        if let Some(some) = self.r#key.value.as_ref() {
            state.serialize_entry("key", some)?;
        }
        if self.r#key.id.is_some() || !self.r#key.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#key.id,
                extension: &self.r#key.extension,
            };
            state.serialize_entry("_key", &primitive_element)?;
        }
        if let Some(some) = self.r#requirements.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("requirements", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_requirements", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#severity.value.as_ref() {
            state.serialize_entry("severity", some)?;
        }
        if self.r#severity.id.is_some() || !self.r#severity.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#severity.id,
                extension: &self.r#severity.extension,
            };
            state.serialize_entry("_severity", &primitive_element)?;
        }
        if let Some(some) = self.r#human.value.as_ref() {
            state.serialize_entry("human", some)?;
        }
        if self.r#human.id.is_some() || !self.r#human.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#human.id,
                extension: &self.r#human.extension,
            };
            state.serialize_entry("_human", &primitive_element)?;
        }
        if let Some(some) = self.r#expression.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("expression", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_expression", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#xpath.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("xpath", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_xpath", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#source.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("source", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_source", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ElementDefinitionConstraint {
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
            #[serde(rename = "key")]
            Key,
            #[serde(rename = "_key")]
            KeyPrimitiveElement,
            #[serde(rename = "requirements")]
            Requirements,
            #[serde(rename = "_requirements")]
            RequirementsPrimitiveElement,
            #[serde(rename = "severity")]
            Severity,
            #[serde(rename = "_severity")]
            SeverityPrimitiveElement,
            #[serde(rename = "human")]
            Human,
            #[serde(rename = "_human")]
            HumanPrimitiveElement,
            #[serde(rename = "expression")]
            Expression,
            #[serde(rename = "_expression")]
            ExpressionPrimitiveElement,
            #[serde(rename = "xpath")]
            Xpath,
            #[serde(rename = "_xpath")]
            XpathPrimitiveElement,
            #[serde(rename = "source")]
            Source,
            #[serde(rename = "_source")]
            SourcePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ElementDefinitionConstraint;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionConstraint")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ElementDefinitionConstraint, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#key: Option<super::super::types::Id> = None;
                let mut r#requirements: Option<super::super::types::String> = None;
                let mut r#severity: Option<super::super::types::Code> = None;
                let mut r#human: Option<super::super::types::String> = None;
                let mut r#expression: Option<super::super::types::String> = None;
                let mut r#xpath: Option<super::super::types::String> = None;
                let mut r#source: Option<super::super::types::Canonical> = None;
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
                        Field::Key => {
                            let some = r#key.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::KeyPrimitiveElement => {
                            let some = r#key.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_key"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Requirements => {
                            let some = r#requirements.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirements"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RequirementsPrimitiveElement => {
                            let some = r#requirements.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_requirements"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Severity => {
                            let some = r#severity.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("severity"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SeverityPrimitiveElement => {
                            let some = r#severity.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_severity"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Human => {
                            let some = r#human.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("human"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::HumanPrimitiveElement => {
                            let some = r#human.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_human"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Expression => {
                            let some = r#expression.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ExpressionPrimitiveElement => {
                            let some = r#expression.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_expression"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Xpath => {
                            let some = r#xpath.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("xpath"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::XpathPrimitiveElement => {
                            let some = r#xpath.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_xpath"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Source => {
                            let some = r#source.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SourcePrimitiveElement => {
                            let some = r#source.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ElementDefinitionConstraint {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#key: r#key.ok_or(serde::de::Error::missing_field("key"))?,
                    r#requirements,
                    r#severity: r#severity.ok_or(serde::de::Error::missing_field("severity"))?,
                    r#human: r#human.ok_or(serde::de::Error::missing_field("human"))?,
                    r#expression,
                    r#xpath,
                    r#source,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ElementDefinitionBinding {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#strength: super::super::types::Code,
    pub r#description: Option<super::super::types::String>,
    pub r#value_set: Option<super::super::types::Canonical>,
}
impl serde::ser::Serialize for ElementDefinitionBinding {
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
        if let Some(some) = self.r#strength.value.as_ref() {
            state.serialize_entry("strength", some)?;
        }
        if self.r#strength.id.is_some() || !self.r#strength.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#strength.id,
                extension: &self.r#strength.extension,
            };
            state.serialize_entry("_strength", &primitive_element)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#value_set.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("valueSet", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_valueSet", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ElementDefinitionBinding {
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
            #[serde(rename = "strength")]
            Strength,
            #[serde(rename = "_strength")]
            StrengthPrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "valueSet")]
            ValueSet,
            #[serde(rename = "_valueSet")]
            ValueSetPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ElementDefinitionBinding;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionBinding")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinitionBinding, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#strength: Option<super::super::types::Code> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#value_set: Option<super::super::types::Canonical> = None;
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
                        Field::Strength => {
                            let some = r#strength.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("strength"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::StrengthPrimitiveElement => {
                            let some = r#strength.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_strength"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
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
                        Field::ValueSet => {
                            let some = r#value_set.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSet"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ValueSetPrimitiveElement => {
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
                        }
                    }
                }
                Ok(ElementDefinitionBinding {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#strength: r#strength.ok_or(serde::de::Error::missing_field("strength"))?,
                    r#description,
                    r#value_set,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ElementDefinitionMapping {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identity: super::super::types::Id,
    pub r#language: Option<super::super::types::Code>,
    pub r#map: super::super::types::String,
    pub r#comment: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ElementDefinitionMapping {
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
        if let Some(some) = self.r#identity.value.as_ref() {
            state.serialize_entry("identity", some)?;
        }
        if self.r#identity.id.is_some() || !self.r#identity.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#identity.id,
                extension: &self.r#identity.extension,
            };
            state.serialize_entry("_identity", &primitive_element)?;
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
        if let Some(some) = self.r#map.value.as_ref() {
            state.serialize_entry("map", some)?;
        }
        if self.r#map.id.is_some() || !self.r#map.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#map.id,
                extension: &self.r#map.extension,
            };
            state.serialize_entry("_map", &primitive_element)?;
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
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ElementDefinitionMapping {
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
            #[serde(rename = "identity")]
            Identity,
            #[serde(rename = "_identity")]
            IdentityPrimitiveElement,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "_language")]
            LanguagePrimitiveElement,
            #[serde(rename = "map")]
            Map,
            #[serde(rename = "_map")]
            MapPrimitiveElement,
            #[serde(rename = "comment")]
            Comment,
            #[serde(rename = "_comment")]
            CommentPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ElementDefinitionMapping;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionMapping")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinitionMapping, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#identity: Option<super::super::types::Id> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#map: Option<super::super::types::String> = None;
                let mut r#comment: Option<super::super::types::String> = None;
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
                        Field::Identity => {
                            let some = r#identity.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("identity"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IdentityPrimitiveElement => {
                            let some = r#identity.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_identity"));
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
                        Field::Map => {
                            let some = r#map.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("map"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::MapPrimitiveElement => {
                            let some = r#map.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_map"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
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
                    }
                }
                Ok(ElementDefinitionMapping {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#identity: r#identity.ok_or(serde::de::Error::missing_field("identity"))?,
                    r#language,
                    r#map: r#map.ok_or(serde::de::Error::missing_field("map"))?,
                    r#comment,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ElementDefinition {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: super::super::types::String,
    pub r#representation: Vec<super::super::types::Code>,
    pub r#slice_name: Option<super::super::types::String>,
    pub r#slice_is_constraining: Option<super::super::types::Boolean>,
    pub r#label: Option<super::super::types::String>,
    pub r#code: Vec<Box<super::super::types::Coding>>,
    pub r#slicing: Option<ElementDefinitionSlicing>,
    pub r#short: Option<super::super::types::String>,
    pub r#definition: Option<super::super::types::Markdown>,
    pub r#comment: Option<super::super::types::Markdown>,
    pub r#requirements: Option<super::super::types::Markdown>,
    pub r#alias: Vec<super::super::types::String>,
    pub r#min: Option<super::super::types::UnsignedInt>,
    pub r#max: Option<super::super::types::String>,
    pub r#base: Option<ElementDefinitionBase>,
    pub r#content_reference: Option<super::super::types::Uri>,
    pub r#type: Vec<ElementDefinitionType>,
    pub r#default_value: Option<ElementDefinitionDefaultValue>,
    pub r#meaning_when_missing: Option<super::super::types::Markdown>,
    pub r#order_meaning: Option<super::super::types::String>,
    pub r#fixed: Option<ElementDefinitionFixed>,
    pub r#pattern: Option<ElementDefinitionPattern>,
    pub r#example: Vec<ElementDefinitionExample>,
    pub r#min_value: Option<ElementDefinitionMinValue>,
    pub r#max_value: Option<ElementDefinitionMaxValue>,
    pub r#max_length: Option<super::super::types::Integer>,
    pub r#condition: Vec<super::super::types::Id>,
    pub r#constraint: Vec<ElementDefinitionConstraint>,
    pub r#must_support: Option<super::super::types::Boolean>,
    pub r#is_modifier: Option<super::super::types::Boolean>,
    pub r#is_modifier_reason: Option<super::super::types::String>,
    pub r#is_summary: Option<super::super::types::Boolean>,
    pub r#binding: Option<ElementDefinitionBinding>,
    pub r#mapping: Vec<ElementDefinitionMapping>,
}
impl serde::ser::Serialize for ElementDefinition {
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
        if let Some(some) = self.r#path.value.as_ref() {
            state.serialize_entry("path", some)?;
        }
        if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#path.id,
                extension: &self.r#path.extension,
            };
            state.serialize_entry("_path", &primitive_element)?;
        }
        if !self.r#representation.is_empty() {
            let values: Vec<_> = self.r#representation.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("representation", &values)?;
            }
            let requires_elements = self
                .r#representation
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#representation
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_representation", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#slice_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("sliceName", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_sliceName", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#slice_is_constraining.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("sliceIsConstraining", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_sliceIsConstraining", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#label.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("label", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_label", &primitive_element)?;
            }
        }
        if !self.r#code.is_empty() {
            state.serialize_entry("code", &self.r#code)?;
        }
        if let Some(some) = self.r#slicing.as_ref() {
            state.serialize_entry("slicing", some)?;
        }
        if let Some(some) = self.r#short.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("short", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_short", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#definition.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("definition", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_definition", &primitive_element)?;
            }
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
        if let Some(some) = self.r#requirements.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("requirements", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_requirements", &primitive_element)?;
            }
        }
        if !self.r#alias.is_empty() {
            let values: Vec<_> = self.r#alias.iter().map(|v| &v.value).collect();
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
                                id: &e.id,
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
        if let Some(some) = self.r#min.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("min", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_min", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#max.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("max", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_max", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#base.as_ref() {
            state.serialize_entry("base", some)?;
        }
        if let Some(some) = self.r#content_reference.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("contentReference", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_contentReference", &primitive_element)?;
            }
        }
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
        }
        if let Some(some) = self.r#default_value.as_ref() {
            match some {
                ElementDefinitionDefaultValue::Base64Binary(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueBase64Binary", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueBase64Binary", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueBoolean", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Canonical(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueCanonical", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueCanonical", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Code(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueCode", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueCode", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueDate", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueDateTime", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Decimal(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueDecimal", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueDecimal", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Id(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueId", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueId", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Instant(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueInstant", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueInstant", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Integer(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueInteger", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueInteger", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Markdown(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueMarkdown", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueMarkdown", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Oid(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueOid", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueOid", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValuePositiveInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValuePositiveInt", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueString", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Time(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueTime", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::UnsignedInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueUnsignedInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueUnsignedInt", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Uri(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueUri", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueUri", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Url(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueUrl", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueUrl", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Uuid(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueUuid", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_defaultValueUuid", &primitive_element)?;
                    }
                }
                ElementDefinitionDefaultValue::Address(ref value) => {
                    state.serialize_entry("defaultValueAddress", value)?;
                }
                ElementDefinitionDefaultValue::Age(ref value) => {
                    state.serialize_entry("defaultValueAge", value)?;
                }
                ElementDefinitionDefaultValue::Annotation(ref value) => {
                    state.serialize_entry("defaultValueAnnotation", value)?;
                }
                ElementDefinitionDefaultValue::Attachment(ref value) => {
                    state.serialize_entry("defaultValueAttachment", value)?;
                }
                ElementDefinitionDefaultValue::CodeableConcept(ref value) => {
                    state.serialize_entry("defaultValueCodeableConcept", value)?;
                }
                ElementDefinitionDefaultValue::Coding(ref value) => {
                    state.serialize_entry("defaultValueCoding", value)?;
                }
                ElementDefinitionDefaultValue::ContactPoint(ref value) => {
                    state.serialize_entry("defaultValueContactPoint", value)?;
                }
                ElementDefinitionDefaultValue::Count(ref value) => {
                    state.serialize_entry("defaultValueCount", value)?;
                }
                ElementDefinitionDefaultValue::Distance(ref value) => {
                    state.serialize_entry("defaultValueDistance", value)?;
                }
                ElementDefinitionDefaultValue::Duration(ref value) => {
                    state.serialize_entry("defaultValueDuration", value)?;
                }
                ElementDefinitionDefaultValue::HumanName(ref value) => {
                    state.serialize_entry("defaultValueHumanName", value)?;
                }
                ElementDefinitionDefaultValue::Identifier(ref value) => {
                    state.serialize_entry("defaultValueIdentifier", value)?;
                }
                ElementDefinitionDefaultValue::Money(ref value) => {
                    state.serialize_entry("defaultValueMoney", value)?;
                }
                ElementDefinitionDefaultValue::Period(ref value) => {
                    state.serialize_entry("defaultValuePeriod", value)?;
                }
                ElementDefinitionDefaultValue::Quantity(ref value) => {
                    state.serialize_entry("defaultValueQuantity", value)?;
                }
                ElementDefinitionDefaultValue::Range(ref value) => {
                    state.serialize_entry("defaultValueRange", value)?;
                }
                ElementDefinitionDefaultValue::Ratio(ref value) => {
                    state.serialize_entry("defaultValueRatio", value)?;
                }
                ElementDefinitionDefaultValue::Reference(ref value) => {
                    state.serialize_entry("defaultValueReference", value)?;
                }
                ElementDefinitionDefaultValue::SampledData(ref value) => {
                    state.serialize_entry("defaultValueSampledData", value)?;
                }
                ElementDefinitionDefaultValue::Signature(ref value) => {
                    state.serialize_entry("defaultValueSignature", value)?;
                }
                ElementDefinitionDefaultValue::Timing(ref value) => {
                    state.serialize_entry("defaultValueTiming", value)?;
                }
                ElementDefinitionDefaultValue::ContactDetail(ref value) => {
                    state.serialize_entry("defaultValueContactDetail", value)?;
                }
                ElementDefinitionDefaultValue::Contributor(ref value) => {
                    state.serialize_entry("defaultValueContributor", value)?;
                }
                ElementDefinitionDefaultValue::DataRequirement(ref value) => {
                    state.serialize_entry("defaultValueDataRequirement", value)?;
                }
                ElementDefinitionDefaultValue::Expression(ref value) => {
                    state.serialize_entry("defaultValueExpression", value)?;
                }
                ElementDefinitionDefaultValue::ParameterDefinition(ref value) => {
                    state.serialize_entry("defaultValueParameterDefinition", value)?;
                }
                ElementDefinitionDefaultValue::RelatedArtifact(ref value) => {
                    state.serialize_entry("defaultValueRelatedArtifact", value)?;
                }
                ElementDefinitionDefaultValue::TriggerDefinition(ref value) => {
                    state.serialize_entry("defaultValueTriggerDefinition", value)?;
                }
                ElementDefinitionDefaultValue::UsageContext(ref value) => {
                    state.serialize_entry("defaultValueUsageContext", value)?;
                }
                ElementDefinitionDefaultValue::Dosage(ref value) => {
                    state.serialize_entry("defaultValueDosage", value)?;
                }
                ElementDefinitionDefaultValue::Meta(ref value) => {
                    state.serialize_entry("defaultValueMeta", value)?;
                }
                ElementDefinitionDefaultValue::Invalid => {
                    return Err(serde::ser::Error::custom("default_value is invalid"))
                }
            }
        }
        if let Some(some) = self.r#meaning_when_missing.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("meaningWhenMissing", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_meaningWhenMissing", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#order_meaning.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("orderMeaning", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_orderMeaning", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#fixed.as_ref() {
            match some {
                ElementDefinitionFixed::Base64Binary(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedBase64Binary", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedBase64Binary", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedBoolean", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Canonical(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedCanonical", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedCanonical", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Code(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedCode", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedCode", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedDate", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedDateTime", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Decimal(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedDecimal", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedDecimal", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Id(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedId", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedId", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Instant(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedInstant", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedInstant", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Integer(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedInteger", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedInteger", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Markdown(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedMarkdown", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedMarkdown", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Oid(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedOid", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedOid", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedPositiveInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedPositiveInt", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedString", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Time(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedTime", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::UnsignedInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedUnsignedInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedUnsignedInt", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Uri(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedUri", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedUri", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Url(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedUrl", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedUrl", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Uuid(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedUuid", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_fixedUuid", &primitive_element)?;
                    }
                }
                ElementDefinitionFixed::Address(ref value) => {
                    state.serialize_entry("fixedAddress", value)?;
                }
                ElementDefinitionFixed::Age(ref value) => {
                    state.serialize_entry("fixedAge", value)?;
                }
                ElementDefinitionFixed::Annotation(ref value) => {
                    state.serialize_entry("fixedAnnotation", value)?;
                }
                ElementDefinitionFixed::Attachment(ref value) => {
                    state.serialize_entry("fixedAttachment", value)?;
                }
                ElementDefinitionFixed::CodeableConcept(ref value) => {
                    state.serialize_entry("fixedCodeableConcept", value)?;
                }
                ElementDefinitionFixed::Coding(ref value) => {
                    state.serialize_entry("fixedCoding", value)?;
                }
                ElementDefinitionFixed::ContactPoint(ref value) => {
                    state.serialize_entry("fixedContactPoint", value)?;
                }
                ElementDefinitionFixed::Count(ref value) => {
                    state.serialize_entry("fixedCount", value)?;
                }
                ElementDefinitionFixed::Distance(ref value) => {
                    state.serialize_entry("fixedDistance", value)?;
                }
                ElementDefinitionFixed::Duration(ref value) => {
                    state.serialize_entry("fixedDuration", value)?;
                }
                ElementDefinitionFixed::HumanName(ref value) => {
                    state.serialize_entry("fixedHumanName", value)?;
                }
                ElementDefinitionFixed::Identifier(ref value) => {
                    state.serialize_entry("fixedIdentifier", value)?;
                }
                ElementDefinitionFixed::Money(ref value) => {
                    state.serialize_entry("fixedMoney", value)?;
                }
                ElementDefinitionFixed::Period(ref value) => {
                    state.serialize_entry("fixedPeriod", value)?;
                }
                ElementDefinitionFixed::Quantity(ref value) => {
                    state.serialize_entry("fixedQuantity", value)?;
                }
                ElementDefinitionFixed::Range(ref value) => {
                    state.serialize_entry("fixedRange", value)?;
                }
                ElementDefinitionFixed::Ratio(ref value) => {
                    state.serialize_entry("fixedRatio", value)?;
                }
                ElementDefinitionFixed::Reference(ref value) => {
                    state.serialize_entry("fixedReference", value)?;
                }
                ElementDefinitionFixed::SampledData(ref value) => {
                    state.serialize_entry("fixedSampledData", value)?;
                }
                ElementDefinitionFixed::Signature(ref value) => {
                    state.serialize_entry("fixedSignature", value)?;
                }
                ElementDefinitionFixed::Timing(ref value) => {
                    state.serialize_entry("fixedTiming", value)?;
                }
                ElementDefinitionFixed::ContactDetail(ref value) => {
                    state.serialize_entry("fixedContactDetail", value)?;
                }
                ElementDefinitionFixed::Contributor(ref value) => {
                    state.serialize_entry("fixedContributor", value)?;
                }
                ElementDefinitionFixed::DataRequirement(ref value) => {
                    state.serialize_entry("fixedDataRequirement", value)?;
                }
                ElementDefinitionFixed::Expression(ref value) => {
                    state.serialize_entry("fixedExpression", value)?;
                }
                ElementDefinitionFixed::ParameterDefinition(ref value) => {
                    state.serialize_entry("fixedParameterDefinition", value)?;
                }
                ElementDefinitionFixed::RelatedArtifact(ref value) => {
                    state.serialize_entry("fixedRelatedArtifact", value)?;
                }
                ElementDefinitionFixed::TriggerDefinition(ref value) => {
                    state.serialize_entry("fixedTriggerDefinition", value)?;
                }
                ElementDefinitionFixed::UsageContext(ref value) => {
                    state.serialize_entry("fixedUsageContext", value)?;
                }
                ElementDefinitionFixed::Dosage(ref value) => {
                    state.serialize_entry("fixedDosage", value)?;
                }
                ElementDefinitionFixed::Meta(ref value) => {
                    state.serialize_entry("fixedMeta", value)?;
                }
                ElementDefinitionFixed::Invalid => {
                    return Err(serde::ser::Error::custom("fixed is invalid"))
                }
            }
        }
        if let Some(some) = self.r#pattern.as_ref() {
            match some {
                ElementDefinitionPattern::Base64Binary(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternBase64Binary", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternBase64Binary", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternBoolean", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Canonical(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternCanonical", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternCanonical", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Code(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternCode", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternCode", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternDate", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternDateTime", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Decimal(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternDecimal", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternDecimal", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Id(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternId", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternId", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Instant(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternInstant", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternInstant", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Integer(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternInteger", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternInteger", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Markdown(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternMarkdown", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternMarkdown", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Oid(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternOid", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternOid", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternPositiveInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternPositiveInt", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternString", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Time(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternTime", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::UnsignedInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternUnsignedInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternUnsignedInt", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Uri(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternUri", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternUri", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Url(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternUrl", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternUrl", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Uuid(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternUuid", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_patternUuid", &primitive_element)?;
                    }
                }
                ElementDefinitionPattern::Address(ref value) => {
                    state.serialize_entry("patternAddress", value)?;
                }
                ElementDefinitionPattern::Age(ref value) => {
                    state.serialize_entry("patternAge", value)?;
                }
                ElementDefinitionPattern::Annotation(ref value) => {
                    state.serialize_entry("patternAnnotation", value)?;
                }
                ElementDefinitionPattern::Attachment(ref value) => {
                    state.serialize_entry("patternAttachment", value)?;
                }
                ElementDefinitionPattern::CodeableConcept(ref value) => {
                    state.serialize_entry("patternCodeableConcept", value)?;
                }
                ElementDefinitionPattern::Coding(ref value) => {
                    state.serialize_entry("patternCoding", value)?;
                }
                ElementDefinitionPattern::ContactPoint(ref value) => {
                    state.serialize_entry("patternContactPoint", value)?;
                }
                ElementDefinitionPattern::Count(ref value) => {
                    state.serialize_entry("patternCount", value)?;
                }
                ElementDefinitionPattern::Distance(ref value) => {
                    state.serialize_entry("patternDistance", value)?;
                }
                ElementDefinitionPattern::Duration(ref value) => {
                    state.serialize_entry("patternDuration", value)?;
                }
                ElementDefinitionPattern::HumanName(ref value) => {
                    state.serialize_entry("patternHumanName", value)?;
                }
                ElementDefinitionPattern::Identifier(ref value) => {
                    state.serialize_entry("patternIdentifier", value)?;
                }
                ElementDefinitionPattern::Money(ref value) => {
                    state.serialize_entry("patternMoney", value)?;
                }
                ElementDefinitionPattern::Period(ref value) => {
                    state.serialize_entry("patternPeriod", value)?;
                }
                ElementDefinitionPattern::Quantity(ref value) => {
                    state.serialize_entry("patternQuantity", value)?;
                }
                ElementDefinitionPattern::Range(ref value) => {
                    state.serialize_entry("patternRange", value)?;
                }
                ElementDefinitionPattern::Ratio(ref value) => {
                    state.serialize_entry("patternRatio", value)?;
                }
                ElementDefinitionPattern::Reference(ref value) => {
                    state.serialize_entry("patternReference", value)?;
                }
                ElementDefinitionPattern::SampledData(ref value) => {
                    state.serialize_entry("patternSampledData", value)?;
                }
                ElementDefinitionPattern::Signature(ref value) => {
                    state.serialize_entry("patternSignature", value)?;
                }
                ElementDefinitionPattern::Timing(ref value) => {
                    state.serialize_entry("patternTiming", value)?;
                }
                ElementDefinitionPattern::ContactDetail(ref value) => {
                    state.serialize_entry("patternContactDetail", value)?;
                }
                ElementDefinitionPattern::Contributor(ref value) => {
                    state.serialize_entry("patternContributor", value)?;
                }
                ElementDefinitionPattern::DataRequirement(ref value) => {
                    state.serialize_entry("patternDataRequirement", value)?;
                }
                ElementDefinitionPattern::Expression(ref value) => {
                    state.serialize_entry("patternExpression", value)?;
                }
                ElementDefinitionPattern::ParameterDefinition(ref value) => {
                    state.serialize_entry("patternParameterDefinition", value)?;
                }
                ElementDefinitionPattern::RelatedArtifact(ref value) => {
                    state.serialize_entry("patternRelatedArtifact", value)?;
                }
                ElementDefinitionPattern::TriggerDefinition(ref value) => {
                    state.serialize_entry("patternTriggerDefinition", value)?;
                }
                ElementDefinitionPattern::UsageContext(ref value) => {
                    state.serialize_entry("patternUsageContext", value)?;
                }
                ElementDefinitionPattern::Dosage(ref value) => {
                    state.serialize_entry("patternDosage", value)?;
                }
                ElementDefinitionPattern::Meta(ref value) => {
                    state.serialize_entry("patternMeta", value)?;
                }
                ElementDefinitionPattern::Invalid => {
                    return Err(serde::ser::Error::custom("pattern is invalid"))
                }
            }
        }
        if !self.r#example.is_empty() {
            state.serialize_entry("example", &self.r#example)?;
        }
        if let Some(some) = self.r#min_value.as_ref() {
            match some {
                ElementDefinitionMinValue::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("minValueDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_minValueDate", &primitive_element)?;
                    }
                }
                ElementDefinitionMinValue::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("minValueDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_minValueDateTime", &primitive_element)?;
                    }
                }
                ElementDefinitionMinValue::Instant(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("minValueInstant", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_minValueInstant", &primitive_element)?;
                    }
                }
                ElementDefinitionMinValue::Time(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("minValueTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_minValueTime", &primitive_element)?;
                    }
                }
                ElementDefinitionMinValue::Decimal(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("minValueDecimal", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_minValueDecimal", &primitive_element)?;
                    }
                }
                ElementDefinitionMinValue::Integer(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("minValueInteger", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_minValueInteger", &primitive_element)?;
                    }
                }
                ElementDefinitionMinValue::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("minValuePositiveInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_minValuePositiveInt", &primitive_element)?;
                    }
                }
                ElementDefinitionMinValue::UnsignedInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("minValueUnsignedInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_minValueUnsignedInt", &primitive_element)?;
                    }
                }
                ElementDefinitionMinValue::Quantity(ref value) => {
                    state.serialize_entry("minValueQuantity", value)?;
                }
                ElementDefinitionMinValue::Invalid => {
                    return Err(serde::ser::Error::custom("min_value is invalid"))
                }
            }
        }
        if let Some(some) = self.r#max_value.as_ref() {
            match some {
                ElementDefinitionMaxValue::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("maxValueDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_maxValueDate", &primitive_element)?;
                    }
                }
                ElementDefinitionMaxValue::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("maxValueDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_maxValueDateTime", &primitive_element)?;
                    }
                }
                ElementDefinitionMaxValue::Instant(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("maxValueInstant", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_maxValueInstant", &primitive_element)?;
                    }
                }
                ElementDefinitionMaxValue::Time(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("maxValueTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_maxValueTime", &primitive_element)?;
                    }
                }
                ElementDefinitionMaxValue::Decimal(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("maxValueDecimal", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_maxValueDecimal", &primitive_element)?;
                    }
                }
                ElementDefinitionMaxValue::Integer(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("maxValueInteger", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_maxValueInteger", &primitive_element)?;
                    }
                }
                ElementDefinitionMaxValue::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("maxValuePositiveInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_maxValuePositiveInt", &primitive_element)?;
                    }
                }
                ElementDefinitionMaxValue::UnsignedInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("maxValueUnsignedInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_maxValueUnsignedInt", &primitive_element)?;
                    }
                }
                ElementDefinitionMaxValue::Quantity(ref value) => {
                    state.serialize_entry("maxValueQuantity", value)?;
                }
                ElementDefinitionMaxValue::Invalid => {
                    return Err(serde::ser::Error::custom("max_value is invalid"))
                }
            }
        }
        if let Some(some) = self.r#max_length.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("maxLength", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_maxLength", &primitive_element)?;
            }
        }
        if !self.r#condition.is_empty() {
            let values: Vec<_> = self.r#condition.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("condition", &values)?;
            }
            let requires_elements = self
                .r#condition
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#condition
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_condition", &primitive_elements)?;
            }
        }
        if !self.r#constraint.is_empty() {
            state.serialize_entry("constraint", &self.r#constraint)?;
        }
        if let Some(some) = self.r#must_support.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("mustSupport", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_mustSupport", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#is_modifier.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isModifier", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isModifier", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#is_modifier_reason.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isModifierReason", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isModifierReason", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#is_summary.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isSummary", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isSummary", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#binding.as_ref() {
            state.serialize_entry("binding", some)?;
        }
        if !self.r#mapping.is_empty() {
            state.serialize_entry("mapping", &self.r#mapping)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ElementDefinition {
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
            #[serde(rename = "path")]
            Path,
            #[serde(rename = "_path")]
            PathPrimitiveElement,
            #[serde(rename = "representation")]
            Representation,
            #[serde(rename = "_representation")]
            RepresentationPrimitiveElement,
            #[serde(rename = "sliceName")]
            SliceName,
            #[serde(rename = "_sliceName")]
            SliceNamePrimitiveElement,
            #[serde(rename = "sliceIsConstraining")]
            SliceIsConstraining,
            #[serde(rename = "_sliceIsConstraining")]
            SliceIsConstrainingPrimitiveElement,
            #[serde(rename = "label")]
            Label,
            #[serde(rename = "_label")]
            LabelPrimitiveElement,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "slicing")]
            Slicing,
            #[serde(rename = "short")]
            Short,
            #[serde(rename = "_short")]
            ShortPrimitiveElement,
            #[serde(rename = "definition")]
            Definition,
            #[serde(rename = "_definition")]
            DefinitionPrimitiveElement,
            #[serde(rename = "comment")]
            Comment,
            #[serde(rename = "_comment")]
            CommentPrimitiveElement,
            #[serde(rename = "requirements")]
            Requirements,
            #[serde(rename = "_requirements")]
            RequirementsPrimitiveElement,
            #[serde(rename = "alias")]
            Alias,
            #[serde(rename = "_alias")]
            AliasPrimitiveElement,
            #[serde(rename = "min")]
            Min,
            #[serde(rename = "_min")]
            MinPrimitiveElement,
            #[serde(rename = "max")]
            Max,
            #[serde(rename = "_max")]
            MaxPrimitiveElement,
            #[serde(rename = "base")]
            Base,
            #[serde(rename = "contentReference")]
            ContentReference,
            #[serde(rename = "_contentReference")]
            ContentReferencePrimitiveElement,
            #[serde(rename = "type")]
            Type,
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
            #[serde(rename = "meaningWhenMissing")]
            MeaningWhenMissing,
            #[serde(rename = "_meaningWhenMissing")]
            MeaningWhenMissingPrimitiveElement,
            #[serde(rename = "orderMeaning")]
            OrderMeaning,
            #[serde(rename = "_orderMeaning")]
            OrderMeaningPrimitiveElement,
            #[serde(rename = "fixedBase64Binary")]
            FixedBase64Binary,
            #[serde(rename = "_fixedBase64Binary")]
            FixedBase64BinaryPrimitiveElement,
            #[serde(rename = "fixedBoolean")]
            FixedBoolean,
            #[serde(rename = "_fixedBoolean")]
            FixedBooleanPrimitiveElement,
            #[serde(rename = "fixedCanonical")]
            FixedCanonical,
            #[serde(rename = "_fixedCanonical")]
            FixedCanonicalPrimitiveElement,
            #[serde(rename = "fixedCode")]
            FixedCode,
            #[serde(rename = "_fixedCode")]
            FixedCodePrimitiveElement,
            #[serde(rename = "fixedDate")]
            FixedDate,
            #[serde(rename = "_fixedDate")]
            FixedDatePrimitiveElement,
            #[serde(rename = "fixedDateTime")]
            FixedDateTime,
            #[serde(rename = "_fixedDateTime")]
            FixedDateTimePrimitiveElement,
            #[serde(rename = "fixedDecimal")]
            FixedDecimal,
            #[serde(rename = "_fixedDecimal")]
            FixedDecimalPrimitiveElement,
            #[serde(rename = "fixedId")]
            FixedId,
            #[serde(rename = "_fixedId")]
            FixedIdPrimitiveElement,
            #[serde(rename = "fixedInstant")]
            FixedInstant,
            #[serde(rename = "_fixedInstant")]
            FixedInstantPrimitiveElement,
            #[serde(rename = "fixedInteger")]
            FixedInteger,
            #[serde(rename = "_fixedInteger")]
            FixedIntegerPrimitiveElement,
            #[serde(rename = "fixedMarkdown")]
            FixedMarkdown,
            #[serde(rename = "_fixedMarkdown")]
            FixedMarkdownPrimitiveElement,
            #[serde(rename = "fixedOid")]
            FixedOid,
            #[serde(rename = "_fixedOid")]
            FixedOidPrimitiveElement,
            #[serde(rename = "fixedPositiveInt")]
            FixedPositiveInt,
            #[serde(rename = "_fixedPositiveInt")]
            FixedPositiveIntPrimitiveElement,
            #[serde(rename = "fixedString")]
            FixedString,
            #[serde(rename = "_fixedString")]
            FixedStringPrimitiveElement,
            #[serde(rename = "fixedTime")]
            FixedTime,
            #[serde(rename = "_fixedTime")]
            FixedTimePrimitiveElement,
            #[serde(rename = "fixedUnsignedInt")]
            FixedUnsignedInt,
            #[serde(rename = "_fixedUnsignedInt")]
            FixedUnsignedIntPrimitiveElement,
            #[serde(rename = "fixedUri")]
            FixedUri,
            #[serde(rename = "_fixedUri")]
            FixedUriPrimitiveElement,
            #[serde(rename = "fixedUrl")]
            FixedUrl,
            #[serde(rename = "_fixedUrl")]
            FixedUrlPrimitiveElement,
            #[serde(rename = "fixedUuid")]
            FixedUuid,
            #[serde(rename = "_fixedUuid")]
            FixedUuidPrimitiveElement,
            #[serde(rename = "fixedAddress")]
            FixedAddress,
            #[serde(rename = "fixedAge")]
            FixedAge,
            #[serde(rename = "fixedAnnotation")]
            FixedAnnotation,
            #[serde(rename = "fixedAttachment")]
            FixedAttachment,
            #[serde(rename = "fixedCodeableConcept")]
            FixedCodeableConcept,
            #[serde(rename = "fixedCoding")]
            FixedCoding,
            #[serde(rename = "fixedContactPoint")]
            FixedContactPoint,
            #[serde(rename = "fixedCount")]
            FixedCount,
            #[serde(rename = "fixedDistance")]
            FixedDistance,
            #[serde(rename = "fixedDuration")]
            FixedDuration,
            #[serde(rename = "fixedHumanName")]
            FixedHumanName,
            #[serde(rename = "fixedIdentifier")]
            FixedIdentifier,
            #[serde(rename = "fixedMoney")]
            FixedMoney,
            #[serde(rename = "fixedPeriod")]
            FixedPeriod,
            #[serde(rename = "fixedQuantity")]
            FixedQuantity,
            #[serde(rename = "fixedRange")]
            FixedRange,
            #[serde(rename = "fixedRatio")]
            FixedRatio,
            #[serde(rename = "fixedReference")]
            FixedReference,
            #[serde(rename = "fixedSampledData")]
            FixedSampledData,
            #[serde(rename = "fixedSignature")]
            FixedSignature,
            #[serde(rename = "fixedTiming")]
            FixedTiming,
            #[serde(rename = "fixedContactDetail")]
            FixedContactDetail,
            #[serde(rename = "fixedContributor")]
            FixedContributor,
            #[serde(rename = "fixedDataRequirement")]
            FixedDataRequirement,
            #[serde(rename = "fixedExpression")]
            FixedExpression,
            #[serde(rename = "fixedParameterDefinition")]
            FixedParameterDefinition,
            #[serde(rename = "fixedRelatedArtifact")]
            FixedRelatedArtifact,
            #[serde(rename = "fixedTriggerDefinition")]
            FixedTriggerDefinition,
            #[serde(rename = "fixedUsageContext")]
            FixedUsageContext,
            #[serde(rename = "fixedDosage")]
            FixedDosage,
            #[serde(rename = "fixedMeta")]
            FixedMeta,
            #[serde(rename = "patternBase64Binary")]
            PatternBase64Binary,
            #[serde(rename = "_patternBase64Binary")]
            PatternBase64BinaryPrimitiveElement,
            #[serde(rename = "patternBoolean")]
            PatternBoolean,
            #[serde(rename = "_patternBoolean")]
            PatternBooleanPrimitiveElement,
            #[serde(rename = "patternCanonical")]
            PatternCanonical,
            #[serde(rename = "_patternCanonical")]
            PatternCanonicalPrimitiveElement,
            #[serde(rename = "patternCode")]
            PatternCode,
            #[serde(rename = "_patternCode")]
            PatternCodePrimitiveElement,
            #[serde(rename = "patternDate")]
            PatternDate,
            #[serde(rename = "_patternDate")]
            PatternDatePrimitiveElement,
            #[serde(rename = "patternDateTime")]
            PatternDateTime,
            #[serde(rename = "_patternDateTime")]
            PatternDateTimePrimitiveElement,
            #[serde(rename = "patternDecimal")]
            PatternDecimal,
            #[serde(rename = "_patternDecimal")]
            PatternDecimalPrimitiveElement,
            #[serde(rename = "patternId")]
            PatternId,
            #[serde(rename = "_patternId")]
            PatternIdPrimitiveElement,
            #[serde(rename = "patternInstant")]
            PatternInstant,
            #[serde(rename = "_patternInstant")]
            PatternInstantPrimitiveElement,
            #[serde(rename = "patternInteger")]
            PatternInteger,
            #[serde(rename = "_patternInteger")]
            PatternIntegerPrimitiveElement,
            #[serde(rename = "patternMarkdown")]
            PatternMarkdown,
            #[serde(rename = "_patternMarkdown")]
            PatternMarkdownPrimitiveElement,
            #[serde(rename = "patternOid")]
            PatternOid,
            #[serde(rename = "_patternOid")]
            PatternOidPrimitiveElement,
            #[serde(rename = "patternPositiveInt")]
            PatternPositiveInt,
            #[serde(rename = "_patternPositiveInt")]
            PatternPositiveIntPrimitiveElement,
            #[serde(rename = "patternString")]
            PatternString,
            #[serde(rename = "_patternString")]
            PatternStringPrimitiveElement,
            #[serde(rename = "patternTime")]
            PatternTime,
            #[serde(rename = "_patternTime")]
            PatternTimePrimitiveElement,
            #[serde(rename = "patternUnsignedInt")]
            PatternUnsignedInt,
            #[serde(rename = "_patternUnsignedInt")]
            PatternUnsignedIntPrimitiveElement,
            #[serde(rename = "patternUri")]
            PatternUri,
            #[serde(rename = "_patternUri")]
            PatternUriPrimitiveElement,
            #[serde(rename = "patternUrl")]
            PatternUrl,
            #[serde(rename = "_patternUrl")]
            PatternUrlPrimitiveElement,
            #[serde(rename = "patternUuid")]
            PatternUuid,
            #[serde(rename = "_patternUuid")]
            PatternUuidPrimitiveElement,
            #[serde(rename = "patternAddress")]
            PatternAddress,
            #[serde(rename = "patternAge")]
            PatternAge,
            #[serde(rename = "patternAnnotation")]
            PatternAnnotation,
            #[serde(rename = "patternAttachment")]
            PatternAttachment,
            #[serde(rename = "patternCodeableConcept")]
            PatternCodeableConcept,
            #[serde(rename = "patternCoding")]
            PatternCoding,
            #[serde(rename = "patternContactPoint")]
            PatternContactPoint,
            #[serde(rename = "patternCount")]
            PatternCount,
            #[serde(rename = "patternDistance")]
            PatternDistance,
            #[serde(rename = "patternDuration")]
            PatternDuration,
            #[serde(rename = "patternHumanName")]
            PatternHumanName,
            #[serde(rename = "patternIdentifier")]
            PatternIdentifier,
            #[serde(rename = "patternMoney")]
            PatternMoney,
            #[serde(rename = "patternPeriod")]
            PatternPeriod,
            #[serde(rename = "patternQuantity")]
            PatternQuantity,
            #[serde(rename = "patternRange")]
            PatternRange,
            #[serde(rename = "patternRatio")]
            PatternRatio,
            #[serde(rename = "patternReference")]
            PatternReference,
            #[serde(rename = "patternSampledData")]
            PatternSampledData,
            #[serde(rename = "patternSignature")]
            PatternSignature,
            #[serde(rename = "patternTiming")]
            PatternTiming,
            #[serde(rename = "patternContactDetail")]
            PatternContactDetail,
            #[serde(rename = "patternContributor")]
            PatternContributor,
            #[serde(rename = "patternDataRequirement")]
            PatternDataRequirement,
            #[serde(rename = "patternExpression")]
            PatternExpression,
            #[serde(rename = "patternParameterDefinition")]
            PatternParameterDefinition,
            #[serde(rename = "patternRelatedArtifact")]
            PatternRelatedArtifact,
            #[serde(rename = "patternTriggerDefinition")]
            PatternTriggerDefinition,
            #[serde(rename = "patternUsageContext")]
            PatternUsageContext,
            #[serde(rename = "patternDosage")]
            PatternDosage,
            #[serde(rename = "patternMeta")]
            PatternMeta,
            #[serde(rename = "example")]
            Example,
            #[serde(rename = "minValueDate")]
            MinValueDate,
            #[serde(rename = "_minValueDate")]
            MinValueDatePrimitiveElement,
            #[serde(rename = "minValueDateTime")]
            MinValueDateTime,
            #[serde(rename = "_minValueDateTime")]
            MinValueDateTimePrimitiveElement,
            #[serde(rename = "minValueInstant")]
            MinValueInstant,
            #[serde(rename = "_minValueInstant")]
            MinValueInstantPrimitiveElement,
            #[serde(rename = "minValueTime")]
            MinValueTime,
            #[serde(rename = "_minValueTime")]
            MinValueTimePrimitiveElement,
            #[serde(rename = "minValueDecimal")]
            MinValueDecimal,
            #[serde(rename = "_minValueDecimal")]
            MinValueDecimalPrimitiveElement,
            #[serde(rename = "minValueInteger")]
            MinValueInteger,
            #[serde(rename = "_minValueInteger")]
            MinValueIntegerPrimitiveElement,
            #[serde(rename = "minValuePositiveInt")]
            MinValuePositiveInt,
            #[serde(rename = "_minValuePositiveInt")]
            MinValuePositiveIntPrimitiveElement,
            #[serde(rename = "minValueUnsignedInt")]
            MinValueUnsignedInt,
            #[serde(rename = "_minValueUnsignedInt")]
            MinValueUnsignedIntPrimitiveElement,
            #[serde(rename = "minValueQuantity")]
            MinValueQuantity,
            #[serde(rename = "maxValueDate")]
            MaxValueDate,
            #[serde(rename = "_maxValueDate")]
            MaxValueDatePrimitiveElement,
            #[serde(rename = "maxValueDateTime")]
            MaxValueDateTime,
            #[serde(rename = "_maxValueDateTime")]
            MaxValueDateTimePrimitiveElement,
            #[serde(rename = "maxValueInstant")]
            MaxValueInstant,
            #[serde(rename = "_maxValueInstant")]
            MaxValueInstantPrimitiveElement,
            #[serde(rename = "maxValueTime")]
            MaxValueTime,
            #[serde(rename = "_maxValueTime")]
            MaxValueTimePrimitiveElement,
            #[serde(rename = "maxValueDecimal")]
            MaxValueDecimal,
            #[serde(rename = "_maxValueDecimal")]
            MaxValueDecimalPrimitiveElement,
            #[serde(rename = "maxValueInteger")]
            MaxValueInteger,
            #[serde(rename = "_maxValueInteger")]
            MaxValueIntegerPrimitiveElement,
            #[serde(rename = "maxValuePositiveInt")]
            MaxValuePositiveInt,
            #[serde(rename = "_maxValuePositiveInt")]
            MaxValuePositiveIntPrimitiveElement,
            #[serde(rename = "maxValueUnsignedInt")]
            MaxValueUnsignedInt,
            #[serde(rename = "_maxValueUnsignedInt")]
            MaxValueUnsignedIntPrimitiveElement,
            #[serde(rename = "maxValueQuantity")]
            MaxValueQuantity,
            #[serde(rename = "maxLength")]
            MaxLength,
            #[serde(rename = "_maxLength")]
            MaxLengthPrimitiveElement,
            #[serde(rename = "condition")]
            Condition,
            #[serde(rename = "_condition")]
            ConditionPrimitiveElement,
            #[serde(rename = "constraint")]
            Constraint,
            #[serde(rename = "mustSupport")]
            MustSupport,
            #[serde(rename = "_mustSupport")]
            MustSupportPrimitiveElement,
            #[serde(rename = "isModifier")]
            IsModifier,
            #[serde(rename = "_isModifier")]
            IsModifierPrimitiveElement,
            #[serde(rename = "isModifierReason")]
            IsModifierReason,
            #[serde(rename = "_isModifierReason")]
            IsModifierReasonPrimitiveElement,
            #[serde(rename = "isSummary")]
            IsSummary,
            #[serde(rename = "_isSummary")]
            IsSummaryPrimitiveElement,
            #[serde(rename = "binding")]
            Binding,
            #[serde(rename = "mapping")]
            Mapping,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ElementDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#path: Option<super::super::types::String> = None;
                let mut r#representation: Option<Vec<super::super::types::Code>> = None;
                let mut r#slice_name: Option<super::super::types::String> = None;
                let mut r#slice_is_constraining: Option<super::super::types::Boolean> = None;
                let mut r#label: Option<super::super::types::String> = None;
                let mut r#code: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#slicing: Option<ElementDefinitionSlicing> = None;
                let mut r#short: Option<super::super::types::String> = None;
                let mut r#definition: Option<super::super::types::Markdown> = None;
                let mut r#comment: Option<super::super::types::Markdown> = None;
                let mut r#requirements: Option<super::super::types::Markdown> = None;
                let mut r#alias: Option<Vec<super::super::types::String>> = None;
                let mut r#min: Option<super::super::types::UnsignedInt> = None;
                let mut r#max: Option<super::super::types::String> = None;
                let mut r#base: Option<ElementDefinitionBase> = None;
                let mut r#content_reference: Option<super::super::types::Uri> = None;
                let mut r#type: Option<Vec<ElementDefinitionType>> = None;
                let mut r#default_value: Option<ElementDefinitionDefaultValue> = None;
                let mut r#meaning_when_missing: Option<super::super::types::Markdown> = None;
                let mut r#order_meaning: Option<super::super::types::String> = None;
                let mut r#fixed: Option<ElementDefinitionFixed> = None;
                let mut r#pattern: Option<ElementDefinitionPattern> = None;
                let mut r#example: Option<Vec<ElementDefinitionExample>> = None;
                let mut r#min_value: Option<ElementDefinitionMinValue> = None;
                let mut r#max_value: Option<ElementDefinitionMaxValue> = None;
                let mut r#max_length: Option<super::super::types::Integer> = None;
                let mut r#condition: Option<Vec<super::super::types::Id>> = None;
                let mut r#constraint: Option<Vec<ElementDefinitionConstraint>> = None;
                let mut r#must_support: Option<super::super::types::Boolean> = None;
                let mut r#is_modifier: Option<super::super::types::Boolean> = None;
                let mut r#is_modifier_reason: Option<super::super::types::String> = None;
                let mut r#is_summary: Option<super::super::types::Boolean> = None;
                let mut r#binding: Option<ElementDefinitionBinding> = None;
                let mut r#mapping: Option<Vec<ElementDefinitionMapping>> = None;
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
                        Field::Path => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PathPrimitiveElement => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_path"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Representation => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#representation.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("representation"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::RepresentationPrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
                            let vec = r#representation.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("_representation"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                if let Some(element) = element {
                                    vec[i].id = element.id;
                                    vec[i].extension = element.extension;
                                }
                            }
                        }
                        Field::SliceName => {
                            let some = r#slice_name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sliceName"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SliceNamePrimitiveElement => {
                            let some = r#slice_name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sliceName"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::SliceIsConstraining => {
                            let some = r#slice_is_constraining.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sliceIsConstraining",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SliceIsConstrainingPrimitiveElement => {
                            let some = r#slice_is_constraining.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_sliceIsConstraining",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Label => {
                            let some = r#label.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LabelPrimitiveElement => {
                            let some = r#label.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_label"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        Field::Slicing => {
                            if r#slicing.is_some() {
                                return Err(serde::de::Error::duplicate_field("slicing"));
                            }
                            r#slicing = Some(map_access.next_value()?);
                        }
                        Field::Short => {
                            let some = r#short.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("short"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ShortPrimitiveElement => {
                            let some = r#short.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_short"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Definition => {
                            let some = r#definition.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("definition"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DefinitionPrimitiveElement => {
                            let some = r#definition.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_definition"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
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
                        Field::Requirements => {
                            let some = r#requirements.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirements"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RequirementsPrimitiveElement => {
                            let some = r#requirements.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_requirements"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Alias => {
                            let values: Vec<_> = map_access.next_value()?;
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
                                vec[i].value = value;
                            }
                        }
                        Field::AliasPrimitiveElement => {
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
                        }
                        Field::Min => {
                            let some = r#min.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            some.value = Some(map_access.next_value()?);
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
                            some.value = Some(map_access.next_value()?);
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
                        Field::Base => {
                            if r#base.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            r#base = Some(map_access.next_value()?);
                        }
                        Field::ContentReference => {
                            let some = r#content_reference.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentReference"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ContentReferencePrimitiveElement => {
                            let some = r#content_reference.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_contentReference"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::DefaultValueBase64Binary => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Base64Binary(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Base64Binary(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueBase64Binary",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueBase64BinaryPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Base64Binary(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Base64Binary(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueBoolean => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Boolean(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueBoolean",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueBooleanPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Boolean(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Boolean(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueCanonical => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Canonical(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Canonical(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCanonical",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueCanonicalPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Canonical(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Canonical(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueCode => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Code(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Code(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCode",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueCodePrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Code(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Code(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueDate => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Date(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDate",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueDatePrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Date(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Date(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueDateTime => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::DateTime(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueDateTimePrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::DateTime(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::DateTime(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueDecimal => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Decimal(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Decimal(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDecimal",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueDecimalPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Decimal(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Decimal(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueId => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Id(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Id(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueId",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueIdPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Id(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Id(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueInstant => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Instant(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Instant(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueInstant",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueInstantPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Instant(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Instant(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueInteger => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Integer(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Integer(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueInteger",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueIntegerPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Integer(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Integer(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueMarkdown => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Markdown(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Markdown(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueMarkdown",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueMarkdownPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Markdown(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Markdown(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueOid => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Oid(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Oid(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueOid",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueOidPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Oid(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Oid(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValuePositiveInt => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::PositiveInt(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::PositiveInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValuePositiveInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValuePositiveIntPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::PositiveInt(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::PositiveInt(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueString => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::String(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueString",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueStringPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::String(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::String(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueTime => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Time(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Time(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueTimePrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Time(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Time(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueUnsignedInt => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::UnsignedInt(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::UnsignedInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUnsignedInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueUnsignedIntPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::UnsignedInt(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::UnsignedInt(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueUri => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Uri(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Uri(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUri",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueUriPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Uri(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Uri(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueUrl => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Url(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Url(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUrl",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueUrlPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Url(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Url(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueUuid => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Uuid(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Uuid(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUuid",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueUuidPrimitiveElement => {
                            let r#enum = r#default_value.get_or_insert(
                                ElementDefinitionDefaultValue::Uuid(Default::default()),
                            );
                            if let ElementDefinitionDefaultValue::Uuid(variant) = r#enum {
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
                                return Err(serde::de::Error::duplicate_field("_defaultValue[x]"));
                            }
                        }
                        Field::DefaultValueAddress => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAddress",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Address(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueAge => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueAge"));
                            }
                            r#default_value =
                                Some(ElementDefinitionDefaultValue::Age(map_access.next_value()?));
                        }
                        Field::DefaultValueAnnotation => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAnnotation",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Annotation(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueAttachment => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAttachment",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Attachment(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueCodeableConcept => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueCodeableConcept",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueCoding => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueCoding",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Coding(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueContactPoint => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContactPoint",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::ContactPoint(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueCount => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueCount"));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Count(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueDistance => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDistance",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Distance(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueDuration => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDuration",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Duration(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueHumanName => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueHumanName",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::HumanName(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueIdentifier => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueIdentifier",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Identifier(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueMoney => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueMoney"));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Money(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValuePeriod => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValuePeriod",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Period(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueQuantity => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueQuantity",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueRange => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueRange"));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Range(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueRatio => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueRatio"));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Ratio(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueReference => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueReference",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueSampledData => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueSampledData",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::SampledData(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueSignature => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueSignature",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Signature(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueTiming => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueTiming",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Timing(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueContactDetail => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContactDetail",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::ContactDetail(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueContributor => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContributor",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Contributor(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueDataRequirement => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDataRequirement",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::DataRequirement(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueExpression => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueExpression",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Expression(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueParameterDefinition => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueParameterDefinition",
                                ));
                            }
                            r#default_value =
                                Some(ElementDefinitionDefaultValue::ParameterDefinition(
                                    map_access.next_value()?,
                                ));
                        }
                        Field::DefaultValueRelatedArtifact => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueRelatedArtifact",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::RelatedArtifact(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueTriggerDefinition => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueTriggerDefinition",
                                ));
                            }
                            r#default_value =
                                Some(ElementDefinitionDefaultValue::TriggerDefinition(
                                    map_access.next_value()?,
                                ));
                        }
                        Field::DefaultValueUsageContext => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueUsageContext",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::UsageContext(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueDosage => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDosage",
                                ));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Dosage(
                                map_access.next_value()?,
                            ));
                        }
                        Field::DefaultValueMeta => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueMeta"));
                            }
                            r#default_value = Some(ElementDefinitionDefaultValue::Meta(
                                map_access.next_value()?,
                            ));
                        }
                        Field::MeaningWhenMissing => {
                            let some = r#meaning_when_missing.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "meaningWhenMissing",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::MeaningWhenMissingPrimitiveElement => {
                            let some = r#meaning_when_missing.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_meaningWhenMissing",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::OrderMeaning => {
                            let some = r#order_meaning.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderMeaning"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::OrderMeaningPrimitiveElement => {
                            let some = r#order_meaning.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_orderMeaning"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::FixedBase64Binary => {
                            let r#enum = r#fixed.get_or_insert(
                                ElementDefinitionFixed::Base64Binary(Default::default()),
                            );
                            if let ElementDefinitionFixed::Base64Binary(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fixedBase64Binary",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedBase64BinaryPrimitiveElement => {
                            let r#enum = r#fixed.get_or_insert(
                                ElementDefinitionFixed::Base64Binary(Default::default()),
                            );
                            if let ElementDefinitionFixed::Base64Binary(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_fixedBase64Binary",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedBoolean => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Boolean(Default::default()));
                            if let ElementDefinitionFixed::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedBoolean"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedBooleanPrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Boolean(Default::default()));
                            if let ElementDefinitionFixed::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedBoolean"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedCanonical => {
                            let r#enum = r#fixed.get_or_insert(ElementDefinitionFixed::Canonical(
                                Default::default(),
                            ));
                            if let ElementDefinitionFixed::Canonical(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fixedCanonical",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedCanonicalPrimitiveElement => {
                            let r#enum = r#fixed.get_or_insert(ElementDefinitionFixed::Canonical(
                                Default::default(),
                            ));
                            if let ElementDefinitionFixed::Canonical(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_fixedCanonical",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedCode => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Code(Default::default()));
                            if let ElementDefinitionFixed::Code(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedCode"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedCodePrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Code(Default::default()));
                            if let ElementDefinitionFixed::Code(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedCode"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedDate => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Date(Default::default()));
                            if let ElementDefinitionFixed::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedDatePrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Date(Default::default()));
                            if let ElementDefinitionFixed::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedDateTime => {
                            let r#enum = r#fixed.get_or_insert(ElementDefinitionFixed::DateTime(
                                Default::default(),
                            ));
                            if let ElementDefinitionFixed::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedDateTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedDateTimePrimitiveElement => {
                            let r#enum = r#fixed.get_or_insert(ElementDefinitionFixed::DateTime(
                                Default::default(),
                            ));
                            if let ElementDefinitionFixed::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_fixedDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedDecimal => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Decimal(Default::default()));
                            if let ElementDefinitionFixed::Decimal(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedDecimal"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedDecimalPrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Decimal(Default::default()));
                            if let ElementDefinitionFixed::Decimal(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedDecimal"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedId => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Id(Default::default()));
                            if let ElementDefinitionFixed::Id(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedId"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedIdPrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Id(Default::default()));
                            if let ElementDefinitionFixed::Id(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedId"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedInstant => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Instant(Default::default()));
                            if let ElementDefinitionFixed::Instant(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedInstant"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedInstantPrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Instant(Default::default()));
                            if let ElementDefinitionFixed::Instant(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedInstant"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedInteger => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Integer(Default::default()));
                            if let ElementDefinitionFixed::Integer(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedInteger"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedIntegerPrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Integer(Default::default()));
                            if let ElementDefinitionFixed::Integer(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedInteger"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedMarkdown => {
                            let r#enum = r#fixed.get_or_insert(ElementDefinitionFixed::Markdown(
                                Default::default(),
                            ));
                            if let ElementDefinitionFixed::Markdown(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedMarkdown"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedMarkdownPrimitiveElement => {
                            let r#enum = r#fixed.get_or_insert(ElementDefinitionFixed::Markdown(
                                Default::default(),
                            ));
                            if let ElementDefinitionFixed::Markdown(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_fixedMarkdown",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedOid => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Oid(Default::default()));
                            if let ElementDefinitionFixed::Oid(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedOid"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedOidPrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Oid(Default::default()));
                            if let ElementDefinitionFixed::Oid(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedOid"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedPositiveInt => {
                            let r#enum = r#fixed.get_or_insert(
                                ElementDefinitionFixed::PositiveInt(Default::default()),
                            );
                            if let ElementDefinitionFixed::PositiveInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fixedPositiveInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedPositiveIntPrimitiveElement => {
                            let r#enum = r#fixed.get_or_insert(
                                ElementDefinitionFixed::PositiveInt(Default::default()),
                            );
                            if let ElementDefinitionFixed::PositiveInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_fixedPositiveInt",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedString => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::String(Default::default()));
                            if let ElementDefinitionFixed::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedStringPrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::String(Default::default()));
                            if let ElementDefinitionFixed::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedString"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedTime => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Time(Default::default()));
                            if let ElementDefinitionFixed::Time(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedTimePrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Time(Default::default()));
                            if let ElementDefinitionFixed::Time(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedTime"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedUnsignedInt => {
                            let r#enum = r#fixed.get_or_insert(
                                ElementDefinitionFixed::UnsignedInt(Default::default()),
                            );
                            if let ElementDefinitionFixed::UnsignedInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fixedUnsignedInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedUnsignedIntPrimitiveElement => {
                            let r#enum = r#fixed.get_or_insert(
                                ElementDefinitionFixed::UnsignedInt(Default::default()),
                            );
                            if let ElementDefinitionFixed::UnsignedInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_fixedUnsignedInt",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedUri => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Uri(Default::default()));
                            if let ElementDefinitionFixed::Uri(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedUri"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedUriPrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Uri(Default::default()));
                            if let ElementDefinitionFixed::Uri(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedUri"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedUrl => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Url(Default::default()));
                            if let ElementDefinitionFixed::Url(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedUrl"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedUrlPrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Url(Default::default()));
                            if let ElementDefinitionFixed::Url(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedUrl"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedUuid => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Uuid(Default::default()));
                            if let ElementDefinitionFixed::Uuid(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedUuid"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("fixed[x]"));
                            }
                        }
                        Field::FixedUuidPrimitiveElement => {
                            let r#enum = r#fixed
                                .get_or_insert(ElementDefinitionFixed::Uuid(Default::default()));
                            if let ElementDefinitionFixed::Uuid(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fixedUuid"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                            }
                        }
                        Field::FixedAddress => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAddress"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Address(map_access.next_value()?));
                        }
                        Field::FixedAge => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAge"));
                            }
                            r#fixed = Some(ElementDefinitionFixed::Age(map_access.next_value()?));
                        }
                        Field::FixedAnnotation => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAnnotation"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Annotation(map_access.next_value()?));
                        }
                        Field::FixedAttachment => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAttachment"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Attachment(map_access.next_value()?));
                        }
                        Field::FixedCodeableConcept => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedCodeableConcept",
                                ));
                            }
                            r#fixed = Some(ElementDefinitionFixed::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        Field::FixedCoding => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedCoding"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Coding(map_access.next_value()?));
                        }
                        Field::FixedContactPoint => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedContactPoint"));
                            }
                            r#fixed = Some(ElementDefinitionFixed::ContactPoint(
                                map_access.next_value()?,
                            ));
                        }
                        Field::FixedCount => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedCount"));
                            }
                            r#fixed = Some(ElementDefinitionFixed::Count(map_access.next_value()?));
                        }
                        Field::FixedDistance => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedDistance"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Distance(map_access.next_value()?));
                        }
                        Field::FixedDuration => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedDuration"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Duration(map_access.next_value()?));
                        }
                        Field::FixedHumanName => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedHumanName"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::HumanName(map_access.next_value()?));
                        }
                        Field::FixedIdentifier => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedIdentifier"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Identifier(map_access.next_value()?));
                        }
                        Field::FixedMoney => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedMoney"));
                            }
                            r#fixed = Some(ElementDefinitionFixed::Money(map_access.next_value()?));
                        }
                        Field::FixedPeriod => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedPeriod"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Period(map_access.next_value()?));
                        }
                        Field::FixedQuantity => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedQuantity"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Quantity(map_access.next_value()?));
                        }
                        Field::FixedRange => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedRange"));
                            }
                            r#fixed = Some(ElementDefinitionFixed::Range(map_access.next_value()?));
                        }
                        Field::FixedRatio => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedRatio"));
                            }
                            r#fixed = Some(ElementDefinitionFixed::Ratio(map_access.next_value()?));
                        }
                        Field::FixedReference => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedReference"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Reference(map_access.next_value()?));
                        }
                        Field::FixedSampledData => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedSampledData"));
                            }
                            r#fixed = Some(ElementDefinitionFixed::SampledData(
                                map_access.next_value()?,
                            ));
                        }
                        Field::FixedSignature => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedSignature"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Signature(map_access.next_value()?));
                        }
                        Field::FixedTiming => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedTiming"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Timing(map_access.next_value()?));
                        }
                        Field::FixedContactDetail => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedContactDetail",
                                ));
                            }
                            r#fixed = Some(ElementDefinitionFixed::ContactDetail(
                                map_access.next_value()?,
                            ));
                        }
                        Field::FixedContributor => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedContributor"));
                            }
                            r#fixed = Some(ElementDefinitionFixed::Contributor(
                                map_access.next_value()?,
                            ));
                        }
                        Field::FixedDataRequirement => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedDataRequirement",
                                ));
                            }
                            r#fixed = Some(ElementDefinitionFixed::DataRequirement(
                                map_access.next_value()?,
                            ));
                        }
                        Field::FixedExpression => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedExpression"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Expression(map_access.next_value()?));
                        }
                        Field::FixedParameterDefinition => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedParameterDefinition",
                                ));
                            }
                            r#fixed = Some(ElementDefinitionFixed::ParameterDefinition(
                                map_access.next_value()?,
                            ));
                        }
                        Field::FixedRelatedArtifact => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedRelatedArtifact",
                                ));
                            }
                            r#fixed = Some(ElementDefinitionFixed::RelatedArtifact(
                                map_access.next_value()?,
                            ));
                        }
                        Field::FixedTriggerDefinition => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedTriggerDefinition",
                                ));
                            }
                            r#fixed = Some(ElementDefinitionFixed::TriggerDefinition(
                                map_access.next_value()?,
                            ));
                        }
                        Field::FixedUsageContext => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedUsageContext"));
                            }
                            r#fixed = Some(ElementDefinitionFixed::UsageContext(
                                map_access.next_value()?,
                            ));
                        }
                        Field::FixedDosage => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedDosage"));
                            }
                            r#fixed =
                                Some(ElementDefinitionFixed::Dosage(map_access.next_value()?));
                        }
                        Field::FixedMeta => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedMeta"));
                            }
                            r#fixed = Some(ElementDefinitionFixed::Meta(map_access.next_value()?));
                        }
                        Field::PatternBase64Binary => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Base64Binary(Default::default()),
                            );
                            if let ElementDefinitionPattern::Base64Binary(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternBase64Binary",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternBase64BinaryPrimitiveElement => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Base64Binary(Default::default()),
                            );
                            if let ElementDefinitionPattern::Base64Binary(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patternBase64Binary",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternBoolean => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Boolean(Default::default()),
                            );
                            if let ElementDefinitionPattern::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternBoolean",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternBooleanPrimitiveElement => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Boolean(Default::default()),
                            );
                            if let ElementDefinitionPattern::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patternBoolean",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternCanonical => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Canonical(Default::default()),
                            );
                            if let ElementDefinitionPattern::Canonical(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternCanonical",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternCanonicalPrimitiveElement => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Canonical(Default::default()),
                            );
                            if let ElementDefinitionPattern::Canonical(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patternCanonical",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternCode => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Code(Default::default()));
                            if let ElementDefinitionPattern::Code(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternCode"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternCodePrimitiveElement => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Code(Default::default()));
                            if let ElementDefinitionPattern::Code(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_patternCode"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternDate => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Date(Default::default()));
                            if let ElementDefinitionPattern::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternDatePrimitiveElement => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Date(Default::default()));
                            if let ElementDefinitionPattern::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_patternDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternDateTime => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::DateTime(Default::default()),
                            );
                            if let ElementDefinitionPattern::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternDateTimePrimitiveElement => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::DateTime(Default::default()),
                            );
                            if let ElementDefinitionPattern::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patternDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternDecimal => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Decimal(Default::default()),
                            );
                            if let ElementDefinitionPattern::Decimal(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternDecimal",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternDecimalPrimitiveElement => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Decimal(Default::default()),
                            );
                            if let ElementDefinitionPattern::Decimal(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patternDecimal",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternId => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Id(Default::default()));
                            if let ElementDefinitionPattern::Id(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternId"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternIdPrimitiveElement => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Id(Default::default()));
                            if let ElementDefinitionPattern::Id(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_patternId"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternInstant => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Instant(Default::default()),
                            );
                            if let ElementDefinitionPattern::Instant(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternInstant",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternInstantPrimitiveElement => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Instant(Default::default()),
                            );
                            if let ElementDefinitionPattern::Instant(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patternInstant",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternInteger => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Integer(Default::default()),
                            );
                            if let ElementDefinitionPattern::Integer(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternInteger",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternIntegerPrimitiveElement => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Integer(Default::default()),
                            );
                            if let ElementDefinitionPattern::Integer(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patternInteger",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternMarkdown => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Markdown(Default::default()),
                            );
                            if let ElementDefinitionPattern::Markdown(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternMarkdown",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternMarkdownPrimitiveElement => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::Markdown(Default::default()),
                            );
                            if let ElementDefinitionPattern::Markdown(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patternMarkdown",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternOid => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Oid(Default::default()));
                            if let ElementDefinitionPattern::Oid(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternOid"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternOidPrimitiveElement => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Oid(Default::default()));
                            if let ElementDefinitionPattern::Oid(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_patternOid"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternPositiveInt => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::PositiveInt(Default::default()),
                            );
                            if let ElementDefinitionPattern::PositiveInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternPositiveInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternPositiveIntPrimitiveElement => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::PositiveInt(Default::default()),
                            );
                            if let ElementDefinitionPattern::PositiveInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patternPositiveInt",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternString => {
                            let r#enum = r#pattern.get_or_insert(ElementDefinitionPattern::String(
                                Default::default(),
                            ));
                            if let ElementDefinitionPattern::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternStringPrimitiveElement => {
                            let r#enum = r#pattern.get_or_insert(ElementDefinitionPattern::String(
                                Default::default(),
                            ));
                            if let ElementDefinitionPattern::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patternString",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternTime => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Time(Default::default()));
                            if let ElementDefinitionPattern::Time(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternTimePrimitiveElement => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Time(Default::default()));
                            if let ElementDefinitionPattern::Time(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_patternTime"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternUnsignedInt => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::UnsignedInt(Default::default()),
                            );
                            if let ElementDefinitionPattern::UnsignedInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternUnsignedInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternUnsignedIntPrimitiveElement => {
                            let r#enum = r#pattern.get_or_insert(
                                ElementDefinitionPattern::UnsignedInt(Default::default()),
                            );
                            if let ElementDefinitionPattern::UnsignedInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patternUnsignedInt",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternUri => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Uri(Default::default()));
                            if let ElementDefinitionPattern::Uri(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternUri"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternUriPrimitiveElement => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Uri(Default::default()));
                            if let ElementDefinitionPattern::Uri(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_patternUri"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternUrl => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Url(Default::default()));
                            if let ElementDefinitionPattern::Url(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternUrl"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternUrlPrimitiveElement => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Url(Default::default()));
                            if let ElementDefinitionPattern::Url(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_patternUrl"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternUuid => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Uuid(Default::default()));
                            if let ElementDefinitionPattern::Uuid(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternUuid"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("pattern[x]"));
                            }
                        }
                        Field::PatternUuidPrimitiveElement => {
                            let r#enum = r#pattern
                                .get_or_insert(ElementDefinitionPattern::Uuid(Default::default()));
                            if let ElementDefinitionPattern::Uuid(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_patternUuid"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                            }
                        }
                        Field::PatternAddress => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternAddress"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Address(map_access.next_value()?));
                        }
                        Field::PatternAge => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternAge"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Age(map_access.next_value()?));
                        }
                        Field::PatternAnnotation => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternAnnotation"));
                            }
                            r#pattern = Some(ElementDefinitionPattern::Annotation(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternAttachment => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternAttachment"));
                            }
                            r#pattern = Some(ElementDefinitionPattern::Attachment(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternCodeableConcept => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternCodeableConcept",
                                ));
                            }
                            r#pattern = Some(ElementDefinitionPattern::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternCoding => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternCoding"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Coding(map_access.next_value()?));
                        }
                        Field::PatternContactPoint => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternContactPoint",
                                ));
                            }
                            r#pattern = Some(ElementDefinitionPattern::ContactPoint(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternCount => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternCount"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Count(map_access.next_value()?));
                        }
                        Field::PatternDistance => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternDistance"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Distance(map_access.next_value()?));
                        }
                        Field::PatternDuration => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternDuration"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Duration(map_access.next_value()?));
                        }
                        Field::PatternHumanName => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternHumanName"));
                            }
                            r#pattern = Some(ElementDefinitionPattern::HumanName(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternIdentifier => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternIdentifier"));
                            }
                            r#pattern = Some(ElementDefinitionPattern::Identifier(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternMoney => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternMoney"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Money(map_access.next_value()?));
                        }
                        Field::PatternPeriod => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternPeriod"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Period(map_access.next_value()?));
                        }
                        Field::PatternQuantity => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternQuantity"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Quantity(map_access.next_value()?));
                        }
                        Field::PatternRange => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternRange"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Range(map_access.next_value()?));
                        }
                        Field::PatternRatio => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternRatio"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Ratio(map_access.next_value()?));
                        }
                        Field::PatternReference => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternReference"));
                            }
                            r#pattern = Some(ElementDefinitionPattern::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternSampledData => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternSampledData",
                                ));
                            }
                            r#pattern = Some(ElementDefinitionPattern::SampledData(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternSignature => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternSignature"));
                            }
                            r#pattern = Some(ElementDefinitionPattern::Signature(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternTiming => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternTiming"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Timing(map_access.next_value()?));
                        }
                        Field::PatternContactDetail => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternContactDetail",
                                ));
                            }
                            r#pattern = Some(ElementDefinitionPattern::ContactDetail(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternContributor => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternContributor",
                                ));
                            }
                            r#pattern = Some(ElementDefinitionPattern::Contributor(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternDataRequirement => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternDataRequirement",
                                ));
                            }
                            r#pattern = Some(ElementDefinitionPattern::DataRequirement(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternExpression => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternExpression"));
                            }
                            r#pattern = Some(ElementDefinitionPattern::Expression(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternParameterDefinition => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternParameterDefinition",
                                ));
                            }
                            r#pattern = Some(ElementDefinitionPattern::ParameterDefinition(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternRelatedArtifact => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternRelatedArtifact",
                                ));
                            }
                            r#pattern = Some(ElementDefinitionPattern::RelatedArtifact(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternTriggerDefinition => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternTriggerDefinition",
                                ));
                            }
                            r#pattern = Some(ElementDefinitionPattern::TriggerDefinition(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternUsageContext => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternUsageContext",
                                ));
                            }
                            r#pattern = Some(ElementDefinitionPattern::UsageContext(
                                map_access.next_value()?,
                            ));
                        }
                        Field::PatternDosage => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternDosage"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Dosage(map_access.next_value()?));
                        }
                        Field::PatternMeta => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternMeta"));
                            }
                            r#pattern =
                                Some(ElementDefinitionPattern::Meta(map_access.next_value()?));
                        }
                        Field::Example => {
                            if r#example.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            r#example = Some(map_access.next_value()?);
                        }
                        Field::MinValueDate => {
                            let r#enum = r#min_value
                                .get_or_insert(ElementDefinitionMinValue::Date(Default::default()));
                            if let ElementDefinitionMinValue::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("minValueDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("minValue[x]"));
                            }
                        }
                        Field::MinValueDatePrimitiveElement => {
                            let r#enum = r#min_value
                                .get_or_insert(ElementDefinitionMinValue::Date(Default::default()));
                            if let ElementDefinitionMinValue::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_minValueDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                            }
                        }
                        Field::MinValueDateTime => {
                            let r#enum = r#min_value.get_or_insert(
                                ElementDefinitionMinValue::DateTime(Default::default()),
                            );
                            if let ElementDefinitionMinValue::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("minValue[x]"));
                            }
                        }
                        Field::MinValueDateTimePrimitiveElement => {
                            let r#enum = r#min_value.get_or_insert(
                                ElementDefinitionMinValue::DateTime(Default::default()),
                            );
                            if let ElementDefinitionMinValue::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_minValueDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                            }
                        }
                        Field::MinValueInstant => {
                            let r#enum = r#min_value.get_or_insert(
                                ElementDefinitionMinValue::Instant(Default::default()),
                            );
                            if let ElementDefinitionMinValue::Instant(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueInstant",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("minValue[x]"));
                            }
                        }
                        Field::MinValueInstantPrimitiveElement => {
                            let r#enum = r#min_value.get_or_insert(
                                ElementDefinitionMinValue::Instant(Default::default()),
                            );
                            if let ElementDefinitionMinValue::Instant(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_minValueInstant",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                            }
                        }
                        Field::MinValueTime => {
                            let r#enum = r#min_value
                                .get_or_insert(ElementDefinitionMinValue::Time(Default::default()));
                            if let ElementDefinitionMinValue::Time(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("minValueTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("minValue[x]"));
                            }
                        }
                        Field::MinValueTimePrimitiveElement => {
                            let r#enum = r#min_value
                                .get_or_insert(ElementDefinitionMinValue::Time(Default::default()));
                            if let ElementDefinitionMinValue::Time(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_minValueTime"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                            }
                        }
                        Field::MinValueDecimal => {
                            let r#enum = r#min_value.get_or_insert(
                                ElementDefinitionMinValue::Decimal(Default::default()),
                            );
                            if let ElementDefinitionMinValue::Decimal(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueDecimal",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("minValue[x]"));
                            }
                        }
                        Field::MinValueDecimalPrimitiveElement => {
                            let r#enum = r#min_value.get_or_insert(
                                ElementDefinitionMinValue::Decimal(Default::default()),
                            );
                            if let ElementDefinitionMinValue::Decimal(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_minValueDecimal",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                            }
                        }
                        Field::MinValueInteger => {
                            let r#enum = r#min_value.get_or_insert(
                                ElementDefinitionMinValue::Integer(Default::default()),
                            );
                            if let ElementDefinitionMinValue::Integer(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueInteger",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("minValue[x]"));
                            }
                        }
                        Field::MinValueIntegerPrimitiveElement => {
                            let r#enum = r#min_value.get_or_insert(
                                ElementDefinitionMinValue::Integer(Default::default()),
                            );
                            if let ElementDefinitionMinValue::Integer(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_minValueInteger",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                            }
                        }
                        Field::MinValuePositiveInt => {
                            let r#enum = r#min_value.get_or_insert(
                                ElementDefinitionMinValue::PositiveInt(Default::default()),
                            );
                            if let ElementDefinitionMinValue::PositiveInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValuePositiveInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("minValue[x]"));
                            }
                        }
                        Field::MinValuePositiveIntPrimitiveElement => {
                            let r#enum = r#min_value.get_or_insert(
                                ElementDefinitionMinValue::PositiveInt(Default::default()),
                            );
                            if let ElementDefinitionMinValue::PositiveInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_minValuePositiveInt",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                            }
                        }
                        Field::MinValueUnsignedInt => {
                            let r#enum = r#min_value.get_or_insert(
                                ElementDefinitionMinValue::UnsignedInt(Default::default()),
                            );
                            if let ElementDefinitionMinValue::UnsignedInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueUnsignedInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("minValue[x]"));
                            }
                        }
                        Field::MinValueUnsignedIntPrimitiveElement => {
                            let r#enum = r#min_value.get_or_insert(
                                ElementDefinitionMinValue::UnsignedInt(Default::default()),
                            );
                            if let ElementDefinitionMinValue::UnsignedInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_minValueUnsignedInt",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                            }
                        }
                        Field::MinValueQuantity => {
                            if r#min_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("minValueQuantity"));
                            }
                            r#min_value = Some(ElementDefinitionMinValue::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                        Field::MaxValueDate => {
                            let r#enum = r#max_value
                                .get_or_insert(ElementDefinitionMaxValue::Date(Default::default()));
                            if let ElementDefinitionMaxValue::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxValueDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                            }
                        }
                        Field::MaxValueDatePrimitiveElement => {
                            let r#enum = r#max_value
                                .get_or_insert(ElementDefinitionMaxValue::Date(Default::default()));
                            if let ElementDefinitionMaxValue::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_maxValueDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                            }
                        }
                        Field::MaxValueDateTime => {
                            let r#enum = r#max_value.get_or_insert(
                                ElementDefinitionMaxValue::DateTime(Default::default()),
                            );
                            if let ElementDefinitionMaxValue::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                            }
                        }
                        Field::MaxValueDateTimePrimitiveElement => {
                            let r#enum = r#max_value.get_or_insert(
                                ElementDefinitionMaxValue::DateTime(Default::default()),
                            );
                            if let ElementDefinitionMaxValue::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_maxValueDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                            }
                        }
                        Field::MaxValueInstant => {
                            let r#enum = r#max_value.get_or_insert(
                                ElementDefinitionMaxValue::Instant(Default::default()),
                            );
                            if let ElementDefinitionMaxValue::Instant(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueInstant",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                            }
                        }
                        Field::MaxValueInstantPrimitiveElement => {
                            let r#enum = r#max_value.get_or_insert(
                                ElementDefinitionMaxValue::Instant(Default::default()),
                            );
                            if let ElementDefinitionMaxValue::Instant(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_maxValueInstant",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                            }
                        }
                        Field::MaxValueTime => {
                            let r#enum = r#max_value
                                .get_or_insert(ElementDefinitionMaxValue::Time(Default::default()));
                            if let ElementDefinitionMaxValue::Time(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxValueTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                            }
                        }
                        Field::MaxValueTimePrimitiveElement => {
                            let r#enum = r#max_value
                                .get_or_insert(ElementDefinitionMaxValue::Time(Default::default()));
                            if let ElementDefinitionMaxValue::Time(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_maxValueTime"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                            }
                        }
                        Field::MaxValueDecimal => {
                            let r#enum = r#max_value.get_or_insert(
                                ElementDefinitionMaxValue::Decimal(Default::default()),
                            );
                            if let ElementDefinitionMaxValue::Decimal(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueDecimal",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                            }
                        }
                        Field::MaxValueDecimalPrimitiveElement => {
                            let r#enum = r#max_value.get_or_insert(
                                ElementDefinitionMaxValue::Decimal(Default::default()),
                            );
                            if let ElementDefinitionMaxValue::Decimal(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_maxValueDecimal",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                            }
                        }
                        Field::MaxValueInteger => {
                            let r#enum = r#max_value.get_or_insert(
                                ElementDefinitionMaxValue::Integer(Default::default()),
                            );
                            if let ElementDefinitionMaxValue::Integer(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueInteger",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                            }
                        }
                        Field::MaxValueIntegerPrimitiveElement => {
                            let r#enum = r#max_value.get_or_insert(
                                ElementDefinitionMaxValue::Integer(Default::default()),
                            );
                            if let ElementDefinitionMaxValue::Integer(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_maxValueInteger",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                            }
                        }
                        Field::MaxValuePositiveInt => {
                            let r#enum = r#max_value.get_or_insert(
                                ElementDefinitionMaxValue::PositiveInt(Default::default()),
                            );
                            if let ElementDefinitionMaxValue::PositiveInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValuePositiveInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                            }
                        }
                        Field::MaxValuePositiveIntPrimitiveElement => {
                            let r#enum = r#max_value.get_or_insert(
                                ElementDefinitionMaxValue::PositiveInt(Default::default()),
                            );
                            if let ElementDefinitionMaxValue::PositiveInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_maxValuePositiveInt",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                            }
                        }
                        Field::MaxValueUnsignedInt => {
                            let r#enum = r#max_value.get_or_insert(
                                ElementDefinitionMaxValue::UnsignedInt(Default::default()),
                            );
                            if let ElementDefinitionMaxValue::UnsignedInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueUnsignedInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                            }
                        }
                        Field::MaxValueUnsignedIntPrimitiveElement => {
                            let r#enum = r#max_value.get_or_insert(
                                ElementDefinitionMaxValue::UnsignedInt(Default::default()),
                            );
                            if let ElementDefinitionMaxValue::UnsignedInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_maxValueUnsignedInt",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                            }
                        }
                        Field::MaxValueQuantity => {
                            if r#max_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxValueQuantity"));
                            }
                            r#max_value = Some(ElementDefinitionMaxValue::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                        Field::MaxLength => {
                            let some = r#max_length.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLength"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::MaxLengthPrimitiveElement => {
                            let some = r#max_length.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_maxLength"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Condition => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#condition.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::ConditionPrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
                            let vec = r#condition.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("_condition"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                if let Some(element) = element {
                                    vec[i].id = element.id;
                                    vec[i].extension = element.extension;
                                }
                            }
                        }
                        Field::Constraint => {
                            if r#constraint.is_some() {
                                return Err(serde::de::Error::duplicate_field("constraint"));
                            }
                            r#constraint = Some(map_access.next_value()?);
                        }
                        Field::MustSupport => {
                            let some = r#must_support.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("mustSupport"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::MustSupportPrimitiveElement => {
                            let some = r#must_support.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_mustSupport"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::IsModifier => {
                            let some = r#is_modifier.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("isModifier"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IsModifierPrimitiveElement => {
                            let some = r#is_modifier.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_isModifier"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::IsModifierReason => {
                            let some = r#is_modifier_reason.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("isModifierReason"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IsModifierReasonPrimitiveElement => {
                            let some = r#is_modifier_reason.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_isModifierReason"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::IsSummary => {
                            let some = r#is_summary.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSummary"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IsSummaryPrimitiveElement => {
                            let some = r#is_summary.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_isSummary"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Binding => {
                            if r#binding.is_some() {
                                return Err(serde::de::Error::duplicate_field("binding"));
                            }
                            r#binding = Some(map_access.next_value()?);
                        }
                        Field::Mapping => {
                            if r#mapping.is_some() {
                                return Err(serde::de::Error::duplicate_field("mapping"));
                            }
                            r#mapping = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ElementDefinition {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#path: r#path.ok_or(serde::de::Error::missing_field("path"))?,
                    r#representation: r#representation.unwrap_or(vec![]),
                    r#slice_name,
                    r#slice_is_constraining,
                    r#label,
                    r#code: r#code.unwrap_or(vec![]),
                    r#slicing,
                    r#short,
                    r#definition,
                    r#comment,
                    r#requirements,
                    r#alias: r#alias.unwrap_or(vec![]),
                    r#min,
                    r#max,
                    r#base,
                    r#content_reference,
                    r#type: r#type.unwrap_or(vec![]),
                    r#default_value,
                    r#meaning_when_missing,
                    r#order_meaning,
                    r#fixed,
                    r#pattern,
                    r#example: r#example.unwrap_or(vec![]),
                    r#min_value,
                    r#max_value,
                    r#max_length,
                    r#condition: r#condition.unwrap_or(vec![]),
                    r#constraint: r#constraint.unwrap_or(vec![]),
                    r#must_support,
                    r#is_modifier,
                    r#is_modifier_reason,
                    r#is_summary,
                    r#binding,
                    r#mapping: r#mapping.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
