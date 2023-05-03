use linked_hash_set::LinkedHashSet;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::ir::ElementMap;

pub fn generate_element_map(element_map: &ElementMap) -> TokenStream {
    let map_entry_idents = element_map.0.iter().map(|(k, _v)| {
        let entry_ident = format_ident!("{}", entry_name(k));

        quote! {
            #k => &#entry_ident,
        }
    });

    let map_entries = element_map.0.iter().map(|(k, v)| generate_map_entry(k, v));

    quote! {
        use phf::{phf_map, phf_ordered_set};

        pub static ELEMENT_MAP: phf::Map<&str, &phf::OrderedSet<&str>> = phf_map! {
            #(
                #map_entry_idents
            )*
        };

        #(
            #map_entries
        )*
    }
}

fn generate_map_entry(key: &str, set: &LinkedHashSet<String>) -> TokenStream {
    let entry_ident = format_ident!("{}", entry_name(key));

    let entries = set.iter().map(|v| {
        quote! {
            #v,
        }
    });

    quote! {
        pub static #entry_ident: phf::OrderedSet<&'static str> = phf_ordered_set! {
            #(
                #entries
            )*
        };
    }
}

fn entry_name(path: &str) -> String {
    path.to_uppercase().replace('.', "_")
}
