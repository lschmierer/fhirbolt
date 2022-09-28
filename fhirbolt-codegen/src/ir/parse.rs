use linked_hash_map::LinkedHashMap;

use crate::casing::RustCasing;
use crate::model::{Bundle, ElementDefinition, Resource, StructureDefinition};

use super::{
    RustFhirEnum, RustFhirEnumVariant, RustFhirFieldType, RustFhirModule, RustFhirStruct,
    RustFhirStructField,
};

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

const ABSTRACT_RESOURCES: &[&str] = &[
    "Resource",
    "DomainResource",
    "MetadataResource",
    "CanonicalResource",
];

pub fn parse_bundle<'a>(bundle: &Bundle) -> Vec<RustFhirModule> {
    flatten_bundle(bundle)
        .filter(|s| s.kind != "logical")
        .filter(|s| !ABSTRACT_RESOURCES.contains(&s.name.as_str()))
        .filter(|s| s.name != "Element")
        .map(|s| {
            let is_resource = s.kind == "resource";

            let doc_comment = if let Some(purpose) = s.purpose.as_ref() {
                format!("{}\n\n{}", s.description.as_ref().unwrap(), purpose)
            } else {
                s.description.clone().unwrap()
            };

            let resource_name = if is_resource {
                Some(s.name.to_string())
            } else {
                None
            };

            let mut module = RustFhirModule {
                module_name: s.name.to_rust_mod_casing(),
                resource_name: resource_name.clone(),
                doc_comment: doc_comment.clone(),
                ..Default::default()
            };

            parse_struct(
                &mut module,
                &s.name,
                resource_name,
                &s.snapshot.element.iter().collect::<Vec<_>>(),
                &s.r#type,
                doc_comment,
            );
            module
        })
        .collect()
}

fn flatten_bundle(bundle: &Bundle) -> impl Iterator<Item = &StructureDefinition> {
    bundle.entry.iter().filter_map(|e| match &e.resource {
        Resource::StructureDefinition(s) => Some(s),
        _ => None,
    })
}

fn parse_struct(
    module_collector: &mut RustFhirModule,
    name: &str,
    resource_name: Option<String>,
    element_definitions: &[&ElementDefinition],
    element_path_strip_prefix: &str,
    doc_comment: String,
) {
    let fields = parse_fields(
        module_collector,
        name,
        element_definitions,
        element_path_strip_prefix,
    );

    module_collector.structs.push(RustFhirStruct {
        struct_name: name.to_rust_type_casing(),
        resource_name,
        is_primitive: PRIMITIVES.contains(&name),
        fields,
        doc_comment: doc_comment,
    })
}

fn parse_fields(
    module_collector: &mut RustFhirModule,
    struct_name: &str,
    element_definitions: &[&ElementDefinition],
    element_path_strip_prefix: &str,
) -> Vec<RustFhirStructField> {
    let mut element_definition_grouped = LinkedHashMap::<_, Vec<_>>::new();

    for element_definition in element_definitions
        .iter()
        .filter(|e| e.path != element_path_strip_prefix)
    {
        let stripped_path = element_definition.path[element_path_strip_prefix.len() + 1..]
            .split(".")
            .next()
            .unwrap();

        element_definition_grouped
            .entry(stripped_path)
            .or_default()
            .push(*element_definition);
    }

    element_definition_grouped
        .iter()
        .map(|(field_name, element_definitions)| {
            let type_name = format!("{}{}", struct_name, field_name.to_rust_type_casing());

            if element_definitions.len() > 1 {
                parse_struct(
                    module_collector,
                    &type_name,
                    None,
                    &element_definitions,
                    &element_definitions[0].path,
                    element_definitions[0].definition.clone().unwrap(),
                );
            }

            parse_field(
                module_collector,
                struct_name,
                &element_definitions[0],
                element_path_strip_prefix,
                element_definitions[0].definition.clone().unwrap(),
            )
        })
        .collect()
}

