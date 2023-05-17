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

    let mut fields = r#struct.fields.clone();

    // make sure that id, url and value are serialized first
    // these are serialized as XML attributes, the opening tag can thus be writtern earlier
    // and nested state can be avoided in the XML serializer
    if r#struct.is_primitive {
        fields.sort_by_key(|f| match f.name.as_str() {
            "id" => 0,
            "value" => 1,
            _ => 2,
        });
    }

    if r#struct.struct_name == "Extension" {
        fields.sort_by_key(|f| match f.name.as_str() {
            "id" => 0,
            "url" => 1,
            "value" => 2,
            _ => 3,
        });
    }

    let serialized_fields_tokens = fields.iter().map(|f| {
        serialize_field(
            f,
            enums,
            r#struct.is_primitive,
            r#struct.struct_name == "Extension",
            r#struct.resource_name.is_some(),
            r#struct.struct_name == "Decimal",
            namespace,
        )
    });

    let path = &r#struct.path;

    quote! {
        impl serde::ser::Serialize for SerializationContext<&#struct_name_ident> {
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

        impl serde::ser::Serialize for SerializationContext<&Box<#struct_name_ident>> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
            }
        }

        impl serde::ser::Serialize for SerializationContext<&Vec<#struct_name_ident>> {
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

pub fn implement_serialize_resource_enum(resource_modules: &[RustFhirModule]) -> TokenStream {
    let match_resource_type = resource_modules.iter().map(|r| {
        let ident = format_ident!("{}", r.resource_name.as_ref().unwrap());

        quote! {
            Resource::#ident(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            },
        }
    });

    quote! {
        impl serde::ser::Serialize for SerializationContext<&Resource> {
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

        impl serde::ser::Serialize for SerializationContext<&Vec<Resource>> {
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
    in_resource: bool,
    is_decimal: bool,
    namespace: &TokenStream,
) -> TokenStream {
    if field.polymorph {
        serialize_enum(field, enums, namespace)
    } else if in_primitive && field.name == "value"
        || in_extension && field.name == "url"
        || !in_resource && field.name == "id"
    {
        serialize_primitive_value(field, is_decimal)
    } else if field.r#type.contains_primitive {
        serialize_primitive(field)
    } else {
        serialize_element(field)
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
        .map(|v| serialize_enum_variant(field, v));

    let value_invalid_error_message = format!("{} is invalid", field.name);
    let value_required_error_message = format!("{} is a required field", field.name);

    if field.optional {
        quote! {
            {
                // we alias the enum because rustfmt has issues with very long names like e.g. MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic
                use #namespace::#enum_ident as _Enum;

                if let Some(some) = self.value.#field_name_ident.as_ref() {
                    match some {
                        #(
                            #enum_variants_tokens
                        )*
                        _Enum::Invalid => {
                            return Err(serde::ser::Error::custom(#value_invalid_error_message))
                        }
                    }
                }
            }
        }
    } else {
        quote! {
            {
                use #namespace::#enum_ident as _Enum;

                match self.value.#field_name_ident {
                    #(
                        #enum_variants_tokens
                    )*
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom(#value_required_error_message))
                    }
                }
            }
        }
    }
}

fn serialize_enum_variant(
    field: &RustFhirStructField,
    variant: &RustFhirEnumVariant,
) -> TokenStream {
    let variant_ident = format_ident!("{}", variant.name);

    let fhir_name = format!("{}{}", field.fhir_name, variant.name);
    let fhir_primitive_element_name = format!("_{}", fhir_name);

    let map_intermediate_type_tokens: TokenStream = if variant.r#type.name == DECIMAL_TYPE {
        quote! { |v| v.parse::<serde_json::Number>().map_err(|_| serde::ser::Error::custom("error serializing decimal")) }
    } else if variant.r#type.name == INTEGER64_TYPE {
        quote! { |v| Ok(v.to_string()) }
    } else {
        quote! { Ok }
    };

    let serialize_tokens = if variant.r#type.contains_primitive {
        quote! {
            if self.output == crate::context::Format::Json {
                if let Some(some) = value.value.as_ref().map(#map_intermediate_type_tokens) {
                    state.serialize_entry(#fhir_name, &some?)?;
                }

                if value.id.is_some() || !value.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;

                    let primitive_element = PrimitiveElement {
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
        _Enum::#variant_ident(ref value) => {
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
                if self.output == crate::context::Format::Json {
                    let _value = value
                        .parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                    state.serialize_entry("value", &_value)?;
                } else if self.output == crate::context::Format::InternalElement {
                    let _value = crate::decimal::Decimal {
                        d: value,
                    };
                    state.serialize_entry("value", &_value)?;
                } else {
                    state.serialize_entry("value", value)?;
                }
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
    let serialize_element_tokens = serialize_element(field);

    quote! {
        if self.output == crate::context::Format::Json {
            #serialize_json_tokens
        } else #serialize_element_tokens
    }
}

fn serialize_primitive_json(field: &RustFhirStructField) -> TokenStream {
    let fhir_name = &field.fhir_name;
    let field_name_ident = format_ident!("r#{}", field.name);

    let primitive_element_name = format!("_{}", fhir_name);

    let map_intermediate_type_tokens: TokenStream = if field.r#type.name == DECIMAL_TYPE {
        quote! { |v| v.parse::<serde_json::Number>().map_err(|_| serde::ser::Error::custom("error serializing decimal")) }
    } else if field.r#type.name == INTEGER64_TYPE {
        quote! { |v| Ok(v.to_string()) }
    } else {
        quote! { Ok }
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
                    .map(|v| v.as_ref().map(#map_intermediate_type_tokens).transpose())
                    .collect::<Result<Vec<_>, _>>()?;

                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry(#fhir_name, &values)?;
                }

                let requires_elements = self.value.#field_name_ident
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());

                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;

                    let primitive_elements: Vec<_> = self.value.#field_name_ident
                        .iter()
                        .map(|e| if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
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
                if let Some(some) = some.value.as_ref().map(#map_intermediate_type_tokens) {
                    state.serialize_entry(#fhir_name, &some?)?;
                }

                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;

                    let primitive_element  = PrimitiveElement {
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
                if let Some(some) = self.value.#field_name_ident.value.as_ref().map(#map_intermediate_type_tokens) {
                    state.serialize_entry(#fhir_name, &some?)?;
                }
            },
        };
        quote! {
            if self.value.#field_name_ident.id.as_deref() == Some("$invalid") {
                return missing_field_error(#fhir_name)
            }

            #serialize_value_tokens

            if self.value.#field_name_ident.id.is_some() #check_extension_is_empty_tokens {
                use super::super::serde_helpers::PrimitiveElement;

                let primitive_element = PrimitiveElement {
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
            } else {
                self.with_context(&self.value.#field_name_ident, |ctx| state.serialize_entry(#fhir_name, ctx))?;
            }
        }
    }
}
