mod casing;
mod codegen;
mod ir;
pub mod model;

use codegen::{
    generate_resource_enum, generate_resource_enum_serde, generate_serde_helpers,
    generate_serde_modules, generate_struct_modules, generate_type_hints,
};
use proc_macro2::TokenStream;

use ir::parse_modules;
use model::Bundle;

pub struct Generated {
    pub types_struct_source_files: Vec<SourceFile>,
    pub resources_struct_source_files: Vec<SourceFile>,
    pub resource_enum_source_file: SourceFile,
    pub types_serde_source_files: Vec<SourceFile>,
    pub resources_serde_source_files: Vec<SourceFile>,
    pub resource_enum_serde_source_file: SourceFile,
    pub serde_helpers: SourceFile,
    pub type_hints: TokenStream,
}
pub struct SourceFile {
    pub name: String,
    pub source: TokenStream,
}

pub fn generate_all<'a>(
    types_bundle: &'a Bundle,
    resources_bundle: &'a Bundle,
    release: &str,
) -> Generated {
    let mut type_hints = Default::default();

    let type_modules = parse_modules(types_bundle, &mut type_hints);
    let resource_modules = parse_modules(resources_bundle, &mut type_hints);

    Generated {
        types_struct_source_files: generate_struct_modules(&type_modules, release),
        resources_struct_source_files: generate_struct_modules(&resource_modules, release),
        resource_enum_source_file: generate_resource_enum(&resource_modules, release),
        types_serde_source_files: generate_serde_modules(&type_modules, release),
        resources_serde_source_files: generate_serde_modules(&resource_modules, release),
        resource_enum_serde_source_file: generate_resource_enum_serde(&resource_modules, release),
        serde_helpers: generate_serde_helpers(release),
        type_hints: generate_type_hints(&type_hints),
    }
}
