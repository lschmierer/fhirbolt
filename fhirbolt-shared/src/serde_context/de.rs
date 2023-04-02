use std::{cell::RefCell, marker::PhantomData, mem};

use crate::{element::Element, path::ElementPath, FhirRelease};

thread_local! {
    pub static DESERIALIZATION_CONTEXT: RefCell<DeserializationContext<()>> = RefCell::new(Default::default());
}

pub fn with_context<F, R>(context: DeserializationContext<()>, f: F) -> R
where
    F: FnOnce() -> R,
{
    DESERIALIZATION_CONTEXT.with(|c| {
        c.replace(context);
        f()
    })
}

/// Context for deserialization.
#[derive(Default)]
#[repr(C)] // important for safe transmutes
pub struct DeserializationContext<V> {
    _phantom: PhantomData<V>,
    // Deserialization config
    pub config: DeserializationConfig,
    // The JSON data model differs from the FHIR data model
    pub from_json: bool,
    // Used by the element model to keep track of its state in the element tree
    current_path: Option<ElementPath>,
}

impl<V> DeserializationContext<V> {
    pub fn unwrap_current_path(&self) -> &ElementPath {
        self.current_path.as_ref().unwrap()
    }
    pub fn unwrap_current_path_mut(&mut self) -> &mut ElementPath {
        self.current_path.as_mut().unwrap()
    }
}

impl<const R: FhirRelease> DeserializationContext<Element<R>> {
    pub fn with_path_tracking(from_json: bool) -> Self {
        DeserializationContext {
            _phantom: PhantomData,
            config: Default::default(),
            from_json,
            current_path: Some(ElementPath::new(R)),
        }
    }
}

impl DeserializationContext<()> {
    pub fn without_path_tracking(config: DeserializationConfig, from_json: bool) -> Self {
        DeserializationContext {
            _phantom: PhantomData,
            config,
            from_json,
            current_path: None,
        }
    }
}

impl<V> DeserializationContext<V> {
    pub fn transmute<F>(&mut self) -> &mut DeserializationContext<F> {
        // DeserializationContext uses #[repr(C)] to make sure this is safe
        unsafe { mem::transmute(self) }
    }
}

/// Deserialization configuration options.
#[derive(Default, Copy, Clone)]
pub struct DeserializationConfig {
    // Deserialization mode
    pub mode: DeserializationMode,
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
