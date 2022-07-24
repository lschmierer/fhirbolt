// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct Oid {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Option<std::string::String>,
}
