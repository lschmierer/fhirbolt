// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The asserted justification for using the data."]
#[derive(Debug, Clone, PartialEq)]
pub struct PermissionJustification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "This would be a codeableconcept, or a coding, which can be constrained to , for example, the 6 grounds for processing in GDPR."]
    pub r#basis: Vec<super::super::types::CodeableConcept>,
    #[doc = "Justifing rational."]
    pub r#evidence: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for PermissionJustification {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#basis: Default::default(),
            r#evidence: Default::default(),
        }
    }
}
#[doc = "Explicit FHIR Resource references."]
#[derive(Debug, Clone, PartialEq)]
pub struct PermissionRuleDataResource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "How the resource reference is interpreted when testing consent restrictions."]
    pub r#meaning: super::super::types::Code,
    #[doc = "A reference to a specific resource that defines which resources are covered by this consent."]
    pub r#reference: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for PermissionRuleDataResource {
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
#[doc = "A description or definition of which activities are allowed to be done on the data."]
#[derive(Debug, Clone, PartialEq)]
pub struct PermissionRuleData {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Explicit FHIR Resource references."]
    pub r#resource: Vec<PermissionRuleDataResource>,
    #[doc = "The data in scope are those with the given codes present in that data .meta.security element."]
    pub r#security: Vec<super::super::types::Coding>,
    #[doc = "Clinical or Operational Relevant period of time that bounds the data controlled by this rule."]
    pub r#period: Vec<super::super::types::Period>,
    #[doc = "Used when other data selection elements are insufficient."]
    pub r#expression: Option<Box<super::super::types::Expression>>,
}
#[allow(clippy::derivable_impls)]
impl Default for PermissionRuleData {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#resource: Default::default(),
            r#security: Default::default(),
            r#period: Default::default(),
            r#expression: Default::default(),
        }
    }
}
#[doc = "A description or definition of which activities are allowed to be done on the data."]
#[derive(Debug, Clone, PartialEq)]
pub struct PermissionRuleActivity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The actor(s) authorized for the defined activity."]
    pub r#actor: Vec<super::super::types::Reference>,
    #[doc = "Actions controlled by this Rule."]
    pub r#action: Vec<super::super::types::CodeableConcept>,
    #[doc = "The purpose for which the permission is given."]
    pub r#purpose: Vec<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for PermissionRuleActivity {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#actor: Default::default(),
            r#action: Default::default(),
            r#purpose: Default::default(),
        }
    }
}
#[doc = "A set of rules."]
#[derive(Debug, Clone, PartialEq)]
pub struct PermissionRule {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "deny | permit."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "A description or definition of which activities are allowed to be done on the data."]
    pub r#data: Vec<PermissionRuleData>,
    #[doc = "A description or definition of which activities are allowed to be done on the data."]
    pub r#activity: Vec<PermissionRuleActivity>,
    #[doc = "What limits apply to the use of the data."]
    pub r#limit: Vec<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for PermissionRule {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#data: Default::default(),
            r#activity: Default::default(),
            r#limit: Default::default(),
        }
    }
}
#[doc = "Permission resource holds access rules for a given data and context."]
#[derive(Debug, Clone, PartialEq)]
pub struct Permission {
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
    #[doc = "Status."]
    pub r#status: super::super::types::Code,
    #[doc = "The person or entity that asserts the permission."]
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date that permission was asserted."]
    pub r#date: Vec<super::super::types::DateTime>,
    #[doc = "The period in which the permission is active."]
    pub r#validity: Option<Box<super::super::types::Period>>,
    #[doc = "The asserted justification for using the data."]
    pub r#justification: Option<PermissionJustification>,
    #[doc = "Defines a procedure for arriving at an access decision given the set of rules."]
    pub r#combining: super::super::types::Code,
    #[doc = "A set of rules."]
    pub r#rule: Vec<PermissionRule>,
}
#[allow(clippy::derivable_impls)]
impl Default for Permission {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#asserter: Default::default(),
            r#date: Default::default(),
            r#validity: Default::default(),
            r#justification: Default::default(),
            r#combining: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#rule: Default::default(),
        }
    }
}
