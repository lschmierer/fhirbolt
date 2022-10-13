mod generated;

pub use generated::*;

pub struct TypeHints {
    pub type_paths: std::collections::HashMap<&'static str, &'static str>,
    pub array_paths: std::collections::HashSet<&'static str>,
    pub boolean_paths: std::collections::HashSet<&'static str>,
    pub integer_paths: std::collections::HashSet<&'static str>,
    pub unsigned_integer_paths: std::collections::HashSet<&'static str>,
    pub positive_integer_paths: std::collections::HashSet<&'static str>,
    pub decimal_paths: std::collections::HashSet<&'static str>,
    pub other_primitives_paths: std::collections::HashSet<&'static str>,
    pub content_reference_paths: std::collections::HashMap<&'static str, &'static str>,
}
