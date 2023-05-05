//! Module to track paths in the FHIR data model.

use std::mem;

use crate::{
    element_map::{self, ElementMap, ElementSet},
    type_hints::{self, TypeHints},
    FhirRelease, FhirReleases,
};

const RESOURCE_COMMON_PRIMITIVE_FIELDS: &[&str] = &["id", "implicitRules", "language"];
const COMMON_SEQUENCE_FIELDS: &[&str] = &["extension", "modifierExtension"];

fn type_hints(fhir_release: FhirRelease) -> &'static TypeHints {
    match fhir_release {
        FhirReleases::R4 => &type_hints::r4::TYPE_HINTS,
        FhirReleases::R4B => &type_hints::r4b::TYPE_HINTS,
        FhirReleases::R5 => &type_hints::r5::TYPE_HINTS,
        _ => panic!("invalid FHIR release"),
    }
}

fn element_map(fhir_release: FhirRelease) -> &'static ElementMap {
    match fhir_release {
        FhirReleases::R4 => &element_map::r4::ELEMENT_MAP,
        FhirReleases::R4B => &element_map::r4b::ELEMENT_MAP,
        FhirReleases::R5 => &element_map::r5::ELEMENT_MAP,
        _ => panic!("invalid FHIR release"),
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

/// ElementPath is aware of the FHIR data model and tracks its position in the tree.
///
/// It can be used to query type information of its current path.
#[derive(Debug, Clone)]
pub struct ElementPath {
    fhir_release: FhirRelease,
    type_stack: TypeStack,
}

impl ElementPath {
    #[inline]
    pub fn new(fhir_release: FhirRelease) -> ElementPath {
        ElementPath {
            fhir_release,
            type_stack: TypeStack::new(fhir_release),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.type_stack.is_empty()
    }

    #[inline]
    pub fn current_element(&self) -> Option<&str> {
        self.current_type_path().split().last()
    }

    #[inline]
    pub fn current_element_is_resource(&self) -> bool {
        self.resolve_current_type() == Some("Resource")
    }

    #[inline]
    pub fn current_element_is_primitive(&self) -> bool {
        let current_type_path = self.current_type_path();

        let common_resource_primitive = current_type_path.len() == 2
            && current_type_path
                .split()
                .last()
                .map(|s| RESOURCE_COMMON_PRIMITIVE_FIELDS.contains(&s))
                .unwrap_or(false);

        common_resource_primitive
            || self.current_element_is_boolean()
            || self.current_element_is_integer64()
            || self.current_element_is_integer()
            || self.current_element_is_unsigned_integer()
            || self.current_element_is_positive_integer()
            || self.current_element_is_decimal()
            || type_hints(self.fhir_release)
                .other_primitives_paths
                .contains(&self.current_type_path().path)
    }

    #[inline]
    pub fn current_element_is_sequence(&self) -> bool {
        let current_type_path = self.current_type_path();

        let is_common_sequence_field = current_type_path
            .split()
            .last()
            .map(|p| COMMON_SEQUENCE_FIELDS.contains(&p))
            .unwrap_or(false);

        if is_common_sequence_field {
            return true;
        }

        let is_contained =
            current_type_path.len() == 2 && current_type_path.split().last() == Some("contained");

        if is_contained {
            return true;
        }

        type_hints(self.fhir_release)
            .array_paths
            .contains(&current_type_path.path)
    }

    #[inline]
    pub fn current_element_is_boolean(&self) -> bool {
        type_hints(self.fhir_release)
            .boolean_paths
            .contains(&self.current_type_path().path)
    }

    #[inline]
    pub fn current_element_is_integer(&self) -> bool {
        type_hints(self.fhir_release)
            .integer_paths
            .contains(&self.current_type_path().path)
    }

    #[inline]
    pub fn current_element_is_integer64(&self) -> bool {
        type_hints(self.fhir_release)
            .integer64_paths
            .contains(&self.current_type_path().path)
    }

    #[inline]
    pub fn current_element_is_unsigned_integer(&self) -> bool {
        type_hints(self.fhir_release)
            .unsigned_integer_paths
            .contains(&self.current_type_path().path)
    }

    #[inline]
    pub fn current_element_is_positive_integer(&self) -> bool {
        type_hints(self.fhir_release)
            .positive_integer_paths
            .contains(&self.current_type_path().path)
    }

    #[inline]
    pub fn current_element_is_decimal(&self) -> bool {
        type_hints(self.fhir_release)
            .decimal_paths
            .contains(&self.current_type_path().path)
    }

    #[inline]
    pub fn parent_element_is_boolean(&self) -> bool {
        if let Some(path) = self.current_type_path().parent() {
            type_hints(self.fhir_release).boolean_paths.contains(path)
        } else {
            false
        }
    }

    #[inline]
    pub fn parent_element_is_integer(&self) -> bool {
        if let Some(path) = self.current_type_path().parent() {
            type_hints(self.fhir_release).integer_paths.contains(path)
        } else {
            false
        }
    }

    #[inline]
    pub fn parent_element_is_integer64(&self) -> bool {
        if let Some(path) = self.current_type_path().parent() {
            type_hints(self.fhir_release).integer64_paths.contains(path)
        } else {
            false
        }
    }

    #[inline]
    pub fn parent_element_is_unsigned_integer(&self) -> bool {
        if let Some(path) = self.current_type_path().parent() {
            type_hints(self.fhir_release)
                .unsigned_integer_paths
                .contains(path)
        } else {
            false
        }
    }

    #[inline]
    pub fn parent_element_is_positive_integer(&self) -> bool {
        if let Some(path) = self.current_type_path().parent() {
            type_hints(self.fhir_release)
                .positive_integer_paths
                .contains(path)
        } else {
            false
        }
    }

    #[inline]
    pub fn parent_element_is_decimal(&self) -> bool {
        if let Some(path) = self.current_type_path().parent() {
            type_hints(self.fhir_release).decimal_paths.contains(path)
        } else {
            false
        }
    }

    #[inline]
    pub fn currently_in_extension(&self) -> bool {
        self.current_type_path().path.starts_with("Extension")
    }

    #[inline]
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
            None => self.type_stack.last_mut().push(element),
        }
    }

    #[inline]
    pub fn pop(&mut self) {
        self.type_stack.last_mut().pop();

        if self.type_stack.len() > 1
            && self.type_stack.last().len() <= 1
            && !self.in_contained_resource()
        {
            self.type_stack.pop();
        }
    }

    #[inline]
    pub fn children(&self) -> Option<&'static ElementSet> {
        let mut type_path = self.current_type_path().path.as_str();

        if let Some(current_type) = self.resolve_current_type() {
            if current_type != "Resource" {
                type_path = current_type;
            }
        } else if let Some(content_reference) = type_hints(self.fhir_release)
            .content_reference_paths
            .get(type_path)
        {
            type_path = content_reference;
        }

        element_map(self.fhir_release).get(type_path).copied()
    }

    #[inline]
    pub fn position_of_child(&self, child: &str) -> usize {
        if child == "resourceType"
            // on R4 ExampleScenario.instance contains a field named "resourceType"
            && !self.current_type_path().path.starts_with("ExampleScenario.instance")
            // on R5 Consent.provision contains a field named "resourceType"
            && !self.current_type_path().path.starts_with("Consent.provision")
            // on R5 Subscription.filterBy contains a field named "resourceType"
            && !self.current_type_path().path.starts_with("Subscription.filterBy")
        {
            0
        } else {
            self.children()
                .and_then(|set| set.get_index(child))
                .map(|i| i + 1)
                // move unknown to the end
                .unwrap_or(usize::MAX)
        }
    }

    fn current_type_path(&self) -> &TypePath {
        self.type_stack.last()
    }

    fn resolve_current_type(&self) -> Option<&str> {
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
            .get(&current_type_path.path)
            .copied()
    }

    fn in_contained_resource(&self) -> bool {
        let path_was_just_replaced = self.current_type_path().len() == 1;

        if !path_was_just_replaced {
            return false;
        }

        let previous_type_path = if let Some(previous) = self.type_stack.second_last() {
            previous
        } else {
            return false;
        };

        let in_contained_field = previous_type_path.split().last() == Some("contained");

        if in_contained_field {
            return true;
        }

        let in_resource = type_hints(self.fhir_release)
            .type_paths
            .get(&previous_type_path.path)
            == Some(&"Resource");

        return in_resource;
    }
}

