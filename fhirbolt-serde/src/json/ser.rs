use serde::ser::Serialize;

use fhirbolt_model::AnyResource;

use crate::json::error::Result;

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: ?Sized + Serialize + AnyResource,
{
    serde_json::to_writer(writer, value)
}

pub fn to_writer_pretty<W, T>(writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    serde_json::to_writer_pretty(writer, value)
}

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    serde_json::to_vec(value)
}

pub fn to_vec_pretty<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    serde_json::to_vec_pretty(value)
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    serde_json::to_string(value)
}

pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: ?Sized + serde::ser::Serialize + AnyResource,
{
    serde_json::to_string_pretty(value)
}

pub fn to_value<T>(value: T) -> Result<serde_json::Value>
where
    T: serde::ser::Serialize + AnyResource,
{
    serde_json::to_value(value)
}
