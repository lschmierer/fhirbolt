use std::io;

use serde::ser::Serialize;

use fhirbolt_shared::{
    serde_context::ser::{with_context, SerializationContext},
    AnyResource,
};

use crate::xml::{error::Result, ser::Serializer};

/// Serialize the given resource as XML into the IO stream.
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize + AnyResource,
{
    with_context(SerializationContext::without_path_tracking(false), || {
        value.serialize(&mut Serializer::new(writer))
    })
}

/// Serialize the given resource as a XML byte vector.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize + AnyResource,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given resource as a String of JSON.
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize + AnyResource,
{
    let vec = to_vec(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}
