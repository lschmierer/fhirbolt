use std::{borrow::Cow, io};

use quick_xml::{
    events::{attributes::Attribute, BytesDecl, BytesEnd, BytesStart, Event as QuickXmlEvent},
    name::QName,
    Reader, Writer,
};

use crate::xml::{
    consts::{FHIR_NAMESPACE, XML_ENCODING, XML_VERSION},
    error::Result,
    event::{Element, Event},
};

fn match_escape(c: u8) -> Option<&'static [u8]> {
    match c {
        b'<' => Some(b"&#60;"),
        b'>' => Some(b"&#62;"),
        b'&' => Some(b"&#38;"),
        // single quotes do not need to be escaped
        // b'\'' => Some(b"&#39;"),
        b'"' => Some(b"&#34;"),
        _ => None,
    }
}

fn requires_escape(c: u8) -> bool {
    match_escape(c).is_some()
}

fn escape(s: &str) -> Cow<'_, [u8]> {
    if s.bytes().any(requires_escape) {
        let mut escaped = Vec::with_capacity(s.len());
        let mut total_remaining = s.as_bytes();

        // find the next occurence
        while let Some(n) = total_remaining.iter().position(|c| requires_escape(*c)) {
            let (start, remaining) = total_remaining.split_at(n);

            escaped.extend_from_slice(start);

            // unwrap is safe because we checked is_some for position n earlier
            let replacement = match_escape(remaining[0]).unwrap();
            escaped.extend_from_slice(replacement);

            total_remaining = &remaining[1..];
        }

        escaped.extend_from_slice(total_remaining);

        Cow::Owned(escaped)
    } else {
        Cow::Borrowed(s.as_bytes())
    }
}

fn push_attribute(start: &mut BytesStart, key: &'static [u8], value: Option<&str>) {
    if let Some(value) = value {
        start.push_attribute(Attribute {
            key: QName(key),
            value: escape(value),
        });
    }
}

pub trait Write {
    fn write_event(&mut self, event: Event) -> Result<()>;
}

pub struct IoWrite<W: io::Write> {
    writer: Writer<W>,
    decl_written: bool,
    fhir_namespace_written: bool,
    open_element_stack: Vec<Cow<'static, str>>,
}

impl<W: io::Write> IoWrite<W> {
    pub fn new(write: W) -> IoWrite<W> {
        IoWrite {
            writer: Writer::new_with_indent(write, b' ', 2),
            decl_written: false,
            fhir_namespace_written: false,
            open_element_stack: vec![],
        }
    }

    fn write_decl(&mut self) -> Result<()> {
        let decl = QuickXmlEvent::Decl(BytesDecl::new(XML_VERSION, Some(XML_ENCODING), None));

        self.writer.write_event(decl)?;

        Ok(())
    }

    fn write_div(&mut self, element: Element) -> Result<()> {
        let mut reader = Reader::from_str(element.value.as_ref().unwrap());
        reader.trim_text(true);

        loop {
            let event = reader.read_event()?;
            match event {
                QuickXmlEvent::Start(_)
                | QuickXmlEvent::End(_)
                | QuickXmlEvent::Empty(_)
                | QuickXmlEvent::Text(_) => self.writer.write_event(event)?,
                QuickXmlEvent::Eof => break,
                _ => (),
            }
        }

        Ok(())
    }

    fn write_start(&mut self, element: Element, empty: bool) -> Result<()> {
        let mut start = BytesStart::new(element.name.clone());

        if !self.fhir_namespace_written {
            push_attribute(&mut start, b"xmlns", Some(FHIR_NAMESPACE));
            self.fhir_namespace_written = true;
        }

        push_attribute(&mut start, b"id", element.id.as_deref());
        push_attribute(&mut start, b"url", element.url.as_deref());
        push_attribute(&mut start, b"value", element.value.as_deref());

        if empty {
            self.writer.write_event(QuickXmlEvent::Empty(start))?;
        } else {
            self.writer.write_event(QuickXmlEvent::Start(start))?;

            self.open_element_stack.push(element.name);
        }

        Ok(())
    }

