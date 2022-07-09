// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Period {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#start: Option<super::super::types::DateTime>,
    pub r#id: Option<std::string::String>,
    pub r#end: Option<super::super::types::DateTime>,
}
