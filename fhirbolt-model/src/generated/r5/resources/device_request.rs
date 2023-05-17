// Generated on 2023-05-15 by fhirbolt-codegen v0.8.0
#[doc = "The value of the device detail."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DeviceRequestParameterValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Boolean(Box<super::super::types::Boolean>),
    #[default]
    Invalid,
}
#[doc = "The timing schedule for the use of the device. The Schedule data type allows many different expressions, for example. \"Every 8 hours\"; \"Three times a day\"; \"1/2 an hour before breakfast for 10 days from 23-Dec 2011:\"; \"15 Oct 2013, 17 Oct 2013 and 1 Nov 2013\"."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DeviceRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    #[default]
    Invalid,
}
#[doc = "Specific parameters for the ordered item.  For example, the prism value for lenses."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceRequestParameter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code or string that identifies the device detail being asserted."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The value of the device detail."]
    pub r#value: Option<DeviceRequestParameterValue>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceRequestParameter {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "Represents a request a device to be provided to a specific patient. The device may be an implantable device to be subsequently implanted, or an external assistive device, such as a walker, to be delivered and subsequently be used."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceRequest {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<Box<super::super::types::Id>>,
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
    #[doc = "Identifiers assigned to this order by the orderer or by the receiver."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this DeviceRequest."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, orderset or other definition that is adhered to in whole or in part by this DeviceRequest."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "Plan/proposal/order fulfilled by this request."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "The request takes the place of the referenced completed or terminated request(s)."]
    pub r#replaces: Vec<super::super::types::Reference>,
    #[doc = "A shared identifier common to multiple independent Request instances that were activated/authorized more or less simultaneously by a single author.  The presence of the same identifier on each request ties those requests together and may have business ramifications in terms of reporting of results, billing, etc.  E.g. a requisition number shared by a set of lab tests ordered together, or a prescription number shared by all meds ordered at one time."]
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The status of the request."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Whether the request is a proposal, plan, an original order or a reflex order."]
    pub r#intent: super::super::types::Code,
    #[doc = "Indicates how quickly the request should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "If true, indicates that the provider is asking for the patient to either stop using or to not start using the specified device or category of devices. For example, the patient has undergone surgery and the provider is indicating that the patient should not wear contact lenses."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "The details of the device to be used."]
    pub r#code: Box<super::super::types::CodeableReference>,
    #[doc = "The number of devices to be provided."]
    pub r#quantity: Option<super::super::types::Integer>,
    #[doc = "Specific parameters for the ordered item.  For example, the prism value for lenses."]
    pub r#parameter: Vec<DeviceRequestParameter>,
    #[doc = "The patient who will use the device."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "An encounter that provides additional context in which this request is made."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The timing schedule for the use of the device. The Schedule data type allows many different expressions, for example. \"Every 8 hours\"; \"Three times a day\"; \"1/2 an hour before breakfast for 10 days from 23-Dec 2011:\"; \"15 Oct 2013, 17 Oct 2013 and 1 Nov 2013\"."]
    pub r#occurrence: Option<DeviceRequestOccurrence>,
    #[doc = "When the request transitioned to being actionable."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "The individual or entity who initiated the request and has responsibility for its activation."]
    pub r#requester: Option<Box<super::super::types::Reference>>,
    #[doc = "The desired individual or entity to provide the device to the subject of the request (e.g., patient, location)."]
    pub r#performer: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Reason or justification for the use of this device."]
    pub r#reason: Vec<super::super::types::CodeableReference>,
    #[doc = "This status is to indicate whether the request is a PRN or may be given as needed."]
    pub r#as_needed: Option<super::super::types::Boolean>,
    #[doc = "The reason for using the device."]
    pub r#as_needed_for: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Insurance plans, coverage extensions, pre-authorizations and/or pre-determinations that may be required for delivering the requested service."]
    pub r#insurance: Vec<super::super::types::Reference>,
    #[doc = "Additional clinical information about the patient that may influence the request fulfilment.  For example, this may include where on the subject's body the device will be used (i.e. the target site)."]
    pub r#supporting_info: Vec<super::super::types::Reference>,
    #[doc = "Details about this request that were not represented at all or sufficiently in one of the attributes provided in a class. These may include for example a comment, an instruction, or a note associated with the statement."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Key events in the history of the request."]
    pub r#relevant_history: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceRequest {
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
            r#replaces: Default::default(),
            r#group_identifier: Default::default(),
            r#status: Default::default(),
            r#intent: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#priority: Default::default(),
            r#do_not_perform: Default::default(),
            r#code: Box::new(super::super::types::CodeableReference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#quantity: Default::default(),
            r#parameter: Default::default(),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#encounter: Default::default(),
            r#occurrence: Default::default(),
            r#authored_on: Default::default(),
            r#requester: Default::default(),
            r#performer: Default::default(),
            r#reason: Default::default(),
            r#as_needed: Default::default(),
            r#as_needed_for: Default::default(),
            r#insurance: Default::default(),
            r#supporting_info: Default::default(),
            r#note: Default::default(),
            r#relevant_history: Default::default(),
        }
    }
}
