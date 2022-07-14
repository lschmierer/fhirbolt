mod de;
mod ser;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    gather::{RustEnum, RustModule, RustStruct, RustStructField},
    SourceFile,
};

use self::{de::implement_deserialze, ser::implement_serialze};

pub fn generate_modules(modules: &[RustModule]) -> Vec<SourceFile> {
    modules.iter().map(|m| generate_module(m)).collect()
}

pub fn generate_resource_enum(resource_modules: &[RustModule]) -> SourceFile {
    let variants_tokens = resource_modules.iter().map(|r| {
        let ident = format_ident!("{}", r.resource_name.as_ref().unwrap());
        quote! { #ident(Box<super::#ident>), }
    });

    SourceFile {
        name: "resource".into(),
        source: quote! {
            #[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
            #[serde(tag = "resourceType")]
            pub enum Resource {
                #(
                    #variants_tokens
                )*
                #[default]
                Invalid,
            }
        },
    }
}

pub fn generate_serde_helpers() -> SourceFile {
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
        },
    }
}

fn generate_module(module: &RustModule) -> SourceFile {
    let structs_tokens = module
        .structs
        .iter()
        .map(|s| generate_struct(s, &module.enums));
    let enums_tokens = module.enums.iter().map(|e| generate_enum(e));

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

fn generate_struct(r#struct: &RustStruct, enums: &[RustEnum]) -> TokenStream {
    let name_ident = format_ident!("{}", r#struct.name);
    let fields_tokens = r#struct.fields.iter().map(|f| generate_field(f));

    let serde_impl_tokens = if !r#struct.is_fhir_primitive {
        let serialize_impl_tokens = implement_serialze(&r#struct, enums);
        let deserialize_impl_tokens = implement_deserialze(&r#struct, enums);
        quote! {
            #serialize_impl_tokens
            #deserialize_impl_tokens
        }
    } else {
        quote! {}
    };

    quote! {
        #[derive(Default, Debug, Clone)]
        pub struct #name_ident {
            #(
                #fields_tokens
            )*
        }

        #serde_impl_tokens
    }
}

fn generate_field(field: &RustStructField) -> TokenStream {
    let name_ident = format_ident!("r#{}", field.name);

    let type_tokens = field.r#type.name.parse().unwrap();

    let type_tokens = if field.r#type.r#box {
        quote! { Box<#type_tokens> }
    } else {
        type_tokens
    };

    let type_tokens = if field.multiple {
        quote! { Vec<#type_tokens> }
    } else if field.optional {
        quote! { Option<#type_tokens> }
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

        let type_tokens = v.r#type.name.parse().unwrap();

        let type_tokens = if v.r#type.r#box {
            quote! { Box<#type_tokens> }
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
            Invalid,
        }

        impl Default for #name_ident {
            fn default() -> #name_ident {
                #name_ident::Invalid
            }
        }
    }
}
