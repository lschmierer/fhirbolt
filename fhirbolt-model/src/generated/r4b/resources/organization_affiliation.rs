// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Defines an affiliation/assotiation/relationship between 2 distinct oganizations, that is not a part-of relationship/sub-division relationship.\n\nNeed to define relationships between organizations that are not sub-divisions of the same organization (part-of relationships)."]
#[derive(Debug, Clone, PartialEq)]
pub struct OrganizationAffiliation {
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
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Business identifiers that are specific to this role."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Whether this organization affiliation record is in active use."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "The period during which the participatingOrganization is affiliated with the primary organization."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Organization where the role is available (primary organization/has members)."]
    pub r#organization: Option<Box<super::super::types::Reference>>,
    #[doc = "The Participating Organization provides/performs the role(s) defined by the code to the Primary Organization (e.g. providing services or is a member of)."]
    pub r#participating_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Health insurance provider network in which the participatingOrganization provides the role's services (if defined) at the indicated locations (if defined)."]
    pub r#network: Vec<Box<super::super::types::Reference>>,
    #[doc = "Definition of the role the participatingOrganization plays in the association."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specific specialty of the participatingOrganization in the context of the role."]
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The location(s) at which the role occurs."]
    pub r#location: Vec<Box<super::super::types::Reference>>,
    #[doc = "Healthcare services provided through the role."]
    pub r#healthcare_service: Vec<Box<super::super::types::Reference>>,
    #[doc = "Contact details at the participatingOrganization relevant to this Affiliation."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "Technical endpoints providing access to services operated for this role."]
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
}
impl Default for OrganizationAffiliation {
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
            r#period: Default::default(),
            r#organization: Default::default(),
            r#participating_organization: Default::default(),
            r#network: Default::default(),
            r#code: Default::default(),
            r#specialty: Default::default(),
            r#location: Default::default(),
            r#healthcare_service: Default::default(),
            r#telecom: Default::default(),
            r#endpoint: Default::default(),
        }
    }
}
