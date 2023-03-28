mod de;
mod ser;

use std::ops::{Deref, DerefMut};

use crate::{AnyResource, FhirRelease};

#[derive(Clone, Debug)]
pub struct Element<const R: FhirRelease> {
    map: indexmap::IndexMap<String, Value<R>>,
}

impl<const R: FhirRelease> Deref for Element<R> {
    type Target = indexmap::IndexMap<String, Value<R>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<const R: FhirRelease> DerefMut for Element<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<const R: FhirRelease> FromIterator<(String, Value<R>)> for Element<R> {
    fn from_iter<I: IntoIterator<Item = (String, Value<R>)>>(iter: I) -> Self {
        Element {
            map: indexmap::IndexMap::from_iter(iter),
        }
    }
}

impl<'a, const R: FhirRelease> IntoIterator for &'a Element<R> {
    type Item = (&'a String, &'a Value<R>);
    type IntoIter = indexmap::map::Iter<'a, String, Value<R>>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const R: FhirRelease> IntoIterator for &'a mut Element<R> {
    type Item = (&'a String, &'a mut Value<R>);
    type IntoIter = indexmap::map::IterMut<'a, String, Value<R>>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const R: FhirRelease> IntoIterator for Element<R> {
    type Item = (String, Value<R>);
    type IntoIter = indexmap::map::IntoIter<String, Value<R>>;
    fn into_iter(self) -> Self::IntoIter {
        indexmap::IndexMap::into_iter(self.map)
    }
}

impl<const R: FhirRelease> AnyResource for Element<R> {
    const FHIR_RELEASE: FhirRelease = R;
}

#[derive(Clone, Debug)]
pub enum Value<const R: FhirRelease> {
    Element(Element<R>),
    Sequence(Vec<Element<R>>),
    Primitive(Primitive),
}

#[derive(Clone, Debug)]
pub enum Primitive {
    Bool(bool),
    Integer(i64),
    Decimal(String),
    String(String),
}
