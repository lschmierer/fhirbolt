// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "An amount of service being requested which can be a quantity ( for example $1,500 home modification), a ratio ( for example, 20 half day visits per month), or a range (2.0 to 1.8 Gy per fraction)."]
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceRequestQuantity {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
    Range(Box<super::super::types::Range>),
    Invalid,
}
impl Default for ServiceRequestQuantity {
    fn default() -> ServiceRequestQuantity {
        ServiceRequestQuantity::Invalid
    }
}
#[doc = "The date/time at which the requested service should occur."]
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for ServiceRequestOccurrence {
    fn default() -> ServiceRequestOccurrence {
        ServiceRequestOccurrence::Invalid
    }
}
#[doc = "If a CodeableConcept is present, it indicates the pre-condition for performing the service.  For example \"pain\", \"on flare-up\", etc."]
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceRequestAsNeeded {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for ServiceRequestAsNeeded {
    fn default() -> ServiceRequestAsNeeded {
        ServiceRequestAsNeeded::Invalid
    }
}
#[doc = "A record of a request for service such as diagnostic investigations, treatments, or operations to be performed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServiceRequest {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifiers assigned to this order instance by the orderer and/or the receiver and/or order fulfiller."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this ServiceRequest."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, orderset or other definition that is adhered to in whole or in part by this ServiceRequest."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "Plan/proposal/order fulfilled by this request."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "The request takes the place of the referenced completed or terminated request(s)."]
    pub r#replaces: Vec<Box<super::super::types::Reference>>,
    #[doc = "A shared identifier common to all service requests that were authorized more or less simultaneously by a single author, representing the composite or group identifier."]
    pub r#requisition: Option<Box<super::super::types::Identifier>>,
    #[doc = "The status of the order."]
    pub r#status: super::super::types::Code,
    #[doc = "Whether the request is a proposal, plan, an original order or a reflex order."]
    pub r#intent: super::super::types::Code,
    #[doc = "A code that classifies the service for searching, sorting and display purposes (e.g. \"Surgical Procedure\")."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates how quickly the ServiceRequest should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "Set this to true if the record is saying that the service/procedure should NOT be performed."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "A code that identifies a particular service (i.e., procedure, diagnostic investigation, or panel of investigations) that have been requested."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Additional details and instructions about the how the services are to be delivered.   For example, and order for a urinary catheter may have an order detail for an external or indwelling catheter, or an order for a bandage may require additional instructions specifying how the bandage should be applied."]
    pub r#order_detail: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "An amount of service being requested which can be a quantity ( for example $1,500 home modification), a ratio ( for example, 20 half day visits per month), or a range (2.0 to 1.8 Gy per fraction)."]
    pub r#quantity: Option<ServiceRequestQuantity>,
    #[doc = "On whom or what the service is to be performed. This is usually a human patient, but can also be requested on animals, groups of humans or animals, devices such as dialysis machines, or even locations (typically for environmental scans)."]
    pub r#subject: Box<super::super::types::Reference>,
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
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    #[doc = "The preferred location(s) where the procedure should actually happen in coded or free text form. E.g. at home or nursing day care center."]
    pub r#location_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A reference to the the preferred location(s) where the procedure should actually happen. E.g. at home or nursing day care center."]
    pub r#location_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "An explanation or justification for why this service is being requested in coded or textual form.   This is often for billing purposes.  May relate to the resources referred to in `supportingInfo`."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates another resource that provides a justification for why this service is being requested.   May relate to the resources referred to in `supportingInfo`."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Insurance plans, coverage extensions, pre-authorizations and/or pre-determinations that may be needed for delivering the requested service."]
    pub r#insurance: Vec<Box<super::super::types::Reference>>,
    #[doc = "Additional clinical information about the patient or specimen that may influence the services or their interpretations.     This information includes diagnosis, clinical findings and other observations.  In laboratory ordering these are typically referred to as \"ask at order entry questions (AOEs)\".  This includes observations explicitly requested by the producer (filler) to provide context or supporting information needed to complete the order. For example,  reporting the amount of inspired oxygen for blood gas measurements."]
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    #[doc = "One or more specimens that the laboratory procedure will use."]
    pub r#specimen: Vec<Box<super::super::types::Reference>>,
    #[doc = "Anatomic location where the procedure should be performed. This is the target site."]
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Any other notes and comments made about the service request. For example, internal billing notes."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Instructions in terms that are understood by the patient or consumer."]
    pub r#patient_instruction: Option<super::super::types::String>,
    #[doc = "Key events in the history of the request."]
    pub r#relevant_history: Vec<Box<super::super::types::Reference>>,
}
