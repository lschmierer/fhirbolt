use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

use fhirbolt::{
    element::{Element, Value},
    FhirRelease,
};

pub fn shuffle_element<const R: FhirRelease>(element: Element<R>) -> Element<R> {
    let mut vec = element
        .into_iter()
        .map(|(k, v)| {
            if let Value::Element(e) = v {
                (k, Value::Element(shuffle_element(e)))
            } else {
                (k, v)
            }
        })
        .collect::<Vec<_>>();

    vec.shuffle(&mut SmallRng::seed_from_u64(1));

    Element::from_iter(vec)
}
