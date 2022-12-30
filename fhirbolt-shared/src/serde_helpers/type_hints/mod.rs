mod generated;

pub use generated::*;

pub struct TypeHints {
    pub type_paths: phf::Map<&'static str, &'static str>,
    pub array_paths: phf::Set<&'static str>,
    pub boolean_paths: phf::Set<&'static str>,
    pub integer_paths: phf::Set<&'static str>,
    pub unsigned_integer_paths: phf::Set<&'static str>,
    pub positive_integer_paths: phf::Set<&'static str>,
    pub decimal_paths: phf::Set<&'static str>,
    pub other_primitives_paths: phf::Set<&'static str>,
    pub content_reference_paths: phf::Map<&'static str, &'static str>,
}
