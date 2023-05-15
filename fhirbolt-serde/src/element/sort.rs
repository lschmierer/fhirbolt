use fhirbolt_element::{Element, Value};
use fhirbolt_shared::{path::ElementPath, FhirRelease};

pub fn element_sorted<'a, const R: FhirRelease>(
    element: &'a Element<R>,
    current_path: &ElementPath,
) -> impl Iterator<Item = (&'a String, &'a Value<R>)> {
    let mut values = element.iter().collect::<Vec<_>>();

    // make sure that id, url and value are serialized first
    // these are serialized as XML attributes, the opening tag can thus be writtern earlier
    // and nested state can be avoided in the XML serializer
    if current_path.current_element_is_primitive() || current_path.current_element_is_extension() {
        values.sort_unstable_by_key(|(k, _v)| match k.as_str() {
            "id" => 0,
            "url" => 1,
            "value" => 2,
            "extension" => 3,
            _ => usize::MAX,
        });
    } else {
        values.sort_unstable_by_key(|(k, _v)| current_path.position_of_child(k));
    }

    values.into_iter()
}
