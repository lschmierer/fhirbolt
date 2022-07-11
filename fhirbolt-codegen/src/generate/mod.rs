use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    gather::{RustEnum, RustModule, RustStruct, RustStructField},
    SourceFile,
};

pub fn generate_modules(modules: &[RustModule]) -> Vec<SourceFile> {
    modules.iter().map(|m| generate_module(m)).collect()
}

pub fn generate_resource_enum(resource_modules: &[RustModule]) -> SourceFile {
    let variants_tokens = resource_modules.iter().map(|r| {
        let ident = format_ident!("{}", r.structs[0].name);
        quote!(#ident(Box<super::#ident>),)
    });

    SourceFile {
        name: "resource".into(),
        source: quote! {
            #[derive(Debug, Clone)]
            pub enum Resource {
                #(
                    #variants_tokens
                )*
            }
        },
    }
}

fn generate_module(module: &RustModule) -> SourceFile {
    let structs_tokens = module.structs.iter().map(|s| generate_struct(s));
    let enums_tokens = module.enums.iter().map(|s| generate_enum(s));

    SourceFile {
        name: module.name.clone(),
        source: quote! {
            #(
                #enums_tokens
            )*
            #(
                #structs_tokens
            )*
        },
    }
}

fn generate_struct(r#struct: &RustStruct) -> TokenStream {
    let name_ident = format_ident!("{}", r#struct.name);
    let fields_tokens = r#struct.fields.iter().map(|f| generate_field(f));

    quote! {
        #[derive(Debug, Clone)]
        pub struct #name_ident {
            #(
                #fields_tokens
            )*
        }
    }
}

fn generate_field(field: &RustStructField) -> TokenStream {
    let name_ident = format_ident!("r#{}", field.name);

    let type_tokens = field.r#type.parse().unwrap();

    let type_tokens = if field.r#box {
        quote!(Box<#type_tokens>)
    } else {
        type_tokens
    };

    let type_tokens = if field.multiple {
        quote!(Vec<#type_tokens>)
    } else if field.optional {
        quote!(Option<#type_tokens>)
    } else {
        type_tokens
    };

    quote! {
        pub #name_ident: #type_tokens,
    }
}

fn generate_enum(r#enum: &RustEnum) -> TokenStream {
    let name_ident = format_ident!("{}", r#enum.name);

    let variants_tokens = r#enum.variants.iter().map(|v| {
        // type like http://hl7.org/fhirpath/System.String
        let variant_name_ident = format_ident!("{}", v.name);

        let type_tokens = v.r#type.parse().unwrap();

        let type_tokens = if v.r#box {
            quote!(Box<#type_tokens>)
        } else {
            type_tokens
        };

        quote! {
            #variant_name_ident(#type_tokens),
        }
    });

    quote! {
        #[derive(Debug, Clone)]
        pub enum #name_ident {
            #(
                #variants_tokens
            )*
        }
    }
}
