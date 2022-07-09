// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct SampledData {
    pub r#origin: Box<super::super::types::Quantity>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#dimensions: super::super::types::PositiveInt,
    pub r#id: Option<std::string::String>,
    pub r#lower_limit: Option<super::super::types::Decimal>,
    pub r#data: Option<super::super::types::String>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#period: super::super::types::Decimal,
    pub r#upper_limit: Option<super::super::types::Decimal>,
}
