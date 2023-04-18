// Generated on 2023-04-18 by fhirbolt-codegen v0.2.0
#[doc = "The ChargeItem contains information such as the billing code, date, amount etc. If no further details are required for the lineItem, inline billing codes can be added using the CodeableConcept data type instead of the Reference."]
#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceLineItemChargeItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for InvoiceLineItemChargeItem {
    fn default() -> InvoiceLineItemChargeItem {
        InvoiceLineItemChargeItem::Invalid
    }
}
#[doc = "Indicates who or what performed or participated in the charged service."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InvoiceParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Describes the type of involvement (e.g. transcriptionist, creator etc.). If the invoice has been created automatically, the Participant may be a billing engine or another kind of device."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The device, practitioner, etc. who performed or participated in the service."]
    pub r#actor: Box<super::super::types::Reference>,
}
#[doc = "The price for a ChargeItem may be calculated as a base price with surcharges/deductions that apply in certain conditions. A ChargeItemDefinition resource that defines the prices, factors and conditions that apply to a billing code is currently under development. The priceComponent element can be used to offer transparency to the recipient of the Invoice as to how the prices have been calculated."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InvoiceLineItemPriceComponent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "This code identifies the type of the component."]
    pub r#type: super::super::types::Code,
    #[doc = "A code that identifies the component. Codes may be used to differentiate between kinds of taxes, surcharges, discounts etc."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The factor that has been applied on the base price for calculating this component."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The amount calculated for this component."]
    pub r#amount: Option<Box<super::super::types::Money>>,
}
#[doc = "Each line item represents one charge for goods and services rendered. Details such as date, code and amount are found in the referenced ChargeItem resource."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InvoiceLineItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Sequence in which the items appear on the invoice."]
    pub r#sequence: Option<super::super::types::PositiveInt>,
    #[doc = "The ChargeItem contains information such as the billing code, date, amount etc. If no further details are required for the lineItem, inline billing codes can be added using the CodeableConcept data type instead of the Reference."]
    pub r#charge_item: InvoiceLineItemChargeItem,
    #[doc = "The price for a ChargeItem may be calculated as a base price with surcharges/deductions that apply in certain conditions. A ChargeItemDefinition resource that defines the prices, factors and conditions that apply to a billing code is currently under development. The priceComponent element can be used to offer transparency to the recipient of the Invoice as to how the prices have been calculated."]
    pub r#price_component: Vec<InvoiceLineItemPriceComponent>,
}
#[doc = "Invoice containing collected ChargeItems from an Account with calculated individual and total price for Billing purpose."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Invoice {
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
    #[doc = "Identifier of this Invoice, often used for reference in correspondence about this invoice or for tracking of payments."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
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
    #[doc = "Date/time(s) of when this Invoice was posted."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Indicates who or what performed or participated in the charged service."]
    pub r#participant: Vec<InvoiceParticipant>,
    #[doc = "The organizationissuing the Invoice."]
    pub r#issuer: Option<Box<super::super::types::Reference>>,
    #[doc = "Account which is supposed to be balanced with this Invoice."]
    pub r#account: Option<Box<super::super::types::Reference>>,
    #[doc = "Each line item represents one charge for goods and services rendered. Details such as date, code and amount are found in the referenced ChargeItem resource."]
    pub r#line_item: Vec<InvoiceLineItem>,
    #[doc = "The total amount for the Invoice may be calculated as the sum of the line items with surcharges/deductions that apply in certain conditions.  The priceComponent element can be used to offer transparency to the recipient of the Invoice of how the total price was calculated."]
    pub r#total_price_component: Vec<InvoiceLineItemPriceComponent>,
    #[doc = "Invoice total , taxes excluded."]
    pub r#total_net: Option<Box<super::super::types::Money>>,
    #[doc = "Invoice total, tax included."]
    pub r#total_gross: Option<Box<super::super::types::Money>>,
    #[doc = "Payment details such as banking details, period of payment, deductibles, methods of payment."]
    pub r#payment_terms: Option<super::super::types::Markdown>,
    #[doc = "Comments made about the invoice by the issuer, subject, or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
