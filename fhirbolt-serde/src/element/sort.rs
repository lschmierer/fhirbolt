use fhirbolt_element::{Element, Value};
use fhirbolt_shared::{path::ElementPath, FhirRelease};

use crate::utils::EitherIter;

struct IdUrlValueFirstIterator<'a, const R: FhirRelease> {
    iter: indexmap::map::Iter<'a, String, Value<R>>,
    id: Option<&'a Value<R>>,
    url: Option<&'a Value<R>>,
    value: Option<&'a Value<R>>,
}

impl<'a, const R: FhirRelease> IdUrlValueFirstIterator<'a, R> {
    fn new(element: &'a Element<R>) -> Self {
        let mut new = Self {
            iter: element.iter(),
            id: None,
            url: None,
            value: None,
        };

        for (k, v) in new.iter.clone() {
            if k == "id" {
                new.id = Some(v);
            }
            if k == "url" {
                new.url = Some(v);
            }
            if k == "value" {
                new.value = Some(v);
            }
        }

        new
    }
}

impl<'a, const R: FhirRelease> Iterator for IdUrlValueFirstIterator<'a, R> {
    type Item = (&'a str, &'a Value<R>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.id.take() {
            return Some(("id", id));
        }
        if let Some(url) = self.url.take() {
            return Some(("url", url));
        }
        if let Some(value) = self.value.take() {
            return Some(("value", value));
        }

        if let Some((key, value)) = self.iter.next() {
            if key == "id" || key == "url" || key == "value" {
                self.next()
            } else {
                Some((key, value))
            }
        } else {
            None
        }
    }
}

pub fn element_sorted<'a, const R: FhirRelease>(
    element: &'a Element<R>,
    current_path: &ElementPath,
) -> impl Iterator<Item = (&'a str, &'a Value<R>)> {
    // make sure that id, url and value are serialized first
    // these are serialized as XML attributes, the opening tag can thus be writtern earlier
    // and nested state can be avoided in the XML serializer
    if current_path.current_element_is_primitive() || current_path.current_element_is_extension() {
        EitherIter::Left(IdUrlValueFirstIterator::new(element))
    } else {
        let mut values: Vec<_> = element.iter().map(|(k, v)| (k.as_str(), v)).collect();

        values.sort_unstable_by_key(|(k, _v)| current_path.position_of_child(k));

        EitherIter::Right(values.into_iter())
    }
}
