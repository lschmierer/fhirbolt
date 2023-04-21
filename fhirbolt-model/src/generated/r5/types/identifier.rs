// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "Identifier Type: An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.\n\nNeed to be able to identify things with confidence and be sure that the identification is not subject to misinterpretation."]
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The purpose of this identifier."]
    pub r#use: Option<super::super::types::Code>,
    #[doc = "A coded type for the identifier that can be used to determine which identifier to use for a specific purpose."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Establishes the namespace for the value - that is, an absolute URL that describes a set values that are unique."]
    pub r#system: Option<super::super::types::Uri>,
    #[doc = "The portion of the identifier typically relevant to the user and which is unique within the context of the system."]
    pub r#value: Option<super::super::types::String>,
    #[doc = "Time period during which identifier is/was valid for use."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Organization that issued/manages the identifier."]
    pub r#assigner: Option<Box<super::super::types::Reference>>,
}
impl Default for Identifier {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#use: Default::default(),
            r#type: Default::default(),
            r#system: Default::default(),
            r#value: Default::default(),
            r#period: Default::default(),
            r#assigner: Default::default(),
        }
    }
}
