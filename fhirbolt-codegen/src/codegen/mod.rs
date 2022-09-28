mod de;
mod ser;

use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::{Captures, Regex};

use crate::{
    casing::RustCasing,
    ir::{RustFhirEnum, RustFhirModule, RustFhirStruct, RustFhirStructField},
    SourceFile,
};

use self::{de::implement_deserialze, ser::implement_serialze};

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

pub fn generate_modules(modules: &[RustFhirModule]) -> Vec<SourceFile> {
    modules.iter().map(|m| generate_module(m)).collect()
}

pub fn generate_resource_enum(resource_modules: &[RustFhirModule]) -> SourceFile {
    let variants_tokens = resource_modules.iter().map(|r| {
        let ident = format_ident!("{}", r.resource_name.as_ref().unwrap());
        let doc_comment = format_doc_comment(&r.doc_comment);

        quote! {
            #[doc=#doc_comment]
            #ident(Box<super::resources::#ident>),
        }
    });

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

            impl crate::model::ResourceOrElement for Resource {}
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

fn generate_module(module: &RustFhirModule) -> SourceFile {
    let structs_tokens = module
        .structs
        .iter()
        .map(|s| generate_struct(s, &module.enums));
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

fn generate_struct(r#struct: &RustFhirStruct, enums: &[RustFhirEnum]) -> TokenStream {
    let name_ident = format_ident!("{}", r#struct.struct_name);
    let fields_tokens = r#struct.fields.iter().map(|f| generate_field(f));

    let serde_impl_tokens = if !r#struct.is_primitive {
        let serialize_impl_tokens = implement_serialze(&r#struct, enums);
        let deserialize_impl_tokens = implement_deserialze(&r#struct, enums);
        quote! {
            #serialize_impl_tokens
            #deserialize_impl_tokens
        }
    } else {
        quote! {}
    };

    let doc_comment = format_doc_comment(&r#struct.doc_comment);

    quote! {
        #[doc=#doc_comment]
        #[derive(Default, Debug, Clone)]
        pub struct #name_ident {
            #(
                #fields_tokens
            )*
        }

        impl crate::model::ResourceOrElement for #name_ident {}

        #serde_impl_tokens
    }
}

fn generate_field(field: &RustFhirStructField) -> TokenStream {
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

    quote! {
        #[doc=#doc_comment]
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

        impl crate::model::ResourceOrElement for #name_ident {}

        impl Default for #name_ident {
            fn default() -> #name_ident {
                #name_ident::Invalid
            }
        }
    }
}
