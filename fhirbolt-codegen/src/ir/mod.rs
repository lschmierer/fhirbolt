mod parse;

use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;

pub use parse::parse_modules;

#[derive(Default, Debug)]
pub struct TypeHints {
    pub type_paths: LinkedHashMap<String, String>,
    pub array_paths: LinkedHashSet<String>,
    pub boolean_paths: LinkedHashSet<String>,
    pub integer_paths: LinkedHashSet<String>,
    pub integer64_paths: LinkedHashSet<String>,
    pub unsigned_integer_paths: LinkedHashSet<String>,
    pub positive_integer_paths: LinkedHashSet<String>,
    pub decimal_integer_paths: LinkedHashSet<String>,
    pub all_primitives_paths: LinkedHashSet<String>,
    pub content_reference_paths: LinkedHashMap<String, String>,
}

#[derive(Default, Debug)]
pub struct ElementMap(pub LinkedHashMap<String, LinkedHashSet<String>>);

#[derive(Default, Debug)]
pub struct RustFhirModule {
    pub module_name: String,
    pub resource_name: Option<String>,
    pub structs: Vec<RustFhirStruct>,
    pub enums: Vec<RustFhirEnum>,
    pub doc_comment: String,
}

#[derive(Debug)]
pub struct RustFhirStruct {
    pub struct_name: String,
    pub resource_name: Option<String>,
    pub path: String,
    pub is_primitive: bool,
    pub fields: Vec<RustFhirStructField>,
    pub doc_comment: String,
}

#[derive(Debug, Clone)]
pub struct RustFhirStructField {
    pub name: String,
    pub fhir_name: String,
    pub r#type: RustFhirFieldType,
    pub polymorph: bool,
    pub multiple: bool,
    pub optional: bool,
    pub doc_comment: String,
}

#[derive(Debug)]
pub struct RustFhirEnum {
    pub name: String,
    pub variants: Vec<RustFhirEnumVariant>,
    pub doc_comment: String,
}

#[derive(Debug)]
pub struct RustFhirEnumVariant {
    pub name: String,
    pub r#type: RustFhirFieldType,
}

#[derive(Debug, Clone)]
pub struct RustFhirFieldType {
    pub name: String,
    pub r#box: bool,
    pub contains_primitive: bool,
}
