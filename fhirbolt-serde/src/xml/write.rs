use std::{borrow::Cow, io};

use crate::xml::{
    consts::{FHIR_NAMESPACE, XML_ENCODING, XML_VERSION},
    error::Result,
    event::{Element, Event},
};

fn escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '<' => escaped.push_str("&#60;"),
            '>' => escaped.push_str("&#62;"),
            '&' => escaped.push_str("&#38;"),
            // single quotes do not need to be escaped
            // '\'' => escaped.push_str("&#39;"),
            '"' => escaped.push_str("&#34;"),
            _ => escaped.push(c),
        }
    }

    escaped
}

pub trait Write {
    fn write_event(&mut self, event: Event) -> Result<()>;
}

pub struct IoWrite<W: io::Write> {
    writer: quick_xml::writer::Writer<W>,
    decl_written: bool,
    fhir_namespace_written: bool,
    open_element_stack: Vec<Cow<'static, str>>,
}

impl<W: io::Write> IoWrite<W> {
    pub fn new(write: W) -> IoWrite<W> {
        IoWrite {
            writer: quick_xml::writer::Writer::new_with_indent(write, b' ', 2),
            decl_written: false,
            fhir_namespace_written: false,
            open_element_stack: vec![],
        }
    }

    fn write_decl(&mut self) -> Result<()> {
        let decl = quick_xml::events::Event::Decl(quick_xml::events::BytesDecl::new(
            XML_VERSION,
            Some(XML_ENCODING),
            None,
        ));

        self.writer.write_event(decl)?;

        Ok(())
    }

    fn write_div(&mut self, element: Element) -> Result<()> {
        let mut reader = quick_xml::reader::Reader::from_str(element.value.as_ref().unwrap());
        reader.trim_text(true);

        loop {
            let event = reader.read_event()?;
            match event {
                quick_xml::events::Event::Start(_)
                | quick_xml::events::Event::End(_)
                | quick_xml::events::Event::Empty(_)
                | quick_xml::events::Event::Text(_) => self.writer.write_event(event)?,
                quick_xml::events::Event::Eof => break,
                _ => (),
            }
        }

        Ok(())
    }

    fn write_start(&mut self, element: Element, empty: bool) -> Result<()> {
        let mut start = quick_xml::events::BytesStart::new(element.name.clone());

        if !self.fhir_namespace_written {
            start.push_attribute((b"xmlns".as_slice(), FHIR_NAMESPACE.as_bytes()));
            self.fhir_namespace_written = true;
        }

        if let Some(id) = &element.id {
            start.push_attribute((b"id".as_slice(), escape(id).as_bytes()));
        }

        if let Some(url) = &element.url {
            start.push_attribute((b"url".as_slice(), escape(url).as_bytes()));
        }

        if let Some(value) = &element.value {
            start.push_attribute((b"value".as_slice(), escape(value).as_bytes()));
        }

        if empty {
            self.writer
                .write_event(quick_xml::events::Event::Empty(start))?;
        } else {
            self.writer
                .write_event(quick_xml::events::Event::Start(start))?;

            self.open_element_stack.push(element.name);
        }

        Ok(())
    }

    fn write_end(&mut self) -> Result<()> {
        let name = self.open_element_stack.pop().unwrap();

        self.writer.write_event(quick_xml::events::Event::End(
            quick_xml::events::BytesEnd::new(name),
        ))?;

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
            Event::Eof => self.writer.write_event(quick_xml::events::Event::Eof)?,
        }

        Ok(())
    }
}
