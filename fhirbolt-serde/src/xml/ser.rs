//! Serialize FHIR resources to XML.

use std::{
    borrow::Cow,
    io,
    mem::{self},
};

use serde::ser::{self, Impossible, Serialize};

use crate::{
    number::{NumberValueEmitter, NUMBER_TOKEN},
    xml::{
        error::{Error, Result},
        event::{Element, Event},
        write::{self, Write},
    },
    SerializeResource,
};

/// Serialize the given resource as XML into the IO stream.
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: SerializeResource,
{
    value
        .context(false, T::FHIR_RELEASE)
        .serialize(&mut Serializer::new(writer))
}

/// Serialize the given resource as a XML byte vector.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: SerializeResource,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given resource as a String of JSON.
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: SerializeResource,
{
    let vec = to_vec(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

#[derive(Debug)]
struct ElementState {
    element: Element,
    // we can not write extension (and extension suboridnates) immeadiateley because value and url primitives come after
    postponed_children: Vec<ElementState>,
    next_field: Option<String>,
    is_resource: bool,
    in_sequence: bool,
    in_extension: bool,
    was_written: bool,
}

impl ElementState {
    fn new<S: Into<Cow<'static, str>>>(name: S, is_resource: bool) -> Self {
        ElementState {
            element: Element::new(name),
            postponed_children: Vec::new(),
            next_field: None,
            is_resource,
            in_sequence: false,
            in_extension: false,
            was_written: false,
        }
    }

    fn is_empty_element(&self) -> bool {
        if self.is_resource {
            self.postponed_children.is_empty() && self.element.id.is_none()
        } else if self.element.name == "extension" {
            self.postponed_children.is_empty() && self.element.value.is_none()
        } else {
            self.postponed_children.is_empty()
        }
    }
}

pub struct Serializer<W: Write> {
    writer: W,
    state_stack: Vec<ElementState>,
}

impl<W: io::Write> Serializer<write::IoWrite<W>> {
    pub fn new(writer: W) -> Self {
        Serializer {
            writer: write::IoWrite::new(writer),
            state_stack: vec![ElementState::new("", false)],
        }
    }
}

impl<W: Write> Serializer<W> {
    fn current_state(&mut self) -> &ElementState {
        self.state_stack.last().unwrap()
    }

    fn current_state_mut(&mut self) -> &mut ElementState {
        self.state_stack.last_mut().unwrap()
    }

    fn currently_in_extension(&self) -> bool {
        self.state_stack.iter().any(|s| s.in_extension)
    }

    fn enter_sequence(&mut self) {
        self.current_state_mut().in_sequence = true
    }

    fn leave_sequence(&mut self) {
        let current_state = self.current_state_mut();

        current_state.in_sequence = false;
        current_state.next_field = None;
    }

    fn enter_map(&mut self) -> Result<()> {
        // in resources extensions do not require special treatment
        // this is only neccessary for elements where value or url (which need to be written as attributes before extension can be written)
        // can appear after extensions
        if self.current_state().next_field.as_deref() == Some("extension")
            && !self.current_state().is_resource
        {
            self.current_state_mut().in_extension = true;

            if !self.current_state().in_sequence {
                self.current_state_mut().next_field = None;
            }

            self.state_stack.push(ElementState::new("extension", false))
        } else if self.current_state().next_field.is_some() {
            let currently_in_extension = self.currently_in_extension();

            if !self.current_state().was_written && !currently_in_extension {
                self.write_current_element_start()?;
            }

            let name = self.current_state_mut().next_field.take().unwrap();

            if self.current_state().in_sequence {
                self.current_state_mut().next_field = Some(name.clone());
            }

            self.state_stack.push(ElementState::new(name, false))
        }

        Ok(())
    }

    fn leave_map(&mut self) -> Result<()> {
        let popped_state = self.state_stack.pop().unwrap();
        let popped_is_resource = popped_state.is_resource;

        if self.currently_in_extension() {
            self.current_state_mut()
                .postponed_children
                .push(popped_state);
            self.current_state_mut().in_extension = false;
        } else {
            if popped_state.was_written {
                self.write_end()?;
            } else {
                if popped_state.is_empty_element() {
                    self.write_empty(popped_state)?;
                } else {
                    self.write_popped_element_start(popped_state)?;
                    self.write_end()?;
                }
            }
        }

        // nested resources are contained within another element which has to be popped as well
        if popped_is_resource && self.state_stack.len() > 1 {
            self.leave_map()?;
        }

        Ok(())
    }

    fn put<S: Into<String>>(&mut self, value: S) -> Result<()> {
        let value = value.into();

        if self.current_state().next_field.is_none() {
            self.current_state_mut().next_field = Some(value)
        } else {
            match (self.current_state().next_field.as_deref().unwrap(), value) {
                ("resourceType", resource_name) => {
                    if !self.current_state().is_resource {
                        // if not top-level resource (aka nested)
                        if self.state_stack.len() > 1 {
                            self.write_current_element_start()?;
                        }

                        self.state_stack
                            .push(ElementState::new(resource_name, true));
                    }
                }
                ("id", value) => self.current_state_mut().element.id = Some(value),
                ("url", value) => self.current_state_mut().element.url = Some(value),
                ("value", value) => self.current_state_mut().element.value = Some(value),
                _ => (),
            }

            if !self.current_state().in_sequence {
                self.current_state_mut().next_field = None;
            }
        }

        Ok(())
    }

    fn write_current_element_start(&mut self) -> Result<()> {
        let element = mem::take(&mut self.current_state_mut().element);
        let postponed_children = mem::take(&mut self.current_state_mut().postponed_children);
        let is_resource = self.current_state().is_resource;

        self.write_start(element, postponed_children, is_resource)?;

        self.current_state_mut().was_written = true;

        Ok(())
    }

    fn write_popped_element_start(&mut self, popped_state: ElementState) -> Result<()> {
        self.write_start(
            popped_state.element,
            popped_state.postponed_children,
            popped_state.is_resource,
        )
    }

    fn write_start(
        &mut self,
        mut element: Element,
        postponed_children: Vec<ElementState>,
        is_resource: bool,
    ) -> Result<()> {
        // if resource id is its on element, otherwise an attribute
        let resource_id = if is_resource { element.id.take() } else { None };

        self.writer.write_event(Event::ElementStart(element))?;

        if resource_id.is_some() {
            self.writer.write_event(Event::EmptyElement(Element {
                name: "id".into(),
                id: None,
                url: None,
                value: resource_id,
            }))?;
        }

        for postponed in postponed_children {
            self.write_extension(postponed)?;
        }

        Ok(())
    }

    fn write_end(&mut self) -> Result<()> {
        self.writer.write_event(Event::ElementEnd)
    }

    fn write_empty(&mut self, mut element_state: ElementState) -> Result<()> {
        let element = mem::take(&mut element_state.element);

        if element.name == "div" {
            self.writer.write_event(Event::Div(element))
        } else {
            self.writer.write_event(Event::EmptyElement(element))
        }
    }

    fn write_extension(&mut self, extension: ElementState) -> Result<()> {
        if extension.postponed_children.is_empty() {
            self.writer
                .write_event(Event::EmptyElement(extension.element))?;
        } else {
            self.writer
                .write_event(Event::ElementStart(extension.element))?;

            for postponed in extension.postponed_children {
                self.write_extension(postponed)?;
            }

            self.writer.write_event(Event::ElementEnd)?;
        }

        Ok(())
    }
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        if v {
            self.put("true")
        } else {
            self.put("false")
        }
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.put(v.to_string())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.put(v.to_string())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.put(v.to_string())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.put(v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.put(v.to_string())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.put(v.to_string())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.put(v.to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.put(v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.put(v.to_string())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.put(v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.put(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.put(v.to_string())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.enter_sequence();
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.enter_map()?;

        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        // serde_json Number is the only struct that cann occur
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl<'a, W: Write> ser::SerializeSeq for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        self.leave_sequence();
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeMap for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.leave_map()
    }
}

impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if key == NUMBER_TOKEN {
            self.put(
                value
                    .serialize(NumberValueEmitter)
                    .map_err(|err| Error::Message(err.to_string()))?,
            )
        } else {
            Err(Error::Message("expected decimal".into()))
        }
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