    fn write_end(&mut self) -> Result<()> {
        let name = self.open_element_stack.pop().unwrap();

        self.writer
            .write_event(QuickXmlEvent::End(BytesEnd::new(name)))?;

        Ok(())
    }
}

impl<W: io::Write> Write for IoWrite<W> {
    fn write_event(&mut self, event: Event) -> Result<()> {
        if !self.decl_written {
            self.write_decl()?;
            self.decl_written = true;
        }

        match event {
            Event::ElementStart(e) => self.write_start(e, false)?,
            Event::ElementEnd => self.write_end()?,
            Event::EmptyElement(e) => self.write_start(e, true)?,
            Event::Div(e) => self.write_div(e)?,
            Event::Eof => self.writer.write_event(QuickXmlEvent::Eof)?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_escape() {
        assert_eq!(escape("<>&\"").as_ref(), b"&#60;&#62;&#38;&#34;");
        assert_eq!(escape("abc").as_ref(), b"abc");
        assert_eq!(escape(">=").as_ref(), b"&#62;=");
    }

    #[test]
    fn test_write_decl() {
        let mut io_write = IoWrite::new(Cursor::new(Vec::new()));
        io_write.write_event(Event::Eof).unwrap();
        let result = String::from_utf8(io_write.writer.into_inner().into_inner()).unwrap();
        assert_eq!(result, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    }

    #[test]
    fn test_write_start() {
        let mut io_write = IoWrite::new(Cursor::new(Vec::new()));
        let element = Element {
            name: Cow::Borrowed("test"),
            id: None,
            url: None,
            value: None,
        };
        io_write
            .write_event(Event::ElementStart(element.clone()))
            .unwrap();
        io_write.write_event(Event::Eof).unwrap();
        let result = String::from_utf8(io_write.writer.into_inner().into_inner()).unwrap();
        assert_eq!(
            result,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<test xmlns=\"http://hl7.org/fhir\">"
        );
    }

    #[test]
    fn test_write_end() {
        let mut io_write = IoWrite::new(Cursor::new(Vec::new()));
        let element = Element {
            name: Cow::Borrowed("test"),
            id: None,
            url: None,
            value: None,
        };
        io_write
            .write_event(Event::ElementStart(element.clone()))
            .unwrap();
        io_write.write_event(Event::ElementEnd).unwrap();
        io_write.write_event(Event::Eof).unwrap();
        let result = String::from_utf8(io_write.writer.into_inner().into_inner()).unwrap();
        assert_eq!(result, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<test xmlns=\"http://hl7.org/fhir\">\n</test>");
    }

    #[test]
    fn test_empty_element() {
        let mut io_write = IoWrite::new(Cursor::new(Vec::new()));
        let element = Element {
            name: Cow::Borrowed("test"),
            id: None,
            url: None,
            value: None,
        };
        io_write.write_event(Event::EmptyElement(element)).unwrap();
        io_write.write_event(Event::Eof).unwrap();
        let result = String::from_utf8(io_write.writer.into_inner().into_inner()).unwrap();
        assert_eq!(
            result,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<test xmlns=\"http://hl7.org/fhir\"/>"
        );
    }

    #[test]
    fn test_div_element() {
        let mut io_write = IoWrite::new(Cursor::new(Vec::new()));
        let element = Element {
            name: Cow::Borrowed("div"),
            id: None,
            url: None,
            value: Some("<div>Hello World!</div>".into()),
        };
        io_write.write_event(Event::Div(element)).unwrap();
        io_write.write_event(Event::Eof).unwrap();
        let result = String::from_utf8(io_write.writer.into_inner().into_inner()).unwrap();
        assert_eq!(
            result,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<div>Hello World!</div>"
        );
    }
}
