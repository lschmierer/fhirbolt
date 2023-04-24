use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::ir::{
    RustFhirEnum, RustFhirEnumVariant, RustFhirModule, RustFhirStruct, RustFhirStructField,
};

const XHTML_TYPE: &str = "types::Xhtml";
const DECIMAL_TYPE: &str = "types::Decimal";
const INTEGER64_TYPE: &str = "types::Integer64";

pub fn implement_serialize(
    r#struct: &RustFhirStruct,
    enums: &[RustFhirEnum],
    namespace: &TokenStream,
) -> TokenStream {
    let struct_name_ident = format_ident!("{}", r#struct.struct_name);

    let serialize_resource_type_tokens =
        if let Some(resource_name) = r#struct.resource_name.as_ref() {
            quote! { state.serialize_entry("resourceType", #resource_name)?; }
        } else {
            quote!()
        };

    let serialized_fields_tokens = r#struct.fields.iter().map(|f| {
        serialize_field(
            f,
            enums,
            r#struct.is_primitive,
            r#struct.struct_name == "Extension",
            r#struct.struct_name == "Decimal",
            namespace,
        )
    });

    let path = &r#struct.path;

    quote! {
        impl serde::ser::Serialize for crate::context::ser::SerializationContext<&#namespace::#struct_name_ident> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                use serde::ser::SerializeMap;

                #[allow(dead_code)]
                fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
                    Err(E::custom(format!("missing required field `{}.{}`", #path, field)))
                }

                let mut state = serializer.serialize_map(None)?;
                #serialize_resource_type_tokens

                #(
                    #serialized_fields_tokens
                )*

                state.end()
            }
        }

        impl serde::ser::Serialize for crate::context::ser::SerializationContext<&Box<#namespace::#struct_name_ident>> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
            }
        }

        impl serde::ser::Serialize for crate::context::ser::SerializationContext<&Vec<#namespace::#struct_name_ident>> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                use serde::ser::SerializeSeq;

                let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;

                for value in self.value {
                    self.with_context(value, |ctx| {
                        seq_serializer.serialize_element(ctx)
                    })?
                }

                seq_serializer.end()
            }
        }

        impl serde::ser::Serialize for crate::context::ser::SerializationContext<&Vec<Box<#namespace::#struct_name_ident>>> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                use serde::ser::SerializeSeq;

                let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;

                for value in self.value {
                    self.with_context(value, |ctx| {
                        seq_serializer.serialize_element(ctx)
                    })?
                }

                seq_serializer.end()
            }
        }
    }
}

pub fn implement_serialize_resource_enum(
    resource_modules: &[RustFhirModule],
    namespace: &TokenStream,
) -> TokenStream {
    let match_resource_type = resource_modules.iter().map(|r| {
        let ident = format_ident!("{}", r.resource_name.as_ref().unwrap());

        quote! {
            #namespace::Resource::#ident(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            },
        }
    });

    quote! {
        impl serde::ser::Serialize for crate::context::ser::SerializationContext<&#namespace::Resource> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                match self.value {
                    #(
                        #match_resource_type
                    )*
                    _ => Err(serde::ser::Error::custom("invalid resource")),
                }
            }
        }

        impl serde::ser::Serialize for crate::context::ser::SerializationContext<&Vec<#namespace::Resource>> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                use serde::ser::SerializeSeq;

                let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;

                for value in self.value {
                    self.with_context(value, |ctx| {
                        seq_serializer.serialize_element(ctx)
                    })?
                }

                seq_serializer.end()
            }
        }
    }
}

fn serialize_field(
    field: &RustFhirStructField,
    enums: &[RustFhirEnum],
    in_primitive: bool,
    in_extension: bool,
    is_decimal: bool,
    namespace: &TokenStream,
) -> TokenStream {
    if field.polymorph {
        serialize_enum(field, enums, namespace)
    } else {
        if in_primitive && field.name == "value"
            || in_extension && field.name == "url"
            || field.name == "id"
        {
            serialize_primitive_value(field, is_decimal)
        } else if field.r#type.contains_primitive {
            serialize_primitive(field)
        } else {
            serialize_element(field)
        }
    }
}

