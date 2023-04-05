// Generated on 2023-04-05 by fhirbolt-codegen v0.1.0
#[derive(serde :: Serialize)]
pub struct PrimitiveElement<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a std::string::String>,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub extension: &'a [Box<super::types::Extension>],
}
#[derive(serde :: Deserialize)]
pub struct PrimitiveElementOwned {
    pub id: Option<std::string::String>,
    pub extension: Vec<Box<super::types::Extension>>,
}
