//! Serialize FHIR resources to XML.

use std::{borrow::Cow, io, mem};

use serde::ser::{self, Impossible, Serialize, SerializeMap, SerializeStruct};

use crate::{
    context::{ser::SerializationConfig, Format},
    decimal,
    utils::StrSerializer,
    xml::{
        error::{Error, Result},
        event::{Element, Event},
        write::{IoWrite, Write},
    },
    SerializeResource,
};

/// Serialize the given resource as XML into the IO stream.
pub fn to_writer<W, T>(writer: W, value: &T, config: Option<SerializationConfig>) -> Result<()>
where
    W: io::Write,
    T: SerializeResource,
{
    value
        .serialization_context(config.unwrap_or(Default::default()), Format::Xml)
        .serialize(&mut Serializer::new(IoWrite::new(writer)))
}

/// Serialize the given resource as a XML byte vector.
pub fn to_vec<T>(value: &T, config: Option<SerializationConfig>) -> Result<Vec<u8>>
where
    T: SerializeResource,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value, config)?;
    Ok(writer)
}

/// Serialize the given resource as a String of JSON.
pub fn to_string<T>(value: &T, config: Option<SerializationConfig>) -> Result<String>
where
    T: SerializeResource,
{
    let vec = to_vec(value, config)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

fn expected_a_resource() -> Error {
    ser::Error::custom("expected a resource")
}

fn expected_element_or_attribute() -> Error {
    ser::Error::custom("expected element or attribute")
}

pub struct Serializer<W: Write> {
    writer: W,
    element: Option<Element>,
}

impl<W: Write> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Serializer {
            writer,
            element: None,
        }
    }

    fn write_start(&mut self, element: Element) -> Result<()> {
        self.writer.write_event(Event::ElementStart(element))
    }

    fn write_end(&mut self) -> Result<()> {
        self.writer.write_event(Event::ElementEnd)
    }

    fn write_empty(&mut self, element: Element) -> Result<()> {
        if element.name == "div" {
            self.writer.write_event(Event::Div(element))
        } else {
            self.writer.write_event(Event::EmptyElement(element))
        }
    }

    fn serialize_attribute<S: ToString>(&mut self, name: &str, s: S) -> Result<Option<String>> {
        if let Some(element) = &mut self.element {
            match name {
                "id" => element.id = Some(s.to_string()),
                "url" => element.url = Some(s.to_string()),
                "value" => element.value = Some(s.to_string()),
                _ => (),
            }
        }
        Ok(None)
    }
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = SerializeTopLevelResource<'a, W>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    #[inline]
    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_str(self, _v: &str) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    fn serialize_bytes(self, __v: &[u8]) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(expected_a_resource())
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        Err(expected_a_resource())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(expected_a_resource())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(expected_a_resource())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(expected_a_resource())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(expected_a_resource())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(expected_a_resource())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(expected_a_resource())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeTopLevelResource::new(self))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(expected_a_resource())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(expected_a_resource())
    }
}

pub struct SerializeTopLevelResource<'a, W: Write> {
    ser: &'a mut Serializer<W>,
    key: Option<String>,
}

impl<'a, W: Write> SerializeTopLevelResource<'a, W> {
    fn new(ser: &'a mut Serializer<W>) -> Self {
        SerializeTopLevelResource { ser, key: None }
    }
}

impl<'a, W: Write> SerializeMap for SerializeTopLevelResource<'a, W> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.key = Some(key.serialize(StrSerializer::<Error>::new())?);

        Ok(())
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key = match self.key.take() {
            Some(k) => k,
            None => return Err(ser::Error::custom("expected a key")),
        };

        if key == "resourceType" {
            self.ser.write_start(Element {
                name: value.serialize(StrSerializer::<Error>::new())?,
                id: None,
                url: None,
                value: None,
            })?;
        } else {
            let _ =
                value.serialize(&mut ElementOrAttributeSerializer::new(self.ser, key.into()))?;
        }

        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        self.ser.write_end()
    }
}

struct ElementOrAttributeSerializer<'a, 'b, W: Write> {
    ser: &'a mut Serializer<W>,
    name: Cow<'b, str>,
}

