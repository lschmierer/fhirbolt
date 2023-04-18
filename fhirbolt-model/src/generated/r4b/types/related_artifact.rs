// Generated on 2023-04-18 by fhirbolt-codegen v0.2.0
#[doc = "Base StructureDefinition for RelatedArtifact Type: Related artifacts such as additional documentation, justification, or bibliographic references.\n\nKnowledge resources must be able to provide enough information for consumers of the content (and/or interventions or results produced by the content) to be able to determine and understand the justification for and evidence in support of the content."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RelatedArtifact {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of relationship to the related artifact."]
    pub r#type: super::super::types::Code,
    #[doc = "A short label that can be used to reference the citation from elsewhere in the containing artifact, such as a footnote index."]
    pub r#label: Option<super::super::types::String>,
    #[doc = "A brief description of the document or knowledge resource being referenced, suitable for display to a consumer."]
    pub r#display: Option<super::super::types::String>,
    #[doc = "A bibliographic citation for the related artifact. This text SHOULD be formatted according to an accepted citation format."]
    pub r#citation: Option<super::super::types::Markdown>,
    #[doc = "A url for the artifact that can be followed to access the actual content."]
    pub r#url: Option<super::super::types::Url>,
    #[doc = "The document being referenced, represented as an attachment. This is exclusive with the resource element."]
    pub r#document: Option<Box<super::super::types::Attachment>>,
    #[doc = "The related resource, such as a library, value set, profile, or other knowledge resource."]
    pub r#resource: Option<super::super::types::Canonical>,
}
