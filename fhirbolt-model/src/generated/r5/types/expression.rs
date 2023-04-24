// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Expression Type: A expression that is evaluated in a specified context and returns a value. The context of use of the expression must specify the context in which the expression is evaluated, and how the result of the expression is used."]
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "A brief, natural language description of the condition that effectively communicates the intended semantics."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "A short name assigned to the expression to allow for multiple reuse of the expression in the context where it is defined."]
    pub r#name: Option<super::super::types::Code>,
    #[doc = "The media type of the language for the expression."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "An expression in the specified language that returns a value."]
    pub r#expression: Option<super::super::types::String>,
    #[doc = "A URI that defines where the expression is found."]
    pub r#reference: Option<super::super::types::Uri>,
}
impl Default for Expression {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#description: Default::default(),
            r#name: Default::default(),
            r#language: Default::default(),
            r#expression: Default::default(),
            r#reference: Default::default(),
        }
    }
}
