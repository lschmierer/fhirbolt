// Generated on 2023-04-19 by fhirbolt-codegen v0.3.0
#[doc = "A collection of times the practitioner is available or performing this role at the location and/or healthcareservice."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PractitionerRoleAvailableTime {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates which days of the week are available between the start and end Times."]
    pub r#days_of_week: Vec<super::super::types::Code>,
    #[doc = "Is this always available? (hence times are irrelevant) e.g. 24 hour service."]
    pub r#all_day: Option<super::super::types::Boolean>,
    #[doc = "The opening time of day. Note: If the AllDay flag is set, then this time is ignored."]
    pub r#available_start_time: Option<super::super::types::Time>,
    #[doc = "The closing time of day. Note: If the AllDay flag is set, then this time is ignored."]
    pub r#available_end_time: Option<super::super::types::Time>,
}
#[doc = "The practitioner is not available or performing this role during this period of time due to the provided reason."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PractitionerRoleNotAvailable {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The reason that can be presented to the user as to why this time is not available."]
    pub r#description: super::super::types::String,
    #[doc = "Service is not available (seasonally or for a public holiday) from this date."]
    pub r#during: Option<Box<super::super::types::Period>>,
}
#[doc = "A specific set of Roles/Locations/specialties/services that a practitioner may perform at an organization for a period of time.\n\nNeed to track services that a healthcare provider is able to provide at an organization's location, and the services that they can perform there."]
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Business Identifiers that are specific to a role/location."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Whether this practitioner role record is in active use."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "The period during which the person is authorized to act as a practitioner in these role(s) for the organization."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Practitioner that is able to provide the defined services for the organization."]
    pub r#practitioner: Option<Box<super::super::types::Reference>>,
    #[doc = "The organization where the Practitioner performs the roles associated."]
    pub r#organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Roles which this practitioner is authorized to perform for the organization."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specific specialty of the practitioner."]
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The location(s) at which this practitioner provides care."]
    pub r#location: Vec<Box<super::super::types::Reference>>,
    #[doc = "The list of healthcare services that this worker provides for this role's Organization/Location(s)."]
    pub r#healthcare_service: Vec<Box<super::super::types::Reference>>,
    #[doc = "Contact details that are specific to the role/location/service."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "A collection of times the practitioner is available or performing this role at the location and/or healthcareservice."]
    pub r#available_time: Vec<PractitionerRoleAvailableTime>,
    #[doc = "The practitioner is not available or performing this role during this period of time due to the provided reason."]
    pub r#not_available: Vec<PractitionerRoleNotAvailable>,
    #[doc = "A description of site availability exceptions, e.g. public holiday availability. Succinctly describing all possible exceptions to normal site availability as details in the available Times and not available Times."]
    pub r#availability_exceptions: Option<super::super::types::String>,
    #[doc = "Technical endpoints providing access to services operated for the practitioner with this role."]
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
}
