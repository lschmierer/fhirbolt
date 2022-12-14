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
}
