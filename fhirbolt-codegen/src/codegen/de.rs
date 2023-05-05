use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    casing::RustCasing,
    ir::{
        RustFhirEnum, RustFhirEnumVariant, RustFhirFieldType, RustFhirModule, RustFhirStruct,
        RustFhirStructField,
    },
};

const XHTML_TYPE: &str = "types::Xhtml";
const DECIMAL_TYPE: &str = "types::Decimal";

pub fn implement_deserialze(
    r#struct: &RustFhirStruct,
    enums: &[RustFhirEnum],
    namespace: &TokenStream,
    base_namespace: &TokenStream,
) -> TokenStream {
    let struct_name = &r#struct.struct_name;
    let struct_name_ident = format_ident!("{}", struct_name);

    let (resource_type_field_enum_variant_tokens, deserialize_resource_type_field_tokens) =
        if let Some(resource_name) = r#struct.resource_name.as_ref() {
            (
                quote! {
                    #[serde(rename="resourceType")]
                    ResourceType,
                },
                quote! {
                Field::ResourceType => {
                    let value: std::borrow::Cow<str>  = map_access.next_value()?;
                    if value != #resource_name {
                        return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value), &#resource_name))
                    }
                }},
            )
        } else {
            (quote! {}, quote! {})
        };

    let all_possible_fields_names = r#struct
        .fields
        .iter()
        .flat_map(|f| possible_fhir_names(f, enums))
        .collect::<Vec<_>>();

    let field_enum_variants_tokens = r#struct
        .fields
        .iter()
        .flat_map(|f| field_enum_variant(f, enums));

    let field_mut_vars_tokens = r#struct
        .fields
        .iter()
        .map(|f| field_mut_var(f, namespace, base_namespace));
    let field_struct_assign_vars_tokens = r#struct.fields.iter().map(field_struct_assign_var);

    let deserialize_fields_tokens = r#struct.fields.iter().map(|f| {
        deserialize_field(
            f,
            enums,
            r#struct.is_primitive,
            r#struct.struct_name == "Extension",
            r#struct.resource_name.is_some(),
            namespace,
            base_namespace,
        )
    });

    let deserialize_owned_context = if r#struct.resource_name.is_some() {
        quote! {
            impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<#struct_name_ident> {
                type Value = #struct_name_ident;

                fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: serde::de::Deserializer<'de>,
                {
                    (&mut self).deserialize(deserializer)
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #deserialize_owned_context

        impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<#struct_name_ident> {
            type Value = #struct_name_ident;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                struct Visitor<'a>(&'a mut DeserializationContext<#struct_name_ident>);

                impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
                    type Value = #struct_name_ident;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(#struct_name)
                    }

                    fn visit_map<V>(self, mut map_access: V) -> Result<#struct_name_ident, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        #[derive(serde::Deserialize)]
                        #[serde(field_identifier)]
                        enum Field {
                            #resource_type_field_enum_variant_tokens
                            #(
                                #field_enum_variants_tokens
                            )*
                            Unknown(std::string::String),
                        }

                        fn unknown_field_error<T, E:serde::de::Error>(field: &str) -> Result<T, E> {
                            Err(E::unknown_field(
                                field,
                                &[
                                    #(
                                        #all_possible_fields_names,
                                    )*
                                ]
                            ))
                        }

                        #(
                            #field_mut_vars_tokens
                        )*

                        while let Some(map_access_key) = map_access.next_key()? {
                            match map_access_key {
                                #deserialize_resource_type_field_tokens
                                #(
                                    #deserialize_fields_tokens
                                )*
                                Field::Unknown(key) => if self.0.config.mode == crate::context::de::DeserializationMode::Strict {
                                    return unknown_field_error(&key);
                                }
                            }
                        }

                        Ok(#struct_name_ident {
                            #(
                                #field_struct_assign_vars_tokens
                            )*
                        })
                    }
                }

                deserializer.deserialize_map(Visitor(self))
            }
        }

        impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<#struct_name_ident>> {
            type Value = Box<#struct_name_ident>;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                self.transmute::<#struct_name_ident>().deserialize(deserializer).map(Box::new)
            }
        }

        impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<#struct_name_ident>> {
            type Value = Vec<#struct_name_ident>;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                struct Visitor<'a>(&'a mut DeserializationContext<Vec<#struct_name_ident>>);

                impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
                    type Value = Vec<#struct_name_ident>;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a sequence")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        let mut values = Vec::new();

                        let _context: &mut DeserializationContext<#struct_name_ident> = self.0.transmute();
                        while let Some(value) = seq.next_element_seed(&mut *_context)? {
                            values.push(value);
                        }

                        Ok(values)
                    }
                }

                deserializer.deserialize_seq(Visitor(self))
            }
        }
    }
}

