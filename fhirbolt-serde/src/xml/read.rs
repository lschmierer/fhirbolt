use std::{io, str};

mod quick_xml {
    pub use quick_xml::{
        events::{BytesStart, Event},
        name::{LocalName, Namespace, QName, ResolveResult},
        reader::NsReader,
        Result,
    };
}

use crate::xml::error::{Error, Result};

const VALID_XML_VERSION: &[u8] = b"1.0";
const VALID_XML_ENCODING: &[u8] = b"UTF-8";
const VALID_XML_STANDALONE: &[u8] = b"none";
const BOUND_FHIR_NAMESPACE: quick_xml::ResolveResult =
    quick_xml::ResolveResult::Bound(quick_xml::Namespace(b"http://hl7.org/fhir"));
const BOUND_XHTML_NAMESPACE: quick_xml::ResolveResult =
    quick_xml::ResolveResult::Bound(quick_xml::Namespace(b"http://www.w3.org/1999/xhtml"));

#[derive(Debug)]
pub enum Event {
    ElementStart(Element),
    ElementEnd,
    EmptyElement(Element),
    Eof,
}

#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub id: Option<String>,
    pub url: Option<String>,
    pub value: Option<String>,
}

impl Element {
    fn new(name: &str) -> Element {
        Element {
            name: name.to_owned(),
            id: None,
            url: None,
            value: None,
        }
    }
}

#[derive(Default)]
struct QuickXmlEventMapper {
    scratch_div_element: Option<Element>,
    nested_div_count: usize,
}

impl QuickXmlEventMapper {
    fn map_event<R>(
        &mut self,
        reader: &mut quick_xml::NsReader<R>,
        event: quick_xml::Event,
    ) -> Result<Option<Event>> {
        match event {
            quick_xml::Event::Decl(decl) => {
                if let Ok(v) = decl.version() {
                    if v != VALID_XML_VERSION {
                        return Err(Error::InvalidXmlVersion(Some(
                            str::from_utf8(&v)?.to_owned(),
                        )));
                    }
                } else {
                    return Err(Error::InvalidXmlVersion(None));
                }

                if let Some(Ok(e)) = decl.encoding() {
                    if e != VALID_XML_ENCODING {
                        return Err(Error::InvalidXmlEncoding(str::from_utf8(&e)?.to_owned()));
                    }
                }

                if let Some(Ok(e)) = decl.standalone() {
                    if e != VALID_XML_STANDALONE {
                        return Err(Error::InvalidXmlStandalone(str::from_utf8(&e)?.to_owned()));
                    }
                }

                Ok(None)
            }
            quick_xml::Event::Start(start) => {
                let (namespace, local_name) = reader.resolve_element(start.name());

                if self.scratch_div_element.is_some() {
                    if start.name().as_ref() == b"div" {
                        self.nested_div_count += 1;
                    }

                    self.push_to_scratch_div(&format!("<{}>", str::from_utf8(&start)?));

                    Ok(None)
                } else if local_name.as_ref() == b"div" && namespace == BOUND_XHTML_NAMESPACE {
                    let mut div_element = Element::new(str::from_utf8(local_name.as_ref())?);
                    div_element.value =
                        Some("<div xmlns=\"http://www.w3.org/1999/xhtml\">".to_string());

                    self.scratch_div_element = Some(div_element);
                    self.nested_div_count = 1;

                    Ok(None)
                } else {
                    let element = self.map_bytes_start(reader, &start, namespace, local_name)?;

                    Ok(element.map(Event::ElementStart))
                }
            }
            quick_xml::Event::End(end) => {
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
            quick_xml::Event::Empty(start) => {
                let (namespace, local_name) = reader.resolve_element(start.name());

                if self.scratch_div_element.is_some() {
                    self.push_to_scratch_div(&format!("<{}/>", str::from_utf8(&start)?));

                    Ok(None)
                } else {
                    let element = self.map_bytes_start(reader, &start, namespace, local_name)?;

                    Ok(element.map(Event::EmptyElement))
                }
            }
            quick_xml::Event::Text(text) => {
                if self.scratch_div_element.is_some() {
                    self.push_to_scratch_div(str::from_utf8(&text)?);

                    Ok(None)
                } else {
                    Err(Error::InvalidXmlEvent("text"))
                }
            }
            quick_xml::Event::Comment(_) => Ok(None),
            quick_xml::Event::Eof => Ok(Some(Event::Eof)),
            quick_xml::Event::CData(_) => Err(Error::InvalidXmlEvent("CData")),
            quick_xml::Event::PI(_) => Err(Error::InvalidXmlEvent("processing instruction")),
            quick_xml::Event::DocType(_) => Err(Error::InvalidXmlEvent("Doctype")),
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
        start: &quick_xml::BytesStart,
        namespace: quick_xml::ResolveResult,
        local_name: quick_xml::LocalName,
    ) -> Result<Option<Element>> {
        if namespace != BOUND_FHIR_NAMESPACE {
            return Err(Error::InvalidFhirNamespace);
        }

        let mut element = Element::new(str::from_utf8(local_name.as_ref())?);

        for attr in start.attributes() {
            let attr = attr?;

            let (namespace, local_name) = reader.resolve_attribute(attr.key);

            if namespace != BOUND_FHIR_NAMESPACE && namespace != quick_xml::ResolveResult::Unbound {
                return Err(Error::InvalidFhirNamespace);
            }

            match local_name.as_ref() {
                b"id" => element.id = Some(attr.unescape_value()?.to_string()),
                b"url" => element.url = Some(attr.unescape_value()?.to_string()),
                b"value" => element.value = Some(attr.unescape_value()?.to_string()),
                _ => (),
            }
        }

        Ok(Some(element))
    }
}

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

impl<'de, R: io::Read> Read for IoRead<R> {
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
