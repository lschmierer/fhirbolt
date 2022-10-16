use std::{collections::VecDeque, default::Default, io};

use serde::{
    de::{self, Deserialize, DeserializeOwned, DeserializeSeed, MapAccess, SeqAccess, Visitor},
    forward_to_deserialize_any,
};

use fhirbolt_shared::{
    serde_config::de::{with_config, DeserializationConfig},
    AnyResource,
};

use crate::xml::{
    error::{Error, Result},
    number::NumberDeserializer,
    path::ElementPath,
    read::{self, Element, Event, Read},
};

/// Deserialize an instance of resource type `T` directly from an IO stream of XML (e.g. coming from network).
///
/// # Example
/// ```
/// # fn main() {
/// // The `Resource` type is an enum that contains all possible FHIR resources.
/// // If the resource type is known in advance, you could also use a concrete resource type
/// // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
/// use fhirbolt::model::r4b::Resource as R4BResource;
///
/// // The type of `s` is `&str`
/// let s = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
///     <Observation xmlns=\"http://hl7.org/fhir\">
///         <status value=\"final\"/>
///         <code>
///             <text value=\"some code\"/>
///         </code>
///         <valueString value=\"some value\"/>
///     </Observation>";
///
/// // `s.as_bytes()` returns `&[u8]` which implements `std::io::Read`
/// let r: R4BResource = fhirbolt::xml::from_reader(s.as_bytes(), None).unwrap();
/// println!("{:?}", r);
/// # }
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_reader<R: io::Read, T>(rdr: R, config: Option<DeserializationConfig>) -> Result<T>
where
    T: DeserializeOwned + AnyResource,
{
    let mut deserializer = Deserializer::from_reader::<T>(rdr);
    with_config(config, || T::deserialize(&mut deserializer))
}

/// Deserialize an instance of resource type `T` from a bytes of XML.
///
/// # Example
/// ```
/// # fn main() {
/// // The `Resource` type is an enum that contains all possible FHIR resources.
/// // If the resource type is known in advance, you could also use a concrete resource type
/// // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
/// use fhirbolt::model::r4b::Resource as R4BResource;
///
/// // The type of `s` is `&[u8]`
/// let b = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>
///     <Observation xmlns=\"http://hl7.org/fhir\">
///         <status value=\"final\"/>
///         <code>
///             <text value=\"some code\"/>
///         </code>
///         <valueString value=\"some value\"/>
///     </Observation>";
///
/// let r: R4BResource = fhirbolt::xml::from_slice(b, None).unwrap();
/// println!("{:?}", r);
/// # }
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_slice<T>(v: &[u8], config: Option<DeserializationConfig>) -> Result<T>
where
    T: DeserializeOwned + AnyResource,
{
    let mut deserializer = Deserializer::from_slice::<T>(v);
    with_config(config, || T::deserialize(&mut deserializer))
}

/// Deserialize an instance of resource type `T` from a string of XML.
///
/// # Example
/// ```
/// # fn main() {
/// // The `Resource` type is an enum that contains all possible FHIR resources.
/// // If the resource type is known in advance, you could also use a concrete resource type
/// // (like e.g. `fhirbolt::model::r4b::resources::Observation`).
/// use fhirbolt::model::r4b::Resource as R4BResource;
///
/// // The type of `s` is `&str`
/// let s = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
///     <Observation xmlns=\"http://hl7.org/fhir\">
///         <status value=\"final\"/>
///         <code>
///             <text value=\"some code\"/>
///         </code>
///         <valueString value=\"some value\"/>
///     </Observation>";
///
/// let r: R4BResource = fhirbolt::xml::from_str(s, None).unwrap();
/// println!("{:?}", r);
/// # }
/// ```
///
/// # Errors
/// The conversion can fail if the structure of the input does not match the FHIR resource `T`.
/// This behavior can be modified by passing a [`DeserializationConfig`](crate::DeserializationConfig).
pub fn from_str<'a, T>(s: &'a str, config: Option<DeserializationConfig>) -> Result<T>
where
    T: Deserialize<'a> + AnyResource,
{
    let mut deserializer = Deserializer::from_str::<T>(s);
    with_config(config, || T::deserialize(&mut deserializer))
}

trait NextVecExt {
    fn any_not_empty(&self) -> bool;
    fn replace_last_empty_with_null(&mut self);
}

impl NextVecExt for Vec<Next> {
    fn any_not_empty(&self) -> bool {
        self.iter().any(|next| match next {
            Next::Value(value) => match value {
                Value::Null => false,
                _ => true,
            },
            _ => false,
        })
    }

