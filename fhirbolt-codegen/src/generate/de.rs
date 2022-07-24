use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    casing::RustCasing,
    gather::{RustEnum, RustEnumVariant, RustStruct, RustStructField},
};

pub fn implement_deserialze(r#struct: &RustStruct, enums: &[RustEnum]) -> TokenStream {
    let fhir_name = &r#struct.fhir_name;
    let struct_name = &r#struct.name;
    let stuct_name_ident = format_ident!("{}", struct_name);

    let (resource_type_field_enum_variant_tokens, deserialize_resource_type_field_tokens) =
        if r#struct.is_resource {
            (
                quote! {
                    #[serde(rename="resourceType")]
                            ResourceType,
                },
                quote! {
                Field::ResourceType => {
                    let value: std::borrow::Cow<str>  = map_access.next_value()?;
                    if value != #fhir_name {
                        return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value), &#fhir_name))
                    }
                }},
            )
        } else {
            (quote! {}, quote! {})
        };

    let field_enum_variants_tokens = r#struct
        .fields
        .iter()
        .map(|f| field_enum_variant(f, enums))
        .flatten();

    let field_mut_vars_tokens = r#struct.fields.iter().map(|f| field_mut_var(f));
    let field_struct_assign_vars_tokens =
        r#struct.fields.iter().map(|f| field_struct_assign_var(f));

    let deserialize_fields_tokens = r#struct.fields.iter().map(|f| deserialize_field(f, enums));

    quote! {
        impl<'de> serde::de::Deserialize<'de> for #stuct_name_ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                #[derive(serde::Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #resource_type_field_enum_variant_tokens
                    #(
                        #field_enum_variants_tokens
                    )*
                }

                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = #stuct_name_ident;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(#struct_name)
                    }

                    fn visit_map<V>(self, mut map_access: V) -> Result<#stuct_name_ident, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        #(
                            #field_mut_vars_tokens
                        )*

                        while let Some(map_access_key) = map_access.next_key()? {
                            match map_access_key {
                                #deserialize_resource_type_field_tokens
                                #(
                                    #deserialize_fields_tokens
                                )*
                            }
                        }

                        Ok(#stuct_name_ident {
                            #(
                                #field_struct_assign_vars_tokens
                            )*
                        })
                    }
                }

                deserializer.deserialize_map(Visitor)
            }
        }
    }
}

fn field_enum_variant(field: &RustStructField, enums: &[RustEnum]) -> Vec<TokenStream> {
    let fhir_name = &field.fhir_name;
    let fhir_primitive_element_name = format!("_{}", fhir_name);
    let field_type_name = format_ident!("{}", field.fhir_name.to_rust_type_casing());
    let field_type_primitive_element_name = format_ident!("{}PrimitiveElement", field_type_name);

    if field.polymorph {
        let r#enum = enums.iter().find(|e| e.name == field.r#type.name).unwrap();

        r#enum
            .variants
            .iter()
            .map(|v| {
                let fhir_name = format!("{}{}", field.fhir_name, v.name);
                let fhir_primitive_element_name = format!("_{}", fhir_name);

                let field_type_name =
                    format_ident!("{}{}", field.fhir_name.to_rust_type_casing(), v.name);
                let field_type_primitive_element_name =
                    format_ident!("{}PrimitiveElement", field_type_name);

                if v.r#type.maybe_fhir_primitive {
                    quote! {
                        #[serde(rename=#fhir_name)]
                        #field_type_name,
                        #[serde(rename=#fhir_primitive_element_name)]
                        #field_type_primitive_element_name,
                    }
                } else {
                    quote! {
                        #[serde(rename=#fhir_name)]
                        #field_type_name,
                    }
                }
            })
            .collect()
    } else {
        if field.r#type.maybe_fhir_primitive {
            vec![quote! {
                #[serde(rename=#fhir_name)]
                #field_type_name,
                #[serde(rename=#fhir_primitive_element_name)]
                #field_type_primitive_element_name,
            }]
        } else {
            vec![quote! {
                #[serde(rename=#fhir_name)]
                #field_type_name,
            }]
        }
    }
}

