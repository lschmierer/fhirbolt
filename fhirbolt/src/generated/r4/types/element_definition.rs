// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
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
}
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
}
#[derive(Debug, Clone)]
pub struct ElementDefinitionBinding {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#value_set: Option<super::super::types::Canonical>,
    pub r#strength: super::super::types::Code,
}
impl serde::Serialize for ElementDefinitionBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#value_set.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("valueSet", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_valueSet", &primitive_element)?;
            }
        }
        {
            if let Some(some) = self.r#strength.value.as_ref() {
                state.serialize_entry("strength", some)?;
            }
            if self.r#strength.id.is_some() || !self.r#strength.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#strength.id,
                    extension: &self.r#strength.extension,
                };
                state.serialize_entry("_strength", &primitive_element)?;
            }
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct ElementDefinitionMapping {
    pub r#identity: super::super::types::Id,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#map: super::super::types::String,
    pub r#comment: Option<super::super::types::String>,
}
impl serde::Serialize for ElementDefinitionMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        {
            if let Some(some) = self.r#identity.value.as_ref() {
                state.serialize_entry("identity", some)?;
            }
            if self.r#identity.id.is_some() || !self.r#identity.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#identity.id,
                    extension: &self.r#identity.extension,
                };
                state.serialize_entry("_identity", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if let Some(some) = self.r#language.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("language", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
            }
        }
        {
            if let Some(some) = self.r#map.value.as_ref() {
                state.serialize_entry("map", some)?;
            }
            if self.r#map.id.is_some() || !self.r#map.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#map.id,
                    extension: &self.r#map.extension,
                };
                state.serialize_entry("_map", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#comment.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("comment", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_comment", &primitive_element)?;
            }
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct ElementDefinitionSlicingDiscriminator {
    pub r#type: super::super::types::Code,
    pub r#path: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
impl serde::Serialize for ElementDefinitionSlicingDiscriminator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        {
            if let Some(some) = self.r#type.value.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#type.id,
                    extension: &self.r#type.extension,
                };
                state.serialize_entry("_type", &primitive_element)?;
            }
        }
        {
            if let Some(some) = self.r#path.value.as_ref() {
                state.serialize_entry("path", some)?;
            }
            if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#path.id,
                    extension: &self.r#path.extension,
                };
                state.serialize_entry("_path", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct ElementDefinitionSlicing {
    pub r#id: Option<std::string::String>,
    pub r#rules: super::super::types::Code,
    pub r#description: Option<super::super::types::String>,
    pub r#ordered: Option<super::super::types::Boolean>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#discriminator: Vec<Box<super::super::types::Element>>,
}
impl serde::Serialize for ElementDefinitionSlicing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        {
            if let Some(some) = self.r#rules.value.as_ref() {
                state.serialize_entry("rules", some)?;
            }
            if self.r#rules.id.is_some() || !self.r#rules.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#rules.id,
                    extension: &self.r#rules.extension,
                };
                state.serialize_entry("_rules", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
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
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_ordered", &primitive_element)?;
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#discriminator.is_empty() {
            state.serialize_entry("discriminator", &self.r#discriminator)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct ElementDefinitionBase {
    pub r#path: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#max: super::super::types::String,
    pub r#min: super::super::types::UnsignedInt,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
impl serde::Serialize for ElementDefinitionBase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        {
            if let Some(some) = self.r#path.value.as_ref() {
                state.serialize_entry("path", some)?;
            }
            if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#path.id,
                    extension: &self.r#path.extension,
                };
                state.serialize_entry("_path", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        {
            if let Some(some) = self.r#max.value.as_ref() {
                state.serialize_entry("max", some)?;
            }
            if self.r#max.id.is_some() || !self.r#max.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#max.id,
                    extension: &self.r#max.extension,
                };
                state.serialize_entry("_max", &primitive_element)?;
            }
        }
        {
            if let Some(some) = self.r#min.value.as_ref() {
                state.serialize_entry("min", some)?;
            }
            if self.r#min.id.is_some() || !self.r#min.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#min.id,
                    extension: &self.r#min.extension,
                };
                state.serialize_entry("_min", &primitive_element)?;
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct ElementDefinitionType {
    pub r#aggregation: Vec<super::super::types::Code>,
    pub r#target_profile: Vec<super::super::types::Canonical>,
    pub r#profile: Vec<super::super::types::Canonical>,
    pub r#versioning: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#code: super::super::types::Uri,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
impl serde::Serialize for ElementDefinitionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
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
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_elements: Vec<_> = self
                    .r#aggregation
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(PrimtiveElement {
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
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_elements: Vec<_> = self
                    .r#target_profile
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(PrimtiveElement {
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
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_elements: Vec<_> = self
                    .r#profile
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(PrimtiveElement {
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
        if let Some(some) = self.r#versioning.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("versioning", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_versioning", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        {
            if let Some(some) = self.r#code.value.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#code.id,
                    extension: &self.r#code.extension,
                };
                state.serialize_entry("_code", &primitive_element)?;
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct ElementDefinitionExample {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#label: super::super::types::String,
    pub r#value: ElementDefinitionExampleValue,
}
impl serde::Serialize for ElementDefinitionExample {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        {
            if let Some(some) = self.r#label.value.as_ref() {
                state.serialize_entry("label", some)?;
            }
            if self.r#label.id.is_some() || !self.r#label.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#label.id,
                    extension: &self.r#label.extension,
                };
                state.serialize_entry("_label", &primitive_element)?;
            }
        }
        match self.r#value {
            ElementDefinitionExampleValue::Base64Binary(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueBase64Binary", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
                    #[derive(serde :: Serialize)]
                    struct PrimtiveElement<'a> {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        id: &'a Option<std::string::String>,
                        #[serde(skip_serializing_if = "<[_]>::is_empty")]
                        extension: &'a [Box<super::super::types::Extension>],
                    }
                    let primitive_element = PrimtiveElement {
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
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct ElementDefinitionConstraint {
    pub r#source: Option<super::super::types::Canonical>,
    pub r#key: super::super::types::Id,
    pub r#requirements: Option<super::super::types::String>,
    pub r#human: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#xpath: Option<super::super::types::String>,
    pub r#expression: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#severity: super::super::types::Code,
}
impl serde::Serialize for ElementDefinitionConstraint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#source.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("source", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_source", &primitive_element)?;
            }
        }
        {
            if let Some(some) = self.r#key.value.as_ref() {
                state.serialize_entry("key", some)?;
            }
            if self.r#key.id.is_some() || !self.r#key.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#key.id,
                    extension: &self.r#key.extension,
                };
                state.serialize_entry("_key", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#requirements.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("requirements", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_requirements", &primitive_element)?;
            }
        }
        {
            if let Some(some) = self.r#human.value.as_ref() {
                state.serialize_entry("human", some)?;
            }
            if self.r#human.id.is_some() || !self.r#human.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#human.id,
                    extension: &self.r#human.extension,
                };
                state.serialize_entry("_human", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#xpath.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("xpath", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_xpath", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#expression.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("expression", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_expression", &primitive_element)?;
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        {
            if let Some(some) = self.r#severity.value.as_ref() {
                state.serialize_entry("severity", some)?;
            }
            if self.r#severity.id.is_some() || !self.r#severity.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#severity.id,
                    extension: &self.r#severity.extension,
                };
                state.serialize_entry("_severity", &primitive_element)?;
            }
        }
        state.end()
    }
}
#[derive(Debug, Clone)]
pub struct ElementDefinition {
    pub r#is_modifier: Option<super::super::types::Boolean>,
    pub r#is_summary: Option<super::super::types::Boolean>,
    pub r#pattern: Option<ElementDefinitionPattern>,
    pub r#max_value: Option<ElementDefinitionMaxValue>,
    pub r#max: Option<super::super::types::String>,
    pub r#content_reference: Option<super::super::types::Uri>,
    pub r#binding: Option<Box<super::super::types::Element>>,
    pub r#slice_is_constraining: Option<super::super::types::Boolean>,
    pub r#id: Option<std::string::String>,
    pub r#default_value: Option<ElementDefinitionDefaultValue>,
    pub r#meaning_when_missing: Option<super::super::types::Markdown>,
    pub r#order_meaning: Option<super::super::types::String>,
    pub r#alias: Vec<super::super::types::String>,
    pub r#is_modifier_reason: Option<super::super::types::String>,
    pub r#mapping: Vec<Box<super::super::types::Element>>,
    pub r#slicing: Option<Box<super::super::types::Element>>,
    pub r#max_length: Option<super::super::types::Integer>,
    pub r#base: Option<Box<super::super::types::Element>>,
    pub r#representation: Vec<super::super::types::Code>,
    pub r#label: Option<super::super::types::String>,
    pub r#requirements: Option<super::super::types::Markdown>,
    pub r#slice_name: Option<super::super::types::String>,
    pub r#type: Vec<Box<super::super::types::Element>>,
    pub r#min_value: Option<ElementDefinitionMinValue>,
    pub r#definition: Option<super::super::types::Markdown>,
    pub r#example: Vec<Box<super::super::types::Element>>,
    pub r#fixed: Option<ElementDefinitionFixed>,
    pub r#code: Vec<Box<super::super::types::Coding>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#must_support: Option<super::super::types::Boolean>,
    pub r#comment: Option<super::super::types::Markdown>,
    pub r#short: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: super::super::types::String,
    pub r#min: Option<super::super::types::UnsignedInt>,
    pub r#condition: Vec<super::super::types::Id>,
    pub r#constraint: Vec<Box<super::super::types::Element>>,
}
impl serde::Serialize for ElementDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#is_modifier.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isModifier", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isModifier", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#is_summary.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isSummary", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isSummary", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#pattern.as_ref() {
            match some {
                ElementDefinitionPattern::Base64Binary(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("patternBase64Binary", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
            }
        }
        if let Some(some) = self.r#max_value.as_ref() {
            match some {
                ElementDefinitionMaxValue::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("maxValueDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_maxValueUnsignedInt", &primitive_element)?;
                    }
                }
                ElementDefinitionMaxValue::Quantity(ref value) => {
                    state.serialize_entry("maxValueQuantity", value)?;
                }
            }
        }
        if let Some(some) = self.r#max.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("max", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_max", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#content_reference.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("contentReference", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_contentReference", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#binding.as_ref() {
            state.serialize_entry("binding", some)?;
        }
        if let Some(some) = self.r#slice_is_constraining.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("sliceIsConstraining", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_sliceIsConstraining", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#default_value.as_ref() {
            match some {
                ElementDefinitionDefaultValue::Base64Binary(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("defaultValueBase64Binary", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
            }
        }
        if let Some(some) = self.r#meaning_when_missing.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("meaningWhenMissing", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
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
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_orderMeaning", &primitive_element)?;
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
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_elements: Vec<_> = self
                    .r#alias
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(PrimtiveElement {
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
        if let Some(some) = self.r#is_modifier_reason.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isModifierReason", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isModifierReason", &primitive_element)?;
            }
        }
        if !self.r#mapping.is_empty() {
            state.serialize_entry("mapping", &self.r#mapping)?;
        }
        if let Some(some) = self.r#slicing.as_ref() {
            state.serialize_entry("slicing", some)?;
        }
        if let Some(some) = self.r#max_length.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("maxLength", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_maxLength", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#base.as_ref() {
            state.serialize_entry("base", some)?;
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
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_elements: Vec<_> = self
                    .r#representation
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(PrimtiveElement {
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
        if let Some(some) = self.r#label.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("label", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_label", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#requirements.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("requirements", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_requirements", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#slice_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("sliceName", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_sliceName", &primitive_element)?;
            }
        }
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
        }
        if let Some(some) = self.r#min_value.as_ref() {
            match some {
                ElementDefinitionMinValue::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("minValueDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_minValueUnsignedInt", &primitive_element)?;
                    }
                }
                ElementDefinitionMinValue::Quantity(ref value) => {
                    state.serialize_entry("minValueQuantity", value)?;
                }
            }
        }
        if let Some(some) = self.r#definition.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("definition", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_definition", &primitive_element)?;
            }
        }
        if !self.r#example.is_empty() {
            state.serialize_entry("example", &self.r#example)?;
        }
        if let Some(some) = self.r#fixed.as_ref() {
            match some {
                ElementDefinitionFixed::Base64Binary(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("fixedBase64Binary", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
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
            }
        }
        if !self.r#code.is_empty() {
            state.serialize_entry("code", &self.r#code)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#must_support.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("mustSupport", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_mustSupport", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#comment.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("comment", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_comment", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#short.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("short", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_short", &primitive_element)?;
            }
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        {
            if let Some(some) = self.r#path.value.as_ref() {
                state.serialize_entry("path", some)?;
            }
            if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &self.r#path.id,
                    extension: &self.r#path.extension,
                };
                state.serialize_entry("_path", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#min.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("min", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_min", &primitive_element)?;
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
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_elements: Vec<_> = self
                    .r#condition
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(PrimtiveElement {
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
        state.end()
    }
}
