mod generated;

pub use generated::*;

pub type ElementSet = phf::OrderedSet<&'static str>;
pub type ElementMap = phf::Map<&'static str, &'static ElementSet>;
