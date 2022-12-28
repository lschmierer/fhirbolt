mod de;
mod ser;
mod serde_helpers;
mod type_hints;

use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::{Captures, Regex};

use fhirbolt_shared::FhirRelease;

use crate::{
    casing::RustCasing,
    ir::{RustFhirEnum, RustFhirModule, RustFhirStruct, RustFhirStructField},
    SourceFile,
};

use self::{de::implement_deserialze, ser::implement_serialize};
pub use self::{serde_helpers::generate_serde_helpers, type_hints::generate_type_hints};

lazy_static! {
    static ref URL_REGEX: Regex = Regex::new(r"http[\w./:]*[\w/:]").unwrap();
    static ref MARKDOWN_LINK_HTML_REGEX: Regex =
        Regex::new(r"(\[\[?[\w /$]+?\]\]?)\(([\w\- ]+?\.html\))").unwrap();
    static ref MARKDOWN_LINK_INLINE_REGEX: Regex = Regex::new(r"\[\[?\[?(\w+)\]\]?\]?.?").unwrap();
}

fn format_doc_comment(text: &str) -> String {
    let text = text.replace("[x]", r"\[x\]");
    let text = URL_REGEX.replace_all(&text, "<$0>");
    let text = MARKDOWN_LINK_HTML_REGEX.replace_all(&text, "$1(https://hl7.org/FHIR/$2)");
    let text = MARKDOWN_LINK_INLINE_REGEX.replace_all(&text, |caps: &Captures| {
        let all = caps.get(0).unwrap().as_str();
        if all.ends_with("(") {
            all.into()
        } else {
            let last = all.chars().last().unwrap();
            let last = match last {
                ' ' | ',' | '.' => last.to_string(),
                _ => "".into(),
            };

            if all.ends_with("]]] ") {
                format!("`{}`{}", caps.get(1).unwrap().as_str(), last)
            } else {
                format!(
                    "`{}`{}",
                    caps.get(1).unwrap().as_str().to_rust_identifier_casing(),
                    last
                )
            }
        }
    });
    text.to_string()
}

pub fn generate_modules(modules: &[RustFhirModule], release: FhirRelease) -> Vec<SourceFile> {
    modules
        .iter()
        .map(|m| generate_module(m, release))
        .collect()
}

pub fn generate_resource_enum(
    resource_modules: &[RustFhirModule],
    release: FhirRelease,
) -> SourceFile {
    let variants_tokens = resource_modules.iter().map(|r| {
        let ident = format_ident!("{}", r.resource_name.as_ref().unwrap());
        let doc_comment = format_doc_comment(&r.doc_comment);

        quote! {
            #[doc=#doc_comment]
            #ident(Box<super::resources::#ident>),
        }
    });

    let release_ident = format_ident!("{}", release.to_string());

    SourceFile {
        name: "resource".into(),
        source: quote! {
            #[doc="Enum representing all possible FHIR resources."]
            #[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
            #[serde(tag = "resourceType")]
            pub enum Resource {
                #(
                    #variants_tokens
                )*
                #[default]
                Invalid,
            }

            impl crate::AnyResource for Resource {
                fn fhir_release() -> crate::FhirRelease {
                    crate::FhirRelease::#release_ident
                }
            }
        },
    }
}

fn generate_module(module: &RustFhirModule, release: FhirRelease) -> SourceFile {
    let structs_tokens = module
        .structs
        .iter()
        .map(|s| generate_struct(s, &module.enums, release));
    let enums_tokens = module.enums.iter().map(|e| generate_enum(e));

    SourceFile {
        name: module.module_name.clone(),
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

fn generate_struct(
    r#struct: &RustFhirStruct,
    enums: &[RustFhirEnum],
    release: FhirRelease,
) -> TokenStream {
    let name_ident = format_ident!("{}", r#struct.struct_name);
    let fields_tokens = r#struct
        .fields
        .iter()
        .map(|f| generate_field(f, r#struct.is_primitive, r#struct.struct_name == "Xhtml"));

    let impl_any_resource_tokens = if r#struct.resource_name.is_some() {
        let release_ident = format_ident!("{}", release.to_string());

        quote! {
            impl crate::AnyResource for #name_ident {
                fn fhir_release() -> crate::FhirRelease {
                    crate::FhirRelease::#release_ident
                }
            }
        }
    } else {
        quote! {}
    };

    let serde_impl_tokens = if !r#struct.is_primitive {
        let serialize_impl_tokens = implement_serialize(&r#struct, enums);
        let deserialize_impl_tokens = implement_deserialze(&r#struct, enums);
        quote! {
            #serialize_impl_tokens
            #deserialize_impl_tokens
        }
    } else {
        quote! {}
    };

    let doc_comment = format_doc_comment(&r#struct.doc_comment);

    let derive_serialize_tokens = if r#struct.is_primitive {
        quote! {, serde::Serialize, serde::Deserialize}
    } else {
        quote! {}
    };

    quote! {
        #[doc=#doc_comment]
        #[derive(Default, Debug, Clone #derive_serialize_tokens)]
        pub struct #name_ident {
            #(
                #fields_tokens
            )*
        }

        #impl_any_resource_tokens

        #serde_impl_tokens
    }
}

fn generate_field(field: &RustFhirStructField, is_primitive: bool, is_xhtml: bool) -> TokenStream {
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

    let doc_comment = format_doc_comment(&field.doc_comment);

    let serde_attribute_tokens = match (is_primitive, is_xhtml, field.name.as_ref()) {
        (true, _, "id") => {
            quote! { #[serde(skip_serializing_if = "Option::is_none")] }
        }
        (true, false, "value") => {
            quote! { #[serde(skip_serializing_if = "Option::is_none")] }
        }
        (true, _, "extension") => {
            quote! { #[serde(default, skip_serializing_if = "Vec::is_empty")] }
        }
        _ => quote! {},
    };

    quote! {
        #[doc=#doc_comment]
        #serde_attribute_tokens
        pub #name_ident: #type_tokens,
    }
}

fn generate_enum(r#enum: &RustFhirEnum) -> TokenStream {
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

    let doc_comment_tokens = format_doc_comment(&r#enum.doc_comment);

    quote! {
        #[doc=#doc_comment_tokens]
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
