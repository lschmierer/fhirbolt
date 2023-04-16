use std::{io, str};

use crate::xml::{
    consts::{FHIR_NAMESPACE, VALID_XML_STANDALONE, XHTML_NAMESPACE, XML_ENCODING, XML_VERSION},
    error::{Error, Result},
    event::{Element, Event},
};

#[derive(Default)]
struct QuickXmlEventMapper {
    scratch_div_element: Option<Element>,
    nested_div_count: usize,
}

impl QuickXmlEventMapper {
    fn map_event<R>(
        &mut self,
        reader: &mut quick_xml::NsReader<R>,
        event: quick_xml::events::Event,
    ) -> Result<Option<Event>> {
        match event {
            quick_xml::events::Event::Decl(decl) => {
                if let Ok(v) = decl.version() {
                    if v != XML_VERSION.as_bytes() {
                        return Err(Error::InvalidXmlVersion(Some(
                            str::from_utf8(&v)?.to_owned(),
                        )));
                    }
                } else {
                    return Err(Error::InvalidXmlVersion(None));
                }

                if let Some(Ok(e)) = decl.encoding() {
                    if e != XML_ENCODING.as_bytes() {
                        return Err(Error::InvalidXmlEncoding(str::from_utf8(&e)?.to_owned()));
                    }
                }

                if let Some(Ok(e)) = decl.standalone() {
                    if e != VALID_XML_STANDALONE.as_bytes() {
                        return Err(Error::InvalidXmlStandalone(str::from_utf8(&e)?.to_owned()));
                    }
                }

                Ok(None)
            }
            quick_xml::events::Event::Start(start) => {
                let (namespace, local_name) = reader.resolve_element(start.name());

                if self.scratch_div_element.is_some() {
                    if start.name().as_ref() == b"div" {
                        self.nested_div_count += 1;
                    }

                    self.push_to_scratch_div(&format!("<{}>", str::from_utf8(&start)?));

                    Ok(None)
                } else if local_name.as_ref() == b"div" {
                    let mut div_element = self.map_bytes_start(
                        reader,
                        &start,
                        local_name,
                        namespace,
                        XHTML_NAMESPACE,
                    )?;

                    div_element.value =
                        Some("<div xmlns=\"http://www.w3.org/1999/xhtml\">".to_string());

                    self.scratch_div_element = Some(div_element);
                    self.nested_div_count = 1;

                    Ok(None)
                } else {
                    let element = self.map_bytes_start(
                        reader,
                        &start,
                        local_name,
                        namespace,
                        FHIR_NAMESPACE,
                    )?;

                    Ok(Some(Event::ElementStart(element)))
                }
            }
            quick_xml::events::Event::End(end) => {
                if self.scratch_div_element.is_some() {
                    if end.name().as_ref() == b"div" && self.nested_div_count > 0 {
                        self.nested_div_count -= 1;
                    }

                    if end.name().as_ref() == b"div" && self.nested_div_count == 0 {
                        Ok(self
                            .scratch_div_element
                            .take()
                            .map(|mut div_element| {
                                div_element.value.as_mut().unwrap().push_str("</div>");
                                div_element
                            })
                            .map(Event::EmptyElement))
                    } else {
                        self.push_to_scratch_div(&format!("</{}>", str::from_utf8(&end)?));

                        Ok(None)
                    }
                } else {
                    Ok(Some(Event::ElementEnd))
                }
            }
            quick_xml::events::Event::Empty(start) => {
                let (namespace, local_name) = reader.resolve_element(start.name());

                if self.scratch_div_element.is_some() {
                    self.push_to_scratch_div(&format!("<{}/>", str::from_utf8(&start)?));

                    Ok(None)
                } else {
                    let element = self.map_bytes_start(
                        reader,
                        &start,
                        local_name,
                        namespace,
                        FHIR_NAMESPACE,
                    )?;

                    Ok(Some(Event::EmptyElement(element)))
                }
            }
            quick_xml::events::Event::Text(text) => {
                if self.scratch_div_element.is_some() {
                    self.push_to_scratch_div(str::from_utf8(&text)?);

                    Ok(None)
                } else {
                    Err(Error::InvalidXmlEvent("text"))
                }
            }
            quick_xml::events::Event::Comment(_) => Ok(None),
            quick_xml::events::Event::Eof => Ok(Some(Event::Eof)),
            quick_xml::events::Event::CData(_) => Err(Error::InvalidXmlEvent("CData")),
            quick_xml::events::Event::PI(_) => {
                Err(Error::InvalidXmlEvent("processing instruction"))
            }
            quick_xml::events::Event::DocType(_) => Err(Error::InvalidXmlEvent("Doctype")),
        }
    }

