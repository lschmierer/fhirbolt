mod parse;

pub use parse::parse_bundle;

#[derive(Default)]
pub struct RustFhirModule {
    pub module_name: String,
    pub resource_name: Option<String>,
    pub structs: Vec<RustFhirStruct>,
    pub enums: Vec<RustFhirEnum>,
    pub doc_comment: String,
}

pub struct RustFhirStruct {
    pub struct_name: String,
    pub resource_name: Option<String>,
    pub is_primitive: bool,
    pub fields: Vec<RustFhirStructField>,
    pub doc_comment: String,
}

pub struct RustFhirStructField {
    pub name: String,
    pub fhir_name: String,
    pub r#type: RustFhirFieldType,
    pub polymorph: bool,
    pub multiple: bool,
    pub optional: bool,
    pub doc_comment: String,
}

pub struct RustFhirEnum {
    pub name: String,
    pub variants: Vec<RustFhirEnumVariant>,
    pub doc_comment: String,
}

pub struct RustFhirEnumVariant {
    pub name: String,
    pub r#type: RustFhirFieldType,
}

pub struct RustFhirFieldType {
    pub name: String,
    pub r#box: bool,
    pub contains_primitive: bool,
}