fn field_mut_var(field: &RustStructField) -> TokenStream {
    let field_name_ident = format_ident!("r#{}", field.name);

    let type_tokens = field.r#type.name.parse().unwrap();

    let type_tokens = if field.r#type.r#box {
        quote! { Box<#type_tokens> }
    } else {
        type_tokens
    };

    let type_tokens = if field.multiple {
        quote! { Vec<#type_tokens> }
    } else {
        type_tokens
    };

    quote! {
        let mut #field_name_ident: Option<#type_tokens> = None;
    }
}

fn field_struct_assign_var(field: &RustStructField) -> TokenStream {
    let field_name = &field.name;
    let field_name_ident = format_ident!("r#{}", field_name);

    let fhir_name = &field.fhir_name;
    let fhir_name_poly = format!("{}[x]", field.fhir_name);

    if field.multiple {
        quote! {
            #field_name_ident: #field_name_ident.unwrap_or(vec![]),
        }
    } else if field.optional {
        quote! {
            #field_name_ident,
        }
    } else {
        if field.polymorph {
            quote! {
                #field_name_ident: #field_name_ident.ok_or(serde::de::Error::missing_field(#fhir_name_poly))?,
            }
        } else {
            quote! {
                #field_name_ident: #field_name_ident.ok_or(serde::de::Error::missing_field(#fhir_name))?,
            }
        }
    }
}

fn deserialize_field(field: &RustStructField, enums: &[RustEnum]) -> TokenStream {
    if field.polymorph {
        deserialize_enum(field, enums)
    } else {
        if field.r#type.maybe_fhir_primitive {
            deserialize_primitive(field)
        } else {
            deserialize_element(field)
        }
    }
}

