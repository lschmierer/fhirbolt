use serde::de::{Deserialize, DeserializeOwned};

pub use serde_json::de::*;

use fhirbolt_shared::{
    serde_config::de::{with_config, DeserializationConfig},
    AnyResource,
};

use crate::json::error::Result;

pub fn from_reader<R, T>(rdr: R, config: Option<DeserializationConfig>) -> Result<T>
where
    R: std::io::Read,
    T: DeserializeOwned + AnyResource,
{
    with_config(config, || serde_json::from_reader(rdr))
}

pub fn from_slice<'a, T>(v: &'a [u8], config: Option<DeserializationConfig>) -> Result<T>
where
    T: Deserialize<'a> + AnyResource,
{
    with_config(config, || serde_json::from_slice(v))
}

pub fn from_str<'a, T>(s: &'a str, config: Option<DeserializationConfig>) -> Result<T>
where
    T: Deserialize<'a> + AnyResource,
{
    with_config(config, || serde_json::from_str(s))
}

pub fn from_value<T>(value: serde_json::Value, config: Option<DeserializationConfig>) -> Result<T>
where
    T: DeserializeOwned + AnyResource,
{
    with_config(config, || serde_json::from_value(value))
}
