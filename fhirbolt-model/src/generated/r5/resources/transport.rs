// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The value of the input parameter as a basic type."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum TransportInputValue {
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
    Integer64(super::super::types::Integer64),
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
    DataRequirement(Box<super::super::types::DataRequirement>),
    Expression(Box<super::super::types::Expression>),
    ParameterDefinition(Box<super::super::types::ParameterDefinition>),
    RelatedArtifact(Box<super::super::types::RelatedArtifact>),
    TriggerDefinition(Box<super::super::types::TriggerDefinition>),
    UsageContext(Box<super::super::types::UsageContext>),
    Availability(Box<super::super::types::Availability>),
    ExtendedContactDetail(Box<super::super::types::ExtendedContactDetail>),
    Dosage(Box<super::super::types::Dosage>),
    Meta(Box<super::super::types::Meta>),
    #[default]
    Invalid,
}
#[doc = "The value of the Output parameter as a basic type."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum TransportOutputValue {
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
    Integer64(super::super::types::Integer64),
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
    DataRequirement(Box<super::super::types::DataRequirement>),
    Expression(Box<super::super::types::Expression>),
    ParameterDefinition(Box<super::super::types::ParameterDefinition>),
    RelatedArtifact(Box<super::super::types::RelatedArtifact>),
    TriggerDefinition(Box<super::super::types::TriggerDefinition>),
    UsageContext(Box<super::super::types::UsageContext>),
    Availability(Box<super::super::types::Availability>),
    ExtendedContactDetail(Box<super::super::types::ExtendedContactDetail>),
    Dosage(Box<super::super::types::Dosage>),
    Meta(Box<super::super::types::Meta>),
    #[default]
    Invalid,
}
#[doc = "If the Transport.focus is a request resource and the transport is seeking fulfillment (i.e. is asking for the request to be actioned), this element identifies any limitations on what parts of the referenced request should be actioned."]
#[derive(Debug, Clone, PartialEq)]
pub struct TransportRestriction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Indicates the number of times the requested action should occur."]
    pub r#repetitions: Option<super::super::types::PositiveInt>,
    #[doc = "Over what time-period is fulfillment sought."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "For requests that are targeted to more than one potential recipient/target, to identify who is fulfillment is sought for."]
    pub r#recipient: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for TransportRestriction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#repetitions: Default::default(),
            r#period: Default::default(),
            r#recipient: Default::default(),
        }
    }
}
#[doc = "Additional information that may be needed in the execution of the transport."]
#[derive(Debug, Clone, PartialEq)]
pub struct TransportInput {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code or description indicating how the input is intended to be used as part of the transport execution."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The value of the input parameter as a basic type."]
    pub r#value: TransportInputValue,
}
#[allow(clippy::derivable_impls)]
impl Default for TransportInput {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Default::default(),
        }
    }
}
#[doc = "Outputs produced by the Transport."]
#[derive(Debug, Clone, PartialEq)]
pub struct TransportOutput {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The name of the Output parameter."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The value of the Output parameter as a basic type."]
    pub r#value: TransportOutputValue,
}
#[allow(clippy::derivable_impls)]
impl Default for TransportOutput {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Default::default(),
        }
    }
}
#[doc = "Record of transport."]
#[derive(Debug, Clone, PartialEq)]
pub struct Transport {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<super::super::types::Id>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifier for the transport event that is used to identify it across multiple disparate systems."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The URL pointing to a *FHIR*-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this Transport."]
    pub r#instantiates_canonical: Option<super::super::types::Canonical>,
    #[doc = "The URL pointing to an *externally* maintained  protocol, guideline, orderset or other definition that is adhered to in whole or in part by this Transport."]
    pub r#instantiates_uri: Option<super::super::types::Uri>,
    #[doc = "BasedOn refers to a higher-level authorization that triggered the creation of the transport.  It references a \"request\" resource such as a ServiceRequest or Transport, which is distinct from the \"request\" resource the Transport is seeking to fulfill.  This latter resource is referenced by FocusOn.  For example, based on a ServiceRequest (= BasedOn), a transport is created to fulfill a procedureRequest ( = FocusOn ) to transport a specimen to the lab."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "A shared identifier common to multiple independent Request instances that were activated/authorized more or less simultaneously by a single author.  The presence of the same identifier on each request ties those requests together and may have business ramifications in terms of reporting of results, billing, etc.  E.g. a requisition number shared by a set of lab tests ordered together, or a prescription number shared by all meds ordered at one time."]
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "A larger event of which this particular event is a component or step."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "A code specifying the state of the transport event."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "An explanation as to why this transport is held, failed, was refused, etc."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the \"level\" of actionability associated with the Transport, i.e. i+R`9`s this a proposed transport, a planned transport, an actionable transport, etc."]
    pub r#intent: super::super::types::Code,
    #[doc = "Indicates how quickly the Transport should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "A name or code (or both) briefly describing what the transport involves."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A free-text description of what is to be performed."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The request being actioned or the resource being manipulated by this transport."]
    pub r#focus: Option<Box<super::super::types::Reference>>,
    #[doc = "The entity who benefits from the performance of the service specified in the transport (e.g., the patient)."]
    pub r#for: Option<Box<super::super::types::Reference>>,
    #[doc = "The healthcare event  (e.g. a patient and healthcare provider interaction) during which this transport was created."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies the completion time of the event (the occurrence)."]
    pub r#completion_time: Option<super::super::types::DateTime>,
    #[doc = "The date and time this transport was created."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "The date and time of last modification to this transport."]
    pub r#last_modified: Option<super::super::types::DateTime>,
    #[doc = "The creator of the transport."]
    pub r#requester: Option<Box<super::super::types::Reference>>,
    #[doc = "The kind of participant that should perform the transport."]
    pub r#performer_type: Vec<super::super::types::CodeableConcept>,
    #[doc = "Individual organization or Device currently responsible for transport execution."]
    pub r#owner: Option<Box<super::super::types::Reference>>,
    #[doc = "Principal physical location where this transport is performed."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Insurance plans, coverage extensions, pre-authorizations and/or pre-determinations that may be relevant to the Transport."]
    pub r#insurance: Vec<super::super::types::Reference>,
    #[doc = "Free-text information captured about the transport as it progresses."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Links to Provenance records for past versions of this Transport that identify key state transitions or updates that are likely to be relevant to a user looking at the current version of the transport."]
    pub r#relevant_history: Vec<super::super::types::Reference>,
    #[doc = "If the Transport.focus is a request resource and the transport is seeking fulfillment (i.e. is asking for the request to be actioned), this element identifies any limitations on what parts of the referenced request should be actioned."]
    pub r#restriction: Option<TransportRestriction>,
    #[doc = "Additional information that may be needed in the execution of the transport."]
    pub r#input: Vec<TransportInput>,
    #[doc = "Outputs produced by the Transport."]
    pub r#output: Vec<TransportOutput>,
    #[doc = "The desired or final location for the transport."]
    pub r#requested_location: Box<super::super::types::Reference>,
    #[doc = "The current location for the entity to be transported."]
    pub r#current_location: Box<super::super::types::Reference>,
    #[doc = "A resource reference indicating why this transport needs to be performed."]
    pub r#reason: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The transport event prior to this one."]
    pub r#history: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for Transport {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#text: Default::default(),
            r#contained: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#instantiates_canonical: Default::default(),
            r#instantiates_uri: Default::default(),
            r#based_on: Default::default(),
            r#group_identifier: Default::default(),
            r#part_of: Default::default(),
            r#status: Default::default(),
            r#status_reason: Default::default(),
            r#intent: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#priority: Default::default(),
            r#code: Default::default(),
            r#description: Default::default(),
            r#focus: Default::default(),
            r#for: Default::default(),
            r#encounter: Default::default(),
            r#completion_time: Default::default(),
            r#authored_on: Default::default(),
            r#last_modified: Default::default(),
            r#requester: Default::default(),
            r#performer_type: Default::default(),
            r#owner: Default::default(),
            r#location: Default::default(),
            r#insurance: Default::default(),
            r#note: Default::default(),
            r#relevant_history: Default::default(),
            r#restriction: Default::default(),
            r#input: Default::default(),
            r#output: Default::default(),
            r#requested_location: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#current_location: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#reason: Default::default(),
            r#history: Default::default(),
        }
    }
}
