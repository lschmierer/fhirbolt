use std::{marker::PhantomData, mem};

use serde::de::DeserializeSeed;

use crate::Resource;

pub trait DeserializeResource<'de>: Resource {
    type Context: DeserializeSeed<'de, Value = Self>;

    fn deserialization_context(config: DeserializationConfig, from_json: bool) -> Self::Context;
}

impl<'de, T> DeserializeResource<'de> for T
where
    T: Resource,
    DeserializationContext<T>: DeserializeSeed<'de, Value = T>,
{
    type Context = DeserializationContext<Self>;

    fn deserialization_context(config: DeserializationConfig, from_json: bool) -> Self::Context {
        DeserializationContext::new(config, from_json)
    }
}

pub trait DeserializeResourceOwned: for<'de> DeserializeResource<'de> {}
impl<T> DeserializeResourceOwned for T where T: for<'de> DeserializeResource<'de> {}

/// Context for deserialization.
#[repr(C)] // important for safe transmutes
pub struct DeserializationContext<V> {
    _phantom: PhantomData<V>,
    /// Deserialization config
    pub(crate) config: DeserializationConfig,
    /// The JSON data model differs from the FHIR data model
    pub(crate) from_json: bool,
    // Used by the element model to keep track of its state in the element tree
    current_element_stack: Vec<CurrentElement>,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum CurrentElement {
    Id,
    // ExampleScenario.instance has a "resourceType" field which requires special handling
    ExampleScenarioInstance,
    // Consent.provision has a "resourceType" field which requires special handling
    ConsentProvision,
    // Subscription.filterBy has a "resourceType" field which requires special handling
    SubscriptionFilterBy,
    // Extension.url can not have extensions
    Extension,
    // Extension.url can not have extensions
    ExtensionUrl,
    #[default]
    Other,
}

impl<V> DeserializationContext<V> {
    fn new(config: DeserializationConfig, from_json: bool) -> Self {
        DeserializationContext {
            _phantom: PhantomData,
            config,
            from_json,
            current_element_stack: vec![],
        }
    }

    pub(crate) fn transmute<F>(&mut self) -> &mut DeserializationContext<F> {
        // DeserializationContext uses #[repr(C)] to make sure this is safe
        unsafe { mem::transmute(self) }
    }

    pub(crate) fn clone<F>(&self) -> DeserializationContext<F> {
        DeserializationContext {
            _phantom: PhantomData,
            config: self.config,
            from_json: self.from_json,
            current_element_stack: self.current_element_stack.clone(),
        }
    }

    pub(crate) fn current_element(&self) -> CurrentElement {
        self.current_element_stack
            .last()
            .copied()
            .unwrap_or(CurrentElement::Other)
    }

    pub(crate) fn push_current_element(&mut self, element: CurrentElement) {
        self.current_element_stack.push(element)
    }

    pub(crate) fn pop_current_element(&mut self) {
        self.current_element_stack.pop();
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
