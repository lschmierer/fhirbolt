// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
#[doc = "Base StructureDefinition for ParameterDefinition Type: The parameters to the module. This collection specifies both the input and output parameters. Input parameters are provided by the caller as part of the $evaluate operation. Output parameters are included in the GuidanceResponse."]
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The name of the parameter used to allow access to the value of the parameter in evaluation contexts."]
    pub r#name: Option<super::super::types::Code>,
    #[doc = "Whether the parameter is input or output for the module."]
    pub r#use: super::super::types::Code,
    #[doc = "The minimum number of times this parameter SHALL appear in the request or response."]
    pub r#min: Option<super::super::types::Integer>,
    #[doc = "The maximum number of times this element is permitted to appear in the request or response."]
    pub r#max: Option<super::super::types::String>,
    #[doc = "A brief discussion of what the parameter is for and how it is used by the module."]
    pub r#documentation: Option<super::super::types::String>,
    #[doc = "The type of the parameter."]
    pub r#type: super::super::types::Code,
    #[doc = "If specified, this indicates a profile that the input data must conform to, or that the output data will conform to."]
    pub r#profile: Option<super::super::types::Canonical>,
}
#[allow(clippy::derivable_impls)]
impl Default for ParameterDefinition {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#name: Default::default(),
            r#use: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#min: Default::default(),
            r#max: Default::default(),
            r#documentation: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#profile: Default::default(),
        }
    }
}
