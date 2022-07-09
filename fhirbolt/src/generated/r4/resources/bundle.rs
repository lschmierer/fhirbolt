// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct BundleEntrySearch {
    pub r#mode: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#score: Option<super::super::types::Decimal>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct BundleEntryRequest {
    pub r#url: super::super::types::Uri,
    pub r#if_modified_since: Option<super::super::types::Instant>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#method: super::super::types::Code,
    pub r#if_none_exist: Option<super::super::types::String>,
    pub r#if_none_match: Option<super::super::types::String>,
    pub r#if_match: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct BundleEntryResponse {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#location: Option<super::super::types::Uri>,
    pub r#outcome: Option<Box<super::Resource>>,
    pub r#status: super::super::types::String,
    pub r#etag: Option<super::super::types::String>,
    pub r#last_modified: Option<super::super::types::Instant>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct BundleEntry {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#full_url: Option<super::super::types::Uri>,
    pub r#search: Option<BundleEntrySearch>,
    pub r#resource: Option<Box<super::Resource>>,
    pub r#request: Option<BundleEntryRequest>,
    pub r#link: Vec<BundleLink>,
    pub r#response: Option<BundleEntryResponse>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct BundleLink {
    pub r#url: super::super::types::Uri,
    pub r#relation: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct Bundle {
    pub r#type: super::super::types::Code,
    pub r#total: Option<super::super::types::UnsignedInt>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#signature: Option<Box<super::super::types::Signature>>,
    pub r#timestamp: Option<super::super::types::Instant>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#id: Option<std::string::String>,
    pub r#entry: Vec<BundleEntry>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#link: Vec<BundleLink>,
    pub r#language: Option<super::super::types::Code>,
}
