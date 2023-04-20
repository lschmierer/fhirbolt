use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::ir::{RustFhirStruct, RustFhirStructField};

pub fn implement_default(r#struct: &RustFhirStruct) -> TokenStream {
    let name_ident = format_ident!("{}", r#struct.struct_name);
    let fields_tokens = r#struct.fields.iter().map(|f| generate_field(f));

    quote! {
        impl Default for #name_ident {
            fn default() -> Self {
                Self {
                    #(
                        #fields_tokens
                    )*
                }
            }
        }
    }
}

fn generate_field(field: &RustFhirStructField) -> TokenStream {
    let name_ident = format_ident!("r#{}", field.name);

    let default_tokens = if !field.optional
        && !field.polymorph
        && !field.multiple
        && field.r#type.name != "bool"
        && field.r#type.name != "u32"
        && field.r#type.name != "i32"
        && field.r#type.name != "std::string::String"
        && field.r#type.name != "Resource"
    {
        let type_tokens: TokenStream = field.r#type.name.parse().unwrap();
        let type_tokens = if !(field.r#type.name.starts_with("types")
            || field.r#type.name.starts_with("resources"))
        {
            type_tokens
        } else if field.r#type.r#box {
            quote! { Box<super::super::#type_tokens> }
        } else {
            quote! { super::super::#type_tokens }
        };

        quote! {
            {
                let mut default: #type_tokens = Default::default();
                default.id = Some("$invalid".to_string());
                default
            }
        }
    } else {
        quote! { Default::default() }
    };

    quote! {
        #name_ident: #default_tokens,
    }
}