pub fn implement_deserialze_resource_enum(
    resource_modules: &[RustFhirModule],
    namespace: &TokenStream,
    release: &str,
) -> TokenStream {
    let release_ident = format_ident!("{}", release.to_string());

    let match_resource_type = resource_modules.iter().map(|r| {
        let ident = format_ident!("{}", r.resource_name.as_ref().unwrap());
        let name = r.resource_name.as_ref().unwrap();

        quote! {
            #name => {
                let deserializer = crate::element::de::Deserializer(element);
                let _context: &mut DeserializationContext<Box<#namespace::resources::#ident>> = self.transmute();
                _context
                    .deserialize(deserializer)
                    .map(Resource::#ident)
                    .map_err(serde::de::Error::custom)
            },
        }
    });

    quote! {
        impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<Resource> {
            type Value = Resource;

            fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                (&mut self).deserialize(deserializer)
            }
        }

        impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Resource> {
            type Value = Resource;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let mut element_context = self.clone::<crate::element::internal::de::InternalElement<{ fhirbolt_shared::FhirReleases::#release_ident }>>();
                let element = element_context.deserialize(deserializer)?;

                self.from_json = false;

                if let Some(
                    fhirbolt_element::Value::Primitive(
                        fhirbolt_element::Primitive::String(resource_type)
                    )
                ) = element.0.get("resourceType") {
                    match resource_type.as_str() {
                        #(
                            #match_resource_type
                        )*
                        _ => Err(serde::de::Error::invalid_type(serde::de::Unexpected::Other("resourceType"), &"valid resourceType")),
                    }
                } else {
                    Err(serde::de::Error::invalid_type(serde::de::Unexpected::Other("an element"), &"a resource"))
                }
            }
        }

        impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Resource>> {
            type Value = Vec<Resource>;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                struct Visitor<'a>(&'a mut DeserializationContext<Vec<Resource>>);

                impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
                    type Value = Vec<Resource>;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a sequence of resources")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        let mut values = Vec::new();

                        while let Some(value) = seq.next_element_seed(self.0.transmute::<Resource>())? {
                            values.push(value);
                        }

                        Ok(values)
                    }
                }

                deserializer.deserialize_seq(Visitor(self))
            }
        }
    }
}

fn possible_fhir_names(field: &RustFhirStructField, enums: &[RustFhirEnum]) -> Vec<String> {
    if field.polymorph {
        let r#enum = enums.iter().find(|e| e.name == field.r#type.name).unwrap();

        r#enum
            .variants
            .iter()
            .map(|v| format!("{}{}", field.fhir_name, v.name))
            .collect()
    } else {
        vec![field.fhir_name.clone()]
    }
}

fn field_enum_variant(field: &RustFhirStructField, enums: &[RustFhirEnum]) -> Vec<TokenStream> {
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

                if v.r#type.contains_primitive {
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
    } else if field.r#type.contains_primitive {
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

