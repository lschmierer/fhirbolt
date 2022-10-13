use std::cell::Cell;

thread_local! {
    pub static DESERIALIZATION_CONFIG: Cell<DeserializationConfig> = Cell::new(Default::default());
}

pub fn with_config<F, R>(config: Option<DeserializationConfig>, f: F) -> R
where
    F: FnOnce() -> R,
{
    DESERIALIZATION_CONFIG.with(|c| {
        c.set(config.unwrap_or(Default::default()));
        f()
    })
}

/// The deserializer supports different operating modes.
/// The default mode is **strict**.
///
/// In *strict* mode, only fields known to the FHIR data model
/// are supported.
/// If the deserializer encounters any other data, an error
/// is returned.
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub enum DeserializationMode {
    /// Sets the deserializer to **strict** mode (default).
    ///
    /// In strict mode, only fields known to the FHIR data model
    /// are supported.
    /// If the deserializer encounters any other data, an error
    /// is returned.
    #[default]
    Strict,

    /// Sets the deserializer to **compatibility** mode.
    ///
    /// In compatibility mode, input is allowed to contain
    /// fields unknown to the FHIR specification.
    /// This enables backwards-compatibility for normative content
    /// (e.g. parsing normative R3 resources with the R4 data model).
    ///
    /// Note that serializing previously deserialized data will not
    /// neccessarily result in identical output.
    /// The content of unknown fields will be lost.
    Compatibility,

    /// Sets the deserializer to **lax** mode.
    ///
    /// In lax mode, the deserializer tries its best to always return
    /// deserialized data.
    /// Unknown fields are ignored and missing required
    /// fields are padded with `Default::default()`.
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