fn parse_field(
    module_collector: &mut RustFhirModule,
    struct_name: &str,
    element_definition: &ElementDefinition,
    element_path_strip_prefix: &str,
    doc_comment: String,
) -> RustFhirStructField {
    let field_name = &element_definition.path[element_path_strip_prefix.len() + 1..];

    let (field_name, polymorph) = {
        if field_name.ends_with("[x]") {
            (&field_name[..field_name.len() - 3], true)
        } else {
            (field_name, false)
        }
    };

    let field_type = if polymorph {
        let name = format!("{}{}", struct_name, field_name.to_rust_type_casing());

        let variants = element_definition
            .r#type
            .as_ref()
            .unwrap()
            .iter()
            .map(|t| t.code.as_ref())
            .collect();

        let contains_primitive = create_value_enum(
            module_collector,
            &element_definition.path,
            &name,
            variants,
            doc_comment,
        );

        RustFhirFieldType {
            name,
            r#box: false,
            contains_primitive,
        }
    } else {
        if let Some(types) = element_definition.r#type.as_ref() {
            let code = &types.first().unwrap().code;
            if code == "BackboneElement" || code == "Element" {
                RustFhirFieldType {
                    name: format!("{}{}", struct_name, field_name.to_rust_type_casing()),
                    r#box: false,
                    contains_primitive: false,
                }
            } else {
                match_field_type(&element_definition.path, code, false)
            }
        } else {
            RustFhirFieldType {
                name: map_content_reference(element_definition.content_reference.as_ref().unwrap()),
                r#box: false,
                contains_primitive: false,
            }
        }
    };

    RustFhirStructField {
        name: field_name.to_rust_identifier_casing(),
        fhir_name: field_name.into(),
        r#type: field_type,
        polymorph,
        multiple: element_definition.max.as_ref().unwrap() != "1",
        optional: element_definition.min.unwrap() == 0,
        doc_comment: element_definition.definition.clone().unwrap(),
    }
}

fn create_value_enum<'a>(
    module_collector: &mut RustFhirModule,
    path: &str,
    enum_name: &str,
    types: Vec<&str>,
    doc_comment: String,
) -> bool {
    let mut may_contain_primitive = false;

    let variants = types
        .iter()
        .map(|t| {
            let field_type = match_field_type(path, t, true);

            if field_type.contains_primitive {
                may_contain_primitive = true;
            }

            RustFhirEnumVariant {
                // type like http://hl7.org/fhirpath/System.String
                name: t.rsplit(".").next().unwrap().to_rust_type_casing(),
                r#type: field_type,
            }
        })
        .collect();

    module_collector.enums.push(RustFhirEnum {
        name: enum_name.into(),
        variants,
        doc_comment,
    });

    may_contain_primitive
}

fn match_field_type(path: &str, code: &str, force_box: bool) -> RustFhirFieldType {
    // type like http://hl7.org/fhirpath/System.String
    match code.rsplit("/").next().unwrap() {
        "System.Boolean" => RustFhirFieldType {
            name: "bool".into(),
            r#box: false,
            contains_primitive: false,
        },
        "System.Integer" => RustFhirFieldType {
            name: "i32".into(),
            r#box: false,
            contains_primitive: false,
        },
        "System.String" => match path {
            "unsignedInt.value" | "positiveInt.value" => RustFhirFieldType {
                name: "u32".into(),
                r#box: false,
                contains_primitive: false,
            },
            _ => RustFhirFieldType {
                name: "std::string::String".into(),
                r#box: false,
                contains_primitive: false,
            },
        },
        "System.Decimal" => RustFhirFieldType {
            name: "std::string::String".into(),
            r#box: false,
            contains_primitive: false,
        },
        "System.Date" | "System.DateTime" | "System.Time" => RustFhirFieldType {
            name: "std::string::String".into(),
            r#box: false,
            contains_primitive: false,
        },
        "Resource" => RustFhirFieldType {
            name: "super::super::Resource".into(),
            r#box: true,
            contains_primitive: false,
        },
        r#type => {
            let primitive = PRIMITIVES.contains(&r#type);
            RustFhirFieldType {
                name: format!("super::super::types::{}", r#type.to_rust_type_casing()),
                r#box: !primitive || force_box,
                contains_primitive: primitive,
            }
        }
    }
}

fn map_content_reference(content_reference: &str) -> String {
    content_reference[1..]
        .split(".")
        .map(|s| s.to_rust_type_casing())
        .collect::<String>()
}
