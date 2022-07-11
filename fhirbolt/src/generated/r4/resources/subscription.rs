// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct SubscriptionChannel {
    pub r#type: super::super::types::Code,
    pub r#endpoint: Option<super::super::types::Url>,
    pub r#payload: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#header: Vec<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct Subscription {
    pub r#channel: SubscriptionChannel,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#error: Option<super::super::types::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#end: Option<super::super::types::Instant>,
    pub r#reason: super::super::types::String,
    pub r#language: Option<super::super::types::Code>,
    pub r#contact: Vec<Box<super::super::types::ContactPoint>>,
    pub r#criteria: super::super::types::String,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
}
