/// Deserialize FHIR resources from JSON.
use serde::de::{Deserialize, DeserializeOwned};

use fhirbolt_shared::{
    serde_context::de::{with_context, DeserializationConfig, DeserializationContext},
    AnyResource,
};

use crate::json::error::Result;

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
    R: std::io::Read,
    T: DeserializeOwned + AnyResource,
{
    with_context(
        DeserializationContext {
            config: config.unwrap_or_default(),
            from_json: true,
        },
        || serde_json::from_reader(rdr),
    )
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
pub fn from_slice<'a, T>(v: &'a [u8], config: Option<DeserializationConfig>) -> Result<T>
where
    T: Deserialize<'a> + AnyResource,
{
    with_context(
        DeserializationContext {
            config: config.unwrap_or_default(),
            from_json: true,
        },
        || serde_json::from_slice(v),
    )
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
pub fn from_str<'a, T>(s: &'a str, config: Option<DeserializationConfig>) -> Result<T>
where
    T: Deserialize<'a> + AnyResource,
{
    with_context(
        DeserializationContext {
            config: config.unwrap_or_default(),
            from_json: true,
        },
        || serde_json::from_str(s),
    )
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
    T: DeserializeOwned + AnyResource,
{
    with_context(
        DeserializationContext {
            config: config.unwrap_or_default(),
            from_json: true,
        },
        || serde_json::from_value(value),
    )
}
