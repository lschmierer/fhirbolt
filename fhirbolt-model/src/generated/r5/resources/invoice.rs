// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Date/time(s) range of services included in this invoice."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum InvoicePeriod {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "Date/time(s) range when this service was delivered or completed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum InvoiceLineItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "The ChargeItem contains information such as the billing code, date, amount etc. If no further details are required for the lineItem, inline billing codes can be added using the CodeableConcept data type instead of the Reference."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum InvoiceLineItemChargeItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "Indicates who or what performed or participated in the charged service."]
#[derive(Debug, Clone, PartialEq)]
pub struct InvoiceParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Describes the type of involvement (e.g. transcriptionist, creator etc.). If the invoice has been created automatically, the Participant may be a billing engine or another kind of device."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The device, practitioner, etc. who performed or participated in the service."]
    pub r#actor: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for InvoiceParticipant {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#role: Default::default(),
            r#actor: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "Each line item represents one charge for goods and services rendered. Details such.ofType(date), code and amount are found in the referenced ChargeItem resource."]
#[derive(Debug, Clone, PartialEq)]
pub struct InvoiceLineItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Sequence in which the items appear on the invoice."]
    pub r#sequence: Option<super::super::types::PositiveInt>,
    #[doc = "Date/time(s) range when this service was delivered or completed."]
    pub r#serviced: Option<InvoiceLineItemServiced>,
    #[doc = "The ChargeItem contains information such as the billing code, date, amount etc. If no further details are required for the lineItem, inline billing codes can be added using the CodeableConcept data type instead of the Reference."]
    pub r#charge_item: InvoiceLineItemChargeItem,
    #[doc = "The price for a ChargeItem may be calculated as a base price with surcharges/deductions that apply in certain conditions. A ChargeItemDefinition resource that defines the prices, factors and conditions that apply to a billing code is currently under development. The priceComponent element can be used to offer transparency to the recipient of the Invoice as to how the prices have been calculated."]
    pub r#price_component: Vec<super::super::types::MonetaryComponent>,
}
#[allow(clippy::derivable_impls)]
impl Default for InvoiceLineItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#sequence: Default::default(),
            r#serviced: Default::default(),
            r#charge_item: Default::default(),
            r#price_component: Default::default(),
        }
    }
}
#[doc = "Invoice containing collected ChargeItems from an Account with calculated individual and total price for Billing purpose."]
#[derive(Debug, Clone, PartialEq)]
pub struct Invoice {
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
    #[doc = "Identifier of this Invoice, often used for reference in correspondence about this invoice or for tracking of payments."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The current state of the Invoice."]
    pub r#status: super::super::types::Code,
    #[doc = "In case of Invoice cancellation a reason must be given (entered in error, superseded by corrected invoice etc.)."]
    pub r#cancelled_reason: Option<super::super::types::String>,
    #[doc = "Type of Invoice depending on domain, realm an usage (e.g. internal/external, dental, preliminary)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The individual or set of individuals receiving the goods and services billed in this invoice."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The individual or Organization responsible for balancing of this invoice."]
    pub r#recipient: Option<Box<super::super::types::Reference>>,
    #[doc = "Depricared by the element below."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Date/time(s) of when this Invoice was posted."]
    pub r#creation: Option<super::super::types::DateTime>,
    #[doc = "Date/time(s) range of services included in this invoice."]
    pub r#period: Option<InvoicePeriod>,
    #[doc = "Indicates who or what performed or participated in the charged service."]
    pub r#participant: Vec<InvoiceParticipant>,
    #[doc = "The organizationissuing the Invoice."]
    pub r#issuer: Option<Box<super::super::types::Reference>>,
    #[doc = "Account which is supposed to be balanced with this Invoice."]
    pub r#account: Option<Box<super::super::types::Reference>>,
    #[doc = "Each line item represents one charge for goods and services rendered. Details such.ofType(date), code and amount are found in the referenced ChargeItem resource."]
    pub r#line_item: Vec<InvoiceLineItem>,
    #[doc = "The total amount for the Invoice may be calculated as the sum of the line items with surcharges/deductions that apply in certain conditions.  The priceComponent element can be used to offer transparency to the recipient of the Invoice of how the total price was calculated."]
    pub r#total_price_component: Vec<super::super::types::MonetaryComponent>,
    #[doc = "Invoice total , taxes excluded."]
    pub r#total_net: Option<Box<super::super::types::Money>>,
    #[doc = "Invoice total, tax included."]
    pub r#total_gross: Option<Box<super::super::types::Money>>,
    #[doc = "Payment details such as banking details, period of payment, deductibles, methods of payment."]
    pub r#payment_terms: Option<super::super::types::Markdown>,
    #[doc = "Comments made about the invoice by the issuer, subject, or other participants."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for Invoice {
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
            r#cancelled_reason: Default::default(),
            r#type: Default::default(),
            r#subject: Default::default(),
            r#recipient: Default::default(),
            r#date: Default::default(),
            r#creation: Default::default(),
            r#period: Default::default(),
            r#participant: Default::default(),
            r#issuer: Default::default(),
            r#account: Default::default(),
            r#line_item: Default::default(),
            r#total_price_component: Default::default(),
            r#total_net: Default::default(),
            r#total_gross: Default::default(),
            r#payment_terms: Default::default(),
            r#note: Default::default(),
        }
    }
}
