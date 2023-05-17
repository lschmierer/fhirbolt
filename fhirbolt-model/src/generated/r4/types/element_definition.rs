// Generated on 2023-05-17 by fhirbolt-codegen v0.9.0
#[doc = "The value that should be used if there is no value stated in the instance (e.g. 'if not otherwise specified, the abstract is false')."]
#[derive(Default, Debug, Clone, PartialEq)]
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
    #[default]
    Invalid,
}
#[doc = "Specifies a value that SHALL be exactly the value  for this element in the instance. For purposes of comparison, non-significant whitespace is ignored, and all values must be an exact match (case and accent sensitive). Missing elements/attributes must also be missing."]
#[derive(Default, Debug, Clone, PartialEq)]
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
    #[default]
    Invalid,
}
#[doc = "Specifies a value that the value in the instance SHALL follow - that is, any value in the pattern must be found in the instance. Other additional values may be found too. This is effectively constraint by example.  \n\nWhen pattern\\[x\\] is used to constrain a primitive, it means that the value provided in the pattern\\[x\\] must match the instance value exactly.\n\nWhen pattern\\[x\\] is used to constrain an array, it means that each element provided in the pattern\\[x\\] array must (recursively) match at least one element from the instance array.\n\nWhen pattern\\[x\\] is used to constrain a complex object, it means that each property in the pattern must be present in the complex object, and its value must recursively match -- i.e.,\n\n1. If primitive: it must match exactly the pattern value\n2. If a complex object: it must match (recursively) the pattern value\n3. If an array: it must match (recursively) the pattern value."]
#[derive(Default, Debug, Clone, PartialEq)]
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
    #[default]
    Invalid,
}
#[doc = "The actual value for the element, which must be one of the types allowed for this element."]
#[derive(Default, Debug, Clone, PartialEq)]
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
    #[default]
    Invalid,
}
#[doc = "The minimum allowed value for the element. The value is inclusive. This is allowed for the types date, dateTime, instant, time, decimal, integer, and Quantity."]
#[derive(Default, Debug, Clone, PartialEq)]
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
    #[default]
    Invalid,
}
#[doc = "The maximum allowed value for the element. The value is inclusive. This is allowed for the types date, dateTime, instant, time, decimal, integer, and Quantity."]
#[derive(Default, Debug, Clone, PartialEq)]
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
    #[default]
    Invalid,
}
#[doc = "Designates which child elements are used to discriminate between the slices when processing an instance. If one or more discriminators are provided, the value of the child elements in the instance data SHALL completely distinguish which slice the element in the resource matches based on the allowed values for those elements in each of the slices."]
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionSlicingDiscriminator {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "How the element value is interpreted when discrimination is evaluated."]
    pub r#type: super::super::types::Code,
    #[doc = "A FHIRPath expression, using [the simple subset of FHIRPath](fhirpath.html#simple), that is used to identify the element on which discrimination is based."]
    pub r#path: super::super::types::String,
}
#[allow(clippy::derivable_impls)]
impl Default for ElementDefinitionSlicingDiscriminator {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#path: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Indicates that the element is sliced into a set of alternative definitions (i.e. in a structure definition, there are multiple different constraints on a single element in the base resource). Slicing can be used in any resource that has cardinality ..* on the base resource, or any resource with a choice of types. The set of slices is any elements that come after this in the element sequence that have the same path, until a shorter path occurs (the shorter path terminates the set)."]
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionSlicing {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "Designates which child elements are used to discriminate between the slices when processing an instance. If one or more discriminators are provided, the value of the child elements in the instance data SHALL completely distinguish which slice the element in the resource matches based on the allowed values for those elements in each of the slices."]
    pub r#discriminator: Vec<ElementDefinitionSlicingDiscriminator>,
    #[doc = "A human-readable text description of how the slicing works. If there is no discriminator, this is required to be present to provide whatever information is possible about how the slices can be differentiated."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "If the matching elements have to occur in the same order as defined in the profile."]
    pub r#ordered: Option<super::super::types::Boolean>,
    #[doc = "Whether additional slices are allowed or not. When the slices are ordered, profile authors can also say that additional slices are only allowed at the end."]
    pub r#rules: super::super::types::Code,
}
#[allow(clippy::derivable_impls)]
impl Default for ElementDefinitionSlicing {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#discriminator: Default::default(),
            r#description: Default::default(),
            r#ordered: Default::default(),
            r#rules: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Information about the base definition of the element, provided to make it unnecessary for tools to trace the deviation of the element through the derived and related profiles. When the element definition is not the original definition of an element - i.g. either in a constraint on another type, or for elements from a super type in a snap shot - then the information in provided in the element definition may be different to the base definition. On the original definition of the element, it will be same."]
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionBase {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The Path that identifies the base element - this matches the ElementDefinition.path for that element. Across FHIR, there is only one base definition of any element - that is, an element definition on a [StructureDefinition](structuredefinition.html#) without a StructureDefinition.base."]
    pub r#path: super::super::types::String,
    #[doc = "Minimum cardinality of the base element identified by the path."]
    pub r#min: super::super::types::UnsignedInt,
    #[doc = "Maximum cardinality of the base element identified by the path."]
    pub r#max: super::super::types::String,
}
#[allow(clippy::derivable_impls)]
impl Default for ElementDefinitionBase {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#path: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#min: super::super::types::UnsignedInt {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#max: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "The data type or resource that the value of this element is permitted to be."]
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionType {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
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
#[allow(clippy::derivable_impls)]
impl Default for ElementDefinitionType {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#code: super::super::types::Uri {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#profile: Default::default(),
            r#target_profile: Default::default(),
            r#aggregation: Default::default(),
            r#versioning: Default::default(),
        }
    }
}
#[doc = "A sample value for this element demonstrating the type of information that would typically be found in the element."]
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionExample {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "Describes the purpose of this example amoung the set of examples."]
    pub r#label: super::super::types::String,
    #[doc = "The actual value for the element, which must be one of the types allowed for this element."]
    pub r#value: ElementDefinitionExampleValue,
}
#[allow(clippy::derivable_impls)]
impl Default for ElementDefinitionExample {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#label: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#value: Default::default(),
        }
    }
}
#[doc = "Formal constraints such as co-occurrence and other constraints that can be computationally evaluated within the context of the instance."]
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionConstraint {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
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
#[allow(clippy::derivable_impls)]
impl Default for ElementDefinitionConstraint {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#key: super::super::types::Id {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#requirements: Default::default(),
            r#severity: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#human: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#expression: Default::default(),
            r#xpath: Default::default(),
            r#source: Default::default(),
        }
    }
}
#[doc = "Binds to a value set if this element is coded (code, Coding, CodeableConcept, Quantity), or the data types (string, uri)."]
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionBinding {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "Indicates the degree of conformance expectations associated with this binding - that is, the degree to which the provided value set must be adhered to in the instances."]
    pub r#strength: super::super::types::Code,
    #[doc = "Describes the intended use of this particular set of codes."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Refers to the value set that identifies the set of codes the binding refers to."]
    pub r#value_set: Option<super::super::types::Canonical>,
}
#[allow(clippy::derivable_impls)]
impl Default for ElementDefinitionBinding {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#strength: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#description: Default::default(),
            r#value_set: Default::default(),
        }
    }
}
#[doc = "Identifies a concept from an external specification that roughly corresponds to this element."]
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionMapping {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "An internal reference to the definition of a mapping."]
    pub r#identity: super::super::types::Id,
    #[doc = "Identifies the computable language in which mapping.map is expressed."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "Expresses what part of the target specification corresponds to this element."]
    pub r#map: super::super::types::String,
    #[doc = "Comments that provide information about the mapping or its use."]
    pub r#comment: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for ElementDefinitionMapping {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#identity: super::super::types::Id {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#language: Default::default(),
            r#map: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#comment: Default::default(),
        }
    }
}
#[doc = "Base StructureDefinition for ElementDefinition Type: Captures constraints on each element within the resource, profile, or extension."]
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
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
    pub r#code: Vec<super::super::types::Coding>,
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
#[allow(clippy::derivable_impls)]
impl Default for ElementDefinition {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#path: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#representation: Default::default(),
            r#slice_name: Default::default(),
            r#slice_is_constraining: Default::default(),
            r#label: Default::default(),
            r#code: Default::default(),
            r#slicing: Default::default(),
            r#short: Default::default(),
            r#definition: Default::default(),
            r#comment: Default::default(),
            r#requirements: Default::default(),
            r#alias: Default::default(),
            r#min: Default::default(),
            r#max: Default::default(),
            r#base: Default::default(),
            r#content_reference: Default::default(),
            r#type: Default::default(),
            r#default_value: Default::default(),
            r#meaning_when_missing: Default::default(),
            r#order_meaning: Default::default(),
            r#fixed: Default::default(),
            r#pattern: Default::default(),
            r#example: Default::default(),
            r#min_value: Default::default(),
            r#max_value: Default::default(),
            r#max_length: Default::default(),
            r#condition: Default::default(),
            r#constraint: Default::default(),
            r#must_support: Default::default(),
            r#is_modifier: Default::default(),
            r#is_modifier_reason: Default::default(),
            r#is_summary: Default::default(),
            r#binding: Default::default(),
            r#mapping: Default::default(),
        }
    }
}
