// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "If the parameter is a data type."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ParametersParameterValue {
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
#[doc = "A parameter passed to or received from the operation."]
#[derive(Debug, Clone, PartialEq)]
pub struct ParametersParameter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The name of the parameter (reference to the operation definition)."]
    pub r#name: super::super::types::String,
    #[doc = "If the parameter is a data type."]
    pub r#value: Option<ParametersParameterValue>,
    #[doc = "If the parameter is a whole resource."]
    pub r#resource: Option<super::super::Resource>,
    #[doc = "A named part of a multi-part parameter."]
    pub r#part: Vec<ParametersParameter>,
}
#[allow(clippy::derivable_impls)]
impl Default for ParametersParameter {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#value: Default::default(),
            r#resource: Default::default(),
            r#part: Default::default(),
        }
    }
}
#[doc = "This resource is a non-persisted resource used to pass information into and back from an [operation](https://hl7.org/FHIR/operations.html)). It has no other use, and there is no RESTful endpoint associated with it."]
#[derive(Debug, Clone, PartialEq)]
pub struct Parameters {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<super::super::types::Id>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A parameter passed to or received from the operation."]
    pub r#parameter: Vec<ParametersParameter>,
}
#[allow(clippy::derivable_impls)]
impl Default for Parameters {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#parameter: Default::default(),
        }
    }
}
