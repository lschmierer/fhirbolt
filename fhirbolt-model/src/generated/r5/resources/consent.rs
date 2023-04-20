// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "A Reference or URL used to uniquely identify the policy the organization will enforce for this Consent. This Reference or URL should be specific to the version of the policy and should be dereferencable to a computable policy of some form."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentPolicyBasis {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A Reference that identifies the policy the organization will enforce for this Consent."]
    pub r#reference: Option<Box<super::super::types::Reference>>,
    #[doc = "A URL that links to a computable version of the policy the organization will enforce for this Consent."]
    pub r#url: Option<super::super::types::Url>,
}
impl Default for ConsentPolicyBasis {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#reference: Default::default(),
            r#url: Default::default(),
        }
    }
}
#[doc = "Whether a treatment instruction (e.g. artificial respiration: yes or no) was verified with the patient, his/her family or another authorized person."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentVerification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Has the instruction been verified."]
    pub r#verified: super::super::types::Boolean,
    #[doc = "Extensible list of verification type starting with verification and re-validation."]
    pub r#verification_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The person who conducted the verification/validation of the Grantor decision."]
    pub r#verified_by: Option<Box<super::super::types::Reference>>,
    #[doc = "Who verified the instruction (Patient, Relative or other Authorized Person)."]
    pub r#verified_with: Option<Box<super::super::types::Reference>>,
    #[doc = "Date(s) verification was collected."]
    pub r#verification_date: Vec<super::super::types::DateTime>,
}
impl Default for ConsentVerification {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#verified: {
                let mut default: super::super::types::Boolean = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#verification_type: Default::default(),
            r#verified_by: Default::default(),
            r#verified_with: Default::default(),
            r#verification_date: Default::default(),
        }
    }
}
#[doc = "Who or what is controlled by this provision. Use group to identify a set of actors by some property they share (e.g. 'admitting officers')."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentProvisionActor {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "How the individual is involved in the resources content that is described in the exception."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The resource that identifies the actor. To identify actors by type, use group to identify a set of actors by some property they share (e.g. 'admitting officers')."]
    pub r#reference: Option<Box<super::super::types::Reference>>,
}
impl Default for ConsentProvisionActor {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#role: Default::default(),
            r#reference: Default::default(),
        }
    }
}
#[doc = "The resources controlled by this provision if specific resources are referenced."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentProvisionData {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "How the resource reference is interpreted when testing consent restrictions."]
    pub r#meaning: super::super::types::Code,
    #[doc = "A reference to a specific resource that defines which resources are covered by this consent."]
    pub r#reference: Box<super::super::types::Reference>,
}
impl Default for ConsentProvisionData {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#meaning: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#reference: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "An exception to the base policy of this consent. An exception can be an addition or removal of access permissions."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentProvision {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Timeframe for this provision."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Who or what is controlled by this provision. Use group to identify a set of actors by some property they share (e.g. 'admitting officers')."]
    pub r#actor: Vec<ConsentProvisionActor>,
    #[doc = "Actions controlled by this provision."]
    pub r#action: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A security label, comprised of 0..* security label fields (Privacy tags), which define which resources are controlled by this exception."]
    pub r#security_label: Vec<Box<super::super::types::Coding>>,
    #[doc = "The context of the activities a user is taking - why the user is accessing the data - that are controlled by this provision."]
    pub r#purpose: Vec<Box<super::super::types::Coding>>,
    #[doc = "The documentType(s) covered by this provision. The type can be a CDA document, or some other type that indicates what sort of information the consent relates to."]
    pub r#document_type: Vec<Box<super::super::types::Coding>>,
    #[doc = "The resourceType(s) covered by this provision. The type can be a FHIR resource type or a profile on a type that indicates what information the consent relates to."]
    pub r#resource_type: Vec<Box<super::super::types::Coding>>,
    #[doc = "If this code is found in an instance, then the provision applies."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Clinical or Operational Relevant period of time that bounds the data controlled by this provision."]
    pub r#data_period: Option<Box<super::super::types::Period>>,
    #[doc = "The resources controlled by this provision if specific resources are referenced."]
    pub r#data: Vec<ConsentProvisionData>,
    #[doc = "A computable (FHIRPath or other) definition of what is controlled by this consent."]
    pub r#expression: Option<Box<super::super::types::Expression>>,
    #[doc = "Provisions which provide exceptions to the base provision or subprovisions."]
    pub r#provision: Vec<ConsentProvision>,
}
impl Default for ConsentProvision {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#period: Default::default(),
            r#actor: Default::default(),
            r#action: Default::default(),
            r#security_label: Default::default(),
            r#purpose: Default::default(),
            r#document_type: Default::default(),
            r#resource_type: Default::default(),
            r#code: Default::default(),
            r#data_period: Default::default(),
            r#data: Default::default(),
            r#expression: Default::default(),
            r#provision: Default::default(),
        }
    }
}
#[doc = "A record of a healthcare consumer’s  choices  or choices made on their behalf by a third party, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time."]
#[derive(Debug, Clone, PartialEq)]
pub struct Consent {
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
    #[doc = "Unique identifier for this copy of the Consent Statement."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Indicates the current state of this Consent resource."]
    pub r#status: super::super::types::Code,
    #[doc = "A classification of the type of consents found in the statement. This element supports indexing and retrieval of consent statements."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The patient/healthcare practitioner or group of persons to whom this consent applies."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Date the consent instance was agreed to."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "Effective period for this Consent Resource and all provisions unless specified in that provision."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The entity responsible for granting the rights listed in a Consent Directive."]
    pub r#grantor: Vec<Box<super::super::types::Reference>>,
    #[doc = "The entity responsible for complying with the Consent Directive, including any obligations or limitations on authorizations and enforcement of prohibitions."]
    pub r#grantee: Vec<Box<super::super::types::Reference>>,
    #[doc = "The actor that manages the consent through its lifecycle."]
    pub r#manager: Vec<Box<super::super::types::Reference>>,
    #[doc = "The actor that controls/enforces the access according to the consent."]
    pub r#controller: Vec<Box<super::super::types::Reference>>,
    #[doc = "The source on which this consent statement is based. The source might be a scanned original paper form."]
    pub r#source_attachment: Vec<Box<super::super::types::Attachment>>,
    #[doc = "A reference to a consent that links back to such a source, a reference to a document repository (e.g. XDS) that stores the original consent document."]
    pub r#source_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "A set of codes that indicate the regulatory basis (if any) that this consent supports."]
    pub r#regulatory_basis: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A Reference or URL used to uniquely identify the policy the organization will enforce for this Consent. This Reference or URL should be specific to the version of the policy and should be dereferencable to a computable policy of some form."]
    pub r#policy_basis: Option<ConsentPolicyBasis>,
    #[doc = "A Reference to the human readable policy explaining the basis for the Consent."]
    pub r#policy_text: Vec<Box<super::super::types::Reference>>,
    #[doc = "Whether a treatment instruction (e.g. artificial respiration: yes or no) was verified with the patient, his/her family or another authorized person."]
    pub r#verification: Vec<ConsentVerification>,
    #[doc = "Action to take - permit or deny - as default."]
    pub r#decision: Option<super::super::types::Code>,
    #[doc = "An exception to the base policy of this consent. An exception can be an addition or removal of access permissions."]
    pub r#provision: Vec<ConsentProvision>,
}
impl Default for Consent {
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
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#category: Default::default(),
            r#subject: Default::default(),
            r#date: Default::default(),
            r#period: Default::default(),
            r#grantor: Default::default(),
            r#grantee: Default::default(),
            r#manager: Default::default(),
            r#controller: Default::default(),
            r#source_attachment: Default::default(),
            r#source_reference: Default::default(),
            r#regulatory_basis: Default::default(),
            r#policy_basis: Default::default(),
            r#policy_text: Default::default(),
            r#verification: Default::default(),
            r#decision: Default::default(),
            r#provision: Default::default(),
        }
    }
}
