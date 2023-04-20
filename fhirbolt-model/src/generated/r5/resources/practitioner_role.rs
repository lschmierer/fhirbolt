// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "A specific set of Roles/Locations/specialties/services that a practitioner may perform, or has performed at an organization during a period of time.\n\nNeed to track services that a healthcare provider is able to provide at an organization's location, and the services that they can perform there."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PractitionerRole {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Business Identifiers that are specific to a role/location."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = " Whether this practitioner role record is in active use. Some systems may use this property to mark non-active practitioners, such as those that are not currently employed."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "The period during which the person is authorized to act as a practitioner in these role(s) for the organization."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Practitioner that is able to provide the defined services for the organization."]
    pub r#practitioner: Option<Box<super::super::types::Reference>>,
    #[doc = "The organization where the Practitioner performs the roles associated."]
    pub r#organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Roles which this practitioner is authorized to perform for the organization."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specialty of a practitioner that describes the functional role they are practicing at a given organization or location."]
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The location(s) at which this practitioner provides care."]
    pub r#location: Vec<Box<super::super::types::Reference>>,
    #[doc = "The list of healthcare services that this worker provides for this role's Organization/Location(s)."]
    pub r#healthcare_service: Vec<Box<super::super::types::Reference>>,
    #[doc = "The contact details of communication devices available relevant to the specific PractitionerRole. This can include addresses, phone numbers, fax numbers, mobile numbers, email addresses and web sites."]
    pub r#contact: Vec<Box<super::super::types::ExtendedContactDetail>>,
    #[doc = "Collection of characteristics (attributes)."]
    pub r#characteristic: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A language the practitioner can use in patient communication. The practitioner may know several languages (listed in practitioner.communication), however these are the languages that could be advertised in a directory for a patient to search."]
    pub r#communication: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A collection of times the practitioner is available or performing this role at the location and/or healthcareservice."]
    pub r#availability: Vec<Box<super::super::types::Availability>>,
    #[doc = " Technical endpoints providing access to services operated for the practitioner with this role. Commonly used for locating scheduling services, or identifying where to send referrals electronically."]
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
}
