// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Period {
    pub r#id: Option<std::string::String>,
    pub r#start: Option<super::super::types::DateTime>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#end: Option<super::super::types::DateTime>,
}
