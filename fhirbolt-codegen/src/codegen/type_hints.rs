use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::TypeHints;

pub fn generate_type_hints(type_hints: &TypeHints) -> TokenStream {
    let type_hints_type_paths_tokens = generate_map(&type_hints.type_paths);
    let type_hints_array_paths_tokens = generate_set(&type_hints.array_paths);
    let type_hints_boolean_paths_tokens = generate_set(&type_hints.boolean_paths);
    let type_hints_integer_paths_tokens = generate_set(&type_hints.integer_paths);
    let type_hints_integer64_paths_tokens = generate_set(&type_hints.integer64_paths);
    let type_hints_unsigned_integer_paths_tokens = generate_set(&type_hints.unsigned_integer_paths);
    let type_hints_positive_integer_paths_tokens = generate_set(&type_hints.positive_integer_paths);
    let type_hints_decimal_paths_tokens = generate_set(&type_hints.decimal_integer_paths);
    let type_hints_all_primitives_paths_tokens = generate_set(&type_hints.all_primitives_paths);
    let content_references_paths_tokens = generate_map(&type_hints.content_reference_paths);

    quote! {
        use phf::{phf_map, phf_set};

        pub static TYPE_HINTS: crate::type_hints::TypeHints = crate::type_hints::TypeHints {
            type_paths: #type_hints_type_paths_tokens,
            array_paths: #type_hints_array_paths_tokens,
            boolean_paths: #type_hints_boolean_paths_tokens,
            integer_paths: #type_hints_integer_paths_tokens,
            integer64_paths: #type_hints_integer64_paths_tokens,
            unsigned_integer_paths: #type_hints_unsigned_integer_paths_tokens,
            positive_integer_paths: #type_hints_positive_integer_paths_tokens,
            decimal_paths: #type_hints_decimal_paths_tokens,
            all_primitives_paths: #type_hints_all_primitives_paths_tokens,
            content_reference_paths: #content_references_paths_tokens
        };
    }
}

fn generate_map(map: &LinkedHashMap<String, String>) -> TokenStream {
    let kv_tokens = map.iter().map(|(k, v)| {
        quote! {
            #k => #v,
        }
    });

    quote! {
        phf_map! {
            #(
                #kv_tokens
            )*
        }
    }
}

fn generate_set(set: &LinkedHashSet<String>) -> TokenStream {
    let k_tokens = set.iter().map(|k| {
        quote! {
            #k,
        }
    });

    quote! {
        phf_set! {
            #(
                #k_tokens
            )*
        }
    }
}
