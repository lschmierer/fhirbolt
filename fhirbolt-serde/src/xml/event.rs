use std::borrow::Cow;

#[derive(Debug)]
pub enum Event {
    ElementStart(Element),
    ElementEnd,
    EmptyElement(Element),
    Div(Element),
    Eof,
}

#[derive(Default, Debug, Clone)]
pub struct Element {
    pub name: Cow<'static, str>,
    pub id: Option<String>,
    pub url: Option<String>,
    pub value: Option<String>,
}

impl Element {
    pub fn new<S: Into<Cow<'static, str>>>(name: S) -> Element {
        Element {
            name: name.into(),
            id: None,
            url: None,
            value: None,
        }
    }

    pub fn is_resource(&self) -> bool {
        self.name
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
    }

    pub fn resource_type(&self) -> Option<&str> {
        if self.is_resource() {
            Some(&self.name)
        } else {
            None
        }
    }
}
