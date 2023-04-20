// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "CodeableConcept Type: A concept that may be defined by a formal reference to a terminology or ontology or may be provided by text.\n\nThis is a common pattern in healthcare - a concept that may be defined by one or more codes from formal definitions including LOINC and SNOMED CT, and/or defined by the provision of text that captures a human sense of the concept."]
#[derive(Debug, Clone, PartialEq)]
pub struct CodeableConcept {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A reference to a code defined by a terminology system."]
    pub r#coding: Vec<Box<super::super::types::Coding>>,
    #[doc = "A human language representation of the concept as seen/selected/uttered by the user who entered the data and/or which represents the intended meaning of the user."]
    pub r#text: Option<super::super::types::String>,
}
impl Default for CodeableConcept {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#coding: Default::default(),
            r#text: Default::default(),
        }
    }
}