fn field_mut_var(
    field: &RustFhirStructField,
    namespace: &TokenStream,
    base_namespace: &TokenStream,
) -> TokenStream {
    let field_name_ident = format_ident!("r#{}", field.name);

    let type_tokens = type_tokens(&field.r#type, field.multiple, namespace, base_namespace);

    let type_tokens = if field.multiple {
        quote! { Vec<#type_tokens> }
    } else {
        type_tokens
    };

    quote! {
        let mut #field_name_ident: Option<#type_tokens> = None;
    }
}

fn type_tokens(
    field_type: &RustFhirFieldType,
    multiple: bool,
    namespace: &TokenStream,
    base_namespace: &TokenStream,
) -> TokenStream {
    let type_tokens: TokenStream = field_type.name.parse().unwrap();

    if field_type.name == "bool"
        || field_type.name == "u32"
        || field_type.name == "i32"
        || field_type.name == "i64"
        || field_type.name == "std::string::String"
    {
        type_tokens
    } else if !(field_type.name.starts_with("types")
        || field_type.name.starts_with("resources")
        || field_type.name == "Resource")
    {
        quote! {#namespace::#type_tokens}
    } else if field_type.r#box && !multiple {
        quote! { Box<#base_namespace::#type_tokens> }
    } else {
        quote! {#base_namespace::#type_tokens}
    }
}

fn field_struct_assign_var(field: &RustFhirStructField) -> TokenStream {
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
    } else if field.polymorph {
        quote! {
            #field_name_ident: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                #field_name_ident.unwrap_or(Default::default())
            } else {
                #field_name_ident.ok_or(serde::de::Error::missing_field(#fhir_name_poly))?
            },
        }
    } else {
        quote! {
            #field_name_ident: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                #field_name_ident.unwrap_or(Default::default())
            } else {
                #field_name_ident.ok_or(serde::de::Error::missing_field(#fhir_name))?
            },
        }
    }
}

fn deserialize_field(
    field: &RustFhirStructField,
    enums: &[RustFhirEnum],
    in_primitive: bool,
    in_extension: bool,
    in_resource: bool,
    namespace: &TokenStream,
    base_namespace: &TokenStream,
) -> TokenStream {
    if field.polymorph {
        deserialize_enum(field, enums, namespace, base_namespace)
    } else if in_primitive && field.name == "value"
        || in_extension && field.name == "url"
        || !in_resource && field.name == "id"
    {
        deserialize_primitive_value(field)
    } else if field.r#type.contains_primitive {
        deserialize_primitive(field, namespace, base_namespace)
    } else {
        deserialize_element(field, namespace, base_namespace)
    }
}

