mod casing;
mod codegen;
mod ir;
pub mod model;

use codegen::{generate_modules, generate_resource_enum, generate_serde_helpers};
use proc_macro2::TokenStream;

use ir::parse_bundle;
use model::Bundle;

pub struct Generated {
    pub types_source_files: Vec<SourceFile>,
    pub resources_source_files: Vec<SourceFile>,
    pub resource_enum: SourceFile,
    pub serde_helpers: SourceFile,
}
pub struct SourceFile {
    pub name: String,
    pub source: TokenStream,
}

pub fn generate_all<'a>(types_bundle: &'a Bundle, resources_bundle: &'a Bundle) -> Generated {
    let type_modules = parse_bundle(types_bundle);
    let resource_modules = parse_bundle(resources_bundle);

    let types_source_files = generate_modules(&type_modules);
    let resources_source_files = generate_modules(&resource_modules);

    Generated {
        types_source_files,
        resources_source_files,
        resource_enum: generate_resource_enum(&resource_modules),
        serde_helpers: generate_serde_helpers(),
    }
}
