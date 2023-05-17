// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = " Identifies the claim line item, encounter or other sub-element being paid. Note payment may be partial, that is not match the then outstanding balance or amount incurred."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum PaymentReconciliationAllocationTargetItem {
    String(Box<super::super::types::String>),
    Identifier(Box<super::super::types::Identifier>),
    PositiveInt(Box<super::super::types::PositiveInt>),
    #[default]
    Invalid,
}
#[doc = "Distribution of the payment amount for a previously acknowledged payable."]
#[derive(Debug, Clone, PartialEq)]
pub struct PaymentReconciliationAllocation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Unique identifier for the current payment item for the referenced payable."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Unique identifier for the prior payment item for the referenced payable."]
    pub r#predecessor: Option<Box<super::super::types::Identifier>>,
    #[doc = "Specific resource to which the payment/adjustment/advance applies."]
    pub r#target: Option<Box<super::super::types::Reference>>,
    #[doc = " Identifies the claim line item, encounter or other sub-element being paid. Note payment may be partial, that is not match the then outstanding balance or amount incurred."]
    pub r#target_item: Option<PaymentReconciliationAllocationTargetItem>,
    #[doc = "The Encounter to which this payment applies, may be completed by the receiver, used for search."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The Account to which this payment applies, may be completed by the receiver, used for search."]
    pub r#account: Option<Box<super::super::types::Reference>>,
    #[doc = "Code to indicate the nature of the payment."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
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
#[allow(clippy::derivable_impls)]
impl Default for PaymentReconciliationAllocation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#predecessor: Default::default(),
            r#target: Default::default(),
            r#target_item: Default::default(),
            r#encounter: Default::default(),
            r#account: Default::default(),
            r#type: Default::default(),
            r#submitter: Default::default(),
            r#response: Default::default(),
            r#date: Default::default(),
            r#responsible: Default::default(),
            r#payee: Default::default(),
            r#amount: Default::default(),
        }
    }
}
#[doc = "A note that describes or explains the processing in a human readable form."]
#[derive(Debug, Clone, PartialEq)]
pub struct PaymentReconciliationProcessNote {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The business purpose of the note text."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "The explanation or description associated with the processing."]
    pub r#text: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for PaymentReconciliationProcessNote {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#text: Default::default(),
        }
    }
}
#[doc = "This resource provides the details including amount of a payment and allocates the payment items being paid."]
#[derive(Debug, Clone, PartialEq)]
pub struct PaymentReconciliation {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A unique identifier assigned to this payment reconciliation."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Code to indicate the nature of the payment such as payment, adjustment."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "The workflow or activity which gave rise to or during which the payment ocurred such as a kiosk, deposit on account, periodic payment etc."]
    pub r#kind: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The period of time for which payments have been gathered into this bulk payment for settlement."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The date when the resource was created."]
    pub r#created: super::super::types::DateTime,
    #[doc = "Payment enterer if not the actual payment issuer."]
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    #[doc = "The type of the source such as patient or insurance."]
    pub r#issuer_type: Option<Box<super::super::types::CodeableConcept>>,
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
    pub r#date: super::super::types::Date,
    #[doc = "The location of the site or device for electronic transfers or physical location for cash payments."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "The means of payment such as check, card cash, or electronic funds transfer."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The card brand such as debit, Visa, Amex etc. used if a card is the method of payment."]
    pub r#card_brand: Option<super::super::types::String>,
    #[doc = "A portion of the account number, often the last 4 digits, used for verification not charging purposes."]
    pub r#account_number: Option<super::super::types::String>,
    #[doc = "The year and month (YYYY-MM) when the instrument, typically card, expires."]
    pub r#expiration_date: Option<super::super::types::Date>,
    #[doc = "The name of the card processor, etf processor, bank for checks."]
    pub r#processor: Option<super::super::types::String>,
    #[doc = "The check number, eft reference, car processor reference."]
    pub r#reference_number: Option<super::super::types::String>,
    #[doc = "An alphanumeric issued by the processor to confirm the successful issuance of payment."]
    pub r#authorization: Option<super::super::types::String>,
    #[doc = "The amount offered by the issuer, typically applies to cash when the issuer provides an amount in bank note denominations equal to or excess of the amount actually being paid."]
    pub r#tendered_amount: Option<Box<super::super::types::Money>>,
    #[doc = "The amount returned by the receiver which is excess to the amount payable, often referred to as 'change'."]
    pub r#returned_amount: Option<Box<super::super::types::Money>>,
    #[doc = "Total payment amount as indicated on the financial instrument."]
    pub r#amount: Box<super::super::types::Money>,
    #[doc = "Issuer's unique identifier for the payment instrument."]
    pub r#payment_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Distribution of the payment amount for a previously acknowledged payable."]
    pub r#allocation: Vec<PaymentReconciliationAllocation>,
    #[doc = "A code for the form to be used for printing the content."]
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A note that describes or explains the processing in a human readable form."]
    pub r#process_note: Vec<PaymentReconciliationProcessNote>,
}
#[allow(clippy::derivable_impls)]
impl Default for PaymentReconciliation {
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
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#kind: Default::default(),
            r#period: Default::default(),
            r#created: super::super::types::DateTime {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#enterer: Default::default(),
            r#issuer_type: Default::default(),
            r#payment_issuer: Default::default(),
            r#request: Default::default(),
            r#requestor: Default::default(),
            r#outcome: Default::default(),
            r#disposition: Default::default(),
            r#date: super::super::types::Date {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#location: Default::default(),
            r#method: Default::default(),
            r#card_brand: Default::default(),
            r#account_number: Default::default(),
            r#expiration_date: Default::default(),
            r#processor: Default::default(),
            r#reference_number: Default::default(),
            r#authorization: Default::default(),
            r#tendered_amount: Default::default(),
            r#returned_amount: Default::default(),
            r#amount: Box::new(super::super::types::Money {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#payment_identifier: Default::default(),
            r#allocation: Default::default(),
            r#form_code: Default::default(),
            r#process_note: Default::default(),
        }
    }
}
