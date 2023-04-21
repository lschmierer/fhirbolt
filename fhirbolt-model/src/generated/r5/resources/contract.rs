// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "Narrows the range of legal concerns to focus on the achievement of specific contractual objectives."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ContractTopic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The entity that the term applies to."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ContractTermTopic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Response to an offer clause or question text,  which enables selection of values to be agreed to, e.g., the period of participation, the date of occupancy of a rental, warranty duration, or whether biospecimen may be used for further research."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ContractTermOfferAnswerValue {
    Boolean(Box<super::super::types::Boolean>),
    Decimal(Box<super::super::types::Decimal>),
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Uri(Box<super::super::types::Uri>),
    Attachment(Box<super::super::types::Attachment>),
    Coding(Box<super::super::types::Coding>),
    Quantity(Box<super::super::types::Quantity>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Specific type of Contract Valued Item that may be priced."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ContractTermAssetValuedItemEntity {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "When action happens."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ContractTermActionOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    #[default]
    Invalid,
}
#[doc = "Human readable rendering of this Contract in a format and representation intended to enhance comprehension and ensure understandability."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ContractFriendlyContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Contract legal text in human renderable form."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ContractLegalContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Computable Contract conveyed using a policy rule language (e.g. XACML, DKAL, SecPal)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ContractRuleContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Legally binding Contract: This is the signed and legally recognized representation of the Contract, which is considered the \"source of truth\" and which would be the basis for legal action related to enforcement of this Contract."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ContractLegallyBinding {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Precusory content developed with a focus and intent of supporting the formation a Contract instance, which may be associated with and transformable into a Contract."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractContentDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Precusory content structure and use, i.e., a boilerplate, template, application for a contract such as an insurance policy or benefits under a program, e.g., workers compensation."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Detailed Precusory content type."]
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The  individual or organization that published the Contract precursor content."]
    pub r#publisher: Option<Box<super::super::types::Reference>>,
    #[doc = "The date (and optionally time) when the contract was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the contract changes."]
    pub r#publication_date: Option<super::super::types::DateTime>,
    #[doc = "amended | appended | cancelled | disputed | entered-in-error | executable +."]
    pub r#publication_status: super::super::types::Code,
    #[doc = "A copyright statement relating to Contract precursor content. Copyright statements are generally legal restrictions on the use and publishing of the Contract precursor content."]
    pub r#copyright: Option<super::super::types::Markdown>,
}
impl Default for ContractContentDefinition {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#sub_type: Default::default(),
            r#publisher: Default::default(),
            r#publication_date: Default::default(),
            r#publication_status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#copyright: Default::default(),
        }
    }
}
#[doc = "Security labels that protect the handling of information about the term and its elements, which may be specifically identified."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractTermSecurityLabel {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Number used to link this term or term element to the applicable Security Label."]
    pub r#number: Vec<super::super::types::UnsignedInt>,
    #[doc = "Security label privacy tag that specifies the level of confidentiality protection required for this term and/or term elements."]
    pub r#classification: Box<super::super::types::Coding>,
    #[doc = "Security label privacy tag that specifies the applicable privacy and security policies governing this term and/or term elements."]
    pub r#category: Vec<Box<super::super::types::Coding>>,
    #[doc = "Security label privacy tag that specifies the manner in which term and/or term elements are to be protected."]
    pub r#control: Vec<Box<super::super::types::Coding>>,
}
impl Default for ContractTermSecurityLabel {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#number: Default::default(),
            r#classification: {
                let mut default: Box<super::super::types::Coding> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#category: Default::default(),
            r#control: Default::default(),
        }
    }
}
#[doc = "Offer Recipient."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractTermOfferParty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Participant in the offer."]
    pub r#reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "How the party participates in the offer."]
    pub r#role: Box<super::super::types::CodeableConcept>,
}
impl Default for ContractTermOfferParty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#reference: Default::default(),
            r#role: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "Response to offer text."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractTermOfferAnswer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Response to an offer clause or question text,  which enables selection of values to be agreed to, e.g., the period of participation, the date of occupancy of a rental, warranty duration, or whether biospecimen may be used for further research."]
    pub r#value: ContractTermOfferAnswerValue,
}
impl Default for ContractTermOfferAnswer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "The matter of concern in the context of this provision of the agrement."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractTermOffer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Unique identifier for this particular Contract Provision."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Offer Recipient."]
    pub r#party: Vec<ContractTermOfferParty>,
    #[doc = "The owner of an asset has the residual control rights over the asset: the right to decide all usages of the asset in any way not inconsistent with a prior contract, custom, or law (Hart, 1995, p. 30)."]
    pub r#topic: Option<Box<super::super::types::Reference>>,
    #[doc = "Type of Contract Provision such as specific requirements, purposes for actions, obligations, prohibitions, e.g. life time maximum benefit."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Type of choice made by accepting party with respect to an offer made by an offeror/ grantee."]
    pub r#decision: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "How the decision about a Contract was conveyed."]
    pub r#decision_mode: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Response to offer text."]
    pub r#answer: Vec<ContractTermOfferAnswer>,
    #[doc = "Human readable form of this Contract Offer."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "The id of the clause or question text of the offer in the referenced questionnaire/response."]
    pub r#link_id: Vec<super::super::types::String>,
    #[doc = "Security labels that protects the offer."]
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
}
impl Default for ContractTermOffer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#party: Default::default(),
            r#topic: Default::default(),
            r#type: Default::default(),
            r#decision: Default::default(),
            r#decision_mode: Default::default(),
            r#answer: Default::default(),
            r#text: Default::default(),
            r#link_id: Default::default(),
            r#security_label_number: Default::default(),
        }
    }
}
#[doc = "Circumstance of the asset."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractTermAssetContext {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Asset context reference may include the creator, custodian, or owning Person or Organization (e.g., bank, repository),  location held, e.g., building,  jurisdiction."]
    pub r#reference: Option<Box<super::super::types::Reference>>,
    #[doc = "Coded representation of the context generally or of the Referenced entity, such as the asset holder type or location."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Context description."]
    pub r#text: Option<super::super::types::String>,
}
impl Default for ContractTermAssetContext {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#reference: Default::default(),
            r#code: Default::default(),
            r#text: Default::default(),
        }
    }
}
#[doc = "Contract Valued Item List."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractTermAssetValuedItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Specific type of Contract Valued Item that may be priced."]
    pub r#entity: Option<ContractTermAssetValuedItemEntity>,
    #[doc = "Identifies a Contract Valued Item instance."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Indicates the time during which this Contract ValuedItem information is effective."]
    pub r#effective_time: Option<super::super::types::DateTime>,
    #[doc = "Specifies the units by which the Contract Valued Item is measured or counted, and quantifies the countable or measurable Contract Valued Item instances."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "A Contract Valued Item unit valuation measure."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of the Contract Valued Item delivered. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "An amount that expresses the weighting (based on difficulty, cost and/or resource intensiveness) associated with the Contract Valued Item delivered. The concept of Points allows for assignment of point values for a Contract Valued Item, such that a monetary amount can be assigned to each point."]
    pub r#points: Option<super::super::types::Decimal>,
    #[doc = "Expresses the product of the Contract Valued Item unitQuantity and the unitPriceAmt. For example, the formula: unit Quantity * unit Price (Cost per Point) * factor Number  * points = net Amount. Quantity, factor and points are assumed to be 1 if not supplied."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Terms of valuation."]
    pub r#payment: Option<super::super::types::String>,
    #[doc = "When payment is due."]
    pub r#payment_date: Option<super::super::types::DateTime>,
    #[doc = "Who will make payment."]
    pub r#responsible: Option<Box<super::super::types::Reference>>,
    #[doc = "Who will receive payment."]
    pub r#recipient: Option<Box<super::super::types::Reference>>,
    #[doc = "Id  of the clause or question text related to the context of this valuedItem in the referenced form or QuestionnaireResponse."]
    pub r#link_id: Vec<super::super::types::String>,
    #[doc = "A set of security labels that define which terms are controlled by this condition."]
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
}
impl Default for ContractTermAssetValuedItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#entity: Default::default(),
            r#identifier: Default::default(),
            r#effective_time: Default::default(),
            r#quantity: Default::default(),
            r#unit_price: Default::default(),
            r#factor: Default::default(),
            r#points: Default::default(),
            r#net: Default::default(),
            r#payment: Default::default(),
            r#payment_date: Default::default(),
            r#responsible: Default::default(),
            r#recipient: Default::default(),
            r#link_id: Default::default(),
            r#security_label_number: Default::default(),
        }
    }
}
#[doc = "Contract Term Asset List."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractTermAsset {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Differentiates the kind of the asset ."]
    pub r#scope: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Target entity type about which the term may be concerned."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Associated entities."]
    pub r#type_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "May be a subtype or part of an offered asset."]
    pub r#subtype: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specifies the applicability of the term to an asset resource instance, and instances it refers to or instances that refer to it, and/or are owned by the offeree."]
    pub r#relationship: Option<Box<super::super::types::Coding>>,
    #[doc = "Circumstance of the asset."]
    pub r#context: Vec<ContractTermAssetContext>,
    #[doc = "Description of the quality and completeness of the asset that may be a factor in its valuation."]
    pub r#condition: Option<super::super::types::String>,
    #[doc = "Type of Asset availability for use or ownership."]
    pub r#period_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Asset relevant contractual time period."]
    pub r#period: Vec<Box<super::super::types::Period>>,
    #[doc = "Time period of asset use."]
    pub r#use_period: Vec<Box<super::super::types::Period>>,
    #[doc = "Clause or question text (Prose Object) concerning the asset in a linked form, such as a QuestionnaireResponse used in the formation of the contract."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "Id [identifier??] of the clause or question text about the asset in the referenced form or QuestionnaireResponse."]
    pub r#link_id: Vec<super::super::types::String>,
    #[doc = "Response to assets."]
    pub r#answer: Vec<ContractTermOfferAnswer>,
    #[doc = "Security labels that protects the asset."]
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
    #[doc = "Contract Valued Item List."]
    pub r#valued_item: Vec<ContractTermAssetValuedItem>,
}
impl Default for ContractTermAsset {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#scope: Default::default(),
            r#type: Default::default(),
            r#type_reference: Default::default(),
            r#subtype: Default::default(),
            r#relationship: Default::default(),
            r#context: Default::default(),
            r#condition: Default::default(),
            r#period_type: Default::default(),
            r#period: Default::default(),
            r#use_period: Default::default(),
            r#text: Default::default(),
            r#link_id: Default::default(),
            r#answer: Default::default(),
            r#security_label_number: Default::default(),
            r#valued_item: Default::default(),
        }
    }
}
#[doc = "Entity of the action."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractTermActionSubject {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The entity the action is performed or not performed on or for."]
    pub r#reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Role type of agent assigned roles in this Contract."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
}
impl Default for ContractTermActionSubject {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#reference: Default::default(),
            r#role: Default::default(),
        }
    }
}
#[doc = "An actor taking a role in an activity for which it can be assigned some degree of responsibility for the activity taking place."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractTermAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "True if the term prohibits the  action."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "Activity or service obligation to be done or not done, performed or not performed, effectuated or not by this Contract term."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Entity of the action."]
    pub r#subject: Vec<ContractTermActionSubject>,
    #[doc = "Reason or purpose for the action stipulated by this Contract Provision."]
    pub r#intent: Box<super::super::types::CodeableConcept>,
    #[doc = "Id [identifier??] of the clause or question text related to this action in the referenced form or QuestionnaireResponse."]
    pub r#link_id: Vec<super::super::types::String>,
    #[doc = "Current state of the term action."]
    pub r#status: Box<super::super::types::CodeableConcept>,
    #[doc = "Encounter or Episode with primary association to the specified term activity."]
    pub r#context: Option<Box<super::super::types::Reference>>,
    #[doc = "Id [identifier??] of the clause or question text related to the requester of this action in the referenced form or QuestionnaireResponse."]
    pub r#context_link_id: Vec<super::super::types::String>,
    #[doc = "When action happens."]
    pub r#occurrence: Option<ContractTermActionOccurrence>,
    #[doc = "Who or what initiated the action and has responsibility for its activation."]
    pub r#requester: Vec<Box<super::super::types::Reference>>,
    #[doc = "Id [identifier??] of the clause or question text related to the requester of this action in the referenced form or QuestionnaireResponse."]
    pub r#requester_link_id: Vec<super::super::types::String>,
    #[doc = "The type of individual that is desired or required to perform or not perform the action."]
    pub r#performer_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of role or competency of an individual desired or required to perform or not perform the action."]
    pub r#performer_role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates who or what is being asked to perform (or not perform) the ction."]
    pub r#performer: Option<Box<super::super::types::Reference>>,
    #[doc = "Id [identifier??] of the clause or question text related to the reason type or reference of this  action in the referenced form or QuestionnaireResponse."]
    pub r#performer_link_id: Vec<super::super::types::String>,
    #[doc = "Rationale for the action to be performed or not performed. Describes why the action is permitted or prohibited. Either a coded concept, or another resource whose existence justifies permitting or not permitting this action."]
    pub r#reason: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "Id [identifier??] of the clause or question text related to the reason type or reference of this  action in the referenced form or QuestionnaireResponse."]
    pub r#reason_link_id: Vec<super::super::types::String>,
    #[doc = "Comments made about the term action made by the requester, performer, subject or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Security labels that protects the action."]
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
}
impl Default for ContractTermAction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#do_not_perform: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#subject: Default::default(),
            r#intent: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#link_id: Default::default(),
            r#status: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#context: Default::default(),
            r#context_link_id: Default::default(),
            r#occurrence: Default::default(),
            r#requester: Default::default(),
            r#requester_link_id: Default::default(),
            r#performer_type: Default::default(),
            r#performer_role: Default::default(),
            r#performer: Default::default(),
            r#performer_link_id: Default::default(),
            r#reason: Default::default(),
            r#reason_link_id: Default::default(),
            r#note: Default::default(),
            r#security_label_number: Default::default(),
        }
    }
}
#[doc = "One or more Contract Provisions, which may be related and conveyed as a group, and may contain nested groups."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractTerm {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Unique identifier for this particular Contract Provision."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "When this Contract Provision was issued."]
    pub r#issued: Option<super::super::types::DateTime>,
    #[doc = "Relevant time or time-period when this Contract Provision is applicable."]
    pub r#applies: Option<Box<super::super::types::Period>>,
    #[doc = "The entity that the term applies to."]
    pub r#topic: Option<ContractTermTopic>,
    #[doc = "A legal clause or condition contained within a contract that requires one or both parties to perform a particular requirement by some specified time or prevents one or both parties from performing a particular requirement by some specified time."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A specialized legal clause or condition based on overarching contract type."]
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Statement of a provision in a policy or a contract."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "Security labels that protect the handling of information about the term and its elements, which may be specifically identified."]
    pub r#security_label: Vec<ContractTermSecurityLabel>,
    #[doc = "The matter of concern in the context of this provision of the agrement."]
    pub r#offer: ContractTermOffer,
    #[doc = "Contract Term Asset List."]
    pub r#asset: Vec<ContractTermAsset>,
    #[doc = "An actor taking a role in an activity for which it can be assigned some degree of responsibility for the activity taking place."]
    pub r#action: Vec<ContractTermAction>,
    #[doc = "Nested group of Contract Provisions."]
    pub r#group: Vec<ContractTerm>,
}
impl Default for ContractTerm {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#issued: Default::default(),
            r#applies: Default::default(),
            r#topic: Default::default(),
            r#type: Default::default(),
            r#sub_type: Default::default(),
            r#text: Default::default(),
            r#security_label: Default::default(),
            r#offer: {
                let mut default: ContractTermOffer = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#asset: Default::default(),
            r#action: Default::default(),
            r#group: Default::default(),
        }
    }
}
#[doc = "Parties with legal standing in the Contract, including the principal parties, the grantor(s) and grantee(s), which are any person or organization bound by the contract, and any ancillary parties, which facilitate the execution of the contract such as a notary or witness."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractSigner {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Role of this Contract signer, e.g. notary, grantee."]
    pub r#type: Box<super::super::types::Coding>,
    #[doc = "Party which is a signator to this Contract."]
    pub r#party: Box<super::super::types::Reference>,
    #[doc = "Legally binding Contract DSIG signature contents in Base64."]
    pub r#signature: Vec<Box<super::super::types::Signature>>,
}
impl Default for ContractSigner {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::Coding> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#party: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#signature: Default::default(),
        }
    }
}
#[doc = "The \"patient friendly language\" versionof the Contract in whole or in parts. \"Patient friendly language\" means the representation of the Contract and Contract Provisions in a manner that is readily accessible and understandable by a layperson in accordance with best practices for communication styles that ensure that those agreeing to or signing the Contract understand the roles, actions, obligations, responsibilities, and implication of the agreement."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractFriendly {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Human readable rendering of this Contract in a format and representation intended to enhance comprehension and ensure understandability."]
    pub r#content: ContractFriendlyContent,
}
impl Default for ContractFriendly {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#content: Default::default(),
        }
    }
}
#[doc = "List of Legal expressions or representations of this Contract."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractLegal {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Contract legal text in human renderable form."]
    pub r#content: ContractLegalContent,
}
impl Default for ContractLegal {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#content: Default::default(),
        }
    }
}
#[doc = "List of Computable Policy Rule Language Representations of this Contract."]
#[derive(Debug, Clone, PartialEq)]
pub struct ContractRule {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Computable Contract conveyed using a policy rule language (e.g. XACML, DKAL, SecPal)."]
    pub r#content: ContractRuleContent,
}
impl Default for ContractRule {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#content: Default::default(),
        }
    }
}
#[doc = "Legally enforceable, formally recorded unilateral or bilateral directive i.e., a policy or agreement."]
#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
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
    #[doc = "Unique identifier for this Contract or a derivative that references a Source Contract."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Canonical identifier for this contract, represented as a URI (globally unique)."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "An edition identifier used for business purposes to label business significant variants."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "The status of the resource instance."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Legal states of the formation of a legal instrument, which is a formally executed written document that can be formally attributed to its author, records and formally expresses a legally enforceable act, process, or contractual duty, obligation, or right, and therefore evidences that act, process, or agreement."]
    pub r#legal_state: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The URL pointing to a FHIR-defined Contract Definition that is adhered to in whole or part by this Contract."]
    pub r#instantiates_canonical: Option<Box<super::super::types::Reference>>,
    #[doc = "The URL pointing to an externally maintained definition that is adhered to in whole or in part by this Contract."]
    pub r#instantiates_uri: Option<super::super::types::Uri>,
    #[doc = "The minimal content derived from the basal information source at a specific stage in its lifecycle."]
    pub r#content_derivative: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When this  Contract was issued."]
    pub r#issued: Option<super::super::types::DateTime>,
    #[doc = "Relevant time or time-period when this Contract is applicable."]
    pub r#applies: Option<Box<super::super::types::Period>>,
    #[doc = "Event resulting in discontinuation or termination of this Contract instance by one or more parties to the contract."]
    pub r#expiration_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The target entity impacted by or of interest to parties to the agreement."]
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    #[doc = "A formally or informally recognized grouping of people, principals, organizations, or jurisdictions formed for the purpose of achieving some form of collective action such as the promulgation, administration and enforcement of contracts and policies."]
    pub r#authority: Vec<Box<super::super::types::Reference>>,
    #[doc = "Recognized governance framework or system operating with a circumscribed scope in accordance with specified principles, policies, processes or procedures for managing rights, actions, or behaviors of parties or principals relative to resources."]
    pub r#domain: Vec<Box<super::super::types::Reference>>,
    #[doc = "Sites in which the contract is complied with,  exercised, or in force."]
    pub r#site: Vec<Box<super::super::types::Reference>>,
    #[doc = "A natural language name identifying this Contract definition, derivative, or instance in any legal state. Provides additional information about its content. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for this Contract definition, derivative, or instance in any legal state."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "A more detailed or qualifying explanatory or alternate user-friendly title for this Contract definition, derivative, or instance in any legal state."]
    pub r#subtitle: Option<super::super::types::String>,
    #[doc = "Alternative representation of the title for this Contract definition, derivative, or instance in any legal state., e.g., a domain specific contract number related to legislation."]
    pub r#alias: Vec<super::super::types::String>,
    #[doc = "The individual or organization that authored the Contract definition, derivative, or instance in any legal state."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "A selector of legal concerns for this Contract definition, derivative, or instance in any legal state."]
    pub r#scope: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Narrows the range of legal concerns to focus on the achievement of specific contractual objectives."]
    pub r#topic: Option<ContractTopic>,
    #[doc = "A high-level category for the legal instrument, whether constructed as a Contract definition, derivative, or instance in any legal state.  Provides additional information about its content within the context of the Contract's scope to distinguish the kinds of systems that would be interested in the contract."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Sub-category for the Contract that distinguishes the kinds of systems that would be interested in the Contract within the context of the Contract's scope."]
    pub r#sub_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Precusory content developed with a focus and intent of supporting the formation a Contract instance, which may be associated with and transformable into a Contract."]
    pub r#content_definition: Option<ContractContentDefinition>,
    #[doc = "One or more Contract Provisions, which may be related and conveyed as a group, and may contain nested groups."]
    pub r#term: Vec<ContractTerm>,
    #[doc = "Information that may be needed by/relevant to the performer in their execution of this term action."]
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    #[doc = "Links to Provenance records for past versions of this Contract definition, derivative, or instance, which identify key state transitions or updates that are likely to be relevant to a user looking at the current version of the Contract.  The Provenance.entity indicates the target that was changed in the update (see [Provenance.entity](provenance-definitions.html#Provenance.entity))."]
    pub r#relevant_history: Vec<Box<super::super::types::Reference>>,
    #[doc = "Parties with legal standing in the Contract, including the principal parties, the grantor(s) and grantee(s), which are any person or organization bound by the contract, and any ancillary parties, which facilitate the execution of the contract such as a notary or witness."]
    pub r#signer: Vec<ContractSigner>,
    #[doc = "The \"patient friendly language\" versionof the Contract in whole or in parts. \"Patient friendly language\" means the representation of the Contract and Contract Provisions in a manner that is readily accessible and understandable by a layperson in accordance with best practices for communication styles that ensure that those agreeing to or signing the Contract understand the roles, actions, obligations, responsibilities, and implication of the agreement."]
    pub r#friendly: Vec<ContractFriendly>,
    #[doc = "List of Legal expressions or representations of this Contract."]
    pub r#legal: Vec<ContractLegal>,
    #[doc = "List of Computable Policy Rule Language Representations of this Contract."]
    pub r#rule: Vec<ContractRule>,
    #[doc = "Legally binding Contract: This is the signed and legally recognized representation of the Contract, which is considered the \"source of truth\" and which would be the basis for legal action related to enforcement of this Contract."]
    pub r#legally_binding: Option<ContractLegallyBinding>,
}
impl Default for Contract {
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
            r#url: Default::default(),
            r#version: Default::default(),
            r#status: Default::default(),
            r#legal_state: Default::default(),
            r#instantiates_canonical: Default::default(),
            r#instantiates_uri: Default::default(),
            r#content_derivative: Default::default(),
            r#issued: Default::default(),
            r#applies: Default::default(),
            r#expiration_type: Default::default(),
            r#subject: Default::default(),
            r#authority: Default::default(),
            r#domain: Default::default(),
            r#site: Default::default(),
            r#name: Default::default(),
            r#title: Default::default(),
            r#subtitle: Default::default(),
            r#alias: Default::default(),
            r#author: Default::default(),
            r#scope: Default::default(),
            r#topic: Default::default(),
            r#type: Default::default(),
            r#sub_type: Default::default(),
            r#content_definition: Default::default(),
            r#term: Default::default(),
            r#supporting_info: Default::default(),
            r#relevant_history: Default::default(),
            r#signer: Default::default(),
            r#friendly: Default::default(),
            r#legal: Default::default(),
            r#rule: Default::default(),
            r#legally_binding: Default::default(),
        }
    }
}
