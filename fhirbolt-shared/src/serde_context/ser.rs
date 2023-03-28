use std::{cell::RefCell, rc::Rc};

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
#[derive(Default, Clone)]
pub struct SerializationContext<V> {
    pub value: V,
    // The JSON data model differs from the FHIR data model
    pub output_json: bool,
    // Used by the element model to keep track of its state in the element tree
    pub current_path: Option<Rc<RefCell<ElementPath>>>,
}

impl<V> SerializationContext<V> {
    pub fn unwrap_current_path(&self) -> &Rc<RefCell<ElementPath>> {
        self.current_path.as_ref().unwrap()
    }
}

impl<'a, const R: FhirRelease> SerializationContext<&'a Element<R>> {
    pub fn with_path_tracking(element: &'a Element<R>, output_json: bool) -> Self {
        SerializationContext {
            value: element,
            output_json,
            current_path: Some(Rc::new(RefCell::new(ElementPath::new(R)))),
        }
    }
}

impl SerializationContext<()> {
    pub fn without_path_tracking(output_json: bool) -> Self {
        SerializationContext {
            value: (),
            output_json,
            current_path: None,
        }
    }
}

impl<V: Clone> SerializationContext<V> {
    pub fn clone_with<W>(&self, value: W) -> SerializationContext<W> {
        SerializationContext {
            value,
            output_json: self.output_json,
            current_path: self.current_path.clone(),
        }
    }
}
