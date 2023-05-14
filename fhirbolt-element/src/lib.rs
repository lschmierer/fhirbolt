//! Generic element model.
//!
//! As deserialization differs slightly between FHIR releases,
//! `Element` is generic over a FHIR release.
//!
//! # Example
//! ```
//! use fhirbolt::FhirReleases;
//! use fhirbolt::element::{Element, Value, Primitive};
//!
//! let mut element = Element::<{ FhirReleases:: R4B }>::new();
//! element.insert(
//!     "resourceType".to_string(),
//!     Value::Primitive(
//!         Primitive::String("Observation".to_string())
//!     )
//! );
//! // ...
//! ```
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

pub use fhirbolt_shared::{FhirRelease, FhirReleases};

/// Macro for creating [`Element`].
///
/// # Examples
///
/// ```rust
/// use fhirbolt::FhirReleases;
/// use fhirbolt::element::{Element, Value, Primitive};
///
/// let element: Element<{ FhirReleases::R4 }> = Element! {
///     "value" => Value::Primitive(Primitive::String("123".into())),
/// };
/// ```
#[macro_export]
macro_rules! Element {
    {$($k: expr => $v: expr),* $(,)?} => {
        fhirbolt_element::Element::from([$(($k, $v),)*])
    };
}

/// Generic element in a FHIR resource.
///
/// As deserialization differs slightly between FHIR releases,
/// `Element` is generic over a FHIR release.
///
/// It is recommended to use the `Element!` macro for creating
/// new element.
///
/// # Example
/// ## With macro
/// ```rust
/// use fhirbolt::FhirReleases;
/// use fhirbolt::element::{Element, Value, Primitive};
///
/// let element: Element<{ FhirReleases::R4 }> = Element! {
///     "value" => Value::Primitive(Primitive::String("123".into())),
/// };
/// ```
///
/// ## Without macro
/// ```rust
/// use fhirbolt::FhirReleases;
/// use fhirbolt::element::{Element, Value, Primitive};
///
/// let mut element = Element::<{ FhirReleases:: R4B }>::new();
/// element.insert(
///     "value".to_string(),
///     Value::Primitive(
///         Primitive::String("123".to_string())
///     )
/// );
/// // ...
/// ```
#[derive(Default, Clone, PartialEq)]
pub struct Element<const R: FhirRelease> {
    map: indexmap::IndexMap<String, Value<R>>,
}

impl<const R: FhirRelease> Element<R> {
    /// Create a new element.
    #[inline]
    pub fn new() -> Self {
        Self {
            map: indexmap::IndexMap::new(),
        }
    }

    /// Create a new element wit preallocated capacity.
    #[inline]
    pub fn with_capacity(n: usize) -> Self {
        Self {
            map: indexmap::IndexMap::with_capacity(n),
        }
    }
}

impl<const R: FhirRelease> Deref for Element<R> {
    type Target = indexmap::IndexMap<String, Value<R>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<const R: FhirRelease> DerefMut for Element<R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<const R: FhirRelease, const N: usize> From<[(String, Value<R>); N]> for Element<R> {
    fn from(arr: [(String, Value<R>); N]) -> Self {
        Element {
            map: indexmap::IndexMap::from(arr),
        }
    }
}

impl<const R: FhirRelease, const N: usize> From<[(&str, Value<R>); N]> for Element<R> {
    fn from(arr: [(&str, Value<R>); N]) -> Self {
        Element::from_iter(arr.map(|(k, v)| (k.into(), v)))
    }
}

impl<const R: FhirRelease> FromIterator<(String, Value<R>)> for Element<R> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = (String, Value<R>)>>(iter: I) -> Self {
        Element {
            map: indexmap::IndexMap::from_iter(iter),
        }
    }
}

impl<'a, const R: FhirRelease> IntoIterator for &'a Element<R> {
    type Item = (&'a String, &'a Value<R>);
    type IntoIter = indexmap::map::Iter<'a, String, Value<R>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const R: FhirRelease> IntoIterator for &'a mut Element<R> {
    type Item = (&'a String, &'a mut Value<R>);
    type IntoIter = indexmap::map::IterMut<'a, String, Value<R>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const R: FhirRelease> IntoIterator for Element<R> {
    type Item = (String, Value<R>);
    type IntoIter = indexmap::map::IntoIter<String, Value<R>>;

    #[inline]
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

/// Generic value in a FHIR resource.
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

/// Primitive value in a FHIR resource.
#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Bool(bool),
    Integer(i32),
    Integer64(i64),
    Decimal(String),
    String(String),
}