impl<'a, 'b, W: Write> ElementOrAttributeSerializer<'a, 'b, W> {
    fn new(ser: &'a mut Serializer<W>, name: Cow<'b, str>) -> Self {
        ElementOrAttributeSerializer { ser, name }
    }
}

impl<'a, W: Write> ser::Serializer for &'a mut ElementOrAttributeSerializer<'a, '_, W> {
    type Ok = Option<String>;
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Impossible<Self::Ok, Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Error>;
    type SerializeMap = SerializeElement<'a, W>;
    type SerializeStruct = SerializeDecimal<'a, W>;
    type SerializeStructVariant = Impossible<Self::Ok, Error>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.ser.serialize_attribute(&self.name, v)
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.ser.serialize_attribute(&self.name, v)
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.ser.serialize_attribute(&self.name, v)
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.ser.serialize_attribute(&self.name, v)
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.ser.serialize_attribute(&self.name, v)
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.ser.serialize_attribute(&self.name, v)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.ser.serialize_attribute(&self.name, v)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.ser.serialize_attribute(&self.name, v)
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.ser.serialize_attribute(&self.name, v)
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.ser.serialize_attribute(&self.name, v)
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.ser.serialize_attribute(&self.name, v)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.ser.serialize_attribute(&self.name, v)
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        if self.name == "resourceType" {
            Ok(Some(v.to_string()))
        } else {
            self.ser.serialize_attribute(&self.name, v)
        }
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        Err(expected_element_or_attribute())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(expected_element_or_attribute())
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(expected_element_or_attribute())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(expected_element_or_attribute())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        Err(expected_element_or_attribute())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(expected_element_or_attribute())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(expected_element_or_attribute())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(expected_element_or_attribute())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(expected_element_or_attribute())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(expected_element_or_attribute())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        SerializeElement::new(self.ser, mem::take(&mut self.name).into_owned())
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        // Decimal is the only struct that cann occur
        Ok(SerializeDecimal::new(self.ser))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(expected_element_or_attribute())
    }
}

impl<'a, W: Write> ser::SerializeSeq for &'a mut ElementOrAttributeSerializer<'_, '_, W> {
    type Ok = Option<String>;
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let _ = value.serialize(&mut ElementOrAttributeSerializer::new(
            self.ser,
            self.name.as_ref().into(),
        ))?;

        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(None)
    }
}

struct SerializeElement<'a, W: Write> {
    ser: &'a mut Serializer<W>,
    key: Option<String>,
    in_resource: bool,
}

impl<'a, W: Write> SerializeElement<'a, W> {
    fn new(ser: &'a mut Serializer<W>, name: String) -> Result<Self> {
        if let Some(element) = ser.element.take() {
            ser.write_start(element)?;
        }

        ser.element = Some(Element::new(name));

        Ok(SerializeElement {
            ser,
            key: None,
            in_resource: false,
        })
    }
}

impl<'a, W: Write> SerializeMap for SerializeElement<'a, W> {
    type Ok = Option<String>;
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.key = Some(key.serialize(StrSerializer::<Error>::new())?);

        Ok(())
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key = match self.key.take() {
            Some(k) => k,
            None => return Err(ser::Error::custom("expected a key")),
        };

        if let Some(resource_type) =
            value.serialize(&mut ElementOrAttributeSerializer::new(self.ser, key.into()))?
        {
            // check self.in_resource in case resourceType is written twice
            if self.in_resource {
                return Ok(());
            }

            // write containing element start
            if let Some(element) = self.ser.element.take() {
                self.ser.write_start(element)?;
            }

            self.ser.element = Some(Element::new(resource_type));
            self.in_resource = true;
        }

        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        if let Some(element) = self.ser.element.take() {
            self.ser.write_empty(element)?;
        } else {
            self.ser.write_end()?;
        }

        // close nested resource
        if self.in_resource {
            self.ser.write_end()?;
        }

        Ok(None)
    }
}

struct SerializeDecimal<'a, W: Write> {
    ser: &'a mut Serializer<W>,
    inner: decimal::SerializeDecimal<Error>,
}

