use std::cell::{Ref, RefCell, RefMut};

use serde::Serialize;

use fhirbolt_shared::{path::ElementPath, FhirRelease};

use crate::Resource;

pub trait SerializeResource: Resource {
    type Context<'a>: Serialize
    where
        Self: 'a;

    fn context<'a>(&'a self, output_json: bool, r: FhirRelease) -> Self::Context<'a>;
}

impl<T> SerializeResource for T
where
    T: Resource,
    for<'a> SerializationContext<&'a Self>: Serialize,
{
    type Context<'a> = SerializationContext<&'a Self>
    where
        Self: 'a;

    fn context<'a>(&'a self, output_json: bool, r: FhirRelease) -> Self::Context<'a> {
        SerializationContext::new(self, output_json, r)
    }
}

/// Context for serialization.
#[derive(Default)]
pub struct SerializationContext<V> {
    pub value: V,
    // The JSON data model differs from the FHIR data model
    pub output_json: bool,
    // Used by the element model to keep track of its state in the element tree
    current_path: RefCell<Option<ElementPath>>,
}

impl<V> SerializationContext<V> {
    pub fn new(value: V, output_json: bool, r: FhirRelease) -> Self {
        SerializationContext {
            value,
            output_json,
            current_path: RefCell::new(Some(ElementPath::new(r))),
        }
    }

    #[inline]
    pub fn unwrap_current_path(&self) -> Ref<ElementPath> {
        Ref::map(self.current_path.borrow(), |p| p.as_ref().unwrap())
    }

    #[inline]
    pub fn unwrap_current_path_mut(&self) -> RefMut<ElementPath> {
        RefMut::map(self.current_path.borrow_mut(), |p| p.as_mut().unwrap())
    }

    #[inline]
    pub fn with_context<P, R, F>(&self, value: P, with_fn: F) -> R
    where
        F: FnOnce(&SerializationContext<P>) -> R,
    {
        let context = SerializationContext {
            value,
            output_json: self.output_json,
            current_path: RefCell::new(self.current_path.borrow_mut().take()),
        };
        let result = with_fn(&context);
        self.current_path
            .borrow_mut()
            .replace(context.current_path.borrow_mut().take().unwrap());
        result
    }
}
