use std::{
    io::{self, BufReader},
    str,
};

use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, Event as QuickXmlEvent},
    name::{LocalName, Namespace, QName, ResolveResult},
    reader::NsReader,
};

use crate::xml::{
    consts::{FHIR_NAMESPACE, VALID_XML_STANDALONE, XHTML_NAMESPACE, XML_ENCODING, XML_VERSION},
    error::{Error, Result},
    event::{Element, Event},
};

fn resolve_element<'n, R>(
    reader: &NsReader<R>,
    name: QName<'n>,
    expected_namespace: &str,
) -> Result<LocalName<'n>> {
    let (namespace, local_name) = reader.resolve_element(name);

    if namespace != ResolveResult::Bound(Namespace(expected_namespace.as_bytes())) {
        return Err(Error::InvalidXmlNamespace(
            match namespace {
                ResolveResult::Unbound => None,
                ResolveResult::Bound(n) => Some(str::from_utf8(n.into_inner()).unwrap().to_owned()),
                ResolveResult::Unknown(n) => Some(String::from_utf8(n).unwrap()),
            },
            expected_namespace.to_string(),
        ));
    }

    Ok(local_name)
}

fn resolve_attribute<'n, R>(
    reader: &NsReader<R>,
    name: QName<'n>,
    expected_namespace: &str,
) -> Result<LocalName<'n>> {
    let (namespace, local_name) = reader.resolve_attribute(name);

    match namespace {
        ResolveResult::Unbound => Ok(local_name),
        ResolveResult::Bound(n) if n == Namespace(expected_namespace.as_bytes()) => Ok(local_name),
        ResolveResult::Bound(n) => Err(Error::InvalidXmlNamespace(
            Some(str::from_utf8(n.into_inner()).unwrap().to_owned()),
            expected_namespace.to_string(),
        )),
        ResolveResult::Unknown(n) => Err(Error::InvalidXmlNamespace(
            Some(String::from_utf8(n).unwrap()),
            expected_namespace.to_string(),
        )),
    }
}

#[derive(Default)]
struct QuickXmlEventMapper {
    in_div_element: Option<Element>,
    nested_div_count: usize,
}

impl QuickXmlEventMapper {
    fn map_event<R>(
        &mut self,
        reader: &mut NsReader<R>,
        event: QuickXmlEvent,
    ) -> Result<Option<Event>> {
        match event {
            QuickXmlEvent::Decl(decl) => self.map_decl(decl),
            QuickXmlEvent::Start(start) => {
                if self.in_div() {
                    self.map_start_in_div(reader, start)
                } else if tri!(self.should_enter_div(reader, &start)) {
                    self.map_start_enter_div(reader, start)
                } else {
                    let element = tri!(self.map_start_element(reader, start, FHIR_NAMESPACE));
                    Ok(Some(Event::ElementStart(element)))
                }
            }
            QuickXmlEvent::End(end) => {
                if self.in_div() {
                    self.map_end_in_div(reader, end)
                } else {
                    Ok(Some(Event::ElementEnd))
                }
            }
            QuickXmlEvent::Empty(start) => {
                if self.in_div() {
                    self.map_empty_in_div(reader, start)
                } else {
                    let element = tri!(self.map_start_element(reader, start, FHIR_NAMESPACE));
                    Ok(Some(Event::EmptyElement(element)))
                }
            }
            QuickXmlEvent::Text(text) => {
                if self.in_div() {
                    self.push_to_div(str::from_utf8(&text)?);
                    Ok(None)
                } else {
                    Err(Error::InvalidXmlEvent("text outside of div"))
                }
            }
            QuickXmlEvent::Comment(_) => Ok(None),
            QuickXmlEvent::Eof => Ok(Some(Event::Eof)),
            QuickXmlEvent::CData(_) => Err(Error::InvalidXmlEvent("CData")),
            QuickXmlEvent::PI(_) => Err(Error::InvalidXmlEvent("processing instruction")),
            QuickXmlEvent::DocType(_) => Err(Error::InvalidXmlEvent("Doctype")),
        }
    }

