use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use crate::FhirRelease;

pub mod error;

pub mod de;
pub mod ser;

#[derive(Clone, PartialEq)]
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

impl<const R: FhirRelease> fmt::Debug for Element<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut s = f.debug_struct(&format!("Element<{}>", R));

        for (key, value) in self {
            s.field(key, value);
        }

        s.finish()
    }
}

#[derive(Clone, PartialEq)]
pub enum Value<const R: FhirRelease> {
    Element(Element<R>),
    Sequence(Vec<Element<R>>),
    Primitive(Primitive),
}

impl<const R: FhirRelease> fmt::Debug for Value<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Value::Element(e) => e.fmt(f),
            Value::Sequence(s) => s.fmt(f),
            Value::Primitive(p) => write!(f, "{:?}", p),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Bool(bool),
    Integer(i64),
    Decimal(String),
    String(String),
}
