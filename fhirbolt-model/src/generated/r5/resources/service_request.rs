// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
#[doc = "Indicates a value for the order detail."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ServiceRequestOrderDetailParameterValue {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
    Range(Box<super::super::types::Range>),
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "An amount of service being requested which can be a quantity ( for example $1,500 home modification), a ratio ( for example, 20 half day visits per month), or a range (2.0 to 1.8 Gy per fraction)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ServiceRequestQuantity {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
    Range(Box<super::super::types::Range>),
    #[default]
    Invalid,
}
#[doc = "The date/time at which the requested service should occur."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ServiceRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    #[default]
    Invalid,
}
#[doc = "If a CodeableConcept is present, it indicates the pre-condition for performing the service.  For example \"pain\", \"on flare-up\", etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ServiceRequestAsNeeded {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "Instructions in terms that are understood by the patient or consumer."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ServiceRequestPatientInstructionInstruction {
    Markdown(Box<super::super::types::Markdown>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The parameter details for the service being requested."]
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceRequestOrderDetailParameter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A value representing the additional detail or instructions for the order (e.g., catheter insertion, body elevation, descriptive device configuration and/or setting instructions)."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates a value for the order detail."]
    pub r#value: ServiceRequestOrderDetailParameterValue,
}
#[allow(clippy::derivable_impls)]
impl Default for ServiceRequestOrderDetailParameter {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Default::default(),
        }
    }
}
#[doc = "Additional details and instructions about the how the services are to be delivered.   For example, and order for a urinary catheter may have an order detail for an external or indwelling catheter, or an order for a bandage may require additional instructions specifying how the bandage should be applied."]
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceRequestOrderDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Indicates the context of the order details by reference."]
    pub r#parameter_focus: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The parameter details for the service being requested."]
    pub r#parameter: Vec<ServiceRequestOrderDetailParameter>,
}
#[allow(clippy::derivable_impls)]
impl Default for ServiceRequestOrderDetail {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#parameter_focus: Default::default(),
            r#parameter: Default::default(),
        }
    }
}
#[doc = "Instructions in terms that are understood by the patient or consumer."]
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceRequestPatientInstruction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Instructions in terms that are understood by the patient or consumer."]
    pub r#instruction: Option<ServiceRequestPatientInstructionInstruction>,
}
#[allow(clippy::derivable_impls)]
impl Default for ServiceRequestPatientInstruction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#instruction: Default::default(),
        }
    }
}
#[doc = "A record of a request for service such as diagnostic investigations, treatments, or operations to be performed."]
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceRequest {
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
    #[doc = "Identifiers assigned to this order instance by the orderer and/or the receiver and/or order fulfiller."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this ServiceRequest."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, orderset or other definition that is adhered to in whole or in part by this ServiceRequest."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "Plan/proposal/order fulfilled by this request."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "The request takes the place of the referenced completed or terminated request(s)."]
    pub r#replaces: Vec<super::super::types::Reference>,
    #[doc = "A shared identifier common to all service requests that were authorized more or less simultaneously by a single author, representing the composite or group identifier."]
    pub r#requisition: Option<Box<super::super::types::Identifier>>,
    #[doc = "The status of the order."]
    pub r#status: super::super::types::Code,
    #[doc = "Whether the request is a proposal, plan, an original order or a reflex order."]
    pub r#intent: super::super::types::Code,
    #[doc = "A code that classifies the service for searching, sorting and display purposes (e.g. \"Surgical Procedure\")."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indicates how quickly the ServiceRequest should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "Set this to true if the record is saying that the service/procedure should NOT be performed."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "A code or reference that identifies a particular service (i.e., procedure, diagnostic investigation, or panel of investigations) that have been requested."]
    pub r#code: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Additional details and instructions about the how the services are to be delivered.   For example, and order for a urinary catheter may have an order detail for an external or indwelling catheter, or an order for a bandage may require additional instructions specifying how the bandage should be applied."]
    pub r#order_detail: Vec<ServiceRequestOrderDetail>,
    #[doc = "An amount of service being requested which can be a quantity ( for example $1,500 home modification), a ratio ( for example, 20 half day visits per month), or a range (2.0 to 1.8 Gy per fraction)."]
    pub r#quantity: Option<ServiceRequestQuantity>,
    #[doc = "On whom or what the service is to be performed. This is usually a human patient, but can also be requested on animals, groups of humans or animals, devices such as dialysis machines, or even locations (typically for environmental scans)."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The actual focus of a service request when it is not the subject of record representing something or someone associated with the subject such as a spouse, parent, fetus, or donor. The focus of a service request could also be an existing condition,  an intervention, the subject's diet,  another service request on the subject,  or a body structure such as tumor or implanted device."]
    pub r#focus: Vec<super::super::types::Reference>,
    #[doc = "An encounter that provides additional information about the healthcare context in which this request is made."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date/time at which the requested service should occur."]
    pub r#occurrence: Option<ServiceRequestOccurrence>,
    #[doc = "If a CodeableConcept is present, it indicates the pre-condition for performing the service.  For example \"pain\", \"on flare-up\", etc."]
    pub r#as_needed: Option<ServiceRequestAsNeeded>,
    #[doc = "When the request transitioned to being actionable."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "The individual who initiated the request and has responsibility for its activation."]
    pub r#requester: Option<Box<super::super::types::Reference>>,
    #[doc = "Desired type of performer for doing the requested service."]
    pub r#performer_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The desired performer for doing the requested service.  For example, the surgeon, dermatopathologist, endoscopist, etc."]
    pub r#performer: Vec<super::super::types::Reference>,
    #[doc = "The preferred location(s) where the procedure should actually happen in coded or free text form. E.g. at home or nursing day care center."]
    pub r#location: Vec<super::super::types::CodeableReference>,
    #[doc = "An explanation or justification for why this service is being requested in coded or textual form.   This is often for billing purposes.  May relate to the resources referred to in `supportingInfo`."]
    pub r#reason: Vec<super::super::types::CodeableReference>,
    #[doc = "Insurance plans, coverage extensions, pre-authorizations and/or pre-determinations that may be needed for delivering the requested service."]
    pub r#insurance: Vec<super::super::types::Reference>,
    #[doc = "Additional clinical information about the patient or specimen that may influence the services or their interpretations.     This information includes diagnosis, clinical findings and other observations.  In laboratory ordering these are typically referred to as \"ask at order entry questions (AOEs)\".  This includes observations explicitly requested by the producer (filler) to provide context or supporting information needed to complete the order. For example,  reporting the amount of inspired oxygen for blood gas measurements."]
    pub r#supporting_info: Vec<super::super::types::CodeableReference>,
    #[doc = "One or more specimens that the laboratory procedure will use."]
    pub r#specimen: Vec<super::super::types::Reference>,
    #[doc = "Anatomic location where the procedure should be performed. This is the target site."]
    pub r#body_site: Vec<super::super::types::CodeableConcept>,
    #[doc = "Anatomic location where the procedure should be performed. This is the target site."]
    pub r#body_structure: Option<Box<super::super::types::Reference>>,
    #[doc = "Any other notes and comments made about the service request. For example, internal billing notes."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Instructions in terms that are understood by the patient or consumer."]
    pub r#patient_instruction: Vec<ServiceRequestPatientInstruction>,
    #[doc = "Key events in the history of the request."]
    pub r#relevant_history: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for ServiceRequest {
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
            r#requisition: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#intent: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#category: Default::default(),
            r#priority: Default::default(),
            r#do_not_perform: Default::default(),
            r#code: Default::default(),
            r#order_detail: Default::default(),
            r#quantity: Default::default(),
            r#subject: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#focus: Default::default(),
            r#encounter: Default::default(),
            r#occurrence: Default::default(),
            r#as_needed: Default::default(),
            r#authored_on: Default::default(),
            r#requester: Default::default(),
            r#performer_type: Default::default(),
            r#performer: Default::default(),
            r#location: Default::default(),
            r#reason: Default::default(),
            r#insurance: Default::default(),
            r#supporting_info: Default::default(),
            r#specimen: Default::default(),
            r#body_site: Default::default(),
            r#body_structure: Default::default(),
            r#note: Default::default(),
            r#patient_instruction: Default::default(),
            r#relevant_history: Default::default(),
        }
    }
}
