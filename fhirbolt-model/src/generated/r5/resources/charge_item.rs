// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Date/time(s) or duration when the charged service was applied."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ChargeItemOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    #[default]
    Invalid,
}
#[doc = "Indicates who or what performed or participated in the charged service."]
#[derive(Debug, Clone, PartialEq)]
pub struct ChargeItemPerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Describes the type of performance or participation(e.g. primary surgeon, anesthesiologiest, etc.)."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The device, practitioner, etc. who performed or participated in the service."]
    pub r#actor: Box<super::super::types::Reference>,
}
impl Default for ChargeItemPerformer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#function: Default::default(),
            r#actor: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "The resource ChargeItem describes the provision of healthcare provider products for a certain patient, therefore referring not only to the product, but containing in addition details of the provision, like date, time, amounts and participating organizations and persons. Main Usage of the ChargeItem is to enable the billing process and internal cost allocation."]
#[derive(Debug, Clone, PartialEq)]
pub struct ChargeItem {
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
    #[doc = "Identifiers assigned to this event performer or other systems."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "References the (external) source of pricing information, rules of application for the code this ChargeItem uses."]
    pub r#definition_uri: Vec<super::super::types::Uri>,
    #[doc = "References the source of pricing information, rules of application for the code this ChargeItem uses."]
    pub r#definition_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The current state of the ChargeItem."]
    pub r#status: super::super::types::Code,
    #[doc = "ChargeItems can be grouped to larger ChargeItems covering the whole set."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "A code that identifies the charge, like a billing code."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The individual or set of individuals the action is being or was performed on."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "This ChargeItem has the details of how the associated Encounter should be billed or otherwise be handled by finance systems."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Date/time(s) or duration when the charged service was applied."]
    pub r#occurrence: Option<ChargeItemOccurrence>,
    #[doc = "Indicates who or what performed or participated in the charged service."]
    pub r#performer: Vec<ChargeItemPerformer>,
    #[doc = "The organization performing the service."]
    pub r#performing_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "The organization requesting the service."]
    pub r#requesting_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "The financial cost center permits the tracking of charge attribution."]
    pub r#cost_center: Option<Box<super::super::types::Reference>>,
    #[doc = "Quantity of which the charge item has been serviced."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The anatomical location where the related service has been applied."]
    pub r#bodysite: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The unit price of the chargable item."]
    pub r#unit_price_component: Option<Box<super::super::types::MonetaryComponent>>,
    #[doc = "The total price for the chargable item, accounting for the quantity."]
    pub r#total_price_component: Option<Box<super::super::types::MonetaryComponent>>,
    #[doc = "If the list price or the rule-based factor associated with the code is overridden, this attribute can capture a text to indicate the  reason for this action."]
    pub r#override_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The device, practitioner, etc. who entered the charge item."]
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    #[doc = "Date the charge item was entered."]
    pub r#entered_date: Option<super::super::types::DateTime>,
    #[doc = "Describes why the event occurred in coded or textual form."]
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicated the rendered service that caused this charge."]
    pub r#service: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "Identifies the device, food, drug or other product being charged either by type code or reference to an instance."]
    pub r#product: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "Account into which this ChargeItems belongs."]
    pub r#account: Vec<Box<super::super::types::Reference>>,
    #[doc = "Comments made about the event by the performer, subject or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Further information supporting this charge."]
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
}
impl Default for ChargeItem {
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
            r#definition_uri: Default::default(),
            r#definition_canonical: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#part_of: Default::default(),
            r#code: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#subject: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#encounter: Default::default(),
            r#occurrence: Default::default(),
            r#performer: Default::default(),
            r#performing_organization: Default::default(),
            r#requesting_organization: Default::default(),
            r#cost_center: Default::default(),
            r#quantity: Default::default(),
            r#bodysite: Default::default(),
            r#unit_price_component: Default::default(),
            r#total_price_component: Default::default(),
            r#override_reason: Default::default(),
            r#enterer: Default::default(),
            r#entered_date: Default::default(),
            r#reason: Default::default(),
            r#service: Default::default(),
            r#product: Default::default(),
            r#account: Default::default(),
            r#note: Default::default(),
            r#supporting_information: Default::default(),
        }
    }
}