    fn map_decl(&self, decl: BytesDecl) -> Result<Option<Event>> {
        let version = decl.version()?;
        if decl.version()? != XML_VERSION.as_bytes() {
            return Err(Error::InvalidXmlVersion(
                str::from_utf8(&version)?.to_owned(),
            ));
        }

        let encoding = decl
            .encoding()
            .transpose()?
            .unwrap_or(XML_ENCODING.as_bytes().into());
        if encoding != XML_ENCODING.as_bytes() {
            return Err(Error::InvalidXmlEncoding(
                str::from_utf8(&encoding)?.to_owned(),
            ));
        }

        let standalone = decl
            .standalone()
            .transpose()?
            .unwrap_or(VALID_XML_STANDALONE.as_bytes().into());
        if standalone != VALID_XML_STANDALONE.as_bytes() {
            return Err(Error::InvalidXmlStandalone(
                str::from_utf8(&standalone)?.to_owned(),
            ));
        }

        Ok(None)
    }

    fn map_start_element<R>(
        &mut self,
        reader: &NsReader<R>,
        start: BytesStart,
        expected_namespace: &str,
    ) -> Result<Element> {
        let local_name = tri!(resolve_element(reader, start.name(), expected_namespace));

        let mut element = Element::new(str::from_utf8(local_name.as_ref())?.to_owned());

        for attr in start.attributes() {
            let attr = attr?;
            let attr_local_name = tri!(resolve_attribute(reader, attr.key, expected_namespace));

            match attr_local_name.as_ref() {
                b"id" => element.id = Some(attr.unescape_value()?.to_string()),
                b"url" => element.url = Some(attr.unescape_value()?.to_string()),
                b"value" => element.value = Some(attr.unescape_value()?.to_string()),
                _ => (),
            }
        }

        Ok(element)
    }

    fn map_start_in_div<R>(
        &mut self,
        reader: &NsReader<R>,
        start: BytesStart,
    ) -> Result<Option<Event>> {
        let local_name = tri!(resolve_element(reader, start.name(), XHTML_NAMESPACE));

        if local_name.as_ref() == b"div" {
            self.nested_div_count += 1;
        }

        self.push_to_div(&format!("<{}>", str::from_utf8(&start)?));

        Ok(None)
    }

    fn map_start_enter_div<R>(
        &mut self,
        reader: &NsReader<R>,
        start: BytesStart,
    ) -> Result<Option<Event>> {
        let mut div_element = tri!(self.map_start_element(reader, start, XHTML_NAMESPACE));
        div_element.value = Some("<div xmlns=\"http://www.w3.org/1999/xhtml\">".into());

        self.in_div_element = Some(div_element);
        self.nested_div_count = 1;

        Ok(None)
    }

    fn map_empty_in_div<R>(
        &mut self,
        reader: &NsReader<R>,
        start: BytesStart,
    ) -> Result<Option<Event>> {
        // this checks if namespace is correct
        let _ = tri!(resolve_element(reader, start.name(), XHTML_NAMESPACE));
        for attr in start.attributes() {
            let _ = tri!(resolve_attribute(reader, attr?.key, XHTML_NAMESPACE));
        }

        self.push_to_div(&format!("<{}/>", str::from_utf8(&start)?));

        Ok(None)
    }

    fn map_end_in_div<R>(&mut self, reader: &NsReader<R>, end: BytesEnd) -> Result<Option<Event>> {
        let local_name = tri!(resolve_element(reader, end.name(), XHTML_NAMESPACE));

        if local_name.as_ref() == b"div" {
            self.nested_div_count -= 1;

            if self.nested_div_count == 0 {
                return self.map_end_leave_div();
            }
        }

        self.push_to_div(&format!("</{}>", str::from_utf8(local_name.as_ref())?));
        Ok(None)
    }

    fn map_end_leave_div(&mut self) -> Result<Option<Event>> {
        Ok(self
            .in_div_element
            .take()
            .map(|mut div_element| {
                div_element.value.as_mut().unwrap().push_str("</div>");
                div_element
            })
            .map(Event::EmptyElement))
    }

    fn push_to_div(&mut self, text: &str) {
        if let Some(div) = self.in_div_element.as_mut() {
            let xhtml = div.value.get_or_insert(String::new());
            xhtml.push_str(text);
        }
    }

    fn should_enter_div<R>(&self, reader: &NsReader<R>, start: &BytesStart) -> Result<bool> {
        // we doent use resolve_local_name because we dont care about namespace at this point
        // namespace is checked later
        let (_, local_name) = reader.resolve_element(start.name());

        Ok(local_name.as_ref() == b"div")
    }

    fn in_div(&self) -> bool {
        self.in_div_element.is_some()
    }
}

/// Trait for iterating over XML input.
pub trait Read {
    fn next_event(&mut self) -> Result<Event>;
}