#[derive(Debug, Clone)]
struct TypeStack {
    root: TypePath,
    stack: Vec<TypePath>,
}

impl TypeStack {
    fn new(fhir_release: FhirRelease) -> TypeStack {
        TypeStack {
            root: TypePath::empty(fhir_release),
            stack: vec![],
        }
    }

    fn push(&mut self, value: TypePath) {
        self.stack.push(value)
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty() && self.root.is_empty()
    }

    fn len(&self) -> usize {
        self.stack.len() + 1
    }

    fn last(&self) -> &TypePath {
        if let Some(last) = self.stack.last() {
            last
        } else {
            &self.root
        }
    }

    fn last_mut(&mut self) -> &mut TypePath {
        if let Some(last) = self.stack.last_mut() {
            last
        } else {
            &mut self.root
        }
    }

    fn second_last(&self) -> Option<&TypePath> {
        match self.stack.as_slice() {
            [_] => Some(&self.root),
            [.., second_last, _] => Some(second_last),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct TypePath {
    fhir_release: FhirRelease,
    path: String,
    content_reference_replacement_stack: Vec<ContentReferenceReplacement>,
}

#[derive(Debug, Clone)]
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

    fn parent(&self) -> Option<&str> {
        self.path.rsplit_once('.').map(|s| s.0)
    }

    fn split(&self) -> impl Iterator<Item = &str> {
        self.path.split('.')
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
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
            .get(&self.path)
        {
            self.content_reference_replacement_stack
                .push(ContentReferenceReplacement {
                    content_reference,
                    replaced: mem::replace(&mut self.path, content_reference.to_string()),
                })
        }

        if !self.path.is_empty() {
            self.path.push('.');
        }
        self.path.push_str(element);
    }

    fn pop(&mut self) {
        self.path.truncate(self.path.rfind('.').unwrap_or(0));

        let last_replacement = match self.content_reference_replacement_stack.last_mut() {
            Some(last_replacement) => last_replacement,
            None => return,
        };

        if last_replacement.content_reference == self.path {
            self.path = mem::take(&mut last_replacement.replaced);

            self.content_reference_replacement_stack.pop();
        };
    }
}