fn deserialize_enum(
    field: &RustFhirStructField,
    enums: &[RustFhirEnum],
    namespace: &TokenStream,
    base_namespace: &TokenStream,
) -> TokenStream {
    let r#enum = enums.iter().find(|e| e.name == field.r#type.name).unwrap();

    let deserialize_enum_variants_tokens = r#enum
        .variants
        .iter()
        .map(|v| deserialize_enum_variant(field, r#enum, v, namespace, base_namespace));

    quote! {
        #(
            #deserialize_enum_variants_tokens
        )*
    }
}

fn deserialize_enum_variant(
    field: &RustFhirStructField,
    r#enum: &RustFhirEnum,
    variant: &RustFhirEnumVariant,
    namespace: &TokenStream,
    base_namespace: &TokenStream,
) -> TokenStream {
    let field_name_ident = format_ident!("r#{}", field.name);

    let enum_ident = format_ident!("{}", r#enum.name);
    let variant_ident = format_ident!("{}", variant.name);

    let fhir_name = format!("{}{}", field.fhir_name, variant.name);
    let fhir_name_poly = format!("{}[x]", field.fhir_name);
    let fhir_primitive_element_name = format!("_{}", fhir_name);
    let fhir_primitive_element_name_poly = format!("_{}", fhir_name_poly);

    let type_tokens = type_tokens(&variant.r#type, false, namespace, base_namespace);

    let field_enum_type_name =
        format_ident!("{}{}", field.fhir_name.to_rust_type_casing(), variant.name);
    let field_enum_type_primitive_element_name =
        format_ident!("{}PrimitiveElement", field_enum_type_name);

    if variant.r#type.contains_primitive {
        let deserialize_interemdiate_type_tokens = if variant.r#type.name == DECIMAL_TYPE {
            quote! {
                let value: serde_json::Number = map_access.next_value()?;

                variant.value = Some(format!("{}", value));
            }
        } else {
            quote! {
                variant.value = Some(map_access.next_value()?)
            }
        };
        quote! {
            Field::#field_enum_type_name => {
                use #namespace::#enum_ident as _Enum;

                if self.0.from_json {
                    let r#enum = #field_name_ident.get_or_insert(_Enum::#variant_ident(Default::default()));

                    if let _Enum::#variant_ident(variant) = r#enum {
                        if variant.value.is_some() {
                            return Err(serde::de::Error::duplicate_field(#fhir_name));
                        }

                        #deserialize_interemdiate_type_tokens
                    } else {
                        return Err(serde::de::Error::duplicate_field(#fhir_name_poly));
                    }
                } else {
                    if #field_name_ident.is_some() {
                        return Err(serde::de::Error::duplicate_field(#fhir_name));
                    }
                    let _context: &mut DeserializationContext<#type_tokens> = self.0.transmute();
                    #field_name_ident = Some(_Enum::#variant_ident(map_access.next_value_seed(&mut *_context)?));
                }
            },
            Field::#field_enum_type_primitive_element_name => {
                use #namespace::#enum_ident as _Enum;

                if self.0.from_json {
                    let r#enum = #field_name_ident.get_or_insert(_Enum::#variant_ident(Default::default()));

                    if let _Enum::#variant_ident(variant) = r#enum {
                        if variant.id.is_some() || !variant.extension.is_empty() {
                            return Err(serde::de::Error::duplicate_field(#fhir_primitive_element_name));
                        }

                        use super::super::serde_helpers::PrimitiveElementOwned;

                        let _context: &mut DeserializationContext<PrimitiveElementOwned> = self.0.transmute();
                        let PrimitiveElementOwned { id, extension } =
                            map_access.next_value_seed(&mut *_context)?;
                        variant.id = id;
                        variant.extension = extension;
                    } else {
                        return Err(serde::de::Error::duplicate_field(#fhir_primitive_element_name_poly));
                    }
                } else {
                    return unknown_field_error(#fhir_name);
                }
            },
        }
    } else {
        quote! {
            Field::#field_enum_type_name => {
                use #namespace::#enum_ident as _Enum;

                if #field_name_ident.is_some() {
                    return Err(serde::de::Error::duplicate_field(#fhir_name));
                }
                let _context: &mut DeserializationContext<#type_tokens> = self.0.transmute();
                #field_name_ident = Some(_Enum::#variant_ident(map_access.next_value_seed(&mut *_context)?));
            },
        }
    }
}

fn deserialize_primitive_value(field: &RustFhirStructField) -> TokenStream {
    let fhir_name = &field.fhir_name;
    let field_name_ident = format_ident!("r#{}", field.name);
    let field_type_ident: TokenStream = field.r#type.name.parse().unwrap();

    let field_enum_type_name = format_ident!("{}", field.fhir_name.to_rust_type_casing());

    if field.r#type.name == "std::string::String" {
        quote! {
            Field::#field_enum_type_name => {
                if #field_name_ident.is_some() {
                    return Err(serde::de::Error::duplicate_field(#fhir_name));
                }
                #field_name_ident = Some(map_access.next_value()?);
            },
        }
    } else {
        quote! {
            Field::#field_enum_type_name => {
                if #field_name_ident.is_some() {
                    return Err(serde::de::Error::duplicate_field(#fhir_name));
                }

                #[derive(serde::Deserialize)]
                #[serde(untagged)]
                enum StringOrValue{
                    String(String),
                    Value(#field_type_ident)
                }

                match map_access.next_value()? {
                    StringOrValue::String(s) => {
                        #field_name_ident = Some(s.parse().map_err(|err| serde::de::Error::custom(format!("{:?}", err)))?);
                    },
                    StringOrValue::Value(v) => {
                        #field_name_ident = Some(v);
                    }
                }
            },
        }
    }
}

fn deserialize_primitive(
    field: &RustFhirStructField,
    namespace: &TokenStream,
    base_namespace: &TokenStream,
) -> TokenStream {
    let fhir_name = &field.fhir_name;
    let field_name_ident = format_ident!("r#{}", field.name);

    let primitive_element_name = format!("_{}", fhir_name);

    let field_enum_type_name = format_ident!("{}", field.fhir_name.to_rust_type_casing());
    let field_enum_type_primitive_element_name =
        format_ident!("{}PrimitiveElement", field_enum_type_name);

    let type_tokens = type_tokens(&field.r#type, field.multiple, namespace, base_namespace);

    let (intermediate_type_tokens, convert_intermediate_type_tokens): (TokenStream, TokenStream) =
        if field.r#type.name == DECIMAL_TYPE {
            (
                quote! { serde_json::Number },
                quote! { format!("{}", value) },
            )
        } else {
            (quote! { _ }, quote! { value })
        };

    let (check_extension_is_empty_tokens, extension_tokens, assign_extension_tokens) =
        if field.fhir_name != "div" {
            (
                quote! { || !some.extension.is_empty() },
                quote! { extension },
                quote! { some.extension = extension; },
            )
        } else {
            (quote! {}, quote! { .. }, quote! {})
        };

    if field.multiple {
        quote! {
            Field::#field_enum_type_name => {
                if self.0.from_json {
                    let values: Vec<Option<#intermediate_type_tokens>> = map_access.next_value()?;

                    let vec = #field_name_ident.get_or_insert(std::iter::repeat(Default::default()).take(values.len()).collect::<Vec<_>>());
                    if vec.len() != values.len() {
                        return Err(serde::de::Error::invalid_length(values.len(), &"primitive elements length"));
                    }
                    if vec.iter().any(|v| v.value.is_some()) {
                        return Err(serde::de::Error::duplicate_field(#fhir_name));
                    }

                    for (i, value) in values.into_iter().enumerate() {
                        if let Some(value) = value {
                            vec[i].value = Some(#convert_intermediate_type_tokens);
                        }
                    }
                } else {
                    let vec = #field_name_ident.get_or_insert(Default::default());
                    let _context: &mut DeserializationContext<#type_tokens> = self.0.transmute();
                    vec.push(map_access.next_value_seed(&mut *_context)?);
                }
            },
            Field::#field_enum_type_primitive_element_name => {
                if self.0.from_json {
                    use super::super::serde_helpers::PrimitiveElementOwned;

                    let _context: &mut DeserializationContext<Vec<Option<PrimitiveElementOwned>>> = self.0.transmute();
                    let elements: Vec<Option<PrimitiveElementOwned>> =
                        map_access.next_value_seed(&mut *_context)?;

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
                } else {
                    return unknown_field_error(#fhir_name);
                }
            },
        }
    } else {
        let deserialize_value_tokens = match field.r#type.name.as_str() {
            // xhtml is the only FHIR primitive where value is not optional
            XHTML_TYPE => quote! {
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
                if self.0.from_json {
                    let some = #field_name_ident.get_or_insert(Default::default());

                    #deserialize_value_tokens
                } else {
                    if #field_name_ident.is_some() {
                        return Err(serde::de::Error::duplicate_field(#fhir_name));
                    }

                    let _context: &mut DeserializationContext<#type_tokens> = self.0.transmute();
                    #field_name_ident = Some(map_access.next_value_seed(&mut *_context)?);
                }
            },
            Field::#field_enum_type_primitive_element_name => {
                if self.0.from_json {
                    let some = #field_name_ident.get_or_insert(Default::default());

                    if some.id.is_some() #check_extension_is_empty_tokens {
                        return Err(serde::de::Error::duplicate_field(#primitive_element_name));
                    }

                    use super::super::serde_helpers::PrimitiveElementOwned;

                    let _context: &mut DeserializationContext<PrimitiveElementOwned> = self.0.transmute();
                    let PrimitiveElementOwned { id, #extension_tokens } =
                        map_access.next_value_seed(&mut *_context)?;
                    some.id = id;
                    #assign_extension_tokens
                } else {
                    return unknown_field_error(#fhir_name);
                }
            },
        }
    }
}

fn deserialize_element(
    field: &RustFhirStructField,
    namespace: &TokenStream,
    base_namespace: &TokenStream,
) -> TokenStream {
    let fhir_name = &field.fhir_name;
    let field_name_ident = format_ident!("r#{}", field.name);

    let type_tokens = type_tokens(&field.r#type, field.multiple, namespace, base_namespace);

    let field_enum_type_name = format_ident!("{}", field.fhir_name.to_rust_type_casing());

    if field.multiple {
        quote! {
            Field::#field_enum_type_name => {
                if self.0.from_json {
                    if #field_name_ident.is_some() {
                        return Err(serde::de::Error::duplicate_field(#fhir_name));
                    }

                    let _context: &mut DeserializationContext<Vec<#type_tokens>> = self.0.transmute();
                    #field_name_ident = Some(map_access.next_value_seed(&mut *_context)?);
                } else {
                    let vec = #field_name_ident.get_or_insert(Default::default());

                    let _context: &mut DeserializationContext<#type_tokens> = self.0.transmute();
                    vec.push(map_access.next_value_seed(&mut *_context)?);
                }
            },
        }
    } else {
        quote! {
            Field::#field_enum_type_name => {
                if #field_name_ident.is_some() {
                    return Err(serde::de::Error::duplicate_field(#fhir_name));
                }

                let _context: &mut DeserializationContext<#type_tokens> = self.0.transmute();
                #field_name_ident = Some(map_access.next_value_seed(&mut *_context)?);
            },
        }
    }
}
