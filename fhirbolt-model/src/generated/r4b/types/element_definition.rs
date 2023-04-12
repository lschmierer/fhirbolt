// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The value that should be used if there is no value stated in the instance (e.g. 'if not otherwise specified, the abstract is false')."]
#[derive(Debug, Clone, PartialEq)]
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
    CodeableReference(Box<super::super::types::CodeableReference>),
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
    RatioRange(Box<super::super::types::RatioRange>),
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
    Invalid,
}
impl Default for ElementDefinitionDefaultValue {
    fn default() -> ElementDefinitionDefaultValue {
        ElementDefinitionDefaultValue::Invalid
    }
}
#[doc = "Specifies a value that SHALL be exactly the value  for this element in the instance. For purposes of comparison, non-significant whitespace is ignored, and all values must be an exact match (case and accent sensitive). Missing elements/attributes must also be missing."]
#[derive(Debug, Clone, PartialEq)]
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
    CodeableReference(Box<super::super::types::CodeableReference>),
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
    RatioRange(Box<super::super::types::RatioRange>),
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
    Invalid,
}
impl Default for ElementDefinitionFixed {
    fn default() -> ElementDefinitionFixed {
        ElementDefinitionFixed::Invalid
    }
}
#[doc = "Specifies a value that the value in the instance SHALL follow - that is, any value in the pattern must be found in the instance. Other additional values may be found too. This is effectively constraint by example.  \n\nWhen pattern\\[x\\] is used to constrain a primitive, it means that the value provided in the pattern\\[x\\] must match the instance value exactly.\n\nWhen pattern\\[x\\] is used to constrain an array, it means that each element provided in the pattern\\[x\\] array must (recursively) match at least one element from the instance array.\n\nWhen pattern\\[x\\] is used to constrain a complex object, it means that each property in the pattern must be present in the complex object, and its value must recursively match -- i.e.,\n\n1. If primitive: it must match exactly the pattern value\n2. If a complex object: it must match (recursively) the pattern value\n3. If an array: it must match (recursively) the pattern value."]
#[derive(Debug, Clone, PartialEq)]
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
    CodeableReference(Box<super::super::types::CodeableReference>),
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
    RatioRange(Box<super::super::types::RatioRange>),
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
    Invalid,
}
impl Default for ElementDefinitionPattern {
    fn default() -> ElementDefinitionPattern {
        ElementDefinitionPattern::Invalid
    }
}
#[doc = "The actual value for the element, which must be one of the types allowed for this element."]
#[derive(Debug, Clone, PartialEq)]
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
    CodeableReference(Box<super::super::types::CodeableReference>),
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
    RatioRange(Box<super::super::types::RatioRange>),
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
    Invalid,
}
impl Default for ElementDefinitionExampleValue {
    fn default() -> ElementDefinitionExampleValue {
        ElementDefinitionExampleValue::Invalid
    }
}
#[doc = "The minimum allowed value for the element. The value is inclusive. This is allowed for the types date, dateTime, instant, time, decimal, integer, and Quantity."]
#[derive(Debug, Clone, PartialEq)]
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
#[doc = "The maximum allowed value for the element. The value is inclusive. This is allowed for the types date, dateTime, instant, time, decimal, integer, and Quantity."]
#[derive(Debug, Clone, PartialEq)]
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
#[doc = "Designates which child elements are used to discriminate between the slices when processing an instance. If one or more discriminators are provided, the value of the child elements in the instance data SHALL completely distinguish which slice the element in the resource matches based on the allowed values for those elements in each of the slices."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElementDefinitionSlicingDiscriminator {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "How the element value is interpreted when discrimination is evaluated."]
    pub r#type: super::super::types::Code,
    #[doc = "A FHIRPath expression, using [the simple subset of FHIRPath](fhirpath.html#simple), that is used to identify the element on which discrimination is based."]
    pub r#path: super::super::types::String,
}
impl serde::ser::Serialize for ElementDefinitionSlicingDiscriminator {
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
            if _ctx.output_json {
                if let Some(some) = self.r#path.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("path", &some)?;
                }
                if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#path.id.as_ref(),
                        extension: &self.r#path.extension,
                    };
                    state.serialize_entry("_path", &primitive_element)?;
                }
            } else {
                state.serialize_entry("path", &self.r#path)?;
            }
            state.end()
        })
    }
}
#[doc = "Indicates that the element is sliced into a set of alternative definitions (i.e. in a structure definition, there are multiple different constraints on a single element in the base resource). Slicing can be used in any resource that has cardinality ..* on the base resource, or any resource with a choice of types. The set of slices is any elements that come after this in the element sequence that have the same path, until a shorter path occurs (the shorter path terminates the set)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElementDefinitionSlicing {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Designates which child elements are used to discriminate between the slices when processing an instance. If one or more discriminators are provided, the value of the child elements in the instance data SHALL completely distinguish which slice the element in the resource matches based on the allowed values for those elements in each of the slices."]
    pub r#discriminator: Vec<ElementDefinitionSlicingDiscriminator>,
    #[doc = "A human-readable text description of how the slicing works. If there is no discriminator, this is required to be present to provide whatever information is possible about how the slices can be differentiated."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "If the matching elements have to occur in the same order as defined in the profile."]
    pub r#ordered: Option<super::super::types::Boolean>,
    #[doc = "Whether additional slices are allowed or not. When the slices are ordered, profile authors can also say that additional slices are only allowed at the end."]
    pub r#rules: super::super::types::Code,
}
impl serde::ser::Serialize for ElementDefinitionSlicing {
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
            if !self.r#discriminator.is_empty() {
                state.serialize_entry("discriminator", &self.r#discriminator)?;
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
                if let Some(some) = self.r#ordered.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("ordered", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_ordered", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#ordered.as_ref() {
                    state.serialize_entry("ordered", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#rules.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("rules", &some)?;
                }
                if self.r#rules.id.is_some() || !self.r#rules.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#rules.id.as_ref(),
                        extension: &self.r#rules.extension,
                    };
                    state.serialize_entry("_rules", &primitive_element)?;
                }
            } else {
                state.serialize_entry("rules", &self.r#rules)?;
            }
            state.end()
        })
    }
}
#[doc = "Information about the base definition of the element, provided to make it unnecessary for tools to trace the deviation of the element through the derived and related profiles. When the element definition is not the original definition of an element - i.g. either in a constraint on another type, or for elements from a super type in a snap shot - then the information in provided in the element definition may be different to the base definition. On the original definition of the element, it will be same."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElementDefinitionBase {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The Path that identifies the base element - this matches the ElementDefinition.path for that element. Across FHIR, there is only one base definition of any element - that is, an element definition on a [StructureDefinition](structuredefinition.html#) without a StructureDefinition.base."]
    pub r#path: super::super::types::String,
    #[doc = "Minimum cardinality of the base element identified by the path."]
    pub r#min: super::super::types::UnsignedInt,
    #[doc = "Maximum cardinality of the base element identified by the path."]
    pub r#max: super::super::types::String,
}
impl serde::ser::Serialize for ElementDefinitionBase {
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
            if _ctx.output_json {
                if let Some(some) = self.r#path.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("path", &some)?;
                }
                if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#path.id.as_ref(),
                        extension: &self.r#path.extension,
                    };
                    state.serialize_entry("_path", &primitive_element)?;
                }
            } else {
                state.serialize_entry("path", &self.r#path)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#min.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("min", &some)?;
                }
                if self.r#min.id.is_some() || !self.r#min.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#min.id.as_ref(),
                        extension: &self.r#min.extension,
                    };
                    state.serialize_entry("_min", &primitive_element)?;
                }
            } else {
                state.serialize_entry("min", &self.r#min)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#max.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("max", &some)?;
                }
                if self.r#max.id.is_some() || !self.r#max.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#max.id.as_ref(),
                        extension: &self.r#max.extension,
                    };
                    state.serialize_entry("_max", &primitive_element)?;
                }
            } else {
                state.serialize_entry("max", &self.r#max)?;
            }
            state.end()
        })
    }
}
#[doc = "The data type or resource that the value of this element is permitted to be."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElementDefinitionType {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "URL of Data type or Resource that is a(or the) type used for this element. References are URLs that are relative to <http://hl7.org/fhir/StructureDefinition> e.g. \"string\" is a reference to <http://hl7.org/fhir/StructureDefinition/string>. Absolute URLs are only allowed in logical models."]
    pub r#code: super::super::types::Uri,
    #[doc = "Identifies a profile structure or implementation Guide that applies to the datatype this element refers to. If any profiles are specified, then the content must conform to at least one of them. The URL can be a local reference - to a contained StructureDefinition, or a reference to another StructureDefinition or Implementation Guide by a canonical URL. When an implementation guide is specified, the type SHALL conform to at least one profile defined in the implementation guide."]
    pub r#profile: Vec<super::super::types::Canonical>,
    #[doc = "Used when the type is \"Reference\" or \"canonical\", and identifies a profile structure or implementation Guide that applies to the target of the reference this element refers to. If any profiles are specified, then the content must conform to at least one of them. The URL can be a local reference - to a contained StructureDefinition, or a reference to another StructureDefinition or Implementation Guide by a canonical URL. When an implementation guide is specified, the target resource SHALL conform to at least one profile defined in the implementation guide."]
    pub r#target_profile: Vec<super::super::types::Canonical>,
    #[doc = "If the type is a reference to another resource, how the resource is or can be aggregated - is it a contained resource, or a reference, and if the context is a bundle, is it included in the bundle."]
    pub r#aggregation: Vec<super::super::types::Code>,
    #[doc = "Whether this reference needs to be version specific or version independent, or whether either can be used."]
    pub r#versioning: Option<super::super::types::Code>,
}
impl serde::ser::Serialize for ElementDefinitionType {
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
                if !self.r#profile.is_empty() {
                    let values = self
                        .r#profile
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#profile.is_empty() {
                    state.serialize_entry("profile", &self.r#profile)?;
                }
            }
            if _ctx.output_json {
                if !self.r#target_profile.is_empty() {
                    let values = self
                        .r#target_profile
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#target_profile.is_empty() {
                    state.serialize_entry("targetProfile", &self.r#target_profile)?;
                }
            }
            if _ctx.output_json {
                if !self.r#aggregation.is_empty() {
                    let values = self
                        .r#aggregation
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#aggregation.is_empty() {
                    state.serialize_entry("aggregation", &self.r#aggregation)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#versioning.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("versioning", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_versioning", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#versioning.as_ref() {
                    state.serialize_entry("versioning", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "A sample value for this element demonstrating the type of information that would typically be found in the element."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElementDefinitionExample {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Describes the purpose of this example amoung the set of examples."]
    pub r#label: super::super::types::String,
    #[doc = "The actual value for the element, which must be one of the types allowed for this element."]
    pub r#value: ElementDefinitionExampleValue,
}
impl serde::ser::Serialize for ElementDefinitionExample {
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
            if _ctx.output_json {
                if let Some(some) = self.r#label.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("label", &some)?;
                }
                if self.r#label.id.is_some() || !self.r#label.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#label.id.as_ref(),
                        extension: &self.r#label.extension,
                    };
                    state.serialize_entry("_label", &primitive_element)?;
                }
            } else {
                state.serialize_entry("label", &self.r#label)?;
            }
            match self.r#value {
                ElementDefinitionExampleValue::Base64Binary(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueBase64Binary", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueBase64Binary", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueBase64Binary", value)?;
                    }
                }
                ElementDefinitionExampleValue::Boolean(ref value) => {
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
                ElementDefinitionExampleValue::Canonical(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueCanonical", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueCanonical", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueCanonical", value)?;
                    }
                }
                ElementDefinitionExampleValue::Code(ref value) => {
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
                ElementDefinitionExampleValue::Date(ref value) => {
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
                ElementDefinitionExampleValue::DateTime(ref value) => {
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
                ElementDefinitionExampleValue::Decimal(ref value) => {
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
                ElementDefinitionExampleValue::Id(ref value) => {
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
                ElementDefinitionExampleValue::Instant(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueInstant", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueInstant", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueInstant", value)?;
                    }
                }
                ElementDefinitionExampleValue::Integer(ref value) => {
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
                ElementDefinitionExampleValue::Markdown(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueMarkdown", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueMarkdown", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueMarkdown", value)?;
                    }
                }
                ElementDefinitionExampleValue::Oid(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueOid", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueOid", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueOid", value)?;
                    }
                }
                ElementDefinitionExampleValue::PositiveInt(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valuePositiveInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valuePositiveInt", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valuePositiveInt", value)?;
                    }
                }
                ElementDefinitionExampleValue::String(ref value) => {
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
                ElementDefinitionExampleValue::Time(ref value) => {
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
                ElementDefinitionExampleValue::UnsignedInt(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueUnsignedInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueUnsignedInt", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueUnsignedInt", value)?;
                    }
                }
                ElementDefinitionExampleValue::Uri(ref value) => {
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
                ElementDefinitionExampleValue::Url(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueUrl", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueUrl", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueUrl", value)?;
                    }
                }
                ElementDefinitionExampleValue::Uuid(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueUuid", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_valueUuid", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("valueUuid", value)?;
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
                ElementDefinitionExampleValue::CodeableReference(ref value) => {
                    state.serialize_entry("valueCodeableReference", value)?;
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
                ElementDefinitionExampleValue::RatioRange(ref value) => {
                    state.serialize_entry("valueRatioRange", value)?;
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
                ElementDefinitionExampleValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is a required field"))
                }
            }
            state.end()
        })
    }
}
#[doc = "Formal constraints such as co-occurrence and other constraints that can be computationally evaluated within the context of the instance."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElementDefinitionConstraint {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Allows identification of which elements have their cardinalities impacted by the constraint.  Will not be referenced for constraints that do not affect cardinality."]
    pub r#key: super::super::types::Id,
    #[doc = "Description of why this constraint is necessary or appropriate."]
    pub r#requirements: Option<super::super::types::String>,
    #[doc = "Identifies the impact constraint violation has on the conformance of the instance."]
    pub r#severity: super::super::types::Code,
    #[doc = "Text that can be used to describe the constraint in messages identifying that the constraint has been violated."]
    pub r#human: super::super::types::String,
    #[doc = "A [FHIRPath](https://hl7.org/FHIR/fhirpath.html)) expression of constraint that can be executed to see if this constraint is met."]
    pub r#expression: Option<super::super::types::String>,
    #[doc = "An XPath expression of constraint that can be executed to see if this constraint is met."]
    pub r#xpath: Option<super::super::types::String>,
    #[doc = "A reference to the original source of the constraint, for traceability purposes."]
    pub r#source: Option<super::super::types::Canonical>,
}
impl serde::ser::Serialize for ElementDefinitionConstraint {
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
            if _ctx.output_json {
                if let Some(some) = self.r#key.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("key", &some)?;
                }
                if self.r#key.id.is_some() || !self.r#key.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#key.id.as_ref(),
                        extension: &self.r#key.extension,
                    };
                    state.serialize_entry("_key", &primitive_element)?;
                }
            } else {
                state.serialize_entry("key", &self.r#key)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#requirements.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("requirements", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_requirements", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#requirements.as_ref() {
                    state.serialize_entry("requirements", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#severity.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("severity", &some)?;
                }
                if self.r#severity.id.is_some() || !self.r#severity.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#severity.id.as_ref(),
                        extension: &self.r#severity.extension,
                    };
                    state.serialize_entry("_severity", &primitive_element)?;
                }
            } else {
                state.serialize_entry("severity", &self.r#severity)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#human.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("human", &some)?;
                }
                if self.r#human.id.is_some() || !self.r#human.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#human.id.as_ref(),
                        extension: &self.r#human.extension,
                    };
                    state.serialize_entry("_human", &primitive_element)?;
                }
            } else {
                state.serialize_entry("human", &self.r#human)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#expression.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("expression", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_expression", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#expression.as_ref() {
                    state.serialize_entry("expression", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#xpath.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("xpath", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_xpath", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#xpath.as_ref() {
                    state.serialize_entry("xpath", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#source.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("source", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_source", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#source.as_ref() {
                    state.serialize_entry("source", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Binds to a value set if this element is coded (code, Coding, CodeableConcept, Quantity), or the data types (string, uri)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElementDefinitionBinding {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates the degree of conformance expectations associated with this binding - that is, the degree to which the provided value set must be adhered to in the instances."]
    pub r#strength: super::super::types::Code,
    #[doc = "Describes the intended use of this particular set of codes."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Refers to the value set that identifies the set of codes the binding refers to."]
    pub r#value_set: Option<super::super::types::Canonical>,
}
impl serde::ser::Serialize for ElementDefinitionBinding {
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
            if _ctx.output_json {
                if let Some(some) = self.r#strength.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("strength", &some)?;
                }
                if self.r#strength.id.is_some() || !self.r#strength.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#strength.id.as_ref(),
                        extension: &self.r#strength.extension,
                    };
                    state.serialize_entry("_strength", &primitive_element)?;
                }
            } else {
                state.serialize_entry("strength", &self.r#strength)?;
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
            state.end()
        })
    }
}
#[doc = "Identifies a concept from an external specification that roughly corresponds to this element."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElementDefinitionMapping {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An internal reference to the definition of a mapping."]
    pub r#identity: super::super::types::Id,
    #[doc = "Identifies the computable language in which mapping.map is expressed."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "Expresses what part of the target specification corresponds to this element."]
    pub r#map: super::super::types::String,
    #[doc = "Comments that provide information about the mapping or its use."]
    pub r#comment: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ElementDefinitionMapping {
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
            if _ctx.output_json {
                if let Some(some) = self.r#identity.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("identity", &some)?;
                }
                if self.r#identity.id.is_some() || !self.r#identity.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#identity.id.as_ref(),
                        extension: &self.r#identity.extension,
                    };
                    state.serialize_entry("_identity", &primitive_element)?;
                }
            } else {
                state.serialize_entry("identity", &self.r#identity)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#map.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("map", &some)?;
                }
                if self.r#map.id.is_some() || !self.r#map.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#map.id.as_ref(),
                        extension: &self.r#map.extension,
                    };
                    state.serialize_entry("_map", &primitive_element)?;
                }
            } else {
                state.serialize_entry("map", &self.r#map)?;
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
            state.end()
        })
    }
}
#[doc = "Base StructureDefinition for ElementDefinition Type: Captures constraints on each element within the resource, profile, or extension."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElementDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The path identifies the element and is expressed as a \".\"-separated list of ancestor elements, beginning with the name of the resource or extension."]
    pub r#path: super::super::types::String,
    #[doc = "Codes that define how this element is represented in instances, when the deviation varies from the normal case."]
    pub r#representation: Vec<super::super::types::Code>,
    #[doc = "The name of this element definition slice, when slicing is working. The name must be a token with no dots or spaces. This is a unique name referring to a specific set of constraints applied to this element, used to provide a name to different slices of the same element."]
    pub r#slice_name: Option<super::super::types::String>,
    #[doc = "If true, indicates that this slice definition is constraining a slice definition with the same name in an inherited profile. If false, the slice is not overriding any slice in an inherited profile. If missing, the slice might or might not be overriding a slice in an inherited profile, depending on the sliceName."]
    pub r#slice_is_constraining: Option<super::super::types::Boolean>,
    #[doc = "A single preferred label which is the text to display beside the element indicating its meaning or to use to prompt for the element in a user display or form."]
    pub r#label: Option<super::super::types::String>,
    #[doc = "A code that has the same meaning as the element in a particular terminology."]
    pub r#code: Vec<Box<super::super::types::Coding>>,
    #[doc = "Indicates that the element is sliced into a set of alternative definitions (i.e. in a structure definition, there are multiple different constraints on a single element in the base resource). Slicing can be used in any resource that has cardinality ..* on the base resource, or any resource with a choice of types. The set of slices is any elements that come after this in the element sequence that have the same path, until a shorter path occurs (the shorter path terminates the set)."]
    pub r#slicing: Option<ElementDefinitionSlicing>,
    #[doc = "A concise description of what this element means (e.g. for use in autogenerated summaries)."]
    pub r#short: Option<super::super::types::String>,
    #[doc = "Provides a complete explanation of the meaning of the data element for human readability.  For the case of elements derived from existing elements (e.g. constraints), the definition SHALL be consistent with the base definition, but convey the meaning of the element in the particular context of use of the resource. (Note: The text you are reading is specified in ElementDefinition.definition)."]
    pub r#definition: Option<super::super::types::Markdown>,
    #[doc = "Explanatory notes and implementation guidance about the data element, including notes about how to use the data properly, exceptions to proper use, etc. (Note: The text you are reading is specified in ElementDefinition.comment)."]
    pub r#comment: Option<super::super::types::Markdown>,
    #[doc = "This element is for traceability of why the element was created and why the constraints exist as they do. This may be used to point to source materials or specifications that drove the structure of this element."]
    pub r#requirements: Option<super::super::types::Markdown>,
    #[doc = "Identifies additional names by which this element might also be known."]
    pub r#alias: Vec<super::super::types::String>,
    #[doc = "The minimum number of times this element SHALL appear in the instance."]
    pub r#min: Option<super::super::types::UnsignedInt>,
    #[doc = "The maximum number of times this element is permitted to appear in the instance."]
    pub r#max: Option<super::super::types::String>,
    #[doc = "Information about the base definition of the element, provided to make it unnecessary for tools to trace the deviation of the element through the derived and related profiles. When the element definition is not the original definition of an element - i.g. either in a constraint on another type, or for elements from a super type in a snap shot - then the information in provided in the element definition may be different to the base definition. On the original definition of the element, it will be same."]
    pub r#base: Option<ElementDefinitionBase>,
    #[doc = "Identifies an element defined elsewhere in the definition whose content rules should be applied to the current element. ContentReferences bring across all the rules that are in the ElementDefinition for the element, including definitions, cardinality constraints, bindings, invariants etc."]
    pub r#content_reference: Option<super::super::types::Uri>,
    #[doc = "The data type or resource that the value of this element is permitted to be."]
    pub r#type: Vec<ElementDefinitionType>,
    #[doc = "The value that should be used if there is no value stated in the instance (e.g. 'if not otherwise specified, the abstract is false')."]
    pub r#default_value: Option<ElementDefinitionDefaultValue>,
    #[doc = "The Implicit meaning that is to be understood when this element is missing (e.g. 'when this element is missing, the period is ongoing')."]
    pub r#meaning_when_missing: Option<super::super::types::Markdown>,
    #[doc = "If present, indicates that the order of the repeating element has meaning and describes what that meaning is.  If absent, it means that the order of the element has no meaning."]
    pub r#order_meaning: Option<super::super::types::String>,
    #[doc = "Specifies a value that SHALL be exactly the value  for this element in the instance. For purposes of comparison, non-significant whitespace is ignored, and all values must be an exact match (case and accent sensitive). Missing elements/attributes must also be missing."]
    pub r#fixed: Option<ElementDefinitionFixed>,
    #[doc = "Specifies a value that the value in the instance SHALL follow - that is, any value in the pattern must be found in the instance. Other additional values may be found too. This is effectively constraint by example.  \n\nWhen pattern\\[x\\] is used to constrain a primitive, it means that the value provided in the pattern\\[x\\] must match the instance value exactly.\n\nWhen pattern\\[x\\] is used to constrain an array, it means that each element provided in the pattern\\[x\\] array must (recursively) match at least one element from the instance array.\n\nWhen pattern\\[x\\] is used to constrain a complex object, it means that each property in the pattern must be present in the complex object, and its value must recursively match -- i.e.,\n\n1. If primitive: it must match exactly the pattern value\n2. If a complex object: it must match (recursively) the pattern value\n3. If an array: it must match (recursively) the pattern value."]
    pub r#pattern: Option<ElementDefinitionPattern>,
    #[doc = "A sample value for this element demonstrating the type of information that would typically be found in the element."]
    pub r#example: Vec<ElementDefinitionExample>,
    #[doc = "The minimum allowed value for the element. The value is inclusive. This is allowed for the types date, dateTime, instant, time, decimal, integer, and Quantity."]
    pub r#min_value: Option<ElementDefinitionMinValue>,
    #[doc = "The maximum allowed value for the element. The value is inclusive. This is allowed for the types date, dateTime, instant, time, decimal, integer, and Quantity."]
    pub r#max_value: Option<ElementDefinitionMaxValue>,
    #[doc = "Indicates the maximum length in characters that is permitted to be present in conformant instances and which is expected to be supported by conformant consumers that support the element."]
    pub r#max_length: Option<super::super::types::Integer>,
    #[doc = "A reference to an invariant that may make additional statements about the cardinality or value in the instance."]
    pub r#condition: Vec<super::super::types::Id>,
    #[doc = "Formal constraints such as co-occurrence and other constraints that can be computationally evaluated within the context of the instance."]
    pub r#constraint: Vec<ElementDefinitionConstraint>,
    #[doc = "If true, implementations that produce or consume resources SHALL provide \"support\" for the element in some meaningful way.  If false, the element may be ignored and not supported. If false, whether to populate or use the data element in any way is at the discretion of the implementation."]
    pub r#must_support: Option<super::super::types::Boolean>,
    #[doc = "If true, the value of this element affects the interpretation of the element or resource that contains it, and the value of the element cannot be ignored. Typically, this is used for status, negation and qualification codes. The effect of this is that the element cannot be ignored by systems: they SHALL either recognize the element and process it, and/or a pre-determination has been made that it is not relevant to their particular system."]
    pub r#is_modifier: Option<super::super::types::Boolean>,
    #[doc = "Explains how that element affects the interpretation of the resource or element that contains it."]
    pub r#is_modifier_reason: Option<super::super::types::String>,
    #[doc = "Whether the element should be included if a client requests a search with the parameter _summary=true."]
    pub r#is_summary: Option<super::super::types::Boolean>,
    #[doc = "Binds to a value set if this element is coded (code, Coding, CodeableConcept, Quantity), or the data types (string, uri)."]
    pub r#binding: Option<ElementDefinitionBinding>,
    #[doc = "Identifies a concept from an external specification that roughly corresponds to this element."]
    pub r#mapping: Vec<ElementDefinitionMapping>,
}
impl serde::ser::Serialize for ElementDefinition {
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
            if _ctx.output_json {
                if let Some(some) = self.r#path.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("path", &some)?;
                }
                if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#path.id.as_ref(),
                        extension: &self.r#path.extension,
                    };
                    state.serialize_entry("_path", &primitive_element)?;
                }
            } else {
                state.serialize_entry("path", &self.r#path)?;
            }
            if _ctx.output_json {
                if !self.r#representation.is_empty() {
                    let values = self
                        .r#representation
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#representation.is_empty() {
                    state.serialize_entry("representation", &self.r#representation)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#slice_name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sliceName", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sliceName", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#slice_name.as_ref() {
                    state.serialize_entry("sliceName", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#slice_is_constraining.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sliceIsConstraining", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sliceIsConstraining", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#slice_is_constraining.as_ref() {
                    state.serialize_entry("sliceIsConstraining", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#label.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("label", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_label", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#label.as_ref() {
                    state.serialize_entry("label", some)?;
                }
            }
            if !self.r#code.is_empty() {
                state.serialize_entry("code", &self.r#code)?;
            }
            if let Some(some) = self.r#slicing.as_ref() {
                state.serialize_entry("slicing", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#short.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("short", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_short", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#short.as_ref() {
                    state.serialize_entry("short", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#requirements.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("requirements", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_requirements", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#requirements.as_ref() {
                    state.serialize_entry("requirements", some)?;
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
            if let Some(some) = self.r#base.as_ref() {
                state.serialize_entry("base", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#content_reference.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("contentReference", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_contentReference", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#content_reference.as_ref() {
                    state.serialize_entry("contentReference", some)?;
                }
            }
            if !self.r#type.is_empty() {
                state.serialize_entry("type", &self.r#type)?;
            }
            if let Some(some) = self.r#default_value.as_ref() {
                match some {
                    ElementDefinitionDefaultValue::Base64Binary(ref value) => {
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
                    ElementDefinitionDefaultValue::Boolean(ref value) => {
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
                    ElementDefinitionDefaultValue::Canonical(ref value) => {
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
                    ElementDefinitionDefaultValue::Code(ref value) => {
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
                    ElementDefinitionDefaultValue::Date(ref value) => {
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
                    ElementDefinitionDefaultValue::DateTime(ref value) => {
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
                    ElementDefinitionDefaultValue::Decimal(ref value) => {
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
                    ElementDefinitionDefaultValue::Id(ref value) => {
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
                    ElementDefinitionDefaultValue::Instant(ref value) => {
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
                    ElementDefinitionDefaultValue::Integer(ref value) => {
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
                    ElementDefinitionDefaultValue::Markdown(ref value) => {
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
                    ElementDefinitionDefaultValue::Oid(ref value) => {
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
                    ElementDefinitionDefaultValue::PositiveInt(ref value) => {
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
                    ElementDefinitionDefaultValue::String(ref value) => {
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
                    ElementDefinitionDefaultValue::Time(ref value) => {
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
                    ElementDefinitionDefaultValue::UnsignedInt(ref value) => {
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
                    ElementDefinitionDefaultValue::Uri(ref value) => {
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
                    ElementDefinitionDefaultValue::Url(ref value) => {
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
                    ElementDefinitionDefaultValue::Uuid(ref value) => {
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
                    ElementDefinitionDefaultValue::CodeableReference(ref value) => {
                        state.serialize_entry("defaultValueCodeableReference", value)?;
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
                    ElementDefinitionDefaultValue::RatioRange(ref value) => {
                        state.serialize_entry("defaultValueRatioRange", value)?;
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
                    ElementDefinitionDefaultValue::Invalid => {
                        return Err(serde::ser::Error::custom("default_value is invalid"))
                    }
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#meaning_when_missing.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("meaningWhenMissing", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_meaningWhenMissing", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#meaning_when_missing.as_ref() {
                    state.serialize_entry("meaningWhenMissing", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#order_meaning.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("orderMeaning", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_orderMeaning", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#order_meaning.as_ref() {
                    state.serialize_entry("orderMeaning", some)?;
                }
            }
            if let Some(some) = self.r#fixed.as_ref() {
                match some {
                    ElementDefinitionFixed::Base64Binary(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedBase64Binary", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedBase64Binary", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedBase64Binary", value)?;
                        }
                    }
                    ElementDefinitionFixed::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedBoolean", value)?;
                        }
                    }
                    ElementDefinitionFixed::Canonical(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedCanonical", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedCanonical", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedCanonical", value)?;
                        }
                    }
                    ElementDefinitionFixed::Code(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedCode", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedCode", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedCode", value)?;
                        }
                    }
                    ElementDefinitionFixed::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedDate", value)?;
                        }
                    }
                    ElementDefinitionFixed::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedDateTime", value)?;
                        }
                    }
                    ElementDefinitionFixed::Decimal(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = some.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })?;
                                state.serialize_entry("fixedDecimal", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedDecimal", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedDecimal", value)?;
                        }
                    }
                    ElementDefinitionFixed::Id(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedId", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedId", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedId", value)?;
                        }
                    }
                    ElementDefinitionFixed::Instant(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedInstant", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedInstant", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedInstant", value)?;
                        }
                    }
                    ElementDefinitionFixed::Integer(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedInteger", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedInteger", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedInteger", value)?;
                        }
                    }
                    ElementDefinitionFixed::Markdown(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedMarkdown", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedMarkdown", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedMarkdown", value)?;
                        }
                    }
                    ElementDefinitionFixed::Oid(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedOid", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedOid", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedOid", value)?;
                        }
                    }
                    ElementDefinitionFixed::PositiveInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedPositiveInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedPositiveInt", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedPositiveInt", value)?;
                        }
                    }
                    ElementDefinitionFixed::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedString", value)?;
                        }
                    }
                    ElementDefinitionFixed::Time(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedTime", value)?;
                        }
                    }
                    ElementDefinitionFixed::UnsignedInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedUnsignedInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedUnsignedInt", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedUnsignedInt", value)?;
                        }
                    }
                    ElementDefinitionFixed::Uri(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedUri", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedUri", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedUri", value)?;
                        }
                    }
                    ElementDefinitionFixed::Url(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedUrl", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedUrl", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedUrl", value)?;
                        }
                    }
                    ElementDefinitionFixed::Uuid(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("fixedUuid", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_fixedUuid", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("fixedUuid", value)?;
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
                    ElementDefinitionFixed::CodeableReference(ref value) => {
                        state.serialize_entry("fixedCodeableReference", value)?;
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
                    ElementDefinitionFixed::RatioRange(ref value) => {
                        state.serialize_entry("fixedRatioRange", value)?;
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
                    ElementDefinitionFixed::Invalid => {
                        return Err(serde::ser::Error::custom("fixed is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#pattern.as_ref() {
                match some {
                    ElementDefinitionPattern::Base64Binary(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternBase64Binary", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_patternBase64Binary", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternBase64Binary", value)?;
                        }
                    }
                    ElementDefinitionPattern::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternBoolean", value)?;
                        }
                    }
                    ElementDefinitionPattern::Canonical(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternCanonical", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternCanonical", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternCanonical", value)?;
                        }
                    }
                    ElementDefinitionPattern::Code(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternCode", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternCode", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternCode", value)?;
                        }
                    }
                    ElementDefinitionPattern::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternDate", value)?;
                        }
                    }
                    ElementDefinitionPattern::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternDateTime", value)?;
                        }
                    }
                    ElementDefinitionPattern::Decimal(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = some.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })?;
                                state.serialize_entry("patternDecimal", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternDecimal", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternDecimal", value)?;
                        }
                    }
                    ElementDefinitionPattern::Id(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternId", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternId", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternId", value)?;
                        }
                    }
                    ElementDefinitionPattern::Instant(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternInstant", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternInstant", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternInstant", value)?;
                        }
                    }
                    ElementDefinitionPattern::Integer(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternInteger", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternInteger", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternInteger", value)?;
                        }
                    }
                    ElementDefinitionPattern::Markdown(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternMarkdown", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternMarkdown", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternMarkdown", value)?;
                        }
                    }
                    ElementDefinitionPattern::Oid(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternOid", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternOid", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternOid", value)?;
                        }
                    }
                    ElementDefinitionPattern::PositiveInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternPositiveInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternPositiveInt", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternPositiveInt", value)?;
                        }
                    }
                    ElementDefinitionPattern::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternString", value)?;
                        }
                    }
                    ElementDefinitionPattern::Time(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternTime", value)?;
                        }
                    }
                    ElementDefinitionPattern::UnsignedInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternUnsignedInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternUnsignedInt", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternUnsignedInt", value)?;
                        }
                    }
                    ElementDefinitionPattern::Uri(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternUri", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternUri", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternUri", value)?;
                        }
                    }
                    ElementDefinitionPattern::Url(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternUrl", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternUrl", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternUrl", value)?;
                        }
                    }
                    ElementDefinitionPattern::Uuid(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("patternUuid", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_patternUuid", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("patternUuid", value)?;
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
                    ElementDefinitionPattern::CodeableReference(ref value) => {
                        state.serialize_entry("patternCodeableReference", value)?;
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
                    ElementDefinitionPattern::RatioRange(ref value) => {
                        state.serialize_entry("patternRatioRange", value)?;
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
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("minValueDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_minValueDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("minValueDate", value)?;
                        }
                    }
                    ElementDefinitionMinValue::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("minValueDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_minValueDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("minValueDateTime", value)?;
                        }
                    }
                    ElementDefinitionMinValue::Instant(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("minValueInstant", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_minValueInstant", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("minValueInstant", value)?;
                        }
                    }
                    ElementDefinitionMinValue::Time(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("minValueTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_minValueTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("minValueTime", value)?;
                        }
                    }
                    ElementDefinitionMinValue::Decimal(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = some.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })?;
                                state.serialize_entry("minValueDecimal", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_minValueDecimal", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("minValueDecimal", value)?;
                        }
                    }
                    ElementDefinitionMinValue::Integer(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("minValueInteger", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_minValueInteger", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("minValueInteger", value)?;
                        }
                    }
                    ElementDefinitionMinValue::PositiveInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("minValuePositiveInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_minValuePositiveInt", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("minValuePositiveInt", value)?;
                        }
                    }
                    ElementDefinitionMinValue::UnsignedInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("minValueUnsignedInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_minValueUnsignedInt", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("minValueUnsignedInt", value)?;
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
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("maxValueDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_maxValueDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("maxValueDate", value)?;
                        }
                    }
                    ElementDefinitionMaxValue::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("maxValueDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_maxValueDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("maxValueDateTime", value)?;
                        }
                    }
                    ElementDefinitionMaxValue::Instant(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("maxValueInstant", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_maxValueInstant", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("maxValueInstant", value)?;
                        }
                    }
                    ElementDefinitionMaxValue::Time(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("maxValueTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_maxValueTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("maxValueTime", value)?;
                        }
                    }
                    ElementDefinitionMaxValue::Decimal(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = some.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })?;
                                state.serialize_entry("maxValueDecimal", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_maxValueDecimal", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("maxValueDecimal", value)?;
                        }
                    }
                    ElementDefinitionMaxValue::Integer(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("maxValueInteger", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_maxValueInteger", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("maxValueInteger", value)?;
                        }
                    }
                    ElementDefinitionMaxValue::PositiveInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("maxValuePositiveInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_maxValuePositiveInt", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("maxValuePositiveInt", value)?;
                        }
                    }
                    ElementDefinitionMaxValue::UnsignedInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("maxValueUnsignedInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_maxValueUnsignedInt", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("maxValueUnsignedInt", value)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#max_length.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("maxLength", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_maxLength", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#max_length.as_ref() {
                    state.serialize_entry("maxLength", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#condition.is_empty() {
                    let values = self
                        .r#condition
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
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
                                        id: e.id.as_ref(),
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
            } else {
                if !self.r#condition.is_empty() {
                    state.serialize_entry("condition", &self.r#condition)?;
                }
            }
            if !self.r#constraint.is_empty() {
                state.serialize_entry("constraint", &self.r#constraint)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#must_support.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("mustSupport", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_mustSupport", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#must_support.as_ref() {
                    state.serialize_entry("mustSupport", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#is_modifier.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("isModifier", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_isModifier", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#is_modifier.as_ref() {
                    state.serialize_entry("isModifier", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#is_modifier_reason.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("isModifierReason", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_isModifierReason", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#is_modifier_reason.as_ref() {
                    state.serialize_entry("isModifierReason", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#is_summary.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("isSummary", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_isSummary", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#is_summary.as_ref() {
                    state.serialize_entry("isSummary", some)?;
                }
            }
            if let Some(some) = self.r#binding.as_ref() {
                state.serialize_entry("binding", some)?;
            }
            if !self.r#mapping.is_empty() {
                state.serialize_entry("mapping", &self.r#mapping)?;
            }
            state.end()
        })
    }
}
