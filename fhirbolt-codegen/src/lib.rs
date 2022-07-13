mod casing;
mod gather;
mod generate;
pub mod model;

use generate::{generate_modules, generate_resource_enum, generate_serde_helpers};
use proc_macro2::TokenStream;

use gather::gather_all_modules;
use model::Bundle;

pub struct Generated {
    pub types_source_files: Vec<SourceFile>,
    pub resources_source_files: Vec<SourceFile>,
    pub serde_helpers: SourceFile
}
pub struct SourceFile {
    pub name: String,
    pub source: TokenStream,
}

pub fn generate_all<'a>(types_bundle: &'a Bundle, resources_bundle: &'a Bundle) -> Generated {
    let type_modules = gather_all_modules(types_bundle);
    let resource_modules = gather_all_modules(resources_bundle);

    let types_source_files = generate_modules(&type_modules);
    let mut resources_source_files = generate_modules(&resource_modules);

    resources_source_files.push(generate_resource_enum(&resource_modules));

    Generated {
        types_source_files,
        resources_source_files,
        serde_helpers: generate_serde_helpers()
    }
}