fn serialize_enum(
    field: &RustFhirStructField,
    enums: &[RustFhirEnum],
    namespace: &TokenStream,
) -> TokenStream {
    let field_name_ident = format_ident!("r#{}", field.name);

    let r#enum = enums.iter().find(|e| e.name == field.r#type.name).unwrap();
    let enum_ident = format_ident!("{}", r#enum.name);

    let enum_variants_tokens = r#enum
        .variants
        .iter()
        .map(|v| serialize_enum_variant(field, r#enum, v, namespace));

    let value_invalid_error_message = format!("{} is invalid", field.name);
    let value_required_error_message = format!("{} is a required field", field.name);

    if field.optional {
        quote! {
            if let Some(some) = self.value.#field_name_ident.as_ref() {
                match some {
                    #(
                        #enum_variants_tokens
                    )*
                    #namespace::#enum_ident::Invalid => {
                        return Err(serde::ser::Error::custom(#value_invalid_error_message))
                    }
                }
            }
        }
    } else {
        quote! {
            match self.value.#field_name_ident {
                #(
                    #enum_variants_tokens
                )*
                #namespace::#enum_ident::Invalid => {
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
    namespace: &TokenStream,
) -> TokenStream {
    let enum_ident = format_ident!("{}", r#enum.name);
    let variant_ident = format_ident!("{}", variant.name);

    let fhir_name = format!("{}{}", field.fhir_name, variant.name);
    let fhir_primitive_element_name = format!("_{}", fhir_name);

    let map_intermediate_type_tokens: TokenStream = if variant.r#type.name == DECIMAL_TYPE {
        quote! { some.parse::<serde_json::Number>().map_err(|_| serde::ser::Error::custom("error serializing decimal")) }
    } else if variant.r#type.name == INTEGER64_TYPE {
        quote! { Ok(some.to_string()) }
    } else {
        quote! { Ok(some) }
    };

    let serialize_tokens = if variant.r#type.contains_primitive {
        quote! {
            if self.output_json {
                if let Some(some) = value.value.as_ref() {
                    let some = #map_intermediate_type_tokens?;
                    state.serialize_entry(#fhir_name, &some)?;
                }

                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: value.id.as_ref(),
                        extension: &value.extension,
                    };

                    self.with_context(&primitive_element, |ctx| state.serialize_entry(#fhir_primitive_element_name, ctx))?;
                }
            } else {
                self.with_context(value, |ctx| state.serialize_entry(#fhir_name, ctx))?;
            }
        }
    } else {
        quote! {
            self.with_context(value, |ctx| state.serialize_entry(#fhir_name, ctx))?;
        }
    };

    quote! {
        #namespace::#enum_ident::#variant_ident(ref value) => {
            #serialize_tokens
        },
    }
}

fn serialize_primitive_value(field: &RustFhirStructField, is_decimal: bool) -> TokenStream {
    let fhir_name = &field.fhir_name;
    let field_name_ident = format_ident!("r#{}", field.name);

    if is_decimal && field.name == "value" {
        quote! {
            if let Some(value) = self.value.value.as_ref() {
                let _value = value
                    .parse::<serde_json::Number>()
                    .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                state.serialize_entry("value", &_value)?;
            }
        }
    } else if field.optional {
        quote! {
            if let Some(value) = self.value.#field_name_ident.as_ref() {
                state.serialize_entry(#fhir_name, value)?;
            }
        }
    } else {
        quote! {
            state.serialize_entry(#fhir_name, &self.value.#field_name_ident)?;
        }
    }
}

fn serialize_primitive(field: &RustFhirStructField) -> TokenStream {
    let serialize_json_tokens = serialize_primitive_json(field);
    let serialize_xml_tokens = serialize_element(field);

    quote! {
        if self.output_json {
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
    } else if field.r#type.name == INTEGER64_TYPE {
        quote! { Ok(some.to_string()) }
    } else {
        quote! { Ok(some) }
    };

    let (check_extension_is_empty_tokens, extension_reference_tokens) = if field.fhir_name != "div"
    {
        (
            quote! { || !self.value.#field_name_ident.extension.is_empty() },
            quote! { &self.value.#field_name_ident.extension },
        )
    } else {
        (quote! {}, quote! { &vec![] })
    };

    if field.multiple {
        quote! {
            if !self.value.#field_name_ident.is_empty() {
                let values = self.value.#field_name_ident
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| #map_intermediate_type_tokens).transpose())
                    .collect::<Result<Vec<_>, _>>()?;

                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry(#fhir_name, &values)?;
                }

                let requires_elements = self.value.#field_name_ident
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());

                if requires_elements {
                    let primitive_elements: Vec<_> = self.value.#field_name_ident
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

                    self.with_context(&primitive_elements, |ctx| state.serialize_entry(#primitive_element_name, ctx))?;
                }
            }
        }
    } else if field.optional {
        quote! {
            if let Some(some) = self.value.#field_name_ident.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = #map_intermediate_type_tokens?;
                    state.serialize_entry(#fhir_name, &some)?;
                }

                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element  = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };

                    self.with_context(&primitive_element, |ctx| state.serialize_entry(#primitive_element_name, ctx))?;
                }

            }
        }
    } else {
        let serialize_value_tokens = match field.r#type.name.as_str() {
            // xhtml is the only FHIR primtive where value is not optional
            XHTML_TYPE => quote! {
                state.serialize_entry(#fhir_name, &self.value.#field_name_ident.value)?;
            },
            _ => quote! {
                if let Some(some) = self.value.#field_name_ident.value.as_ref() {
                    let some = #map_intermediate_type_tokens?;
                    state.serialize_entry(#fhir_name, &some)?;
                }
            },
        };
        quote! {
            if self.value.#field_name_ident.id.as_deref() == Some("$invalid") {
                return missing_field_error(#fhir_name)
            }

            #serialize_value_tokens

            if self.value.#field_name_ident.id.is_some() #check_extension_is_empty_tokens {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.#field_name_ident.id.as_ref(),
                    extension: #extension_reference_tokens,
                };

                self.with_context(&primitive_element, |ctx| state.serialize_entry(#primitive_element_name, ctx))?;
            }
        }
    }
}

fn serialize_element(field: &RustFhirStructField) -> TokenStream {
    let fhir_name = &field.fhir_name;
    let field_name_ident = format_ident!("r#{}", field.name);

    if field.multiple {
        quote! {
            if !self.value.#field_name_ident.is_empty() {
                self.with_context(&self.value.#field_name_ident, |ctx| state.serialize_entry(#fhir_name, ctx))?;
            }
        }
    } else if field.optional {
        quote! {
            if let Some(some) = self.value.#field_name_ident.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry(#fhir_name, ctx))?;
            }
        }
    } else {
        quote! {
            if self.value.#field_name_ident.id.as_deref() == Some("$invalid") {
                return missing_field_error(#fhir_name)
            }

            self.with_context(&self.value.#field_name_ident, |ctx| state.serialize_entry(#fhir_name, ctx))?;
        }
    }
}
