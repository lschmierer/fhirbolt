// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct SubscriptionChannel {
    pub r#payload: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#header: Vec<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#endpoint: Option<super::super::types::Url>,
}
#[derive(Debug, Clone)]
pub struct Subscription {
    pub r#status: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#channel: SubscriptionChannel,
    pub r#error: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#criteria: super::super::types::String,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#reason: super::super::types::String,
    pub r#contact: Vec<Box<super::super::types::ContactPoint>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#end: Option<super::super::types::Instant>,
}
