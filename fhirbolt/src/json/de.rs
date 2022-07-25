use std::cell::Cell;

thread_local! {
    pub static DESERIALIZATION_CONFIG: Cell<DeserializationConfig> = Cell::new(Default::default());
}

fn with_config<F, R>(config: Option<DeserializationConfig>, f: F) -> R
where
    F: FnOnce() -> R,
{
    DESERIALIZATION_CONFIG.with(|c| {
        c.set(config.unwrap_or(Default::default()));
        f()
    })
}

/// Deserialization mode
///
/// The deserializer supports different operating modes.
/// The default mode is "strict."
/// In strict mode, only fields known to the FHIR data model
/// are supported.
/// If the deserializer encounters any other data, an error
/// is returned.
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub enum DeserializationMode {
    /// Sets the deserializer in "strict" mode (default).
    ///
    /// In strict mode, only fields known to the FHIR data model
    /// are supported.
    /// If the deserializer encounters any other data, an error
    /// is returned.
    #[default]
    Strict,

    /// Sets the deserializer in "compatibility" mode.
    ///
    /// In compatibility mode, the input is allowed to contain
    /// fields unknown to the FHIR specification.
    /// This enables backwards-compatibility for normative content
    /// (e.g. parsing normative R3 resources with the R4 data model).
    ///
    /// Note that serializing previously deserialized data will not
    /// neccessarily result in identical output.
    /// The content of unknown fields will be lost.
    Compatibility,

    /// Sets the deserializer in "lax" mode.
    ///
    /// In lax mode, the deserializer tries its best to always return
    /// deserialized FHIR data.
    /// Unknown fields are ignored and even the missing of required
    /// fields is connived (filled with `Default::default()`).
    ///
    /// Note that lax mode might return invalid data and serializing
    /// previously deserialized data will not neccessarily result in
    /// identical output.
    /// The content of unknown fields will be lost.
    Lax,
}

/// JSON Deserialization configuration options
#[derive(Default, Copy, Clone)]
pub struct DeserializationConfig {
    // Deserialization mode
    pub mode: DeserializationMode,
}

pub fn from_reader<R, T>(rdr: R, config: Option<DeserializationConfig>) -> serde_json::Result<T>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    with_config(config, || serde_json::from_reader(rdr))
}

pub fn from_slice<'a, T>(
    v: &'a [u8],
    config: Option<DeserializationConfig>,
) -> serde_json::Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    with_config(config, || serde_json::from_slice(v))
}

pub fn from_str<'a, T>(s: &'a str, config: Option<DeserializationConfig>) -> serde_json::Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    with_config(config, || serde_json::from_str(s))
}

pub fn from_value<T>(
    value: serde_json::Value,
    config: Option<DeserializationConfig>,
) -> serde_json::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    with_config(config, || serde_json::from_value(value))
}
