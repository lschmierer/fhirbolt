mod casing;
pub mod model;

use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use casing::RustCasing;
use model::{Bundle, ElementDefinition, Resource, StructureDefinition};

const PRIMITIVES: &[&str] = &[
    "base64Binary",
    "boolean",
    "canonical",
    "code",
    "date",
    "dateTime",
    "decimal",
    "id",
    "instant",
    "integer",
    "markdown",
    "oid",
    "positiveInt",
    "string",
    "time",
    "unsignedInt",
    "uri",
    "url",
    "uuid",
    "xhtml",
];

fn flatten_bundle<'a>(bundle: &'a Bundle) -> Vec<&'a StructureDefinition> {
    bundle
        .entry
        .iter()
        .filter_map(|e| match &e.resource {
            Resource::StructureDefinition(s) => Some(s),
            _ => None,
        })
        .collect()
}

pub struct SourceFile {
    pub name: String,
    pub source: TokenStream,
}

#[derive(Default)]
struct Context {
    pub types: Vec<TokenStream>,
}

pub fn generate<'a>(bundle: &'a Bundle) -> Vec<SourceFile> {
    let structure_definitions = flatten_bundle(bundle);

    let resources = structure_definitions
        .iter()
        .filter(|s| s.kind == "resource")
        .filter(|s| s.name != "Resource")
        .map(|s| s.name.as_ref())
        .collect::<Vec<_>>();

    let mut source_files = if resources.is_empty() {
        Vec::with_capacity(structure_definitions.len())
    } else {
        let mut source_files = Vec::with_capacity(structure_definitions.len() + 1);
        source_files.push(generate_resource_enum(&resources));
        source_files
    };

    for structure_definition in structure_definitions {
        if structure_definition.name == "Resource" {
            continue;
        }

        let mut context = Context::default();

        generate_type(
            &mut context,
            &structure_definition.name,
            &structure_definition
                .snapshot
                .element
                .iter()
                .collect::<Vec<_>>(),
            &structure_definition.r#type,
        );

        let types = context.types;

        source_files.push(SourceFile {
            name: structure_definition.name.to_rust_mod_casing(),
            source: quote! {
                #(
                    #types
                )*
            },
        });
    }

    source_files
}

fn generate_resource_enum(resources: &[&str]) -> SourceFile {
    let variants = resources.iter().map(|r| {
        let ident = format_ident!("{}", r);
        quote!(#ident(Box<super::#ident>),)
    });

    SourceFile {
        name: "resource".into(),
        source: quote! {
            #[derive(Debug, Clone)]
            pub enum Resource {
                #(
                    #variants
                )*
            }
        },
    }
}

fn generate_type(
    context: &mut Context,
    name: &str,
    element_definitions: &[&ElementDefinition],
    path_strip_prefix: &str,
) {
    let type_ident = format_ident!("{}", name.to_rust_type_casing());

    let fields = generate_fields(context, name, element_definitions, path_strip_prefix);

    context.types.push(quote! {
        #[derive(Debug, Clone)]
        pub struct #type_ident {
            #fields
        }
    });
}

fn generate_fields(
    context: &mut Context,
    struct_name: &str,
    element_definitions: &[&ElementDefinition],
    path_strip_prefix: &str,
) -> TokenStream {
    let mut element_definition_groups = HashMap::<_, Vec<_>>::new();

    for element_definition in element_definitions
        .iter()
        .filter(|e| e.path != path_strip_prefix)
    {
        let stripped_path = element_definition.path[path_strip_prefix.len() + 1..]
            .split(".")
            .next()
            .unwrap();

        element_definition_groups
            .entry(stripped_path)
            .or_default()
            .push(*element_definition);
    }

    let fields = element_definition_groups
        .iter()
        .map(|(field_name, element_definitions)| {
            let type_name = format!("{}{}", struct_name, field_name.to_rust_type_casing());

            if element_definitions.len() > 1 {
                generate_type(
                    context,
                    &type_name,
                    &element_definitions,
                    &element_definitions[0].path,
                );
            }

            generate_field(
                context,
                struct_name,
                &element_definitions[0],
                path_strip_prefix,
            )
        });
    quote! {
        #(
            #fields
        )*
    }
}

fn generate_field<'a>(
    context: &mut Context,
    struct_name: &str,
    element_definition: &ElementDefinition,
    path_strip_prefix: &str,
) -> TokenStream {
    let name = &element_definition.path[path_strip_prefix.len() + 1..];

    let (name, polymorph) = {
        if name.ends_with("[x]") {
            (&name[..name.len() - 3], true)
        } else {
            (name, false)
        }
    };

    let name_ident = format_ident!("r#{}", name.to_rust_identifier_casing());

    let type_tokens = if polymorph {
        let type_ident = format_ident!("{}{}", struct_name, name.to_rust_type_casing());

        let types = element_definition
            .r#type
            .as_ref()
            .unwrap()
            .iter()
            .map(|t| t.code.as_ref());

        context.types.push(generate_value_enum(&type_ident, types));

        quote!(#type_ident)
    } else {
        if let Some(types) = element_definition.r#type.as_ref() {
            let code = &types.first().unwrap().code;
            if code == "BackboneElement" {
                let i = format_ident!("{}{}", struct_name, name.to_rust_type_casing());
                quote!(#i)
            } else {
                match_field_type(code, false)
            }
        } else {
            map_content_reference(element_definition.content_reference.as_ref().unwrap())
        }
    };

    let type_tokens = if element_definition.max.as_ref().unwrap() != "1" {
        quote!(Vec<#type_tokens>)
    } else if element_definition.min.unwrap() == 0 {
        quote!(Option<#type_tokens>)
    } else {
        type_tokens
    };

    quote! {
        pub #name_ident: #type_tokens,
    }
}

fn generate_value_enum<'a>(
    type_ident: &Ident,
    types: impl Iterator<Item = &'a str>,
) -> TokenStream {
    let types_tokens = types.map(|r#type| {
        // type like http://hl7.org/fhirpath/System.String
        let case_ident = format_ident!(
            "{}",
            r#type.rsplit(".").next().unwrap().to_rust_type_casing()
        );
        let type_tokens = match_field_type(r#type, true);
        quote! {
            #case_ident(#type_tokens),
        }
    });

    quote! {
        #[derive(Debug, Clone)]
        pub enum #type_ident {
            #(
                #types_tokens
            )*
        }
    }
}

fn match_field_type<S: AsRef<str>>(code: S, force_box: bool) -> TokenStream {
    // type like http://hl7.org/fhirpath/System.String
    match code.as_ref().rsplit("/").next().unwrap() {
        "System.Boolean" => quote!(bool),
        "System.Integer" => quote!(u32),
        "System.String" => quote!(std::string::String),
        "System.Decimal" => quote!(rust_decimal::Decimal),
        "System.Date" => quote!(chrono::naive::NaiveDate),
        "System.DateTime" => quote!(chrono::DateTime<chrono::Utc>),
        "System.Time" => quote!(chrono::naive::NaiveTime),
        "Resource" => quote!(Box<super::Resource>),
        r#type => {
            let ident = format_ident!("{}", r#type.to_rust_type_casing());
            if !PRIMITIVES.contains(&r#type) || force_box {
                quote!(Box<super::super::types::#ident>)
            } else {
                quote!(super::super::types::#ident)
            }
        }
    }
}

fn map_content_reference(content_reference: &str) -> TokenStream {
    let i = format_ident!(
        "{}",
        content_reference[1..]
            .split(".")
            .map(|s| s.to_rust_type_casing())
            .collect::<String>(),
    );
    quote!(#i)
}
