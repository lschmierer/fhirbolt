// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Value of extension - must be one of a constrained set of the data types (see [Extensibility](https://hl7.org/FHIR/extensibility.html)) for a list)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ExtensionValue {
    Base64Binary(super::super::types::Base64Binary),
    Boolean(super::super::types::Boolean),
    Canonical(super::super::types::Canonical),
    Code(super::super::types::Code),
    Date(super::super::types::Date),
    DateTime(super::super::types::DateTime),
    Decimal(super::super::types::Decimal),
    Id(super::super::types::Id),
    Instant(super::super::types::Instant),
    Integer(super::super::types::Integer),
    Markdown(super::super::types::Markdown),
    Oid(super::super::types::Oid),
    PositiveInt(super::super::types::PositiveInt),
    String(super::super::types::String),
    Time(super::super::types::Time),
    UnsignedInt(super::super::types::UnsignedInt),
    Uri(super::super::types::Uri),
    Url(super::super::types::Url),
    Uuid(super::super::types::Uuid),
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
#[doc = "Base StructureDefinition for Extension Type: Optional Extension Element - found in all resources.\n\nThe ability to add extensions in a structured way is what keeps FHIR resources simple."]
#[derive(Debug, Clone, PartialEq)]
pub struct Extension {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "Source of the definition for the extension code - a logical name or a URL."]
    pub r#url: std::string::String,
    #[doc = "Value of extension - must be one of a constrained set of the data types (see [Extensibility](https://hl7.org/FHIR/extensibility.html)) for a list)."]
    pub r#value: Option<ExtensionValue>,
}
#[allow(clippy::derivable_impls)]
impl Default for Extension {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#url: Default::default(),
            r#value: Default::default(),
        }
    }
}
