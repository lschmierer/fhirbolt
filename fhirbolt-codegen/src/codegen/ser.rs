use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::ir::{RustFhirEnum, RustFhirEnumVariant, RustFhirStruct, RustFhirStructField};

const XHTML_TYPE: &str = "super::super::types::Xhtml";
const DECIMAL_TYPE: &str = "super::super::types::Decimal";

pub fn implement_serialize(r#struct: &RustFhirStruct, enums: &[RustFhirEnum]) -> TokenStream {
    let struct_name_ident = format_ident!("{}", r#struct.struct_name);

    let serialize_resource_type_tokens =
        if let Some(resource_name) = r#struct.resource_name.as_ref() {
            quote! { state.serialize_entry("resourceType", #resource_name)?; }
        } else {
            quote!()
        };

    let serialized_fields_tokens = r#struct.fields.iter().map(|f| serialize_field(f, enums));

    quote! {
        impl serde::ser::Serialize for #struct_name_ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                use serde::ser::SerializeMap;

                fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();

                    let mut state = serializer.serialize_map(None)?;
                    #serialize_resource_type_tokens

                    #(
                        #serialized_fields_tokens
                    )*

                    state.end()

                })
            }
        }
    }
}

fn serialize_field(field: &RustFhirStructField, enums: &[RustFhirEnum]) -> TokenStream {
    if field.polymorph {
        serialize_enum(field, enums)
    } else {
        if field.r#type.contains_primitive {
            serialize_primitive(field)
        } else {
            serialize_element(field)
        }
    }
}

fn serialize_enum(field: &RustFhirStructField, enums: &[RustFhirEnum]) -> TokenStream {
    let field_name_ident = format_ident!("r#{}", field.name);

    let r#enum = enums.iter().find(|e| e.name == field.r#type.name).unwrap();
    let enum_ident = format_ident!("{}", r#enum.name);

    let enum_variants_tokens = r#enum
        .variants
        .iter()
        .map(|v| serialize_enum_variant(field, r#enum, v));

    let value_invalid_error_message = format!("{} is invalid", field.name);
    let value_required_error_message = format!("{} is a required field", field.name);

    if field.optional {
        quote! {
            if let Some(some) = self.#field_name_ident.as_ref() {
                match some {
                    #(
                        #enum_variants_tokens
                    )*
                    #enum_ident::Invalid => {
                        return Err(serde::ser::Error::custom(#value_invalid_error_message))
                    }
                }
            }
        }
    } else {
        quote! {
            match self.#field_name_ident {
                #(
                    #enum_variants_tokens
                )*
                #enum_ident::Invalid => {
                    return Err(serde::ser::Error::custom(#value_required_error_message))
                }
            }
        }
    }
}

fn serialize_enum_variant(
    field: &RustFhirStructField,
    r#enum: &RustFhirEnum,
    variant: &RustFhirEnumVariant,
) -> TokenStream {
    let enum_ident = format_ident!("{}", r#enum.name);
    let variant_ident = format_ident!("{}", variant.name);

    let fhir_name = format!("{}{}", field.fhir_name, variant.name);
    let fhir_primitive_element_name = format!("_{}", fhir_name);

    let map_intermediate_type_tokens: TokenStream = if variant.r#type.name == DECIMAL_TYPE {
        quote! { some.parse::<serde_json::Number>().map_err(|_| serde::ser::Error::custom("error serializing decimal")) }
    } else {
        quote! { Ok(some) }
    };

    let serialize_tokens = if variant.r#type.contains_primitive {
        quote! {
            if _ctx.output_json {
                if let Some(some) = value.value.as_ref() {
                    let some = #map_intermediate_type_tokens?;
                    state.serialize_entry(#fhir_name, &some)?;
                }

                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: value.id.as_ref(),
                        extension: &value.extension,
                    };

                    state.serialize_entry(#fhir_primitive_element_name, &primitive_element)?;
                }
            } else {
                state.serialize_entry(#fhir_name, value)?;
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

fn serialize_primitive(field: &RustFhirStructField) -> TokenStream {
    let serialize_json_tokens = serialize_primitive_json(field);
    let serialize_xml_tokens = serialize_element(field);

    quote! {
        if _ctx.output_json {
            #serialize_json_tokens
        } else {
            #serialize_xml_tokens
        }
    }
}

fn serialize_primitive_json(field: &RustFhirStructField) -> TokenStream {
    let fhir_name = &field.fhir_name;
    let field_name_ident = format_ident!("r#{}", field.name);

    let primitive_element_name = format!("_{}", fhir_name);

    let map_intermediate_type_tokens: TokenStream = if field.r#type.name == DECIMAL_TYPE {
        quote! { some.parse::<serde_json::Number>().map_err(|_| serde::ser::Error::custom("error serializing decimal")) }
    } else {
        quote! { Ok(some) }
    };

    let (check_extension_is_empty_tokens, extension_reference_tokens) = if field.fhir_name != "div"
    {
        (
            quote! { || !self.#field_name_ident.extension.is_empty() },
            quote! { &self.#field_name_ident.extension },
        )
    } else {
        (quote! {}, quote! { &[] })
    };

    if field.multiple {
        quote! {
            if !self.#field_name_ident.is_empty() {
                let values = self.#field_name_ident
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| #map_intermediate_type_tokens).transpose())
                    .collect::<Result<Vec<_>, _>>()?;

                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry(#fhir_name, &values)?;
                }

                let requires_elements = self.#field_name_ident
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());

                if requires_elements {
                    let primitive_elements: Vec<_> = self.#field_name_ident
                        .iter()
                        .map(|e| if e.id.is_some() || !e.extension.is_empty() {
                                Some(super::super::serde_helpers::PrimitiveElement {
                                    id: e.id.as_ref(),
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
                    let some = #map_intermediate_type_tokens?;
                    state.serialize_entry(#fhir_name, &some)?;
                }

                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element  = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };

                    state.serialize_entry(#primitive_element_name, &primitive_element)?;
                }

            }
        }
    } else {
        let serialize_value_tokens = match field.r#type.name.as_str() {
            // xhtml is the only FHIR primtive where value is not optional
            XHTML_TYPE => quote! {
                state.serialize_entry(#fhir_name, &self.#field_name_ident.value)?;
            },
            _ => quote! {
                if let Some(some) = self.#field_name_ident.value.as_ref() {
                    let some = #map_intermediate_type_tokens?;
                    state.serialize_entry(#fhir_name, &some)?;
                }
            },
        };
        quote! {
            #serialize_value_tokens

            if self.#field_name_ident.id.is_some() #check_extension_is_empty_tokens {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.#field_name_ident.id.as_ref(),
                    extension: #extension_reference_tokens,
                };

                state.serialize_entry(#primitive_element_name, &primitive_element)?;
            }
        }
    }
}

fn serialize_element(field: &RustFhirStructField) -> TokenStream {
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
