// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The official certifications, accreditations, training, designations and licenses that authorize and/or otherwise endorse the provision of care by the organization.\r\rFor example, an approval to provide a type of services issued by a certifying body (such as the US Joint Commission) to an organization."]
#[derive(Debug, Clone, PartialEq)]
pub struct OrganizationQualification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An identifier allocated to this qualification for this organization."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Coded representation of the qualification."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Period during which the qualification is valid."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Organization that regulates and issues the qualification."]
    pub r#issuer: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for OrganizationQualification {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#period: Default::default(),
            r#issuer: Default::default(),
        }
    }
}
#[doc = "A formally or informally recognized grouping of people or organizations formed for the purpose of achieving some form of collective action.  Includes companies, institutions, corporations, departments, community groups, healthcare practice groups, payer/insurer, etc."]
#[derive(Debug, Clone, PartialEq)]
pub struct Organization {
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
    #[doc = "Identifier for the organization that is used to identify the organization across multiple disparate systems."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Whether the organization's record is still in active use."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "The kind(s) of organization that this is."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "A name associated with the organization."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A list of alternate names that the organization is known as, or was known as in the past."]
    pub r#alias: Vec<super::super::types::String>,
    #[doc = "Description of the organization, which helps provide additional general context on the organization to ensure that the correct organization is selected."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The contact details of communication devices available relevant to the specific Organization. This can include addresses, phone numbers, fax numbers, mobile numbers, email addresses and web sites."]
    pub r#contact: Vec<super::super::types::ExtendedContactDetail>,
    #[doc = "The organization of which this organization forms a part."]
    pub r#part_of: Option<Box<super::super::types::Reference>>,
    #[doc = "Technical endpoints providing access to services operated for the organization."]
    pub r#endpoint: Vec<super::super::types::Reference>,
    #[doc = "The official certifications, accreditations, training, designations and licenses that authorize and/or otherwise endorse the provision of care by the organization.\r\rFor example, an approval to provide a type of services issued by a certifying body (such as the US Joint Commission) to an organization."]
    pub r#qualification: Vec<OrganizationQualification>,
}
#[allow(clippy::derivable_impls)]
impl Default for Organization {
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
            r#type: Default::default(),
            r#name: Default::default(),
            r#alias: Default::default(),
            r#description: Default::default(),
            r#contact: Default::default(),
            r#part_of: Default::default(),
            r#endpoint: Default::default(),
            r#qualification: Default::default(),
        }
    }
}