impl<'a, W: Write> SerializeDecimal<'a, W> {
    fn new(ser: &'a mut Serializer<W>) -> Self {
        SerializeDecimal {
            ser,
            inner: decimal::SerializeDecimal::new(),
        }
    }
}

impl<'a, W: Write> SerializeStruct for SerializeDecimal<'a, W> {
    type Ok = Option<String>;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.ser.serialize_attribute("value", self.inner.end()?)?;

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use serde::ser::Serialize;

    use fhirbolt_element::{Element, FhirReleases, Primitive, Value};

    use crate::{
        context::Format,
        xml::{
            error::Result,
            event::{self, Event},
            write::Write,
        },
        SerializeResource,
    };

    use super::*;

    impl Write for &mut Vec<Event> {
        fn write_event(&mut self, event: Event) -> Result<()> {
            self.push(event);
            Ok(())
        }
    }

    #[test]
    fn test_resource_id() {
        let mut events = vec![];
        let mut ser = Serializer::new(&mut events);

        let element: Element<{ FhirReleases::R4 }> = Element! {
            "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
            "id" => Value::Element(Element! {
                "value" => Value::Primitive(Primitive::String("test_id".into())),
            })
        };
        element
            .serialization_context(Default::default(), Format::Xml)
            .serialize(&mut ser)
            .unwrap();

        assert_eq!(
            events,
            [
                Event::ElementStart(event::Element {
                    name: "Observation".into(),
                    ..Default::default()
                }),
                Event::EmptyElement(event::Element {
                    name: "id".into(),
                    value: Some("test_id".into()),
                    ..Default::default()
                }),
                Event::ElementEnd,
            ]
        );
    }

    #[test]
    fn test_element_id() {
        let mut events = vec![];
        let mut ser = Serializer::new(&mut events);

        let element: Element<{ FhirReleases::R4 }> = Element! {
            "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
            "valueString" => Value::Element(Element! {
                "id" => Value::Primitive(Primitive::String("test_id".into())),
            })
        };
        element
            .serialization_context(Default::default(), Format::Xml)
            .serialize(&mut ser)
            .unwrap();

        assert_eq!(
            events,
            [
                Event::ElementStart(event::Element {
                    name: "Observation".into(),
                    ..Default::default()
                }),
                Event::EmptyElement(event::Element {
                    name: "valueString".into(),
                    id: Some("test_id".into()),
                    ..Default::default()
                }),
                Event::ElementEnd,
            ]
        );
    }

    #[test]
    fn test_element_value() {
        let mut events = vec![];
        let mut ser = Serializer::new(&mut events);

        let element: Element<{ FhirReleases::R4 }> = Element! {
            "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
            "valueString" => Value::Element(Element! {
                "value" => Value::Primitive(Primitive::String("test_value".into())),
            })
        };
        element
            .serialization_context(Default::default(), Format::Xml)
            .serialize(&mut ser)
            .unwrap();

        assert_eq!(
            events,
            [
                Event::ElementStart(event::Element {
                    name: "Observation".into(),
                    ..Default::default()
                }),
                Event::EmptyElement(event::Element {
                    name: "valueString".into(),
                    value: Some("test_value".into()),
                    ..Default::default()
                }),
                Event::ElementEnd,
            ]
        );
    }

    #[test]
    fn test_element_url() {
        let mut events = vec![];
        let mut ser = Serializer::new(&mut events);

        let element: Element<{ FhirReleases::R4 }> = Element! {
            "resourceType" => Value::Primitive(Primitive::String("Observation".into())),
            "extension" => Value::Sequence(vec![Element! {
                "url" => Value::Primitive(Primitive::String("test_url".into())),
            }])
        };
        element
            .serialization_context(Default::default(), Format::Xml)
            .serialize(&mut ser)
            .unwrap();

        assert_eq!(
            events,
            [
                Event::ElementStart(event::Element {
                    name: "Observation".into(),
                    ..Default::default()
                }),
                Event::EmptyElement(event::Element {
                    name: "extension".into(),
                    url: Some("test_url".into()),
                    ..Default::default()
                }),
                Event::ElementEnd,
            ]
        );
    }
}
