use std::cell::{Ref, RefCell, RefMut};

use serde::Serialize;

use fhirbolt_shared::{path::ElementPath, FhirRelease};

use crate::{context::Format, Resource};

pub trait SerializeResource: Resource {
    type Context<'a>: Serialize
    where
        Self: 'a;

    fn serialization_context(
        &self,
        config: SerializationConfig,
        output: Format,
    ) -> Self::Context<'_>;
}

impl<T> SerializeResource for T
where
    T: Resource,
    for<'a> SerializationContext<&'a Self>: Serialize,
{
    type Context<'a> = SerializationContext<&'a Self>
    where
        Self: 'a;

    fn serialization_context(
        &self,
        config: SerializationConfig,
        output: Format,
    ) -> Self::Context<'_> {
        SerializationContext::new(self, config, output, T::FHIR_RELEASE)
    }
}

/// Context for serialization.
#[repr(C)] // important for safe transmutes
pub struct SerializationContext<V> {
    pub(crate) value: V,
    // Serialization config
    pub(crate) config: SerializationConfig,
    // The JSON data model differs from the FHIR data model
    pub(crate) output: Format,
    // Used by the element model to keep track of its state in the element tree
    current_path: RefCell<Option<ElementPath>>,
}

impl<V> SerializationContext<V> {
    fn new(value: V, config: SerializationConfig, output: Format, r: FhirRelease) -> Self {
        SerializationContext {
            value,
            config,
            output,
            current_path: RefCell::new(Some(ElementPath::new(r))),
        }
    }

    pub(crate) fn current_path(&self) -> Ref<ElementPath> {
        Ref::map(self.current_path.borrow(), |p| p.as_ref().unwrap())
    }

    pub(crate) fn current_path_mut(&self) -> RefMut<ElementPath> {
        RefMut::map(self.current_path.borrow_mut(), |p| p.as_mut().unwrap())
    }

    pub(crate) fn with_context<P, R, F>(&self, value: P, with_fn: F) -> R
    where
        F: FnOnce(&SerializationContext<P>) -> R,
    {
        let context = SerializationContext {
            value,
            config: self.config,
            output: self.output,
            current_path: RefCell::new(self.current_path.borrow_mut().take()),
        };

        let result = with_fn(&context);

        self.current_path
            .borrow_mut()
            .replace(context.current_path.borrow_mut().take().unwrap());

        result
    }
}

#[derive(Copy, Clone)]
pub struct SerializationConfig {
    /// XML requires sorting of fields according to the FHIR data model.
    /// Sorting of fields for other data formats can be configured.
    /// This defaults to `true`.
    pub always_sort: bool,
}

impl Default for SerializationConfig {
    fn default() -> Self {
        SerializationConfig { always_sort: true }
    }
}
