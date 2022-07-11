// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ContractLegalContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum ContractLegallyBinding {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum ContractRuleContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum ContractTopic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum ContractFriendlyContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum ContractTermTopic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum ContractTermAssetValuedItemEntity {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
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
}
#[derive(Debug, Clone)]
pub enum ContractTermActionOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
}
#[derive(Debug, Clone)]
pub struct ContractLegal {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#content: ContractLegalContent,
}
#[derive(Debug, Clone)]
pub struct ContractRule {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#content: ContractRuleContent,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ContractContentDefinition {
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#publication_date: Option<super::super::types::DateTime>,
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#publisher: Option<Box<super::super::types::Reference>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#publication_status: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ContractFriendly {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#content: ContractFriendlyContent,
}
#[derive(Debug, Clone)]
pub struct ContractSigner {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::Coding>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#party: Box<super::super::types::Reference>,
    pub r#signature: Vec<Box<super::super::types::Signature>>,
}
#[derive(Debug, Clone)]
pub struct ContractTermAssetContext {
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct ContractTermAssetValuedItem {
    pub r#recipient: Option<Box<super::super::types::Reference>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#entity: Option<ContractTermAssetValuedItemEntity>,
    pub r#payment_date: Option<super::super::types::DateTime>,
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#responsible: Option<Box<super::super::types::Reference>>,
    pub r#payment: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#effective_time: Option<super::super::types::DateTime>,
    pub r#id: Option<std::string::String>,
    pub r#points: Option<super::super::types::Decimal>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#link_id: Vec<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct ContractTermAsset {
    pub r#context: Vec<ContractTermAssetContext>,
    pub r#id: Option<std::string::String>,
    pub r#subtype: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#use_period: Vec<Box<super::super::types::Period>>,
    pub r#period: Vec<Box<super::super::types::Period>>,
    pub r#type_reference: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
    pub r#answer: Vec<ContractTermOfferAnswer>,
    pub r#link_id: Vec<super::super::types::String>,
    pub r#valued_item: Vec<ContractTermAssetValuedItem>,
    pub r#text: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#relationship: Option<Box<super::super::types::Coding>>,
    pub r#scope: Option<Box<super::super::types::CodeableConcept>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#condition: Option<super::super::types::String>,
    pub r#period_type: Vec<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ContractTermOfferAnswer {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#value: ContractTermOfferAnswerValue,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ContractTermOfferParty {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#role: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct ContractTermOffer {
    pub r#decision: Option<Box<super::super::types::CodeableConcept>>,
    pub r#answer: Vec<ContractTermOfferAnswer>,
    pub r#id: Option<std::string::String>,
    pub r#decision_mode: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#party: Vec<ContractTermOfferParty>,
    pub r#text: Option<super::super::types::String>,
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#topic: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#link_id: Vec<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct ContractTermActionSubject {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reference: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct ContractTermAction {
    pub r#status: Box<super::super::types::CodeableConcept>,
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    pub r#reason: Vec<super::super::types::String>,
    pub r#link_id: Vec<super::super::types::String>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#requester: Vec<Box<super::super::types::Reference>>,
    pub r#context_link_id: Vec<super::super::types::String>,
    pub r#occurrence: Option<ContractTermActionOccurrence>,
    pub r#context: Option<Box<super::super::types::Reference>>,
    pub r#performer_role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#performer_link_id: Vec<super::super::types::String>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#intent: Box<super::super::types::CodeableConcept>,
    pub r#subject: Vec<ContractTermActionSubject>,
    pub r#performer_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_link_id: Vec<super::super::types::String>,
    pub r#requester_link_id: Vec<super::super::types::String>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ContractTermSecurityLabel {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#number: Vec<super::super::types::UnsignedInt>,
    pub r#category: Vec<Box<super::super::types::Coding>>,
    pub r#control: Vec<Box<super::super::types::Coding>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#classification: Box<super::super::types::Coding>,
}
#[derive(Debug, Clone)]
pub struct ContractTerm {
    pub r#issued: Option<super::super::types::DateTime>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#topic: Option<ContractTermTopic>,
    pub r#asset: Vec<ContractTermAsset>,
    pub r#offer: ContractTermOffer,
    pub r#group: Vec<ContractTerm>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#action: Vec<ContractTermAction>,
    pub r#applies: Option<Box<super::super::types::Period>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#security_label: Vec<ContractTermSecurityLabel>,
    pub r#text: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct Contract {
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    pub r#relevant_history: Vec<Box<super::super::types::Reference>>,
    pub r#applies: Option<Box<super::super::types::Period>>,
    pub r#id: Option<std::string::String>,
    pub r#legal: Vec<ContractLegal>,
    pub r#title: Option<super::super::types::String>,
    pub r#instantiates_canonical: Option<Box<super::super::types::Reference>>,
    pub r#scope: Option<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#sub_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#legally_binding: Option<ContractLegallyBinding>,
    pub r#status: Option<super::super::types::Code>,
    pub r#legal_state: Option<Box<super::super::types::CodeableConcept>>,
    pub r#content_derivative: Option<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#rule: Vec<ContractRule>,
    pub r#instantiates_uri: Option<super::super::types::Uri>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#site: Vec<Box<super::super::types::Reference>>,
    pub r#content_definition: Option<ContractContentDefinition>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#expiration_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    pub r#name: Option<super::super::types::String>,
    pub r#topic: Option<ContractTopic>,
    pub r#friendly: Vec<ContractFriendly>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#authority: Vec<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#subtitle: Option<super::super::types::String>,
    pub r#alias: Vec<super::super::types::String>,
    pub r#signer: Vec<ContractSigner>,
    pub r#issued: Option<super::super::types::DateTime>,
    pub r#term: Vec<ContractTerm>,
    pub r#language: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#domain: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#version: Option<super::super::types::String>,
}
