// Generated on 2023-04-19 by fhirbolt-codegen v0.3.0
#[doc = "Distribution of the payment amount for a previously acknowledged payable."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PaymentReconciliationDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Unique identifier for the current payment item for the referenced payable."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Unique identifier for the prior payment item for the referenced payable."]
    pub r#predecessor: Option<Box<super::super::types::Identifier>>,
    #[doc = "Code to indicate the nature of the payment."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A resource, such as a Claim, the evaluation of which could lead to payment."]
    pub r#request: Option<Box<super::super::types::Reference>>,
    #[doc = "The party which submitted the claim or financial transaction."]
    pub r#submitter: Option<Box<super::super::types::Reference>>,
    #[doc = "A resource, such as a ClaimResponse, which contains a commitment to payment."]
    pub r#response: Option<Box<super::super::types::Reference>>,
    #[doc = "The date from the response resource containing a commitment to pay."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "A reference to the individual who is responsible for inquiries regarding the response and its payment."]
    pub r#responsible: Option<Box<super::super::types::Reference>>,
    #[doc = "The party which is receiving the payment."]
    pub r#payee: Option<Box<super::super::types::Reference>>,
    #[doc = "The monetary amount allocated from the total payment to the payable."]
    pub r#amount: Option<Box<super::super::types::Money>>,
}
#[doc = "A note that describes or explains the processing in a human readable form."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PaymentReconciliationProcessNote {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The business purpose of the note text."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "The explanation or description associated with the processing."]
    pub r#text: Option<super::super::types::String>,
}
#[doc = "This resource provides the details including amount of a payment and allocates the payment items being paid."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PaymentReconciliation {
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
    #[doc = "A unique identifier assigned to this payment reconciliation."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "The period of time for which payments have been gathered into this bulk payment for settlement."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The date when the resource was created."]
    pub r#created: super::super::types::DateTime,
    #[doc = "The party who generated the payment."]
    pub r#payment_issuer: Option<Box<super::super::types::Reference>>,
    #[doc = "Original request resource reference."]
    pub r#request: Option<Box<super::super::types::Reference>>,
    #[doc = "The practitioner who is responsible for the services rendered to the patient."]
    pub r#requestor: Option<Box<super::super::types::Reference>>,
    #[doc = "The outcome of a request for a reconciliation."]
    pub r#outcome: Option<super::super::types::Code>,
    #[doc = "A human readable description of the status of the request for the reconciliation."]
    pub r#disposition: Option<super::super::types::String>,
    #[doc = "The date of payment as indicated on the financial instrument."]
    pub r#payment_date: super::super::types::Date,
    #[doc = "Total payment amount as indicated on the financial instrument."]
    pub r#payment_amount: Box<super::super::types::Money>,
    #[doc = "Issuer's unique identifier for the payment instrument."]
    pub r#payment_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Distribution of the payment amount for a previously acknowledged payable."]
    pub r#detail: Vec<PaymentReconciliationDetail>,
    #[doc = "A code for the form to be used for printing the content."]
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A note that describes or explains the processing in a human readable form."]
    pub r#process_note: Vec<PaymentReconciliationProcessNote>,
}
