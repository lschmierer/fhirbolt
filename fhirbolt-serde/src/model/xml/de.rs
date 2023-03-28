use std::io;

use serde::Deserialize;

use fhirbolt_shared::{
    serde_context::de::{with_context, DeserializationConfig, DeserializationContext},
    AnyResource,
};

use crate::xml::{de::Deserializer, error::Result, read::Read};

fn from_deserializer<'a, R, T>(
    de: &mut Deserializer<R>,
    config: Option<DeserializationConfig>,
) -> Result<T>
where
    R: Read,
    T: Deserialize<'a> + AnyResource,
{
    with_context(
        DeserializationContext::without_path_tracking(config.unwrap_or_default(), false),
        || T::deserialize(de),
    )
}

/// Deserialize an instance of resource type `T` directly from an IO stream of XML (e.g. coming from network).
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
/// let s = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
///     <Observation xmlns=\"http://hl7.org/fhir\">
///         <status value=\"final\"/>
///         <code>
///             <text value=\"some code\"/>
///         </code>
///         <valueString value=\"some value\"/>
///     </Observation>";
///
/// // `s.as_bytes()` returns `&[u8]` which implements `std::io::Read`
/// let r: R4BResource = fhirbolt::model::xml::from_reader(s.as_bytes(), None).unwrap();
/// println!("{:?}", r);
/// # }
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_reader<'a, R: io::Read, T>(rdr: R, config: Option<DeserializationConfig>) -> Result<T>
where
    T: Deserialize<'a> + AnyResource,
{
    from_deserializer(
        &mut Deserializer::from_reader(rdr, T::FHIR_RELEASE)?,
        config,
    )
}

/// Deserialize an instance of resource type `T` from a bytes of XML.
///
/// # Example
/// ```
/// # fn main() {
/// // The `Resource` type is an enum that contains all possible FHIR resources.
/// // If the resource type is known in advance, you could also use a concrete resource type
/// // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
/// use fhirbolt::model::r4b::Resource as R4BResource;
///
/// // The type of `s` is `&[u8]`
/// let b = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>
///     <Observation xmlns=\"http://hl7.org/fhir\">
///         <status value=\"final\"/>
///         <code>
///             <text value=\"some code\"/>
///         </code>
///         <valueString value=\"some value\"/>
///     </Observation>";
///
/// let r: R4BResource = fhirbolt::model::xml::from_slice(b, None).unwrap();
/// println!("{:?}", r);
/// # }
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_slice<'a, T>(v: &[u8], config: Option<DeserializationConfig>) -> Result<T>
where
    T: Deserialize<'a> + AnyResource,
{
    from_deserializer(&mut Deserializer::from_slice(v, T::FHIR_RELEASE)?, config)
}

/// Deserialize an instance of resource type `T` from a string of XML.
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
/// let s = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
///     <Observation xmlns=\"http://hl7.org/fhir\">
///         <status value=\"final\"/>
///         <code>
///             <text value=\"some code\"/>
///         </code>
///         <valueString value=\"some value\"/>
///     </Observation>";
///
/// let r: R4BResource = fhirbolt::model::xml::from_str(s, None).unwrap();
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
    from_deserializer(&mut Deserializer::from_str(s, T::FHIR_RELEASE)?, config)
}
