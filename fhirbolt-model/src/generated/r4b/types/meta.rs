// Generated on 2023-04-14 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Meta Type: The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Meta {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The version specific identifier, as it appears in the version portion of the URL. This value changes when the resource is created, updated, or deleted."]
    pub r#version_id: Option<super::super::types::Id>,
    #[doc = "When the resource last changed - e.g. when the version changed."]
    pub r#last_updated: Option<super::super::types::Instant>,
    #[doc = "A uri that identifies the source system of the resource. This provides a minimal amount of [Provenance](provenance.html#) information that can be used to track or differentiate the source of information in the resource. The source may identify another FHIR server, document, message, database, etc."]
    pub r#source: Option<super::super::types::Uri>,
    #[doc = "A list of profiles (references to [StructureDefinition](structuredefinition.html#) resources) that this resource claims to conform to. The URL is a reference to [StructureDefinition.url](structuredefinition-definitions.html#StructureDefinition.url)."]
    pub r#profile: Vec<super::super::types::Canonical>,
    #[doc = "Security labels applied to this resource. These tags connect specific resources to the overall security policy and infrastructure."]
    pub r#security: Vec<Box<super::super::types::Coding>>,
    #[doc = "Tags applied to this resource. Tags are intended to be used to identify and relate resources to process and workflow, and applications are not required to consider the tags when interpreting the meaning of a resource."]
    pub r#tag: Vec<Box<super::super::types::Coding>>,
}
