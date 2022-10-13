use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{codegen::SourceFile, ir::TypeHints};

pub fn generate_serde_helpers(type_hints: &TypeHints) -> SourceFile {
    let type_hints_type_paths_tokens = generate_type_hints_type_paths(&type_hints.type_paths);
    let type_hints_array_paths_tokens = generate_type_hints_paths_for(&type_hints.array_paths);
    let type_hints_boolean_paths_tokens = generate_type_hints_paths_for(&type_hints.boolean_paths);
    let type_hints_integer_paths_tokens = generate_type_hints_paths_for(&type_hints.integer_paths);
    let type_hints_unsigned_integer_paths_tokens =
        generate_type_hints_paths_for(&type_hints.unsigned_integer_paths);
    let type_hints_positive_integer_paths_tokens =
        generate_type_hints_paths_for(&type_hints.positive_integer_paths);
    let type_hints_decimal_paths_tokens =
        generate_type_hints_paths_for(&type_hints.decimal_integer_paths);
    let type_hints_other_primitives_paths_tokens =
        generate_type_hints_paths_for(&type_hints.other_primitives_paths);
    let content_references_paths_tokens =
        generate_type_hints_content_reference_paths(&type_hints.content_reference_paths);

    SourceFile {
        name: "serde_helpers".into(),
        source: quote! {
            #[derive(serde::Serialize)]
            pub struct PrimitiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                pub extension: &'a [Box<super::types::Extension>],
            }

            #[derive(serde::Deserialize)]
            pub struct PrimitiveElementOwned {
                pub id: Option<std::string::String>,
                pub extension: Vec<Box<super::types::Extension>>,
            }

            lazy_static::lazy_static! {
                pub(crate) static ref TYPE_HINTS: crate::model::TypeHints = crate::model::TypeHints {
                    type_paths: #type_hints_type_paths_tokens,
                    array_paths: #type_hints_array_paths_tokens,
                    integer_paths: #type_hints_integer_paths_tokens,
                    boolean_paths: #type_hints_boolean_paths_tokens,
                    unsigned_integer_paths: #type_hints_unsigned_integer_paths_tokens,
                    positive_integer_paths: #type_hints_positive_integer_paths_tokens,
                    decimal_paths: #type_hints_decimal_paths_tokens,
                    other_primitives_paths: #type_hints_other_primitives_paths_tokens,
                    content_reference_paths: #content_references_paths_tokens
                };
            }
        },
    }
}

fn generate_type_hints_type_paths(type_paths: &LinkedHashMap<String, String>) -> TokenStream {
    let inserts_tokens = type_paths.iter().map(|(path, ty)| {
        quote! {
            map.insert(#path, #ty);
        }
    });

    quote! {
        {
            let mut map = std::collections::HashMap::<&str, &str>::new();
            #(
                #inserts_tokens
            )*
            map
        }
    }
}

fn generate_type_hints_paths_for(paths: &LinkedHashSet<String>) -> TokenStream {
    let inserts_tokens = paths.iter().map(|path| {
        quote! {
            set.insert(#path);
        }
    });

    quote! {
        {
            let mut set = std::collections::HashSet::<&str>::new();
            #(
                #inserts_tokens
            )*
            set
        }
    }
}

fn generate_type_hints_content_reference_paths(
    type_paths: &LinkedHashMap<String, String>,
) -> TokenStream {
    let inserts_tokens = type_paths.iter().map(|(path, content_ref)| {
        quote! {
            map.insert(#path, #content_ref);
        }
    });

    quote! {
        {
            let mut map = std::collections::HashMap::<&str, &str>::new();
            #(
                #inserts_tokens
            )*
            map
        }
    }
}