    fn replace_last_empty_with_null(&mut self) {
        let len = self.len();

        if len >= 2 && self[len - 2] == Next::MapStart && self[len - 1] == Next::MapEnd {
            self[len - 2] = Next::Value(Value::Null);
            self.pop();
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Next {
    Key(String),
    Value(Value),
    MapStart,
    MapEnd,
    SequenceStart,
    SequenceEnd,
    Eof,
}

#[derive(Clone, PartialEq, Debug)]
enum Value {
    String(String),
    Boolean(bool),
    Integer(i32),
    UnsignedInteger(u32),
    PositiveInteger(u32),
    Decimal(String),
    Null,
}

impl Next {
    pub fn debug_str(&self) -> &'static str {
        match *self {
            Next::Key(_) => "key",
            Next::Value(_) => "value",
            Next::MapStart => "map start",
            Next::MapEnd => "map end",
            Next::SequenceStart => "sequence start",
            Next::SequenceEnd => "sequence end",
            Next::Eof => "eof",
        }
    }
}

#[derive(Debug)]
struct DeserializerState {
    next_queue: VecDeque<Next>,
    state_stack: Vec<ElementState>,
}

#[derive(Default, Debug)]
struct ElementState {
    in_sequence: Option<InSequence>,
    in_primitive: Option<PrimitiveCollector>,
}

#[derive(Debug)]
struct InSequence {
    element_name: String,
    is_primitive: bool,
}

#[derive(Default, Debug)]
struct PrimitiveCollector {
    next_value_queue: Vec<Next>,
    next_element_queue: Vec<Next>,
}

impl DeserializerState {
    fn new() -> DeserializerState {
        DeserializerState {
            next_queue: VecDeque::from(vec![Next::MapStart]),
            state_stack: vec![Default::default()],
        }
    }

    fn peek_next(&self) -> &Next {
        &self.next_queue[0]
    }

    fn pop_next(&mut self) -> Option<Next> {
        self.next_queue.pop_front()
    }

    fn push_next(&mut self, next: Next) {
        if let Some(collector) = self.last_non_empty_primitive_collector() {
            collector.next_element_queue.push(next);
            collector.next_element_queue.replace_last_empty_with_null();
        } else {
            self.next_queue.push_back(next)
        }
    }

    fn push_next_all(&mut self, next_iter: impl Iterator<Item = Next>) {
        if let Some(collector) = self.last_non_empty_primitive_collector() {
            collector.next_element_queue.extend(next_iter)
        } else {
            self.next_queue.extend(next_iter)
        }
    }

    fn last_non_empty_primitive_collector(&mut self) -> Option<&mut PrimitiveCollector> {
        self.state_stack
            .iter_mut()
            .rev()
            .map(|s| s.in_primitive.as_mut())
            .find(|p| p.is_some())
            .flatten()
    }

    fn push_next_primitive_value(&mut self, next: Next) {
        self.last_primitive_collector().next_value_queue.push(next);
    }

    fn push_next_primitive_element(&mut self, next: Next) {
        let collector = self.last_primitive_collector();

        collector.next_element_queue.push(next);
        collector.next_element_queue.replace_last_empty_with_null();
    }

    fn last_primitive_collector(&mut self) -> &mut PrimitiveCollector {
        self.state_stack
            .last_mut()
            .unwrap()
            .in_primitive
            .as_mut()
            .unwrap()
    }

    fn push_state(&mut self) {
        self.state_stack.push(Default::default());
    }

    fn pop_state(&mut self) {
        self.leave_primitive();

        self.state_stack.pop();
    }

    fn in_sequence(&self) -> Option<&InSequence> {
        self.state_stack.last().unwrap().in_sequence.as_ref()
    }

    fn enter_sequence(&mut self, element_name: &str, is_primitive: bool) {
        self.state_stack
            .last_mut()
            .unwrap()
            .in_sequence
            .replace(InSequence {
                element_name: element_name.to_string(),
                is_primitive,
            });
    }

    fn leave_sequence(&mut self) {
        self.state_stack.last_mut().unwrap().in_sequence.take();
    }

    fn enter_primitive(&mut self, is_primitive: bool) {
        self.leave_primitive();

        if is_primitive {
            self.state_stack
                .last_mut()
                .unwrap()
                .in_primitive
                .replace(Default::default());
        }
    }

    fn leave_primitive(&mut self) {
        if let Some(collector) = self.state_stack.last_mut().unwrap().in_primitive.take() {
            // push primitive value sequence if not only empty values
            if collector.next_value_queue.any_not_empty() {
                self.push_next_all(collector.next_value_queue.into_iter());
            }

            // push primitive element sequence if not only empty elements
            if collector.next_element_queue.any_not_empty() {
                self.push_next_all(collector.next_element_queue.into_iter());
            }
        }
    }
}
pub struct Deserializer<R: Read> {
    read: R,
    current_path: ElementPath,
    state: DeserializerState,
}

impl<R: Read> Deserializer<R> {
    pub fn new<T: AnyResource>(read: R) -> Self {
        Deserializer {
            read,
            current_path: ElementPath::new(T::fhir_release()),
            state: DeserializerState::new(),
        }
    }
}

impl<R: io::Read> Deserializer<read::IoRead<R>> {
    pub fn from_reader<T: AnyResource>(reader: R) -> Self {
        Deserializer::new::<T>(read::IoRead::new(reader))
    }
}

impl<'a> Deserializer<read::SliceRead<'a>> {
    pub fn from_slice<T: AnyResource>(bytes: &'a [u8]) -> Self {
        Deserializer::new::<T>(read::SliceRead::new(bytes))
    }
}

impl<'a> Deserializer<read::StrRead<'a>> {
    pub fn from_str<T: AnyResource>(s: &'a str) -> Self {
        Deserializer::new::<T>(read::StrRead::new(s))
    }
}

impl<R: Read> Deserializer<R> {
    fn advance(&mut self) -> Result<()> {
        while self.state.next_queue.is_empty() {
            let event = self.read.next_event()?;

            match event {
                Event::ElementStart(element) => {
                    self.current_path.push(&element.name);

                    if self.current_path.currently_is_empty_resource()
                        || self.current_path.current_element_is_resource()
                    {
                        self.state.push_next(Next::Key("resourceType".to_owned()));
                        self.state
                            .push_next(Next::Value(Value::String(element.name)));
                    } else {
                        self.advance_element(element, false)?;

                        self.state.push_state();
                    }
                }
                Event::ElementEnd => {
                    if !self.current_path.current_element_is_resource() {
                        if let Some(s) = self.state.in_sequence() {
                            self.advance_sequence_end(s.is_primitive);
                        }

                        self.state.pop_state();

                        self.state.push_next(Next::MapEnd);
                    }

                    self.current_path.pop();
                }
                Event::EmptyElement(element) => {
                    self.current_path.push(&element.name);

                    self.advance_element(element, true)?;

                    self.current_path.pop();
                }
                Event::Eof => self.state.push_next(Next::Eof),
            }
        }

        Ok(())
    }

    fn advance_element(&mut self, mut element: Element, is_empty: bool) -> Result<()> {
        match (
            self.state.in_sequence(),
            self.current_path.current_element_is_sequence(),
        ) {
            (Some(s), false) => {
                self.advance_sequence_end(s.is_primitive);
                self.state.leave_sequence();

                self.state
                    .enter_primitive(self.current_path.current_element_is_primitive());
                self.advance_key(&element.name);
            }
            (Some(s), true) => {
                if s.element_name != element.name {
                    self.advance_sequence_end(s.is_primitive);
                    self.state.leave_sequence();

                    self.state
                        .enter_primitive(self.current_path.current_element_is_primitive());
                    self.state.enter_sequence(
                        &element.name,
                        self.current_path.current_element_is_primitive(),
                    );
                    self.advance_key(&element.name);
                    self.advance_sequence_start();
                }
            }
            (None, true) => {
                self.state
                    .enter_primitive(self.current_path.current_element_is_primitive());
                self.state.enter_sequence(
                    &element.name,
                    self.current_path.current_element_is_primitive(),
                );
                self.advance_key(&element.name);
                self.advance_sequence_start();
            }
            (None, false) => {
                self.state
                    .enter_primitive(self.current_path.current_element_is_primitive());
                self.advance_key(&element.name);
            }
        };

        if self.current_path.current_element_is_primitive() {
            self.advance_value(&mut element.value, true)?;

            // primitive element
            self.state.push_next(Next::MapStart);

            self.advance_id_and_url(element)?;

            if is_empty {
                self.state.push_next_primitive_element(Next::MapEnd);
            }
        } else {
            self.state.push_next(Next::MapStart);

            self.advance_id_and_url(element)?;
        }

        Ok(())
    }

    fn advance_sequence_start(&mut self) {
        if self.current_path.current_element_is_primitive() {
            self.state.push_next_primitive_value(Next::SequenceStart);
            self.state.push_next_primitive_element(Next::SequenceStart);
        } else {
            self.state.push_next(Next::SequenceStart);
        }
    }

    fn advance_sequence_end(&mut self, was_primitive: bool) {
        if was_primitive {
            self.state.push_next_primitive_value(Next::SequenceEnd);
            self.state.push_next_primitive_element(Next::SequenceEnd);
        } else {
            self.state.push_next(Next::SequenceEnd);
        }
    }

    fn advance_key(&mut self, name: &str) {
        if self.current_path.current_element_is_primitive() {
            self.state
                .push_next_primitive_value(Next::Key(name.to_string()));
            self.state
                .push_next_primitive_element(Next::Key(format!("_{}", name)));
        } else {
            self.state.push_next(Next::Key(name.to_string()));
        }
    }

    fn advance_value(
        &mut self,
        value: &mut Option<String>,
        is_primitive_value: bool,
    ) -> Result<()> {
        let next = if let Some(value) = value.take() {
            if self.current_path.current_element_is_boolean() {
                Next::Value(Value::Boolean(value.parse()?))
            } else if self.current_path.current_element_is_integer() {
                Next::Value(Value::Integer(value.parse()?))
            } else if self.current_path.current_element_is_unsigned_integer() {
                Next::Value(Value::UnsignedInteger(value.parse()?))
            } else if self.current_path.current_element_is_positive_integer() {
                Next::Value(Value::PositiveInteger(value.parse()?))
            } else if self.current_path.current_element_is_decimal() {
                Next::Value(Value::Decimal(value))
            } else {
                Next::Value(Value::String(value))
            }
        } else {
            Next::Value(Value::Null)
        };

        if is_primitive_value {
            self.state.push_next_primitive_value(next);
        } else {
            self.state.push_next(next);
        };

        Ok(())
    }

    fn advance_id_and_url(&mut self, mut element: Element) -> Result<()> {
        if element.id.is_some() {
            self.advance_key("id");
            self.advance_value(&mut element.id, false)?;
        }

        if element.url.is_some() {
            self.advance_key("url");
            self.advance_value(&mut element.url, false)?;
        }

        Ok(())
    }
}

impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.advance()?;

        match self.state.pop_next().unwrap() {
            Next::Key(s) => visitor.visit_string(s),
            Next::Value(Value::String(s)) => visitor.visit_string(s),
            Next::Value(Value::Boolean(b)) => visitor.visit_bool(b),
            Next::Value(Value::Integer(i)) => visitor.visit_i32(i),
            Next::Value(Value::PositiveInteger(p)) => visitor.visit_u32(p),
            Next::Value(Value::UnsignedInteger(u)) => visitor.visit_u32(u),
            Next::Value(Value::Decimal(d)) => visitor.visit_map(NumberDeserializer::new(d)),
            Next::Value(Value::Null) => visitor.visit_none(),
            Next::MapStart => visitor.visit_map(ElementAccess::new(self)),
            Next::SequenceStart => visitor.visit_seq(ElementAccess::new(self)),
            Next::Eof => Err(Error::Eof),
            Next::MapEnd | Next::SequenceEnd => self.deserialize_any(visitor),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct ElementAccess<'a, R: Read> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: Read> ElementAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        ElementAccess { de }
    }
}

impl<'de, 'a, R: Read> SeqAccess<'de> for ElementAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        self.de.advance()?;

        match self.de.state.peek_next() {
            Next::MapStart | Next::SequenceStart | Next::Value(_) => {
                seed.deserialize(&mut *self.de).map(Some)
            }
            Next::SequenceEnd => {
                self.de.state.pop_next();
                Ok(None)
            }
            n => Err(Error::InvalidFhirEvent {
                found: n.debug_str(),
                expected: "value, map start, sequence start, sequence end",
            }),
        }
    }
}

impl<'de, 'a, R: Read> MapAccess<'de> for ElementAccess<'a, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        self.de.advance()?;

        match self.de.state.peek_next() {
            Next::Key(_) => seed.deserialize(&mut *self.de).map(Some),
            Next::MapEnd | Next::Eof => {
                self.de.state.pop_next();
                Ok(None)
            }
            n => Err(Error::InvalidFhirEvent {
                found: n.debug_str(),
                expected: "key, map end, eof",
            }),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        match self.de.state.peek_next() {
            Next::Value(_) | Next::MapStart | Next::SequenceStart => {
                seed.deserialize(&mut *self.de)
            }
            n => Err(Error::InvalidFhirEvent {
                found: n.debug_str(),
                expected: "value, map start, sequence start",
            }),
        }
    }
}
