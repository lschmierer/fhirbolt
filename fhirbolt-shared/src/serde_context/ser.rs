use std::cell::{Ref, RefCell, RefMut};

use crate::{element::Element, path::ElementPath, FhirRelease};

thread_local! {
    pub static SERIALIZATION_CONTEXT: RefCell<SerializationContext<()>> = RefCell::new(Default::default());
}

pub fn with_context<F, R>(context: SerializationContext<()>, f: F) -> R
where
    F: FnOnce() -> R,
{
    SERIALIZATION_CONTEXT.with(|c| {
        c.replace(context);
        f()
    })
}

/// Context for serialization.
#[derive(Default)]
pub struct SerializationContext<V> {
    pub value: V,
    // The JSON data model differs from the FHIR data model
    pub output_json: bool,
    // Used by the element model to keep track of its state in the element tree
    pub current_path: RefCell<Option<ElementPath>>,
}

impl<V> SerializationContext<V> {
    pub fn unwrap_current_path(&self) -> Ref<ElementPath> {
        Ref::map(self.current_path.borrow(), |p| p.as_ref().unwrap())
    }

    pub fn unwrap_current_path_mut(&self) -> RefMut<ElementPath> {
        RefMut::map(self.current_path.borrow_mut(), |p| p.as_mut().unwrap())
    }

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

impl<'a, const R: FhirRelease> SerializationContext<&'a Element<R>> {
    pub fn with_path_tracking(element: &'a Element<R>, output_json: bool) -> Self {
        SerializationContext {
            value: element,
            output_json,
            current_path: RefCell::new(Some(ElementPath::new(R))),
        }
    }
}

impl SerializationContext<()> {
    pub fn without_path_tracking(output_json: bool) -> Self {
        SerializationContext {
            value: (),
            output_json,
            current_path: RefCell::new(None),
        }
    }
}
