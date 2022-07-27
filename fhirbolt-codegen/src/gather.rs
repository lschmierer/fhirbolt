use linked_hash_map::LinkedHashMap;

use crate::casing::RustCasing;
use crate::model::{Bundle, ElementDefinition, Resource, StructureDefinition};

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

#[derive(Default)]
pub struct RustModule {
    pub name: String,
    pub resource_name: Option<String>,
    pub structs: Vec<RustStruct>,
    pub enums: Vec<RustEnum>,
    pub doc_comment: String,
}

pub struct RustStruct {
    pub name: String,
    pub fhir_name: String,
    pub is_resource: bool,
    pub is_fhir_primitive: bool,
    pub fields: Vec<RustStructField>,
    pub doc_comment: String,
}

pub struct RustStructField {
    pub name: String,
    pub fhir_name: String,
    pub r#type: RustFieldType,
    pub polymorph: bool,
    pub multiple: bool,
    pub optional: bool,
    pub doc_comment: String,
}

pub struct RustEnum {
    pub name: String,
    pub variants: Vec<RustEnumVariant>,
    pub doc_comment: String,
}

pub struct RustEnumVariant {
    pub name: String,
    pub r#type: RustFieldType,
}

pub struct RustFieldType {
    pub name: String,
    pub r#box: bool,
    pub maybe_fhir_primitive: bool,
}

pub fn gather_all_modules<'a>(bundle: &Bundle) -> Vec<RustModule> {
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

            let mut module = RustModule {
                name: s.name.to_rust_mod_casing(),
                resource_name: if is_resource {
                    Some(s.name.to_rust_type_casing())
                } else {
                    None
                },
                doc_comment: doc_comment.clone(),
                ..Default::default()
            };

            gather_struct(
                &mut module,
                &s.name,
                is_resource,
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

fn gather_struct(
    module: &mut RustModule,
    name: &str,
    is_resource: bool,
    element_definitions: &[&ElementDefinition],
    element_path_strip_prefix: &str,
    doc_comment: String,
) {
    let fields = gather_fields(module, name, element_definitions, element_path_strip_prefix);

    module.structs.push(RustStruct {
        name: name.to_rust_type_casing(),
        fhir_name: name.into(),
        is_resource,
        is_fhir_primitive: PRIMITIVES.contains(&name),
        fields,
        doc_comment: doc_comment,
    })
}

fn gather_fields(
    module: &mut RustModule,
    struct_name: &str,
    element_definitions: &[&ElementDefinition],
    element_path_strip_prefix: &str,
) -> Vec<RustStructField> {
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
                gather_struct(
                    module,
                    &type_name,
                    false,
                    &element_definitions,
                    &element_definitions[0].path,
                    element_definitions[0].definition.clone().unwrap(),
                );
            }

            gather_field(
                module,
                struct_name,
                &element_definitions[0],
                element_path_strip_prefix,
                element_definitions[0].definition.clone().unwrap(),
            )
        })
        .collect()
}

fn gather_field(
    module: &mut RustModule,
    struct_name: &str,
    element_definition: &ElementDefinition,
    element_path_strip_prefix: &str,
    doc_comment: String,
) -> RustStructField {
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

        let maybe_fhir_primitive = create_value_enum(
            module,
            &element_definition.path,
            &name,
            variants,
            doc_comment,
        );

        RustFieldType {
            name,
            r#box: false,
            maybe_fhir_primitive,
        }
    } else {
        if let Some(types) = element_definition.r#type.as_ref() {
            let code = &types.first().unwrap().code;
            if code == "BackboneElement" || code == "Element" {
                RustFieldType {
                    name: format!("{}{}", struct_name, field_name.to_rust_type_casing()),
                    r#box: false,
                    maybe_fhir_primitive: false,
                }
            } else {
                match_field_type(&element_definition.path, code, false)
            }
        } else {
            RustFieldType {
                name: map_content_reference(element_definition.content_reference.as_ref().unwrap()),
                r#box: false,
                maybe_fhir_primitive: false,
            }
        }
    };

    RustStructField {
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
    module: &mut RustModule,
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

            if field_type.maybe_fhir_primitive {
                may_contain_primitive = true;
            }

            RustEnumVariant {
                // type like http://hl7.org/fhirpath/System.String
                name: t.rsplit(".").next().unwrap().to_rust_type_casing(),
                r#type: field_type,
            }
        })
        .collect();

    module.enums.push(RustEnum {
        name: enum_name.into(),
        variants,
        doc_comment,
    });

    may_contain_primitive
}

fn match_field_type(path: &str, code: &str, force_box: bool) -> RustFieldType {
    // type like http://hl7.org/fhirpath/System.String
    match code.rsplit("/").next().unwrap() {
        "System.Boolean" => RustFieldType {
            name: "bool".into(),
            r#box: false,
            maybe_fhir_primitive: false,
        },
        "System.Integer" => RustFieldType {
            name: "i32".into(),
            r#box: false,
            maybe_fhir_primitive: false,
        },
        "System.String" => match path {
            "unsignedInt.value" | "positiveInt.value" => RustFieldType {
                name: "u32".into(),
                r#box: false,
                maybe_fhir_primitive: false,
            },
            _ => RustFieldType {
                name: "std::string::String".into(),
                r#box: false,
                maybe_fhir_primitive: false,
            },
        },
        "System.Decimal" => RustFieldType {
            name: "std::string::String".into(),
            r#box: false,
            maybe_fhir_primitive: false,
        },
        "System.Date" | "System.DateTime" | "System.Time" => RustFieldType {
            name: "std::string::String".into(),
            r#box: false,
            maybe_fhir_primitive: false,
        },
        "Resource" => RustFieldType {
            name: "super::super::Resource".into(),
            r#box: true,
            maybe_fhir_primitive: false,
        },
        r#type => {
            let primitive = PRIMITIVES.contains(&r#type);
            RustFieldType {
                name: format!("super::super::types::{}", r#type.to_rust_type_casing()),
                r#box: !primitive || force_box,
                maybe_fhir_primitive: primitive,
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
