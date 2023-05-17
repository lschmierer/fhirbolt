mod de;
mod default;
mod element_map;
mod from;
mod ser;
mod serde_helpers;
mod type_hints;

use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::{Captures, Regex};

use crate::{
    casing::RustCasing,
    codegen::{
        de::implement_deserialze_resource_enum, default::implement_default,
        from::implement_primitive_from,
    },
    ir::{RustFhirEnum, RustFhirModule, RustFhirStruct, RustFhirStructField},
    SourceFile,
};

use self::{
    de::implement_deserialze,
    ser::{implement_serialize, implement_serialize_resource_enum},
};
pub use self::{
    element_map::generate_element_map, serde_helpers::generate_serde_helpers,
    type_hints::generate_type_hints,
};

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
        if all.ends_with('(') {
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

pub fn generate_struct_modules(modules: &[RustFhirModule]) -> Vec<SourceFile> {
    modules.iter().map(generate_struct_module).collect()
}

pub fn generate_resource_enum(resource_modules: &[RustFhirModule], release: &str) -> SourceFile {
    let release_ident_module = format_ident!("{}", release.to_string().to_lowercase());

    let variants_tokens = resource_modules.iter().map(|r| {
        let ident = format_ident!("{}", r.resource_name.as_ref().unwrap());
        let doc_comment = format_doc_comment(&r.doc_comment);

        quote! {
            #[doc=#doc_comment]
            #ident(Box<super::resources::#ident>),
        }
    });

    let (match_id, match_meta): (Vec<_>, Vec<_>) = resource_modules
        .iter()
        .map(|r| {
            let ident = format_ident!("{}", r.resource_name.as_ref().unwrap());

            (
                quote! {
                    Resource::#ident(r) => {
                        r.id.as_ref()
                    },
                },
                quote! {
                    Resource::#ident(r) => {
                        r.meta.as_deref()
                    },
                },
            )
        })
        .unzip();

    SourceFile {
        name: "resource".into(),
        source: quote! {
            #[doc="Enum representing all possible FHIR resources."]
            #[derive(Default, Debug, Clone, PartialEq)]
            pub enum Resource {
                #(
                    #variants_tokens
                )*
                #[default]
                Invalid,
            }

            impl Resource {
                pub fn id(&self) -> Option<&crate::#release_ident_module::types::Id> {
                    match self {
                        #(
                            #match_id
                        )*
                        _ => None,
                    }
                }

                pub fn meta(&self) -> Option<&crate::#release_ident_module::types::Meta> {
                    match self {
                        #(
                            #match_meta
                        )*
                        _ => None,
                    }
                }
            }
        },
    }
}

pub fn generate_resource_enum_serde(
    resource_modules: &[RustFhirModule],
    release: &str,
) -> SourceFile {
    let release_ident = format_ident!("{}", release.to_string());
    let release_ident_module = format_ident!("{}", release.to_string().to_lowercase());

    let namespace = quote! {
            fhirbolt_model::#release_ident_module
    };

    let serialize_impl_tokens = implement_serialize_resource_enum(resource_modules);
    let deserialize_impl_tokens =
        implement_deserialze_resource_enum(resource_modules, &namespace, release);

    SourceFile {
        name: "resource".into(),
        source: quote! {
            use crate::{DeserializationContext, SerializationContext};

            use #namespace::Resource;

            impl crate::Resource for Resource {
                const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::#release_ident;
            }

            #serialize_impl_tokens
            #deserialize_impl_tokens
        },
    }
}

fn generate_struct_module(module: &RustFhirModule) -> SourceFile {
    let structs_tokens = module.structs.iter().map(generate_struct);
    let enums_tokens = module.enums.iter().map(generate_enum);

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

fn generate_struct(r#struct: &RustFhirStruct) -> TokenStream {
    let name_ident = format_ident!("{}", r#struct.struct_name);
    let fields_tokens = r#struct.fields.iter().map(generate_field);

    let doc_comment = format_doc_comment(&r#struct.doc_comment);

    let default_impl = implement_default(r#struct);

    let from_impl = if r#struct.is_primitive {
        implement_primitive_from(r#struct)
    } else {
        quote! {}
    };

    quote! {
        #[doc=#doc_comment]
        #[derive(Debug, Clone, PartialEq)]
        pub struct #name_ident {
            #(
                #fields_tokens
            )*
        }

        #default_impl

        #from_impl
    }
}

fn generate_field(field: &RustFhirStructField) -> TokenStream {
    let name_ident = format_ident!("r#{}", field.name);

    let type_tokens: TokenStream = field.r#type.name.parse().unwrap();

    let type_tokens = if field.r#type.name == "bool"
        || field.r#type.name == "u32"
        || field.r#type.name == "i32"
        || field.r#type.name == "std::string::String"
        || !(field.r#type.name.starts_with("types")
            || field.r#type.name.starts_with("resources")
            || field.r#type.name == "Resource")
    {
        type_tokens
    } else if field.r#type.r#box && !field.multiple {
        quote! { Box<super::super::#type_tokens> }
    } else {
        quote! { super::super::#type_tokens }
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

        let type_tokens: TokenStream = v.r#type.name.parse().unwrap();

        let type_tokens =
            if v.r#type.name.starts_with("types") || v.r#type.name.starts_with("resources") {
                if v.r#type.r#box {
                    quote! { Box<super::super::#type_tokens> }
                } else {
                    quote! { super::super::#type_tokens }
                }
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
        #[derive(Default, Debug, Clone, PartialEq)]
        pub enum #name_ident {
            #(
                #variants_tokens
            )*
            #[default]
            Invalid,
        }
    }
}

pub fn generate_serde_modules(modules: &[RustFhirModule], release: &str) -> Vec<SourceFile> {
    modules
        .iter()
        .map(|m| generate_serde_module(m, release))
        .collect()
}

fn generate_serde_module(module: &RustFhirModule, release: &str) -> SourceFile {
    let release_ident = format_ident!("{}", release.to_string().to_lowercase());
    let base_namespace = quote! {
        fhirbolt_model::#release_ident
    };
    let namespace = if module.resource_name.is_some() {
        quote! {
            #base_namespace::resources
        }
    } else {
        quote! {
            #base_namespace::types
        }
    };

    let serde_tokens = module.structs.iter().map(|s| {
        let name_ident = format_ident!("{}", s.struct_name);
        let impl_resource_tokens = if s.resource_name.is_some() {
            let release_ident = format_ident!("{}", release.to_string());

            quote! {
                impl crate::Resource for #name_ident {
                    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::#release_ident;
                }
            }
        } else {
            quote! {}
        };

        let serialize_impl_tokens = implement_serialize(s, &module.enums, &namespace);
        let deserialize_impl_tokens =
            implement_deserialze(s, &module.enums, &namespace, &base_namespace);

        quote! {
            use #namespace::#name_ident;

            #impl_resource_tokens

            #serialize_impl_tokens
            #deserialize_impl_tokens
        }
    });

    SourceFile {
        name: module.module_name.clone(),
        source: quote! {
            use crate::{DeserializationContext, SerializationContext};

            #(
                #serde_tokens
            )*
        },
    }
}
