// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Does this service have specific eligibility requirements that need to be met in order to use the service?"]
#[derive(Debug, Clone, PartialEq)]
pub struct HealthcareServiceEligibility {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Coded value for the eligibility."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Describes the eligibility conditions for the service."]
    pub r#comment: Option<super::super::types::Markdown>,
}
#[allow(clippy::derivable_impls)]
impl Default for HealthcareServiceEligibility {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#comment: Default::default(),
        }
    }
}
#[doc = "The details of a healthcare service available at a location or in a catalog.  In the case where there is a hierarchy of services (for example, Lab -> Pathology -> Wound Cultures), this can be represented using a set of linked HealthcareServices."]
#[derive(Debug, Clone, PartialEq)]
pub struct HealthcareService {
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
    #[doc = "External identifiers for this item."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "This flag is used to mark the record to not be used. This is not used when a center is closed for maintenance, or for holidays, the notAvailable period is to be used for this."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "The organization that provides this healthcare service."]
    pub r#provided_by: Option<Box<super::super::types::Reference>>,
    #[doc = "When the HealthcareService is representing a specific, schedulable service, the availableIn property can refer to a generic service."]
    pub r#offered_in: Vec<super::super::types::Reference>,
    #[doc = "Identifies the broad category of service being performed or delivered."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "The specific type of service that may be delivered or performed."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "Collection of specialties handled by the Healthcare service. This is more of a medical term."]
    pub r#specialty: Vec<super::super::types::CodeableConcept>,
    #[doc = "The location(s) where this healthcare service may be provided."]
    pub r#location: Vec<super::super::types::Reference>,
    #[doc = "Further description of the service as it would be presented to a consumer while searching."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Any additional description of the service and/or any specific issues not covered by the other attributes, which can be displayed as further detail under the serviceName."]
    pub r#comment: Option<super::super::types::Markdown>,
    #[doc = "Extra details about the service that can't be placed in the other fields."]
    pub r#extra_details: Option<super::super::types::Markdown>,
    #[doc = "If there is a photo/symbol associated with this HealthcareService, it may be included here to facilitate quick identification of the service in a list."]
    pub r#photo: Option<Box<super::super::types::Attachment>>,
    #[doc = "The contact details of communication devices available relevant to the specific HealthcareService. This can include addresses, phone numbers, fax numbers, mobile numbers, email addresses and web sites."]
    pub r#contact: Vec<super::super::types::ExtendedContactDetail>,
    #[doc = "The location(s) that this service is available to (not where the service is provided)."]
    pub r#coverage_area: Vec<super::super::types::Reference>,
    #[doc = "The code(s) that detail the conditions under which the healthcare service is available/offered."]
    pub r#service_provision_code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Does this service have specific eligibility requirements that need to be met in order to use the service?"]
    pub r#eligibility: Vec<HealthcareServiceEligibility>,
    #[doc = "Programs that this service is applicable to."]
    pub r#program: Vec<super::super::types::CodeableConcept>,
    #[doc = "Collection of characteristics (attributes)."]
    pub r#characteristic: Vec<super::super::types::CodeableConcept>,
    #[doc = "Some services are specifically made available in multiple languages, this property permits a directory to declare the languages this is offered in. Typically this is only provided where a service operates in communities with mixed languages used."]
    pub r#communication: Vec<super::super::types::CodeableConcept>,
    #[doc = "Ways that the service accepts referrals, if this is not provided then it is implied that no referral is required."]
    pub r#referral_method: Vec<super::super::types::CodeableConcept>,
    #[doc = "Indicates whether or not a prospective consumer will require an appointment for a particular service at a site to be provided by the Organization. Indicates if an appointment is required for access to this service."]
    pub r#appointment_required: Option<super::super::types::Boolean>,
    #[doc = "A collection of times that the healthcare service is available."]
    pub r#availability: Vec<super::super::types::Availability>,
    #[doc = "Technical endpoints providing access to services operated for the specific healthcare services defined at this resource."]
    pub r#endpoint: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for HealthcareService {
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
            r#active: Default::default(),
            r#provided_by: Default::default(),
            r#offered_in: Default::default(),
            r#category: Default::default(),
            r#type: Default::default(),
            r#specialty: Default::default(),
            r#location: Default::default(),
            r#name: Default::default(),
            r#comment: Default::default(),
            r#extra_details: Default::default(),
            r#photo: Default::default(),
            r#contact: Default::default(),
            r#coverage_area: Default::default(),
            r#service_provision_code: Default::default(),
            r#eligibility: Default::default(),
            r#program: Default::default(),
            r#characteristic: Default::default(),
            r#communication: Default::default(),
            r#referral_method: Default::default(),
            r#appointment_required: Default::default(),
            r#availability: Default::default(),
            r#endpoint: Default::default(),
        }
    }
}
