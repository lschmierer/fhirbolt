// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct BundleLink {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#relation: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#url: super::super::types::Uri,
}
#[derive(Debug, Clone)]
pub struct BundleEntrySearch {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#mode: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#score: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub struct BundleEntryResponse {
    pub r#location: Option<super::super::types::Uri>,
    pub r#outcome: Option<Box<super::Resource>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#etag: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#last_modified: Option<super::super::types::Instant>,
    pub r#status: super::super::types::String,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct BundleEntryRequest {
    pub r#url: super::super::types::Uri,
    pub r#if_none_exist: Option<super::super::types::String>,
    pub r#method: super::super::types::Code,
    pub r#if_match: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#if_none_match: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#if_modified_since: Option<super::super::types::Instant>,
}
#[derive(Debug, Clone)]
pub struct BundleEntry {
    pub r#search: Option<BundleEntrySearch>,
    pub r#response: Option<BundleEntryResponse>,
    pub r#id: Option<std::string::String>,
    pub r#full_url: Option<super::super::types::Uri>,
    pub r#link: Vec<BundleLink>,
    pub r#request: Option<BundleEntryRequest>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#resource: Option<Box<super::Resource>>,
}
#[derive(Debug, Clone)]
pub struct Bundle {
    pub r#total: Option<super::super::types::UnsignedInt>,
    pub r#signature: Option<Box<super::super::types::Signature>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#link: Vec<BundleLink>,
    pub r#entry: Vec<BundleEntry>,
    pub r#language: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#timestamp: Option<super::super::types::Instant>,
    pub r#type: super::super::types::Code,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
}
