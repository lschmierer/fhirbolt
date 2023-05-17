use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::ir::RustFhirStruct;

pub fn implement_primitive_from(r#struct: &RustFhirStruct) -> TokenStream {
    let name_ident = format_ident!("{}", r#struct.struct_name);

    let value_type_name = &r#struct
        .fields
        .iter()
        .find(|f| f.name == "value")
        .unwrap()
        .r#type
        .name;

    match value_type_name.as_str() {
        "bool" | "u32" | "i32" | "i64" => from(&r#struct.struct_name, value_type_name),
        _ if r#struct.struct_name == "Xhtml" => quote! {
            impl<I: Into<std::string::String>> From<I> for #name_ident {
                fn from(v: I) -> Self {
                    #name_ident {
                        value: v.into(),
                        ..Default::default()
                    }
                }
            }
        },
        _ => quote! {
            impl<I: Into<std::string::String>> From<I> for #name_ident {
                fn from(v: I) -> Self {
                    #name_ident {
                        value: Some(v.into()),
                        ..Default::default()
                    }
                }
            }
        },
    }
}

fn from(name: &str, into: &str) -> TokenStream {
    let name_ident = format_ident!("{}", name);
    let into_ident = format_ident!("{}", into);

    quote! {
        impl<I: Into<#into_ident>> From<I> for #name_ident {
            fn from(v: I) -> Self {
                #name_ident {
                    value: Some(v.into()),
                    ..Default::default()
                }
            }
        }
    }
}
