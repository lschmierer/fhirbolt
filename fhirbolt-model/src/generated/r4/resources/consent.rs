// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The source on which this consent statement is based. The source might be a scanned original paper form, or a reference to a consent that links back to such a source, a reference to a document repository (e.g. XDS) that stores the original consent document."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ConsentSource {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The references to the policies that are included in this consent scope. Policies may be organizational, but are often defined jurisdictionally, or in law."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentPolicy {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Entity or Organization having regulatory jurisdiction or accountability for  enforcing policies pertaining to Consent Directives."]
    pub r#authority: Option<super::super::types::Uri>,
    #[doc = "The references to the policies that are included in this consent scope. Policies may be organizational, but are often defined jurisdictionally, or in law."]
    pub r#uri: Option<super::super::types::Uri>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConsentPolicy {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#authority: Default::default(),
            r#uri: Default::default(),
        }
    }
}
#[doc = "Whether a treatment instruction (e.g. artificial respiration yes or no) was verified with the patient, his/her family or another authorized person."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentVerification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Has the instruction been verified."]
    pub r#verified: super::super::types::Boolean,
    #[doc = "Who verified the instruction (Patient, Relative or other Authorized Person)."]
    pub r#verified_with: Option<Box<super::super::types::Reference>>,
    #[doc = "Date verification was collected."]
    pub r#verification_date: Option<super::super::types::DateTime>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConsentVerification {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#verified: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#verified_with: Default::default(),
            r#verification_date: Default::default(),
        }
    }
}
#[doc = "Who or what is controlled by this rule. Use group to identify a set of actors by some property they share (e.g. 'admitting officers')."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentProvisionActor {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "How the individual is involved in the resources content that is described in the exception."]
    pub r#role: Box<super::super::types::CodeableConcept>,
    #[doc = "The resource that identifies the actor. To identify actors by type, use group to identify a set of actors by some property they share (e.g. 'admitting officers')."]
    pub r#reference: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConsentProvisionActor {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#role: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#reference: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "The resources controlled by this rule if specific resources are referenced."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentProvisionData {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "How the resource reference is interpreted when testing consent restrictions."]
    pub r#meaning: super::super::types::Code,
    #[doc = "A reference to a specific resource that defines which resources are covered by this consent."]
    pub r#reference: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConsentProvisionData {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#meaning: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#reference: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "An exception to the base policy of this consent. An exception can be an addition or removal of access permissions."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentProvision {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Action  to take - permit or deny - when the rule conditions are met.  Not permitted in root rule, required in all nested rules."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "The timeframe in this rule is valid."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Who or what is controlled by this rule. Use group to identify a set of actors by some property they share (e.g. 'admitting officers')."]
    pub r#actor: Vec<ConsentProvisionActor>,
    #[doc = "Actions controlled by this Rule."]
    pub r#action: Vec<super::super::types::CodeableConcept>,
    #[doc = "A security label, comprised of 0..* security label fields (Privacy tags), which define which resources are controlled by this exception."]
    pub r#security_label: Vec<super::super::types::Coding>,
    #[doc = "The context of the activities a user is taking - why the user is accessing the data - that are controlled by this rule."]
    pub r#purpose: Vec<super::super::types::Coding>,
    #[doc = "The class of information covered by this rule. The type can be a FHIR resource type, a profile on a type, or a CDA document, or some other type that indicates what sort of information the consent relates to."]
    pub r#class: Vec<super::super::types::Coding>,
    #[doc = "If this code is found in an instance, then the rule applies."]
    pub r#code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Clinical or Operational Relevant period of time that bounds the data controlled by this rule."]
    pub r#data_period: Option<Box<super::super::types::Period>>,
    #[doc = "The resources controlled by this rule if specific resources are referenced."]
    pub r#data: Vec<ConsentProvisionData>,
    #[doc = "Rules which provide exceptions to the base rule or subrules."]
    pub r#provision: Vec<ConsentProvision>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConsentProvision {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#period: Default::default(),
            r#actor: Default::default(),
            r#action: Default::default(),
            r#security_label: Default::default(),
            r#purpose: Default::default(),
            r#class: Default::default(),
            r#code: Default::default(),
            r#data_period: Default::default(),
            r#data: Default::default(),
            r#provision: Default::default(),
        }
    }
}
#[doc = "A record of a healthcare consumer’s  choices, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time."]
#[derive(Debug, Clone, PartialEq)]
pub struct Consent {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Unique identifier for this copy of the Consent Statement."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Indicates the current state of this consent."]
    pub r#status: super::super::types::Code,
    #[doc = "A selector of the type of consent being presented: ADR, Privacy, Treatment, Research.  This list is now extensible."]
    pub r#scope: Box<super::super::types::CodeableConcept>,
    #[doc = "A classification of the type of consents found in the statement. This element supports indexing and retrieval of consent statements."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "The patient/healthcare consumer to whom this consent applies."]
    pub r#patient: Option<Box<super::super::types::Reference>>,
    #[doc = "When this  Consent was issued / created / indexed."]
    pub r#date_time: Option<super::super::types::DateTime>,
    #[doc = "Either the Grantor, which is the entity responsible for granting the rights listed in a Consent Directive or the Grantee, which is the entity responsible for complying with the Consent Directive, including any obligations or limitations on authorizations and enforcement of prohibitions."]
    pub r#performer: Vec<super::super::types::Reference>,
    #[doc = "The organization that manages the consent, and the framework within which it is executed."]
    pub r#organization: Vec<super::super::types::Reference>,
    #[doc = "The source on which this consent statement is based. The source might be a scanned original paper form, or a reference to a consent that links back to such a source, a reference to a document repository (e.g. XDS) that stores the original consent document."]
    pub r#source: Option<ConsentSource>,
    #[doc = "The references to the policies that are included in this consent scope. Policies may be organizational, but are often defined jurisdictionally, or in law."]
    pub r#policy: Vec<ConsentPolicy>,
    #[doc = "A reference to the specific base computable regulation or policy."]
    pub r#policy_rule: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether a treatment instruction (e.g. artificial respiration yes or no) was verified with the patient, his/her family or another authorized person."]
    pub r#verification: Vec<ConsentVerification>,
    #[doc = "An exception to the base policy of this consent. An exception can be an addition or removal of access permissions."]
    pub r#provision: Option<ConsentProvision>,
}
#[allow(clippy::derivable_impls)]
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#scope: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#category: Default::default(),
            r#patient: Default::default(),
            r#date_time: Default::default(),
            r#performer: Default::default(),
            r#organization: Default::default(),
            r#source: Default::default(),
            r#policy: Default::default(),
            r#policy_rule: Default::default(),
            r#verification: Default::default(),
            r#provision: Default::default(),
        }
    }
}
