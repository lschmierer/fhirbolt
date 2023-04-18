// Generated on 2023-04-18 by fhirbolt-codegen v0.2.0
#[doc = "The party(s) that are responsible for covering the payment of this account, and what order should they be applied to the account."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccountCoverage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The party(s) that contribute to payment (or part of) of the charges applied to this account (including self-pay).\n\nA coverage may only be responsible for specific types of charges, and the sequence of the coverages in the account could be important when processing billing."]
    pub r#coverage: Box<super::super::types::Reference>,
    #[doc = "The priority of the coverage in the context of this account."]
    pub r#priority: Option<super::super::types::PositiveInt>,
}
#[doc = "The parties responsible for balancing the account if other payment options fall short."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccountGuarantor {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The entity who is responsible."]
    pub r#party: Box<super::super::types::Reference>,
    #[doc = "A guarantor may be placed on credit hold or otherwise have their role temporarily suspended."]
    pub r#on_hold: Option<super::super::types::Boolean>,
    #[doc = "The timeframe during which the guarantor accepts responsibility for the account."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[doc = "A financial tool for tracking value accrued for a particular purpose.  In the healthcare field, used to track charges for a patient, cost centers, etc."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Account {
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
    #[doc = "Unique identifier used to reference the account.  Might or might not be intended for human use (e.g. credit card number)."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Indicates whether the account is presently used/usable or not."]
    pub r#status: super::super::types::Code,
    #[doc = "Categorizes the account for reporting and searching purposes."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Name used for the account when displaying it to humans in reports, etc."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Identifies the entity which incurs the expenses. While the immediate recipients of services or goods might be entities related to the subject, the expenses were ultimately incurred by the subject of the Account."]
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    #[doc = "The date range of services associated with this account."]
    pub r#service_period: Option<Box<super::super::types::Period>>,
    #[doc = "The party(s) that are responsible for covering the payment of this account, and what order should they be applied to the account."]
    pub r#coverage: Vec<AccountCoverage>,
    #[doc = "Indicates the service area, hospital, department, etc. with responsibility for managing the Account."]
    pub r#owner: Option<Box<super::super::types::Reference>>,
    #[doc = "Provides additional information about what the account tracks and how it is used."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The parties responsible for balancing the account if other payment options fall short."]
    pub r#guarantor: Vec<AccountGuarantor>,
    #[doc = "Reference to a parent Account."]
    pub r#part_of: Option<Box<super::super::types::Reference>>,
}
