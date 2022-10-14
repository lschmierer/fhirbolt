mod casing;
mod codegen;
mod ir;
pub mod model;

use fhirbolt_shared::FhirRelease;

use codegen::{
    generate_modules, generate_resource_enum, generate_serde_helpers, generate_type_hints,
};
use proc_macro2::TokenStream;

use ir::parse_modules;
use model::Bundle;

pub struct Generated {
    pub types_source_files: Vec<SourceFile>,
    pub resources_source_files: Vec<SourceFile>,
    pub resource_enum: SourceFile,
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
    release: FhirRelease,
) -> Generated {
    let mut type_hints = Default::default();
    let type_modules = parse_modules(types_bundle, &mut type_hints);
    let resource_modules = parse_modules(resources_bundle, &mut type_hints);

    let types_source_files = generate_modules(&type_modules, release);
    let resources_source_files = generate_modules(&resource_modules, release);

    Generated {
        types_source_files,
        resources_source_files,
        resource_enum: generate_resource_enum(&resource_modules, release),
        serde_helpers: generate_serde_helpers(),
        type_hints: generate_type_hints(&type_hints),
    }
}
