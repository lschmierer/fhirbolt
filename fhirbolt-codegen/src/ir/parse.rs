use linked_hash_map::LinkedHashMap;

use crate::casing::RustCasing;
use crate::ir::{
    RustFhirEnum, RustFhirEnumVariant, RustFhirFieldType, RustFhirModule, RustFhirStruct,
    RustFhirStructField, TypeHints,
};
use crate::model::{Bundle, ElementDefinition, Resource, StructureDefinition};

use super::ElementMap;

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
    "integer64",
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

const RESOURCE_COMMON_FIELDS: &[&str] = &[
    "id",
    "meta",
    "implicitRules",
    "language",
    "text",
    "contained",
];
const COMMON_SEQUENCE_FIELDS: &[&str] = &["extension", "modifierExtension"];

pub fn parse_modules(
    bundle: &Bundle,
    type_hint_collector: &mut TypeHints,
    element_map_collector: &mut ElementMap,
) -> Vec<RustFhirModule> {
    flatten_bundle(bundle)
        .filter(|s| s.kind != "logical")
        .filter(|s| !s.r#abstract)
        //.filter(|s| !ABSTRACT_TYPES.contains(&s.name.as_str()))
        .filter(|s| s.name != "Element")
        .map(|s| {
            let is_resource = s.kind == "resource";

            let doc_comment = if let Some(purpose) = s.purpose.as_ref() {
                format!("{}\n\n{}", s.description.as_ref().unwrap(), purpose)
            } else {
                s.description.clone().unwrap()
            };

            let resource_name = if is_resource {
                let name = s.name.to_string();
                Some(name)
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
                type_hint_collector,
                element_map_collector,
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

#[allow(clippy::too_many_arguments)]
fn parse_struct(
    module_collector: &mut RustFhirModule,
    type_hint_collector: &mut TypeHints,
    element_map_collector: &mut ElementMap,
    name: &str,
    resource_name: Option<String>,
    element_definitions: &[&ElementDefinition],
    element_path_strip_prefix: &str,
    doc_comment: String,
) {
    let fields = parse_fields(
        module_collector,
        type_hint_collector,
        element_map_collector,
        name,
        element_definitions,
        element_path_strip_prefix,
        resource_name.is_some(),
    );

    module_collector.structs.push(RustFhirStruct {
        struct_name: name.to_rust_type_casing(),
        resource_name,
        path: element_path_strip_prefix.to_string(),
        is_primitive: PRIMITIVES.contains(&name),
        fields,
        doc_comment,
    })
}

fn parse_fields(
    module_collector: &mut RustFhirModule,
    type_hint_collector: &mut TypeHints,
    element_map_collector: &mut ElementMap,
    struct_name: &str,
    element_definitions: &[&ElementDefinition],
    element_path_strip_prefix: &str,
    is_resource: bool,
) -> Vec<RustFhirStructField> {
    let mut element_definition_grouped = LinkedHashMap::<_, Vec<_>>::new();

    for element_definition in element_definitions
        .iter()
        .filter(|e| e.path != element_path_strip_prefix)
    {
        let stripped_path = element_definition.path[element_path_strip_prefix.len() + 1..]
            .split('.')
            .next()
            .unwrap();

        element_definition_grouped
            .entry(stripped_path)
            .or_default()
            .push(*element_definition);
    }

    element_definition_grouped
        .iter()
        .filter(|(_, d)| d[0].max.as_ref().unwrap() != "0")
        .map(|(field_name, element_definitions)| {
            let type_name = format!("{}{}", struct_name, field_name.to_rust_type_casing());

            if element_definitions.len() > 1 {
                parse_struct(
                    module_collector,
                    type_hint_collector,
                    element_map_collector,
                    &type_name,
                    None,
                    element_definitions,
                    &element_definitions[0].path,
                    element_definitions[0].definition.clone().unwrap(),
                );
            }

            parse_field(
                module_collector,
                type_hint_collector,
                element_map_collector,
                struct_name,
                element_definitions[0],
                element_path_strip_prefix,
                element_definitions[0].definition.clone().unwrap(),
                is_resource,
            )
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn parse_field(
    module_collector: &mut RustFhirModule,
    type_hint_collector: &mut TypeHints,
    element_map_collector: &mut ElementMap,
    struct_name: &str,
    element_definition: &ElementDefinition,
    element_path_strip_prefix: &str,
    doc_comment: String,
    is_resource: bool,
) -> RustFhirStructField {
    let field_name = &element_definition.path[element_path_strip_prefix.len() + 1..];

    let (field_name, polymorph) = {
        if let Some(stripped) = field_name.strip_suffix("[x]") {
            (stripped, true)
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
            type_hint_collector,
            element_map_collector,
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
        collect_element_map(element_map_collector, element_path_strip_prefix, field_name);

        if is_resource && field_name == "id" {
            RustFhirFieldType {
                name: "types::Id".to_string(),
                r#box: true,
                contains_primitive: true,
            }
        } else if let Some(types) = element_definition.r#type.as_ref() {
            let code = &types.first().unwrap().code;
            if code == "BackboneElement" || code == "Element" {
                RustFhirFieldType {
                    name: format!("{}{}", struct_name, field_name.to_rust_type_casing()),
                    r#box: false,
                    contains_primitive: false,
                }
            } else {
                collect_type_hint(
                    type_hint_collector,
                    &element_definition.path,
                    code,
                    is_resource,
                );

                match_field_type(&element_definition.path, code, false)
            }
        } else {
            let content_reference = element_definition.content_reference.as_ref().unwrap();

            collect_type_hint_content_reference(
                type_hint_collector,
                &element_definition.path,
                content_reference,
            );

            RustFhirFieldType {
                name: map_content_reference(content_reference),
                r#box: false,
                contains_primitive: false,
            }
        }
    };

    let multiple = element_definition.max.as_deref().unwrap() == "*";
    if multiple {
        collect_type_hint_multiple(type_hint_collector, &element_definition.path, is_resource);
    }

    RustFhirStructField {
        name: field_name.to_rust_identifier_casing(),
        fhir_name: field_name.into(),
        r#type: field_type,
        polymorph,
        multiple,
        optional: element_definition.min.unwrap() == 0,
        doc_comment: element_definition.definition.clone().unwrap(),
    }
}

fn create_value_enum(
    module_collector: &mut RustFhirModule,
    type_hint_collector: &mut TypeHints,
    element_map_collector: &mut ElementMap,
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

            let variant_name = t.rsplit('.').next().unwrap().to_rust_type_casing();
            let variant_path = path.replace("[x]", &variant_name);

            collect_type_hint(type_hint_collector, &variant_path, t, false);

            let (parent, element) = variant_path.rsplit_once('.').unwrap();
            collect_element_map(element_map_collector, parent, element);

            RustFhirEnumVariant {
                // type like http://hl7.org/fhirpath/System.String
                name: variant_name,
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
    match code.rsplit('/').next().unwrap() {
        "System.Boolean" => RustFhirFieldType {
            name: "bool".into(),
            r#box: false,
            contains_primitive: false,
        },
        "System.Integer" => match path {
            "integer64.value" => RustFhirFieldType {
                name: "i64".into(),
                r#box: false,
                contains_primitive: false,
            },
            _ => RustFhirFieldType {
                name: "i32".into(),
                r#box: false,
                contains_primitive: false,
            },
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
            name: "Resource".into(),
            r#box: false,
            contains_primitive: false,
        },
        r#type => {
            let is_primitive = PRIMITIVES.contains(&r#type);

            RustFhirFieldType {
                name: format!("types::{}", r#type.to_rust_type_casing()),
                r#box: !is_primitive || force_box,
                contains_primitive: is_primitive,
            }
        }
    }
}

fn map_content_reference(content_reference: &str) -> String {
    content_reference[1..]
        .split('.')
        .map(|s| s.to_rust_type_casing())
        .collect::<String>()
}

fn collect_type_hint_multiple(type_hint_collector: &mut TypeHints, path: &str, is_resource: bool) {
    if !should_collect_path_to_type_hints(path, is_resource) {
        return;
    }

    type_hint_collector.array_paths.insert(path.to_string());
}

fn collect_type_hint(
    type_hint_collector: &mut TypeHints,
    path: &str,
    type_code: &str,
    is_resource: bool,
) {
    if type_code.contains('.') {
        return;
    }

    if !should_collect_path_to_type_hints(path, is_resource) {
        return;
    }

    if PRIMITIVES.contains(&type_code) {
        type_hint_collector
            .all_primitives_paths
            .insert(path.to_string());

        match type_code {
            "boolean" => {
                type_hint_collector.boolean_paths.insert(path.to_string());
            }
            "integer" => {
                type_hint_collector.integer_paths.insert(path.to_string());
            }
            "integer64" => {
                type_hint_collector.integer64_paths.insert(path.to_string());
            }
            "unsignedInt" => {
                type_hint_collector
                    .unsigned_integer_paths
                    .insert(path.to_string());
            }
            "positiveInt" => {
                type_hint_collector
                    .positive_integer_paths
                    .insert(path.to_string());
            }
            "decimal" => {
                type_hint_collector
                    .decimal_integer_paths
                    .insert(path.to_string());
            }
            _ => (),
        }
    } else {
        type_hint_collector
            .type_paths
            .insert(path.to_string(), type_code.to_string());
    }
}

fn should_collect_path_to_type_hints(path: &str, is_resource: bool) -> bool {
    let path_split: Vec<_> = path.split('.').map(|s| s.to_owned()).collect();

    // common resource fields are handled separately
    if is_resource
        && path_split.len() == 2
        && RESOURCE_COMMON_FIELDS.contains(&path_split[1].as_str())
    {
        return false;
    }

    // some common array fields separately (because they are so many)
    if path_split
        .last()
        .map(|l| COMMON_SEQUENCE_FIELDS.contains(&l.as_str()))
        .unwrap_or(false)
    {
        return false;
    }

    true
}

fn collect_type_hint_content_reference(
    type_hint_collector: &mut TypeHints,
    path: &str,
    content_reference: &str,
) {
    type_hint_collector
        .content_reference_paths
        .insert(path.to_string(), content_reference[1..].to_string());
}

fn collect_element_map(element_map_collector: &mut ElementMap, parent: &str, element: &str) {
    let set = element_map_collector
        .0
        .entry(parent.to_string())
        .or_default();

    if !set.contains(element) {
        set.insert(element.to_string());
    }
}