    fn push_to_scratch_div(&mut self, text: &str) {
        if let Some(div) = self.scratch_div_element.as_mut() {
            let xhtml = div.value.get_or_insert(String::new());
            xhtml.push_str(text);
        }
    }

    fn map_bytes_start<R>(
        &mut self,
        reader: &quick_xml::NsReader<R>,
        start: &quick_xml::events::BytesStart,
        local_name: quick_xml::name::LocalName,
        namespace: quick_xml::name::ResolveResult,
        expected_namespace: &str,
    ) -> Result<Element> {
        if namespace
            != quick_xml::name::ResolveResult::Bound(quick_xml::name::Namespace(
                expected_namespace.as_bytes(),
            ))
        {
            return Err(Error::InvalidXmlNamespace(expected_namespace.to_string()));
        }

        let mut element = Element::new(str::from_utf8(local_name.as_ref())?.to_owned());

        for attr in start.attributes() {
            let attr = attr?;

            let (namespace, local_name) = reader.resolve_attribute(attr.key);

            if namespace
                != quick_xml::name::ResolveResult::Bound(quick_xml::name::Namespace(
                    expected_namespace.as_bytes(),
                ))
                && namespace != quick_xml::name::ResolveResult::Unbound
            {
                return Err(Error::InvalidXmlNamespace(expected_namespace.to_string()));
            }

            match local_name.as_ref() {
                b"id" => element.id = Some(attr.unescape_value()?.to_string()),
                b"url" => element.url = Some(attr.unescape_value()?.to_string()),
                b"value" => element.value = Some(attr.unescape_value()?.to_string()),
                _ => (),
            }
        }

        Ok(element)
    }
}

/// Trait for iterating over XML input.
pub trait Read {
    fn next_event(&mut self) -> Result<Event>;
}

pub struct IoRead<R: io::Read> {
    reader: quick_xml::NsReader<io::BufReader<R>>,
    buf: Vec<u8>,
    mapper: QuickXmlEventMapper,
}

impl<R: io::Read> IoRead<R> {
    pub fn new(reader: R) -> Self {
        let mut reader = quick_xml::NsReader::from_reader(io::BufReader::new(reader));
        reader.trim_text(true);

        IoRead {
            reader,
            buf: Vec::new(),
            mapper: Default::default(),
        }
    }
}

impl<R: io::Read> Read for IoRead<R> {
    fn next_event(&mut self) -> Result<Event> {
        loop {
            let quick_event = self.reader.read_event_into(&mut self.buf)?;
            let event = self.mapper.map_event(&mut self.reader, quick_event);

            self.buf.clear();

            if let Some(event) = event? {
                return Ok(event);
            }
        }
    }
}

pub struct SliceRead<'a> {
    reader: quick_xml::NsReader<&'a [u8]>,
    mapper: QuickXmlEventMapper,
}

impl<'a> SliceRead<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        let mut reader = quick_xml::NsReader::from_reader(slice);
        reader.trim_text(true);

        SliceRead {
            reader,
            mapper: Default::default(),
        }
    }
}

impl<'a> Read for SliceRead<'a> {
    fn next_event(&mut self) -> Result<Event> {
        loop {
            let quick_event = self.reader.read_event()?;
            let event = self.mapper.map_event(&mut self.reader, quick_event);

            if let Some(event) = event? {
                return Ok(event);
            }
        }
    }
}

pub struct StrRead<'a> {
    read: SliceRead<'a>,
}

impl<'a> StrRead<'a> {
    pub fn new(s: &'a str) -> Self {
        StrRead {
            read: SliceRead::new(s.as_bytes()),
        }
    }
}

impl<'a> Read for StrRead<'a> {
    fn next_event(&mut self) -> Result<Event> {
        self.read.next_event()
    }
}
