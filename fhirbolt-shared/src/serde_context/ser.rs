use std::cell::Cell;

thread_local! {
    pub static SERIALIZATION_CONTEXT: Cell<SerializationContext> = Cell::new(Default::default());
}

pub fn with_context<F, R>(config: SerializationContext, f: F) -> R
where
    F: FnOnce() -> R,
{
    SERIALIZATION_CONTEXT.with(|c| {
        c.set(config);
        f()
    })
}

/// Context for serialization.
#[derive(Default, Copy, Clone)]
pub struct SerializationContext {
    // The JSON data model differs from the FHIR data model
    pub output_json: bool,
}
