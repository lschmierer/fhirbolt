use std::collections::HashMap;

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

const SKIP: &[&str] = &["Resource", "DomainResource"];

#[derive(Default)]
pub struct RustModule {
    pub name: String,
    pub structs: Vec<RustStruct>,
    pub enums: Vec<RustEnum>,
}

pub struct RustStruct {
    pub name: String,
    pub fields: Vec<RustStructField>,
}

pub struct RustStructField {
    pub name: String,
    pub r#type: String,
    pub multiple: bool,
    pub optional: bool,
    pub r#box: bool,
}

pub struct RustEnum {
    pub name: String,
    pub variants: Vec<RustEnumVariant>,
}

pub struct RustEnumVariant {
    pub name: String,
    pub r#type: String,
    pub r#box: bool,
}

pub fn gather_all_modules<'a>(bundle: &Bundle) -> Vec<RustModule> {
    flatten_bundle(bundle)
        .filter(|s| !SKIP.contains(&s.name.as_str()))
        .map(|s| {
            let mut module = RustModule {
                name: s.name.to_rust_mod_casing(),
                ..Default::default()
            };
            gather_struct(
                &mut module,
                &s.name,
                &s.snapshot.element.iter().collect::<Vec<_>>(),
                &s.r#type,
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
    element_definitions: &[&ElementDefinition],
    element_path_strip_prefix: &str,
) {
    let fields = gather_fields(module, name, element_definitions, element_path_strip_prefix);

    module.structs.push(RustStruct {
        name: name.to_rust_type_casing(),
        fields,
    })
}

fn gather_fields(
    module: &mut RustModule,
    struct_name: &str,
    element_definitions: &[&ElementDefinition],
    element_path_strip_prefix: &str,
) -> Vec<RustStructField> {
    let mut element_definition_grouped = HashMap::<_, Vec<_>>::new();

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
                    &element_definitions,
                    &element_definitions[0].path,
                );
            }

            gather_field(
                module,
                struct_name,
                &element_definitions[0],
                element_path_strip_prefix,
            )
        })
        .collect()
}

fn gather_field(
    module: &mut RustModule,
    struct_name: &str,
    element_definition: &ElementDefinition,
    element_path_strip_prefix: &str,
) -> RustStructField {
    let field_name = &element_definition.path[element_path_strip_prefix.len() + 1..];

    let (field_name, polymorph) = {
        if field_name.ends_with("[x]") {
            (&field_name[..field_name.len() - 3], true)
        } else {
            (field_name, false)
        }
    };

    let (type_name, r#box) = if polymorph {
        let type_name = format!("{}{}", struct_name, field_name.to_rust_type_casing());

        let variants = element_definition
            .r#type
            .as_ref()
            .unwrap()
            .iter()
            .map(|t| t.code.as_ref())
            .collect();

        create_value_enum(module, &type_name, variants);

        (type_name, false)
    } else {
        if let Some(types) = element_definition.r#type.as_ref() {
            let code = &types.first().unwrap().code;
            if code == "BackboneElement" {
                (
                    format!("{}{}", struct_name, field_name.to_rust_type_casing()),
                    false,
                )
            } else {
                match_field_type(code, false)
            }
        } else {
            (
                map_content_reference(element_definition.content_reference.as_ref().unwrap()),
                false,
            )
        }
    };

    RustStructField {
        name: field_name.to_rust_identifier_casing(),
        r#type: type_name,
        multiple: element_definition.max.as_ref().unwrap() != "1",
        optional: element_definition.min.unwrap() == 0,
        r#box,
    }
}

fn create_value_enum<'a>(module: &mut RustModule, enum_name: &str, types: Vec<&str>) {
    let variants = types
        .iter()
        .map(|t| {
            let (type_name, r#box) = match_field_type(t, true);

            RustEnumVariant {
                // type like http://hl7.org/fhirpath/System.String
                name: t.rsplit(".").next().unwrap().to_rust_type_casing(),
                r#type: type_name,
                r#box,
            }
        })
        .collect();

    module.enums.push(RustEnum {
        name: enum_name.into(),
        variants,
    })
}

fn match_field_type(code: &str, force_box: bool) -> (String, bool) {
    // type like http://hl7.org/fhirpath/System.String
    match code.rsplit("/").next().unwrap() {
        "System.Boolean" => ("bool".into(), false),
        "System.Integer" => ("u32".into(), false),
        "System.String" => ("std::string::String".into(), false),
        "System.Decimal" => ("rust_decimal::Decimal".into(), false),
        "System.Date" => ("chrono::naive::NaiveDate".into(), false),
        "System.DateTime" => ("chrono::DateTime<chrono::Utc>".into(), false),
        "System.Time" => ("chrono::naive::NaiveTime".into(), false),
        "Resource" => ("super::Resource".into(), true),
        r#type => (
            format!("super::super::types::{}", r#type.to_rust_type_casing()),
            !PRIMITIVES.contains(&r#type) || force_box,
        ),
    }
}

fn map_content_reference(content_reference: &str) -> String {
    content_reference[1..]
        .split(".")
        .map(|s| s.to_rust_type_casing())
        .collect::<String>()
}
