use std::mem;

use crate::{
    serde_helpers::type_hints::{self, TypeHints},
    FhirRelease,
};

const RESOURCE_COMMON_PRIMITIVE_FIELDS: &[&str] = &["id", "url", "implicitRules", "language"];
const COMMON_SEQUENCE_FIELDS: &[&str] = &["extension", "modifierExtension"];

fn type_hints(fhir_release: FhirRelease) -> &'static TypeHints {
    match fhir_release {
        FhirRelease::R4 => &type_hints::r4::TYPE_HINTS,
        FhirRelease::R4B => &type_hints::r4b::TYPE_HINTS,
    }
}

trait FirstLetterUppercase {
    fn is_first_letter_uppercase(&self) -> bool;
}

impl FirstLetterUppercase for &str {
    fn is_first_letter_uppercase(&self) -> bool {
        self.chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
    }
}

#[derive(Debug)]
pub struct ElementPath {
    fhir_release: FhirRelease,
    type_stack: Vec<TypePath>,
}

impl ElementPath {
    pub fn new(fhir_release: FhirRelease) -> ElementPath {
        ElementPath {
            fhir_release,
            type_stack: vec![TypePath::empty(fhir_release)],
        }
    }

    pub fn current_element(&self) -> Option<&str> {
        self.current_type_path().split().last()
    }

    pub fn resolve_current_type(&self) -> Option<&str> {
        let current_type_path = self.current_type_path();

        match current_type_path.split().last() {
            Some("extension") => return Some("Extension"),
            Some("modifierExtension") => return Some("Extension"),
            _ => (),
        }

        if current_type_path.len() == 2 {
            match current_type_path.split().last() {
                Some("meta") => return Some("Meta"),
                Some("text") => return Some("Narrative"),
                Some("contained") => return Some("Resource"),
                _ => (),
            }
        }

        type_hints(self.fhir_release)
            .type_paths
            .get(&current_type_path.path())
            .map(|t| *t)
    }

    pub fn currently_is_empty_resource(&self) -> bool {
        self.type_stack.len() == 1
            && self.type_stack[0].len() == 1
            && self.type_stack[0].path().is_first_letter_uppercase()
    }

    pub fn current_element_is_resource(&self) -> bool {
        let contained_resource = self.type_stack.len() >= 2
            && (self.type_stack[self.type_stack.len() - 2]
                .split()
                .last()
                .unwrap()
                == "contained"
                || type_hints(self.fhir_release)
                    .type_paths
                    .get(&self.type_stack[self.type_stack.len() - 2].path())
                    == Some(&"Resource"))
            && self.type_stack.last().unwrap().len() == 1;

        contained_resource
    }

    pub fn current_element_is_primitive(&self) -> bool {
        let current_type_path = self.current_type_path();

        self.current_element_is_boolean()
            || self.current_element_is_integer()
            || self.current_element_is_unsigned_integer()
            || self.current_element_is_positive_integer()
            || self.current_element_is_decimal()
            || type_hints(self.fhir_release)
            .other_primitives_paths
            .contains(&self.current_type_path().path())

            // check if field of resource
            || (current_type_path.len() == 2
            && RESOURCE_COMMON_PRIMITIVE_FIELDS.contains(&current_type_path.split().last().unwrap()))
    }

    pub fn current_element_is_sequence(&self) -> bool {
        let current_type_path = self.current_type_path();

        current_type_path.split()
            .last()
            .map(|p| COMMON_SEQUENCE_FIELDS.contains(&p))
            .unwrap_or(false)
            // Resource.contained
            || current_type_path.len() == 2 && current_type_path.split().last().unwrap() == "contained"
            || type_hints(self.fhir_release)
            .array_paths
            .contains(&current_type_path.path())
    }

    pub fn current_element_is_boolean(&self) -> bool {
        type_hints(self.fhir_release)
            .boolean_paths
            .contains(&self.current_type_path().path())
    }

    pub fn current_element_is_integer(&self) -> bool {
        type_hints(self.fhir_release)
            .integer_paths
            .contains(&self.current_type_path().path())
    }

    pub fn current_element_is_unsigned_integer(&self) -> bool {
        type_hints(self.fhir_release)
            .unsigned_integer_paths
            .contains(&self.current_type_path().path())
    }

    pub fn current_element_is_positive_integer(&self) -> bool {
        type_hints(self.fhir_release)
            .positive_integer_paths
            .contains(&self.current_type_path().path())
    }

    pub fn current_element_is_decimal(&self) -> bool {
        type_hints(self.fhir_release)
            .decimal_paths
            .contains(&self.current_type_path().path())
    }

    pub fn push(&mut self, element: &str) {
        match self.resolve_current_type() {
            Some("Resource") => self
                .type_stack
                .push(TypePath::new(element, self.fhir_release)),
            Some(ty) => {
                let mut type_path = TypePath::new(ty, self.fhir_release);
                type_path.push(element);
                self.type_stack.push(type_path);
            }
            None => self.type_stack.last_mut().unwrap().push(element),
        }
    }

    pub fn pop(&mut self) {
        self.type_stack.last_mut().unwrap().pop();

        if self.type_stack.len() > 1
            && self.type_stack.last().unwrap().len() <= 1
            && !self.current_element_is_resource()
        {
            self.type_stack.pop();
        }
    }

    fn current_type_path(&self) -> &TypePath {
        self.type_stack.last().unwrap()
    }
}

#[derive(Debug)]
struct TypePath {
    fhir_release: FhirRelease,
    path: String,
    content_reference_replacement_stack: Vec<ContentReferenceReplacement>,
}

#[derive(Debug)]
struct ContentReferenceReplacement {
    content_reference: &'static str,
    replaced: String,
}

impl TypePath {
    fn new(typ_name: &str, fhir_release: FhirRelease) -> TypePath {
        TypePath {
            fhir_release,
            path: typ_name.to_string(),
            content_reference_replacement_stack: vec![],
        }
    }

    fn empty(fhir_release: FhirRelease) -> TypePath {
        TypePath {
            fhir_release,
            path: String::new(),
            content_reference_replacement_stack: vec![],
        }
    }

    fn path(&self) -> &str {
        &self.path
    }

    fn split(&self) -> impl Iterator<Item = &str> {
        self.path.split(".")
    }

    fn len(&self) -> usize {
        if self.path.is_empty() {
            0
        } else {
            1 + self.path.chars().filter(|c| *c == '.').count()
        }
    }

    fn push(&mut self, element: &str) {
        if let Some(content_reference) = type_hints(self.fhir_release)
            .content_reference_paths
            .get(self.path())
        {
            self.content_reference_replacement_stack
                .push(ContentReferenceReplacement {
                    content_reference,
                    replaced: mem::replace(&mut self.path, content_reference.to_string()),
                })
        }

        if !self.path.is_empty() {
            self.path.push_str(".");
        }
        self.path.push_str(&element);
    }

    fn pop(&mut self) {
        if self
            .content_reference_replacement_stack
            .last()
            .map(|r| r.content_reference)
            == Some(&self.path)
        {
            self.path = self
                .content_reference_replacement_stack
                .pop()
                .unwrap()
                .replaced
        }

        self.path.truncate(self.path.rfind(".").unwrap_or(0));
    }
}
