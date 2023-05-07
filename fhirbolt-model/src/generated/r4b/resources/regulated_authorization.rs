// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
#[doc = "Relevant date for this case."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum RegulatedAuthorizationCaseDate {
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
    #[default]
    Invalid,
}
#[doc = "The case or regulatory procedure for granting or amending a regulated authorization. An authorization is granted in response to submissions/applications by those seeking authorization. A case is the administrative process that deals with the application(s) that relate to this and assesses them. Note: This area is subject to ongoing review and the workgroup is seeking implementer feedback on its use (see link at bottom of page)."]
#[derive(Debug, Clone, PartialEq)]
pub struct RegulatedAuthorizationCase {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifier by which this case can be referenced."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The defining type of case."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status associated with the case."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Relevant date for this case."]
    pub r#date: Option<RegulatedAuthorizationCaseDate>,
    #[doc = "A regulatory submission from an organization to a regulator, as part of an assessing case. Multiple applications may occur over time, with more or different information to support or modify the submission or the authorization. The applications can be considered as steps within the longer running case or procedure for this authorization process."]
    pub r#application: Vec<RegulatedAuthorizationCase>,
}
#[allow(clippy::derivable_impls)]
impl Default for RegulatedAuthorizationCase {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#type: Default::default(),
            r#status: Default::default(),
            r#date: Default::default(),
            r#application: Default::default(),
        }
    }
}
#[doc = "Regulatory approval, clearance or licencing related to a regulated product, treatment, facility or activity that is cited in a guidance, regulation, rule or legislative act. An example is Market Authorization relating to a Medicinal Product."]
#[derive(Debug, Clone, PartialEq)]
pub struct RegulatedAuthorization {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Business identifier for the authorization, typically assigned by the authorizing body."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The product type, treatment, facility or activity that is being authorized."]
    pub r#subject: Vec<super::super::types::Reference>,
    #[doc = "Overall type of this authorization, for example drug marketing approval, orphan drug designation."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "General textual supporting information."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The territory (e.g., country, jurisdiction etc.) in which the authorization has been granted."]
    pub r#region: Vec<super::super::types::CodeableConcept>,
    #[doc = "The status that is authorised e.g. approved. Intermediate states and actions can be tracked with cases and applications."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date at which the current status was assigned."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "The time period in which the regulatory approval, clearance or licencing is in effect. As an example, a Marketing Authorization includes the date of authorization and/or an expiration date."]
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    #[doc = "Condition for which the use of the regulated product applies."]
    pub r#indication: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The intended use of the product, e.g. prevention, treatment, diagnosis."]
    pub r#intended_use: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The legal or regulatory framework against which this authorization is granted, or other reasons for it."]
    pub r#basis: Vec<super::super::types::CodeableConcept>,
    #[doc = "The organization that has been granted this authorization, by some authoritative body (the 'regulator')."]
    pub r#holder: Option<Box<super::super::types::Reference>>,
    #[doc = "The regulatory authority or authorizing body granting the authorization. For example, European Medicines Agency (EMA), Food and Drug Administration (FDA), Health Canada (HC), etc."]
    pub r#regulator: Option<Box<super::super::types::Reference>>,
    #[doc = "The case or regulatory procedure for granting or amending a regulated authorization. An authorization is granted in response to submissions/applications by those seeking authorization. A case is the administrative process that deals with the application(s) that relate to this and assesses them. Note: This area is subject to ongoing review and the workgroup is seeking implementer feedback on its use (see link at bottom of page)."]
    pub r#case: Option<RegulatedAuthorizationCase>,
}
#[allow(clippy::derivable_impls)]
impl Default for RegulatedAuthorization {
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
            r#subject: Default::default(),
            r#type: Default::default(),
            r#description: Default::default(),
            r#region: Default::default(),
            r#status: Default::default(),
            r#status_date: Default::default(),
            r#validity_period: Default::default(),
            r#indication: Default::default(),
            r#intended_use: Default::default(),
            r#basis: Default::default(),
            r#holder: Default::default(),
            r#regulator: Default::default(),
            r#case: Default::default(),
        }
    }
}