fn deserialize_enum(field: &RustStructField, enums: &[RustEnum]) -> TokenStream {
    let r#enum = enums.iter().find(|e| e.name == field.r#type.name).unwrap();

    let deserialize_enum_variants_tokens = r#enum
        .variants
        .iter()
        .map(|v| deserialize_enum_variant(field, r#enum, v));

    quote! {
        #(
            #deserialize_enum_variants_tokens
        )*
    }
}

fn deserialize_enum_variant(
    field: &RustStructField,
    r#enum: &RustEnum,
    variant: &RustEnumVariant,
) -> TokenStream {
    let field_name_ident = format_ident!("r#{}", field.name);
    let enum_ident = format_ident!("{}", r#enum.name);
    let variant_ident = format_ident!("{}", variant.name);

    let fhir_name = format!("{}{}", field.fhir_name, variant.name);
    let fhir_name_poly = format!("{}[x]", field.fhir_name);
    let fhir_primitive_element_name = format!("_{}", fhir_name);
    let fhir_primitive_element_name_poly = format!("_{}", fhir_name_poly);

    let field_enum_type_name =
        format_ident!("{}{}", field.fhir_name.to_rust_type_casing(), variant.name);
    let field_enum_type_primitive_element_name =
        format_ident!("{}PrimitiveElement", field_enum_type_name);

    if variant.r#type.maybe_fhir_primitive {
        quote! {
            Field::#field_enum_type_name => {
                let r#enum = #field_name_ident.get_or_insert(#enum_ident::#variant_ident(Default::default()));

                if let #enum_ident::#variant_ident(variant) = r#enum {
                    if variant.value.is_some() {
                        return Err(serde::de::Error::duplicate_field(#fhir_name));
                    }

                    variant.value = Some(map_access.next_value()?);
                } else {
                    return Err(serde::de::Error::duplicate_field(#fhir_name_poly));
                }
            },
            Field::#field_enum_type_primitive_element_name => {
                let r#enum = #field_name_ident.get_or_insert(#enum_ident::#variant_ident(Default::default()));

                if let #enum_ident::#variant_ident(variant) = r#enum {
                    if variant.id.is_some() || !variant.extension.is_empty() {
                        return Err(serde::de::Error::duplicate_field(#fhir_primitive_element_name));
                    }

                    let super::super::serde_helpers::PrimitiveElementOwned { id, extension } = map_access.next_value()?;
                    variant.id = id;
                    variant.extension = extension;
                } else {
                    return Err(serde::de::Error::duplicate_field(#fhir_primitive_element_name_poly));
                }
            },
        }
    } else {
        quote! {
            Field::#field_enum_type_name => {
                if #field_name_ident.is_some() {
                    return Err(serde::de::Error::duplicate_field(#fhir_name));
                }
                #field_name_ident = Some(#enum_ident::#variant_ident(map_access.next_value()?));
            },
        }
    }
}

fn deserialize_primitive(field: &RustStructField) -> TokenStream {
    let fhir_name = &field.fhir_name;
    let field_name_ident = format_ident!("r#{}", field.name);

    let primitive_element_name = format!("_{}", fhir_name);

    let field_enum_type_name = format_ident!("{}", field.fhir_name.to_rust_type_casing());
    let field_enum_type_primitive_element_name =
        format_ident!("{}PrimitiveElement", field_enum_type_name);

    if field.multiple {
        quote! {
            Field::#field_enum_type_name => {
                let values: Vec<Option<_>> = map_access.next_value()?;

                let vec = #field_name_ident.get_or_insert(std::iter::repeat(Default::default()).take(values.len()).collect::<Vec<_>>());
                if vec.len() != values.len() {
                    return Err(serde::de::Error::invalid_length(values.len(), &"primitive elements length"));
                }
                if vec.iter().any(|v| v.value.is_some()) {
                    return Err(serde::de::Error::duplicate_field(#fhir_name));
                }

                for (i, value) in values.into_iter().enumerate() {
                    if let Some(value) = value {
                    vec[i].value = value;
                    }
                }
            },
            Field::#field_enum_type_primitive_element_name => {
                let elements: Vec<Option<super::super::serde_helpers::PrimitiveElementOwned>> = map_access.next_value()?;

                let vec = #field_name_ident.get_or_insert(std::iter::repeat(Default::default()).take(elements.len()).collect::<Vec<_>>());
                if vec.len() != elements.len() {
                    return Err(serde::de::Error::invalid_length(elements.len(), &"primitive values length"));
                }
                if vec.iter().any(|e| e.id.is_some() || !e.extension.is_empty()) {
                    return Err(serde::de::Error::duplicate_field(#primitive_element_name));
                }

                for (i, element) in elements.into_iter().enumerate() {
                    if let Some(element) = element {
                    vec[i].id = element.id;
                    vec[i].extension = element.extension;
                    }
                }
            },
        }
    } else {
        let deserialize_value_tokens = match field.r#type.name.as_str() {
            // xhtml is the only FHIR primtive where value is not optional
            "super::super::types::Xhtml" => quote! {
                if !some.value.is_empty() {
                    return Err(serde::de::Error::duplicate_field(#fhir_name));
                }
                some.value = map_access.next_value()?;
            },
            _ => quote! {
                if some.value.is_some() {
                    return Err(serde::de::Error::duplicate_field(#fhir_name));
                }
                some.value = Some(map_access.next_value()?);
            },
        };

        quote! {
            Field::#field_enum_type_name => {
                let some = #field_name_ident.get_or_insert(Default::default());

                #deserialize_value_tokens
            },
            Field::#field_enum_type_primitive_element_name => {
                let some = #field_name_ident.get_or_insert(Default::default());

                if some.id.is_some() || !some.extension.is_empty() {
                    return Err(serde::de::Error::duplicate_field(#primitive_element_name));
                }

                let super::super::serde_helpers::PrimitiveElementOwned { id, extension } = map_access.next_value()?;
                some.id = id;
                some.extension = extension;
            },
        }
    }
}

fn deserialize_element(field: &RustStructField) -> TokenStream {
    let fhir_name = &field.fhir_name;
    let field_name_ident = format_ident!("r#{}", field.name);

    let field_enum_type_name = format_ident!("{}", field.fhir_name.to_rust_type_casing());

    quote! {
        Field::#field_enum_type_name => {
            if #field_name_ident.is_some() {
                return Err(serde::de::Error::duplicate_field(#fhir_name));
            }
            #field_name_ident = Some(map_access.next_value()?);
        },
    }
}
