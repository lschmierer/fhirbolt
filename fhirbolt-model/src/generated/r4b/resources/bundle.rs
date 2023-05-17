// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "A series of links that provide context to this bundle."]
#[derive(Debug, Clone, PartialEq)]
pub struct BundleLink {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A name which details the functional use for this link - see [<http://www.iana.org/assignments/link>-relations/link-relations.xhtml#link-relations-1](<http://www.iana.org/assignments/link>-relations/link-relations.xhtml#link-relations-1)."]
    pub r#relation: super::super::types::String,
    #[doc = "The reference details for the link."]
    pub r#url: super::super::types::Uri,
}
#[allow(clippy::derivable_impls)]
impl Default for BundleLink {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#relation: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#url: super::super::types::Uri {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Information about the search process that lead to the creation of this entry."]
#[derive(Debug, Clone, PartialEq)]
pub struct BundleEntrySearch {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Why this entry is in the result set - whether it's included as a match or because of an _include requirement, or to convey information or warning information about the search process."]
    pub r#mode: Option<super::super::types::Code>,
    #[doc = "When searching, the server's search ranking score for the entry."]
    pub r#score: Option<super::super::types::Decimal>,
}
#[allow(clippy::derivable_impls)]
impl Default for BundleEntrySearch {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#mode: Default::default(),
            r#score: Default::default(),
        }
    }
}
#[doc = "Additional information about how this entry should be processed as part of a transaction or batch.  For history, it shows how the entry was processed to create the version contained in the entry."]
#[derive(Debug, Clone, PartialEq)]
pub struct BundleEntryRequest {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "In a transaction or batch, this is the HTTP action to be executed for this entry. In a history bundle, this indicates the HTTP action that occurred."]
    pub r#method: super::super::types::Code,
    #[doc = "The URL for this entry, relative to the root (the address to which the request is posted)."]
    pub r#url: super::super::types::Uri,
    #[doc = "If the ETag values match, return a 304 Not Modified status. See the API documentation for [\"Conditional Read\"](<http.html>#cread)."]
    pub r#if_none_match: Option<super::super::types::String>,
    #[doc = "Only perform the operation if the last updated date matches. See the API documentation for [\"Conditional Read\"](<http.html>#cread)."]
    pub r#if_modified_since: Option<super::super::types::Instant>,
    #[doc = "Only perform the operation if the Etag value matches. For more information, see the API section [\"Managing Resource Contention\"](<http.html>#concurrency)."]
    pub r#if_match: Option<super::super::types::String>,
    #[doc = "Instruct the server not to perform the create if a specified resource already exists. For further information, see the API documentation for [\"Conditional Create\"](<http.html>#ccreate). This is just the query portion of the URL - what follows the \"?\" (not including the \"?\")."]
    pub r#if_none_exist: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for BundleEntryRequest {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#method: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#url: super::super::types::Uri {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#if_none_match: Default::default(),
            r#if_modified_since: Default::default(),
            r#if_match: Default::default(),
            r#if_none_exist: Default::default(),
        }
    }
}
#[doc = "Indicates the results of processing the corresponding 'request' entry in the batch or transaction being responded to or what the results of an operation where when returning history."]
#[derive(Debug, Clone, PartialEq)]
pub struct BundleEntryResponse {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The status code returned by processing this entry. The status SHALL start with a 3 digit HTTP code (e.g. 404) and may contain the standard HTTP description associated with the status code."]
    pub r#status: super::super::types::String,
    #[doc = "The location header created by processing this operation, populated if the operation returns a location."]
    pub r#location: Option<super::super::types::Uri>,
    #[doc = "The Etag for the resource, if the operation for the entry produced a versioned resource (see [Resource Metadata and Versioning](<http.html>#versioning) and [Managing Resource Contention](<http.html>#concurrency))."]
    pub r#etag: Option<super::super::types::String>,
    #[doc = "The date/time that the resource was modified on the server."]
    pub r#last_modified: Option<super::super::types::Instant>,
    #[doc = "An OperationOutcome containing hints and warnings produced as part of processing this entry in a batch or transaction."]
    pub r#outcome: Option<super::super::Resource>,
}
#[allow(clippy::derivable_impls)]
impl Default for BundleEntryResponse {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#status: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#location: Default::default(),
            r#etag: Default::default(),
            r#last_modified: Default::default(),
            r#outcome: Default::default(),
        }
    }
}
#[doc = "An entry in a bundle resource - will either contain a resource or information about a resource (transactions and history only)."]
#[derive(Debug, Clone, PartialEq)]
pub struct BundleEntry {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A series of links that provide context to this entry."]
    pub r#link: Vec<BundleLink>,
    #[doc = "The Absolute URL for the resource.  The fullUrl SHALL NOT disagree with the id in the resource - i.e. if the fullUrl is not a urn:uuid, the URL shall be version-independent URL consistent with the Resource.id. The fullUrl is a version independent reference to the resource. The fullUrl element SHALL have a value except that: \n* fullUrl can be empty on a POST (although it does not need to when specifying a temporary id for reference in the bundle)\n* Results from operations might involve resources that are not identified."]
    pub r#full_url: Option<super::super::types::Uri>,
    #[doc = "The Resource for the entry. The purpose/meaning of the resource is determined by the Bundle.type."]
    pub r#resource: Option<super::super::Resource>,
    #[doc = "Information about the search process that lead to the creation of this entry."]
    pub r#search: Option<BundleEntrySearch>,
    #[doc = "Additional information about how this entry should be processed as part of a transaction or batch.  For history, it shows how the entry was processed to create the version contained in the entry."]
    pub r#request: Option<BundleEntryRequest>,
    #[doc = "Indicates the results of processing the corresponding 'request' entry in the batch or transaction being responded to or what the results of an operation where when returning history."]
    pub r#response: Option<BundleEntryResponse>,
}
#[allow(clippy::derivable_impls)]
impl Default for BundleEntry {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link: Default::default(),
            r#full_url: Default::default(),
            r#resource: Default::default(),
            r#search: Default::default(),
            r#request: Default::default(),
            r#response: Default::default(),
        }
    }
}
#[doc = "A container for a collection of resources."]
#[derive(Debug, Clone, PartialEq)]
pub struct Bundle {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<Box<super::super::types::Id>>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A persistent identifier for the bundle that won't change as a bundle is copied from server to server."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Indicates the purpose of this bundle - how it is intended to be used."]
    pub r#type: super::super::types::Code,
    #[doc = "The date/time that the bundle was assembled - i.e. when the resources were placed in the bundle."]
    pub r#timestamp: Option<super::super::types::Instant>,
    #[doc = "If a set of search matches, this is the total number of entries of type 'match' across all pages in the search.  It does not include search.mode = 'include' or 'outcome' entries and it does not provide a count of the number of entries in the Bundle."]
    pub r#total: Option<super::super::types::UnsignedInt>,
    #[doc = "A series of links that provide context to this bundle."]
    pub r#link: Vec<BundleLink>,
    #[doc = "An entry in a bundle resource - will either contain a resource or information about a resource (transactions and history only)."]
    pub r#entry: Vec<BundleEntry>,
    #[doc = "Digital Signature - base64 encoded. XML-DSig or a JWT."]
    pub r#signature: Option<Box<super::super::types::Signature>>,
}
#[allow(clippy::derivable_impls)]
impl Default for Bundle {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#identifier: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#timestamp: Default::default(),
            r#total: Default::default(),
            r#link: Default::default(),
            r#entry: Default::default(),
            r#signature: Default::default(),
        }
    }
}
