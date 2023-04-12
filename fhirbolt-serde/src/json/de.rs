//! Deserialize FHIR resources from JSON.

use std::io;

use serde::de::DeserializeSeed;
use serde_json::{error::Result, Deserializer};

use crate::{DeserializationConfig, DeserializeResource};

fn from_deserializer<'a, R, T>(
    de: &mut Deserializer<R>,
    config: Option<DeserializationConfig>,
) -> Result<T>
where
    R: serde_json::de::Read<'a>,
    T: DeserializeResource,
{
    T::new_context(config.unwrap_or(Default::default()), true, T::FHIR_RELEASE).deserialize(de)
}

/// Deserialize an instance of resource type `T` directly from an IO stream of JSON (e.g. coming from network).
///
/// # Example
/// ```
/// # fn main() {
/// // The `Resource` type is an enum that contains all possible FHIR resources.
/// // If the resource type is known in advance, you could also use a concrete resource type
/// // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
/// use fhirbolt::model::r4b::Resource as R4BResource;
///
/// // The type of `s` is `&str`
/// let s = "{
///         \"resourceType\": \"Observation\",
///         \"status\": \"final\",
///         \"code\": {
///             \"text\": \"some code\"
///         },
///         \"valueString\": \"some value\"
///     }";
///
/// // `s.as_bytes()` returns `&[u8]` which implements `std::io::Read`
/// let r: R4BResource = fhirbolt::json::from_reader(s.as_bytes(), None).unwrap();
/// println!("{:?}", r);
/// # }
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_reader<R, T>(rdr: R, config: Option<DeserializationConfig>) -> Result<T>
where
    R: io::Read,
    T: DeserializeResource,
{
    from_deserializer(&mut Deserializer::from_reader(rdr), config)
}

/// Deserialize an instance of resource type `T` from bytes of JSON text.
///
/// # Example
/// ```
/// # fn main() {
/// // The `Resource` type is an enum that contains all possible FHIR resources.
/// // If the resource type is known in advance, you could also use a concrete resource type
/// // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
/// use fhirbolt::model::r4b::Resource as R4BResource;
///
/// // The type of `b` is `&[u8]`
/// let b = b"{
///         \"resourceType\": \"Observation\",
///         \"status\": \"final\",
///         \"code\": {
///             \"text\": \"some code\"
///         },
///         \"valueString\": \"some value\"
///     }";
///
/// let r: R4BResource = fhirbolt::json::from_slice(b, None).unwrap();
/// println!("{:?}", r);
/// # }
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_slice<T>(v: &[u8], config: Option<DeserializationConfig>) -> Result<T>
where
    T: DeserializeResource,
{
    from_deserializer(&mut Deserializer::from_slice(v), config)
}

/// Deserialize an instance of resource type `T` from a string of JSON text.
///
/// # Example
/// ```
/// # fn main() {
/// // The `Resource` type is an enum that contains all possible FHIR resources.
/// // If the resource type is known in advance, you could also use a concrete resource type
/// // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
/// use fhirbolt::model::r4b::Resource as R4BResource;
///
/// // The type of `s` is `&str`
/// let s = "{
///         \"resourceType\": \"Observation\",
///         \"status\": \"final\",
///         \"code\": {
///             \"text\": \"some code\"
///         },
///         \"valueString\": \"some value\"
///     }";
///
/// let r: R4BResource = fhirbolt::json::from_str(s, None).unwrap();
/// println!("{:?}", r);
/// # }
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_str<T>(s: &str, config: Option<DeserializationConfig>) -> Result<T>
where
    T: DeserializeResource,
{
    from_deserializer(&mut Deserializer::from_str(s), config)
}

/// Deserialize an instance of resource type `T` from a `serde_json::Value`.
///
/// # Example
/// ```
/// # fn main() {
/// // The `Resource` type is an enum that contains all possible FHIR resources.
/// // If the resource type is known in advance, you could also use a concrete resource type
/// // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
/// use fhirbolt::model::r4b::Resource as R4BResource;
///
/// // The type of `s` is `&str`
/// let s = "{
///         \"resourceType\": \"Observation\",
///         \"status\": \"final\",
///         \"code\": {
///             \"text\": \"some code\"
///         },
///         \"valueString\": \"some value\"
///     }";
///
/// let v: serde_json::Value = serde_json::from_str(s).unwrap();
///
/// let r: R4BResource = fhirbolt::json::from_json_value(v, None).unwrap();
/// println!("{:?}", r);
/// # }
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_json_value<T>(
    value: serde_json::Value,
    config: Option<DeserializationConfig>,
) -> Result<T>
where
    T: DeserializeResource,
{
    T::new_context(config.unwrap_or(Default::default()), true, T::FHIR_RELEASE).deserialize(value)
}