pub struct IoRead<R: io::Read> {
    reader: NsReader<BufReader<R>>,
    buf: Vec<u8>,
    mapper: QuickXmlEventMapper,
}

impl<R: io::Read> IoRead<R> {
    pub fn new(reader: R) -> Self {
        let mut reader = NsReader::from_reader(BufReader::new(reader));
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

            if let Some(event) = tri!(event) {
                return Ok(event);
            }
        }
    }
}

pub struct SliceRead<'a> {
    reader: NsReader<&'a [u8]>,
    mapper: QuickXmlEventMapper,
}

impl<'a> SliceRead<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        let mut reader = NsReader::from_reader(slice);
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

            if let Some(event) = tri!(event) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_decl() {
        let xml = r#"<?xml version="1.0"?><root xmlns="http://hl7.org/fhir"></root>"#;
        let mut reader = StrRead::new(xml);

        let event = reader.next_event().unwrap();
        assert_eq!(event, Event::ElementStart(Element::new("root".to_string())));
    }

    #[test]
    fn test_invalid_xml_version() {
        let xml = r#"<?xml version="1.1"?><root></root>"#;
        let mut reader = StrRead::new(xml);

        let error = reader.next_event().unwrap_err();
        assert_eq!(
            error.to_string(),
            Error::InvalidXmlVersion("1.1".into()).to_string()
        );
    }

    #[test]
    fn test_invalid_xml_encoding() {
        let xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?><root></root>"#;
        let mut reader = StrRead::new(xml);

        let error = reader.next_event().unwrap_err();
        assert_eq!(
            error.to_string(),
            Error::InvalidXmlEncoding("ISO-8859-1".to_string()).to_string()
        );
    }

    #[test]
    fn test_invalid_xml_standalone() {
        let xml = r#"<?xml version="1.0" standalone="yes"?><root></root>"#;
        let mut reader = StrRead::new(xml);

        let error = reader.next_event().unwrap_err();
        assert_eq!(
            error.to_string(),
            Error::InvalidXmlStandalone("yes".into()).to_string()
        );
    }

    #[test]
    fn test_empty_element() {
        let xml = r#"<?xml version="1.0"?><root xmlns="http://hl7.org/fhir"><empty_element id="1" url="https://example.com"/></root>"#;
        let mut reader = StrRead::new(xml);

        let event = reader.next_event().unwrap();
        assert_eq!(event, Event::ElementStart(Element::new("root".to_string())));

        let mut element = Element::new("empty_element".to_string());
        element.id = Some("1".to_string());
        element.url = Some("https://example.com".to_string());

        let event = reader.next_event().unwrap();
        assert_eq!(event, Event::EmptyElement(element));
    }

    #[test]
    fn test_start_and_end_element() {
        let xml = r#"<?xml version="1.0"?><root xmlns="http://hl7.org/fhir"><element id="1" url="https://example.com">text</element></root>"#;
        let mut reader = StrRead::new(xml);

        let event = reader.next_event().unwrap();
        assert_eq!(event, Event::ElementStart(Element::new("root".to_string())));

        let mut element = Element::new("element".to_string());
        element.id = Some("1".to_string());
        element.url = Some("https://example.com".to_string());
        let event = reader.next_event().unwrap();
        assert_eq!(event, Event::ElementStart(element));
    }

    #[test]
    fn test_nested_div() {
        let xml = r#"<?xml version="1.0"?><root xmlns="http://hl7.org/fhir"><div xmlns="http://www.w3.org/1999/xhtml"><div></div></div></root>"#;
        let mut reader = StrRead::new(xml);

        let event = reader.next_event().unwrap();
        assert_eq!(event, Event::ElementStart(Element::new("root".to_string())));

        let mut element = Element::new("div".to_string());
        element.value =
            Some("<div xmlns=\"http://www.w3.org/1999/xhtml\"><div></div></div>".to_string());
        let event = reader.next_event().unwrap();
        assert_eq!(event, Event::EmptyElement(element));
    }

    #[test]
    fn test_invalid_xml_event_text_outside_if_div() {
        let xml = r#"<?xml version="1.0"?><root xmlns="http://hl7.org/fhir">text</root>"#;
        let mut reader = StrRead::new(xml);

        let event = reader.next_event().unwrap();
        assert_eq!(event, Event::ElementStart(Element::new("root".to_string())));

        let error = reader.next_event().unwrap_err();
        assert_eq!(
            error.to_string(),
            Error::InvalidXmlEvent("text outside of div").to_string()
        );
    }
}
