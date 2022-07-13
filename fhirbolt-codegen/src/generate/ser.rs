use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::gather::{RustEnum, RustEnumVariant, RustStruct, RustStructField};

pub fn implement_serialze(r#struct: &RustStruct, enums: &[RustEnum]) -> TokenStream {
    let fhir_name = &r#struct.fhir_name;
    let stuct_name_ident = format_ident!("{}", r#struct.name);

    let serialize_resource_type_tokens = if r#struct.is_resource {
        quote! { state.serialize_entry("resourceType", #fhir_name)?; }
    } else {
        quote!()
    };

    let serialized_fields_tokens = r#struct.fields.iter().map(|f| serialize_field(f, enums));

    quote! {
        impl serde::ser::Serialize for #stuct_name_ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                use serde::ser::SerializeMap;

                let mut state = serializer.serialize_map(None)?;
                #serialize_resource_type_tokens

                #(
                    #serialized_fields_tokens
                )*

                state.end()
            }
        }
    }
}

fn serialize_field(field: &RustStructField, enums: &[RustEnum]) -> TokenStream {
    if field.polymorph {
        serialize_enum(field, enums)
    } else {
        if field.r#type.maybe_fhir_primitive {
            serialize_primitive(field)
        } else {
            serialize_element(field)
        }
    }
}

fn serialize_enum(field: &RustStructField, enums: &[RustEnum]) -> TokenStream {
    let field_name_ident = format_ident!("r#{}", field.name);

    let r#enum = enums.iter().find(|e| e.name == field.r#type.name).unwrap();

    let enum_variants_tokens = r#enum
        .variants
        .iter()
        .map(|v| serialize_enum_variant(field, r#enum, v));

    if field.optional {
        quote! {
            if let Some(some) = self.#field_name_ident.as_ref() {
                match some {
                    #(
                        #enum_variants_tokens
                    )*
                }
            }
        }
    } else {
        quote! {
            match self.#field_name_ident {
                #(
                    #enum_variants_tokens
                )*
            }
        }
    }
}

fn serialize_enum_variant(
    field: &RustStructField,
    r#enum: &RustEnum,
    variant: &RustEnumVariant,
) -> TokenStream {
    let enum_ident = format_ident!("{}", r#enum.name);
    let variant_ident = format_ident!("{}", variant.name);

    let fhir_name = format!("{}{}", field.fhir_name, variant.name);
    let fhir_primitive_element_name = format!("_{}", fhir_name);

    let serialize_tokens = if variant.r#type.maybe_fhir_primitive {
        quote! {
            if let Some(some) = value.value.as_ref() {
                state.serialize_entry(#fhir_name, some)?;
            }

            if value.id.is_some() || !value.extension.is_empty() {
                #[derive(serde::Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }

                let primitive_element = PrimtiveElement {
                    id: &value.id,
                    extension: &value.extension,
                };

                state.serialize_entry(#fhir_primitive_element_name, &primitive_element)?;
            }
        }
    } else {
        quote! {
            state.serialize_entry(#fhir_name, value)?;
        }
    };

    quote! {
        #enum_ident::#variant_ident(ref value) => {
            #serialize_tokens
        },
    }
}

fn serialize_primitive(field: &RustStructField) -> TokenStream {
    let fhir_name = &field.fhir_name;
    let field_name_ident = format_ident!("r#{}", field.name);

    let primitive_element_name = format!("_{}", fhir_name);

    let primitive_element_struct_tokens = quote! {
        #[derive(serde::Serialize)]
        struct PrimtiveElement<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: &'a Option<std::string::String>,
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            extension: &'a [Box<super::super::types::Extension>],
        }
    };

    if field.multiple {
        quote! {
            if !self.#field_name_ident.is_empty() {
                let values: Vec<_> = self.#field_name_ident.iter().map(|v| &v.value).collect();

                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry(#fhir_name, &values)?;
                }

                let requires_elements = self.#field_name_ident
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());

                if requires_elements {
                    #primitive_element_struct_tokens

                    let primitive_elements: Vec<_> = self.#field_name_ident
                        .iter()
                        .map(|e| if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimtiveElement {
                                    id: &e.id,
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            })
                        .collect();

                    state.serialize_entry(#primitive_element_name, &primitive_elements)?;
                }
            }
        }
    } else if field.optional {
        quote! {
            if let Some(some) = self.#field_name_ident.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    state.serialize_entry(#fhir_name, some)?;
                }

                if some.id.is_some() || !some.extension.is_empty() {
                    #primitive_element_struct_tokens

                    let primitive_element = PrimtiveElement {
                        id: &some.id,
                        extension: &some.extension,
                    };

                    state.serialize_entry(#primitive_element_name, &primitive_element)?;
                }

            }
        }
    } else {
        let serialize_value_tokens = match field.r#type.name.as_str() {
            // xhtml is the only FHIR primtive where value is not optional
            "super::super::types::Xhtml" => quote! {
                state.serialize_entry(#fhir_name, &self.#field_name_ident.value)?;
            },
            _ => quote! {
                if let Some(some) = self.#field_name_ident.value.as_ref() {
                    state.serialize_entry(#fhir_name, some)?;
                }
            },
        };
        quote! {
            #serialize_value_tokens

            if self.#field_name_ident.id.is_some() || !self.#field_name_ident.extension.is_empty() {
                #primitive_element_struct_tokens

                let primitive_element = PrimtiveElement {
                    id: &self.#field_name_ident.id,
                    extension: &self.#field_name_ident.extension,
                };

                state.serialize_entry(#primitive_element_name, &primitive_element)?;
            }
        }
    }
}

fn serialize_element(field: &RustStructField) -> TokenStream {
    let fhir_name = &field.fhir_name;
    let field_name_ident = format_ident!("r#{}", field.name);

    if field.multiple {
        quote! {
            if !self.#field_name_ident.is_empty() {
                state.serialize_entry(#fhir_name, &self.#field_name_ident)?;
            }
        }
    } else if field.optional {
        quote! {
            if let Some(some) = self.#field_name_ident.as_ref() {
                state.serialize_entry(#fhir_name, some)?;
            }
        }
    } else {
        quote! {
            state.serialize_entry(#fhir_name, &self.#field_name_ident)?;
        }
    }
}
